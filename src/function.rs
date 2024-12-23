use {
  super::*,
  heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToUpperCamelCase,
  },
  rand::{seq::SliceRandom, thread_rng},
  semver::{Version, VersionReq},
  std::collections::HashSet,
  Function::*,
};

pub(crate) enum Function {
  Nullary(fn(Context) -> FunctionResult),
  Unary(fn(Context, &str) -> FunctionResult),
  UnaryOpt(fn(Context, &str, Option<&str>) -> FunctionResult),
  UnaryPlus(fn(Context, &str, &[String]) -> FunctionResult),
  Binary(fn(Context, &str, &str) -> FunctionResult),
  BinaryPlus(fn(Context, &str, &str, &[String]) -> FunctionResult),
  Ternary(fn(Context, &str, &str, &str) -> FunctionResult),
}

pub(crate) struct Context<'src: 'run, 'run> {
  pub(crate) evaluator: &'run Evaluator<'src, 'run>,
  pub(crate) name: Name<'src>,
}

impl<'src: 'run, 'run> Context<'src, 'run> {
  pub(crate) fn new(evaluator: &'run Evaluator<'src, 'run>, name: Name<'src>) -> Self {
    Self { evaluator, name }
  }
}

pub(crate) fn get(name: &str) -> Option<Function> {
  let name = if let Some(prefix) = name.strip_suffix("_dir") {
    format!("{prefix}_directory")
  } else if let Some(prefix) = name.strip_suffix("_dir_native") {
    format!("{prefix}_directory_native")
  } else {
    name.into()
  };

  let function = match name.as_str() {
    "absolute_path" => Unary(absolute_path),
    "append" => Binary(append),
    "arch" => Nullary(arch),
    "blake3" => Unary(blake3),
    "blake3_file" => Unary(blake3_file),
    "cache_directory" => Nullary(|_| dir("cache", dirs::cache_dir)),
    "canonicalize" => Unary(canonicalize),
    "capitalize" => Unary(capitalize),
    "choose" => Binary(choose),
    "clean" => Unary(clean),
    "config_directory" => Nullary(|_| dir("config", dirs::config_dir)),
    "config_local_directory" => Nullary(|_| dir("local config", dirs::config_local_dir)),
    "data_directory" => Nullary(|_| dir("data", dirs::data_dir)),
    "data_local_directory" => Nullary(|_| dir("local data", dirs::data_local_dir)),
    "datetime" => Unary(datetime),
    "datetime_utc" => Unary(datetime_utc),
    "encode_uri_component" => Unary(encode_uri_component),
    "env" => UnaryOpt(env),
    "env_var" => Unary(env_var),
    "env_var_or_default" => Binary(env_var_or_default),
    "error" => Unary(error),
    "executable_directory" => Nullary(|_| dir("executable", dirs::executable_dir)),
    "extension" => Unary(extension),
    "file_name" => Unary(file_name),
    "file_stem" => Unary(file_stem),
    "home_directory" => Nullary(|_| dir("home", dirs::home_dir)),
    "invocation_directory" => Nullary(invocation_directory),
    "invocation_directory_native" => Nullary(invocation_directory_native),
    "is_dependency" => Nullary(is_dependency),
    "join" => BinaryPlus(join),
    "just_executable" => Nullary(just_executable),
    "just_pid" => Nullary(just_pid),
    "justfile" => Nullary(justfile),
    "justfile_directory" => Nullary(justfile_directory),
    "kebabcase" => Unary(kebabcase),
    "lowercamelcase" => Unary(lowercamelcase),
    "lowercase" => Unary(lowercase),
    "module_directory" => Nullary(module_directory),
    "module_file" => Nullary(module_file),
    "num_cpus" => Nullary(num_cpus),
    "os" => Nullary(os),
    "os_family" => Nullary(os_family),
    "parent_directory" => Unary(parent_directory),
    "path_exists" => Unary(path_exists),
    "prepend" => Binary(prepend),
    "quote" => Unary(quote),
    "read" => Unary(read),
    "replace" => Ternary(replace),
    "replace_regex" => Ternary(replace_regex),
    "semver_matches" => Binary(semver_matches),
    "sha256" => Unary(sha256),
    "sha256_file" => Unary(sha256_file),
    "shell" => UnaryPlus(shell),
    "shoutykebabcase" => Unary(shoutykebabcase),
    "shoutysnakecase" => Unary(shoutysnakecase),
    "snakecase" => Unary(snakecase),
    "source_directory" => Nullary(source_directory),
    "source_file" => Nullary(source_file),
    "style" => Unary(style),
    "titlecase" => Unary(titlecase),
    "trim" => Unary(trim),
    "trim_end" => Unary(trim_end),
    "trim_end_match" => Binary(trim_end_match),
    "trim_end_matches" => Binary(trim_end_matches),
    "trim_start" => Unary(trim_start),
    "trim_start_match" => Binary(trim_start_match),
    "trim_start_matches" => Binary(trim_start_matches),
    "uppercamelcase" => Unary(uppercamelcase),
    "uppercase" => Unary(uppercase),
    "uuid" => Nullary(uuid),
    "without_extension" => Unary(without_extension),
    _ => return None,
  };
  Some(function)
}

impl Function {
  pub(crate) fn argc(&self) -> RangeInclusive<usize> {
    match *self {
      Nullary(_) => 0..=0,
      Unary(_) => 1..=1,
      UnaryOpt(_) => 1..=2,
      UnaryPlus(_) => 1..=usize::MAX,
      Binary(_) => 2..=2,
      BinaryPlus(_) => 2..=usize::MAX,
      Ternary(_) => 3..=3,
    }
  }
}

fn absolute_path(context: Context, path: &str) -> FunctionResult {
  let abs_path_unchecked = context
    .evaluator
    .context
    .working_directory()
    .join(path)
    .lexiclean();
  match abs_path_unchecked.to_str() {
    Some(absolute_path) => Ok(absolute_path.to_owned()),
    None => Err(format!(
      "Working directory is not valid unicode: {}",
      context.evaluator.context.search.working_directory.display()
    )),
  }
}

fn append(_context: Context, suffix: &str, s: &str) -> FunctionResult {
  Ok(
    s.split_whitespace()
      .map(|s| format!("{s}{suffix}"))
      .collect::<Vec<String>>()
      .join(" "),
  )
}

fn arch(_context: Context) -> FunctionResult {
  Ok(target::arch().to_owned())
}

fn blake3(_context: Context, s: &str) -> FunctionResult {
  Ok(blake3::hash(s.as_bytes()).to_string())
}

fn blake3_file(context: Context, path: &str) -> FunctionResult {
  let path = context.evaluator.context.working_directory().join(path);
  let mut hasher = blake3::Hasher::new();
  hasher
    .update_mmap_rayon(&path)
    .map_err(|err| format!("Failed to hash `{}`: {err}", path.display()))?;
  Ok(hasher.finalize().to_string())
}

fn canonicalize(context: Context, path: &str) -> FunctionResult {
  let canonical = std::fs::canonicalize(context.evaluator.context.working_directory().join(path))
    .map_err(|err| format!("I/O error canonicalizing path: {err}"))?;

  canonical.to_str().map(str::to_string).ok_or_else(|| {
    format!(
      "Canonical path is not valid unicode: {}",
      canonical.display(),
    )
  })
}

fn capitalize(_context: Context, s: &str) -> FunctionResult {
  let mut capitalized = String::new();
  for (i, c) in s.chars().enumerate() {
    if i == 0 {
      capitalized.extend(c.to_uppercase());
    } else {
      capitalized.extend(c.to_lowercase());
    }
  }
  Ok(capitalized)
}

fn choose(_context: Context, n: &str, alphabet: &str) -> FunctionResult {
  if alphabet.is_empty() {
    return Err("empty alphabet".into());
  }

  let mut chars = HashSet::<char>::with_capacity(alphabet.len());

  for c in alphabet.chars() {
    if !chars.insert(c) {
      return Err(format!("alphabet contains repeated character `{c}`"));
    }
  }

  let alphabet = alphabet.chars().collect::<Vec<char>>();

  let n = n
    .parse::<usize>()
    .map_err(|err| format!("failed to parse `{n}` as positive integer: {err}"))?;

  let mut rng = thread_rng();

  Ok((0..n).map(|_| alphabet.choose(&mut rng).unwrap()).collect())
}

fn clean(_context: Context, path: &str) -> FunctionResult {
  Ok(Path::new(path).lexiclean().to_str().unwrap().to_owned())
}

fn dir(name: &'static str, f: fn() -> Option<PathBuf>) -> FunctionResult {
  match f() {
    Some(path) => path
      .as_os_str()
      .to_str()
      .map(str::to_string)
      .ok_or_else(|| {
        format!(
          "unable to convert {name} directory path to string: {}",
          path.display(),
        )
      }),
    None => Err(format!("{name} directory not found")),
  }
}

fn datetime(_context: Context, format: &str) -> FunctionResult {
  Ok(chrono::Local::now().format(format).to_string())
}

fn datetime_utc(_context: Context, format: &str) -> FunctionResult {
  Ok(chrono::Utc::now().format(format).to_string())
}

fn encode_uri_component(_context: Context, s: &str) -> FunctionResult {
  static PERCENT_ENCODE: percent_encoding::AsciiSet = percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'!')
    .remove(b'~')
    .remove(b'*')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')');
  Ok(percent_encoding::utf8_percent_encode(s, &PERCENT_ENCODE).to_string())
}

fn env(context: Context, key: &str, default: Option<&str>) -> FunctionResult {
  match default {
    Some(val) => env_var_or_default(context, key, val),
    None => env_var(context, key),
  }
}

fn env_var(context: Context, key: &str) -> FunctionResult {
  use std::env::VarError::*;

  if let Some(value) = context.evaluator.context.dotenv.get(key) {
    return Ok(value.clone());
  }

  match env::var(key) {
    Err(NotPresent) => Err(format!("environment variable `{key}` not present")),
    Err(NotUnicode(os_string)) => Err(format!(
      "environment variable `{key}` not unicode: {os_string:?}"
    )),
    Ok(value) => Ok(value),
  }
}

fn env_var_or_default(context: Context, key: &str, default: &str) -> FunctionResult {
  use std::env::VarError::*;

  if let Some(value) = context.evaluator.context.dotenv.get(key) {
    return Ok(value.clone());
  }

  match env::var(key) {
    Err(NotPresent) => Ok(default.to_owned()),
    Err(NotUnicode(os_string)) => Err(format!(
      "environment variable `{key}` not unicode: {os_string:?}"
    )),
    Ok(value) => Ok(value),
  }
}

fn error(_context: Context, message: &str) -> FunctionResult {
  Err(message.to_owned())
}

fn extension(_context: Context, path: &str) -> FunctionResult {
  Utf8Path::new(path)
    .extension()
    .map(str::to_owned)
    .ok_or_else(|| format!("Could not extract extension from `{path}`"))
}

fn file_name(_context: Context, path: &str) -> FunctionResult {
  Utf8Path::new(path)
    .file_name()
    .map(str::to_owned)
    .ok_or_else(|| format!("Could not extract file name from `{path}`"))
}

fn file_stem(_context: Context, path: &str) -> FunctionResult {
  Utf8Path::new(path)
    .file_stem()
    .map(str::to_owned)
    .ok_or_else(|| format!("Could not extract file stem from `{path}`"))
}

fn invocation_directory(context: Context) -> FunctionResult {
  Platform::convert_native_path(
    &context.evaluator.context.search.working_directory,
    &context.evaluator.context.config.invocation_directory,
  )
  .map_err(|e| format!("Error getting shell path: {e}"))
}

fn invocation_directory_native(context: Context) -> FunctionResult {
  context
    .evaluator
    .context
    .config
    .invocation_directory
    .to_str()
    .map(str::to_owned)
    .ok_or_else(|| {
      format!(
        "Invocation directory is not valid unicode: {}",
        context
          .evaluator
          .context
          .config
          .invocation_directory
          .display()
      )
    })
}

fn is_dependency(context: Context) -> FunctionResult {
  Ok(context.evaluator.is_dependency.to_string())
}

fn prepend(_context: Context, prefix: &str, s: &str) -> FunctionResult {
  Ok(
    s.split_whitespace()
      .map(|s| format!("{prefix}{s}"))
      .collect::<Vec<String>>()
      .join(" "),
  )
}

fn join(_context: Context, base: &str, with: &str, and: &[String]) -> FunctionResult {
  let mut result = Utf8Path::new(base).join(with);
  for arg in and {
    result.push(arg);
  }
  Ok(result.to_string())
}

fn just_executable(_context: Context) -> FunctionResult {
  let exe_path =
    env::current_exe().map_err(|e| format!("Error getting current executable: {e}"))?;

  exe_path.to_str().map(str::to_owned).ok_or_else(|| {
    format!(
      "Executable path is not valid unicode: {}",
      exe_path.display()
    )
  })
}

fn just_pid(_context: Context) -> FunctionResult {
  Ok(std::process::id().to_string())
}

fn justfile(context: Context) -> FunctionResult {
  context
    .evaluator
    .context
    .search
    .justfile
    .to_str()
    .map(str::to_owned)
    .ok_or_else(|| {
      format!(
        "Justfile path is not valid unicode: {}",
        context.evaluator.context.search.justfile.display()
      )
    })
}

fn justfile_directory(context: Context) -> FunctionResult {
  let justfile_directory = context
    .evaluator
    .context
    .search
    .justfile
    .parent()
    .ok_or_else(|| {
      format!(
        "Could not resolve justfile directory. Justfile `{}` had no parent.",
        context.evaluator.context.search.justfile.display()
      )
    })?;

  justfile_directory
    .to_str()
    .map(str::to_owned)
    .ok_or_else(|| {
      format!(
        "Justfile directory is not valid unicode: {}",
        justfile_directory.display()
      )
    })
}

fn kebabcase(_context: Context, s: &str) -> FunctionResult {
  Ok(s.to_kebab_case())
}

fn lowercamelcase(_context: Context, s: &str) -> FunctionResult {
  Ok(s.to_lower_camel_case())
}

fn lowercase(_context: Context, s: &str) -> FunctionResult {
  Ok(s.to_lowercase())
}

fn module_directory(context: Context) -> FunctionResult {
  let module_directory = context.evaluator.context.module.source.parent().unwrap();
  module_directory.to_str().map(str::to_owned).ok_or_else(|| {
    format!(
      "Module directory is not valid unicode: {}",
      module_directory.display(),
    )
  })
}

fn module_file(context: Context) -> FunctionResult {
  let module_file = &context.evaluator.context.module.source;
  module_file.to_str().map(str::to_owned).ok_or_else(|| {
    format!(
      "Module file path is not valid unicode: {}",
      module_file.display(),
    )
  })
}

fn num_cpus(_context: Context) -> FunctionResult {
  let num = num_cpus::get();
  Ok(num.to_string())
}

fn os(_context: Context) -> FunctionResult {
  Ok(target::os().to_owned())
}

fn os_family(_context: Context) -> FunctionResult {
  Ok(target::family().to_owned())
}

fn parent_directory(_context: Context, path: &str) -> FunctionResult {
  Utf8Path::new(path)
    .parent()
    .map(Utf8Path::to_string)
    .ok_or_else(|| format!("Could not extract parent directory from `{path}`"))
}

fn path_exists(context: Context, path: &str) -> FunctionResult {
  Ok(
    context
      .evaluator
      .context
      .working_directory()
      .join(path)
      .exists()
      .to_string(),
  )
}

fn quote(_context: Context, s: &str) -> FunctionResult {
  Ok(format!("'{}'", s.replace('\'', "'\\''")))
}

fn read(context: Context, filename: &str) -> FunctionResult {
  fs::read_to_string(context.evaluator.context.working_directory().join(filename))
    .map_err(|err| format!("I/O error reading `{filename}`: {err}"))
}

fn replace(_context: Context, s: &str, from: &str, to: &str) -> FunctionResult {
  Ok(s.replace(from, to))
}

fn replace_regex(_context: Context, s: &str, regex: &str, replacement: &str) -> FunctionResult {
  Ok(
    Regex::new(regex)
      .map_err(|err| err.to_string())?
      .replace_all(s, replacement)
      .to_string(),
  )
}

fn sha256(_context: Context, s: &str) -> FunctionResult {
  use sha2::{Digest, Sha256};
  let mut hasher = Sha256::new();
  hasher.update(s);
  let hash = hasher.finalize();
  Ok(format!("{hash:x}"))
}

fn sha256_file(context: Context, path: &str) -> FunctionResult {
  use sha2::{Digest, Sha256};
  let path = context.evaluator.context.working_directory().join(path);
  let mut hasher = Sha256::new();
  let mut file =
    fs::File::open(&path).map_err(|err| format!("Failed to open `{}`: {err}", path.display()))?;
  std::io::copy(&mut file, &mut hasher)
    .map_err(|err| format!("Failed to read `{}`: {err}", path.display()))?;
  let hash = hasher.finalize();
  Ok(format!("{hash:x}"))
}

fn shell(context: Context, command: &str, args: &[String]) -> FunctionResult {
  let args = iter::once(command)
    .chain(args.iter().map(String::as_str))
    .collect::<Vec<&str>>();

  context
    .evaluator
    .run_command(command, &args)
    .map_err(|output_error| output_error.to_string())
}

fn shoutykebabcase(_context: Context, s: &str) -> FunctionResult {
  Ok(s.to_shouty_kebab_case())
}

fn shoutysnakecase(_context: Context, s: &str) -> FunctionResult {
  Ok(s.to_shouty_snake_case())
}

fn snakecase(_context: Context, s: &str) -> FunctionResult {
  Ok(s.to_snake_case())
}

fn source_directory(context: Context) -> FunctionResult {
  context
    .evaluator
    .context
    .search
    .justfile
    .parent()
    .unwrap()
    .join(context.name.token.path)
    .parent()
    .unwrap()
    .to_str()
    .map(str::to_owned)
    .ok_or_else(|| {
      format!(
        "Source file path not valid unicode: {}",
        context.name.token.path.display(),
      )
    })
}

fn source_file(context: Context) -> FunctionResult {
  context
    .evaluator
    .context
    .search
    .justfile
    .parent()
    .unwrap()
    .join(context.name.token.path)
    .to_str()
    .map(str::to_owned)
    .ok_or_else(|| {
      format!(
        "Source file path not valid unicode: {}",
        context.name.token.path.display(),
      )
    })
}

fn style(context: Context, s: &str) -> FunctionResult {
  match s {
    "command" => Ok(
      Color::always()
        .command(context.evaluator.context.config.command_color)
        .prefix()
        .to_string(),
    ),
    "error" => Ok(Color::always().error().prefix().to_string()),
    "warning" => Ok(Color::always().warning().prefix().to_string()),
    _ => Err(format!("unknown style: `{s}`")),
  }
}

fn titlecase(_context: Context, s: &str) -> FunctionResult {
  Ok(s.to_title_case())
}

fn trim(_context: Context, s: &str) -> FunctionResult {
  Ok(s.trim().to_owned())
}

fn trim_end(_context: Context, s: &str) -> FunctionResult {
  Ok(s.trim_end().to_owned())
}

fn trim_end_match(_context: Context, s: &str, pat: &str) -> FunctionResult {
  Ok(s.strip_suffix(pat).unwrap_or(s).to_owned())
}

fn trim_end_matches(_context: Context, s: &str, pat: &str) -> FunctionResult {
  Ok(s.trim_end_matches(pat).to_owned())
}

fn trim_start(_context: Context, s: &str) -> FunctionResult {
  Ok(s.trim_start().to_owned())
}

fn trim_start_match(_context: Context, s: &str, pat: &str) -> FunctionResult {
  Ok(s.strip_prefix(pat).unwrap_or(s).to_owned())
}

fn trim_start_matches(_context: Context, s: &str, pat: &str) -> FunctionResult {
  Ok(s.trim_start_matches(pat).to_owned())
}

fn uppercamelcase(_context: Context, s: &str) -> FunctionResult {
  Ok(s.to_upper_camel_case())
}

fn uppercase(_context: Context, s: &str) -> FunctionResult {
  Ok(s.to_uppercase())
}

fn uuid(_context: Context) -> FunctionResult {
  Ok(uuid::Uuid::new_v4().to_string())
}

fn without_extension(_context: Context, path: &str) -> FunctionResult {
  let parent = Utf8Path::new(path)
    .parent()
    .ok_or_else(|| format!("Could not extract parent from `{path}`"))?;

  let file_stem = Utf8Path::new(path)
    .file_stem()
    .ok_or_else(|| format!("Could not extract file stem from `{path}`"))?;

  Ok(parent.join(file_stem).to_string())
}

/// Check whether a string processes properly as semver (e.x. "0.1.0")
/// and matches a given semver requirement (e.x. ">=0.1.0")
fn semver_matches(_context: Context, version: &str, requirement: &str) -> FunctionResult {
  Ok(
    requirement
      .parse::<VersionReq>()
      .map_err(|err| format!("invalid semver requirement: {err}"))?
      .matches(
        &version
          .parse::<Version>()
          .map_err(|err| format!("invalid semver version: {err}"))?,
      )
      .to_string(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dir_not_found() {
    assert_eq!(dir("foo", || None).unwrap_err(), "foo directory not found");
  }

  #[cfg(unix)]
  #[test]
  fn dir_not_unicode() {
    use std::os::unix::ffi::OsStrExt;
    assert_eq!(
      dir("foo", || Some(
        std::ffi::OsStr::from_bytes(b"\xe0\x80\x80").into()
      ))
      .unwrap_err(),
      "unable to convert foo directory path to string: ���",
    );
  }
}
