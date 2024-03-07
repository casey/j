use {
  super::*,
  std::process::{ExitStatus, Stdio},
};

/// Return a `Error::Signal` if the process was terminated by a signal,
/// otherwise return an `Error::UnknownFailure`
fn error_from_signal(recipe: &str, line_number: Option<usize>, exit_status: ExitStatus) -> Error {
  match Platform::signal_from_exit_status(exit_status) {
    Some(signal) => Error::Signal {
      recipe,
      line_number,
      signal,
    },
    None => Error::Unknown {
      recipe,
      line_number,
    },
  }
}

/// A recipe, e.g. `foo: bar baz`
#[derive(PartialEq, Debug, Clone, Serialize)]
pub(crate) struct Recipe<'src, D = Dependency<'src>> {
  pub(crate) attributes: BTreeSet<Attribute<'src>>,
  pub(crate) body: Vec<Line<'src>>,
  pub(crate) dependencies: Vec<D>,
  #[serde(skip)]
  pub(crate) depth: u32,
  pub(crate) doc: Option<&'src str>,
  #[serde(skip)]
  pub(crate) file_path: PathBuf,
  pub(crate) name: Name<'src>,
  pub(crate) namepath: Namepath<'src>,
  pub(crate) parameters: Vec<Parameter<'src>>,
  pub(crate) priors: usize,
  pub(crate) private: bool,
  pub(crate) quiet: bool,
  pub(crate) shebang: bool,
  #[serde(skip)]
  pub(crate) working_directory: PathBuf,
}

impl<'src, D> Recipe<'src, D> {
  pub(crate) fn argument_range(&self) -> RangeInclusive<usize> {
    self.min_arguments()..=self.max_arguments()
  }

  pub(crate) fn min_arguments(&self) -> usize {
    self
      .parameters
      .iter()
      .filter(|p| p.default.is_none() && p.kind != ParameterKind::Star)
      .count()
  }

  pub(crate) fn max_arguments(&self) -> usize {
    if self.parameters.iter().any(|p| p.kind.is_variadic()) {
      usize::MAX - 1
    } else {
      self.parameters.len()
    }
  }

  pub(crate) fn name(&self) -> &'src str {
    self.name.lexeme()
  }

  pub(crate) fn line_number(&self) -> usize {
    self.name.line
  }

  pub(crate) fn confirm(&self) -> RunResult<'src, bool> {
    for attribute in &self.attributes {
      if let Attribute::Confirm(prompt) = attribute {
        if let Some(prompt) = prompt {
          eprint!("{} ", prompt.cooked);
        } else {
          eprint!("Run recipe `{}`? ", self.name);
        }
        let mut line = String::new();
        std::io::stdin()
          .read_line(&mut line)
          .map_err(|io_error| Error::GetConfirmation { io_error })?;
        let line = line.trim().to_lowercase();
        return Ok(line == "y" || line == "yes");
      }
    }
    Ok(true)
  }

  pub(crate) fn check_can_be_default_recipe(&self) -> RunResult<'src, ()> {
    let min_arguments = self.min_arguments();
    if min_arguments > 0 {
      return Err(Error::DefaultRecipeRequiresArguments {
        recipe: self.name.lexeme(),
        min_arguments,
      });
    }

    Ok(())
  }

  pub(crate) fn is_public(&self) -> bool {
    !self.private && !self.attributes.contains(&Attribute::Private)
  }

  pub(crate) fn change_directory(&self) -> bool {
    !self.attributes.contains(&Attribute::NoCd)
  }

  pub(crate) fn enabled(&self) -> bool {
    let windows = self.attributes.contains(&Attribute::Windows);
    let linux = self.attributes.contains(&Attribute::Linux);
    let macos = self.attributes.contains(&Attribute::Macos);
    let unix = self.attributes.contains(&Attribute::Unix);

    (!windows && !linux && !macos && !unix)
      || (cfg!(target_os = "windows") && windows)
      || (cfg!(target_os = "linux") && (linux || unix))
      || (cfg!(target_os = "macos") && (macos || unix))
      || (cfg!(windows) && windows)
      || (cfg!(unix) && unix)
  }

  fn print_exit_message(&self) -> bool {
    !self.attributes.contains(&Attribute::NoExitMessage)
  }

  fn working_directory<'a>(&'a self, search: &'a Search) -> Option<&Path> {
    if self.change_directory() {
      Some(if self.depth > 0 {
        &self.working_directory
      } else {
        &search.working_directory
      })
    } else {
      None
    }
  }

  fn no_quiet(&self) -> bool {
    self.attributes.contains(&Attribute::NoQuiet)
  }

  fn cached(&self) -> bool {
    self.attributes.contains(&Attribute::Cached)
  }

  fn get_updated_cache_if_outdated<'run>(
    &self,
    context: &RecipeContext<'src, 'run>,
    mut evaluator: Evaluator<'src, 'run>,
  ) -> RunResult<'src, Option<(PathBuf, JustfileCache)>> {
    let cache_filename = {
      let project_dir = &context.search.working_directory;
      let project_name = project_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("UNKNOWN_PROJECT");
      let mut path_hash = blake3::Hasher::new();
      path_hash.update(project_dir.as_os_str().as_encoded_bytes());
      path_hash.update(context.search.justfile.as_os_str().as_encoded_bytes());
      let path_hash = &path_hash.finalize().to_hex()[..16];

      project_dir
        .join(".justcache")
        .join(format!("{project_name}-{path_hash}.json"))
    };

    let mut cache = if !cache_filename.exists() {
      JustfileCache::new(context.search)
    } else {
      fs::read_to_string(&cache_filename)
        .or(Err(()))
        .and_then(|cache| serde_json::from_str(&cache).or(Err(())))
        .or_else(|_| {
          Err(Error::CacheFileRead {
            cache_filename: Some(cache_filename.clone()),
          })
        })?
    };

    // TODO: Prevent double work/evaluating twice
    let mut recipe_hash = blake3::Hasher::new();
    for line in &self.body {
      recipe_hash.update(evaluator.evaluate_line(line, false)?.as_bytes());
    }
    let recipe_hash = recipe_hash.finalize().to_hex();

    if let Some(previous_run) = cache.recipes.get(self.name()) {
      if previous_run.body_hash == recipe_hash.as_str() {
        return Ok(None);
      }
    }
    cache.recipes.insert(
      self.name().to_string(),
      RecipeCache {
        body_hash: recipe_hash.to_string(),
      },
    );
    Ok(Some((cache_filename, cache)))
  }

  fn save_latest_cache(
    &self,
    cache_filename: PathBuf,
    cache: JustfileCache,
  ) -> RunResult<'src, ()> {
    let cache = serde_json::to_string(&cache).or_else(|_| {
      Err(Error::Internal {
        message: format!("Failed to serialize cache: {cache:?}"),
      })
    })?;

    cache_filename
      .parent()
      .ok_or_else(|| {
        io::Error::new(
          io::ErrorKind::Unsupported,
          format!(
            "Cannot create parent directory of {}",
            cache_filename.display()
          ),
        )
      })
      .and_then(|parent| fs::create_dir_all(parent))
      .and_then(|_| fs::write(&cache_filename, cache))
      .or_else(|io_error| {
        Err(Error::CacheFileWrite {
          cache_filename,
          io_error,
        })
      })
  }

  pub(crate) fn run<'run>(
    &self,
    context: &RecipeContext<'src, 'run>,
    dotenv: &BTreeMap<String, String>,
    scope: Scope<'src, 'run>,
    search: &'run Search,
    positional: &[String],
  ) -> RunResult<'src, ()> {
    let config = &context.config;

    let updated_cache = if !self.cached() {
      None
    } else {
      config.require_unstable("Cached recipes are currently unstable.")?;

      let evaluator = Evaluator::recipe_evaluator(config, dotenv, &scope, context.settings, search);
      match self.get_updated_cache_if_outdated(context, evaluator)? {
        None => {
          if config.dry_run
            || config.verbosity.loquacious()
            || !((context.settings.quiet && !self.no_quiet()) || config.verbosity.quiet())
          {
            let color = if config.highlight {
              config.color.command(config.command_color)
            } else {
              config.color
            };
            eprintln!(
              "{}===> Hash of recipe body of `{}` matches last run. Skipping...{}",
              color.prefix(),
              self.name,
              color.suffix()
            );
          }
          return Ok(());
        }
        Some(cache_info) => Some(cache_info),
      }
    };

    if config.verbosity.loquacious() {
      let color = config.color.stderr().banner();
      eprintln!(
        "{}===> Running recipe `{}`...{}",
        color.prefix(),
        self.name,
        color.suffix()
      );
    }

    let evaluator = Evaluator::recipe_evaluator(config, dotenv, &scope, context.settings, search);

    if self.shebang {
      self.run_shebang(context, dotenv, &scope, positional, config, evaluator)?;
    } else {
      self.run_linewise(context, dotenv, &scope, positional, config, evaluator)?;
    }

    if let Some((cache_filename, cache)) = updated_cache {
      self.save_latest_cache(cache_filename, cache)
    } else {
      Ok(())
    }
  }

  fn run_linewise<'run>(
    &self,
    context: &RecipeContext<'src, 'run>,
    dotenv: &BTreeMap<String, String>,
    scope: &Scope<'src, 'run>,
    positional: &[String],
    config: &Config,
    mut evaluator: Evaluator<'src, 'run>,
  ) -> RunResult<'src, ()> {
    let mut lines = self.body.iter().peekable();
    let mut line_number = self.line_number() + 1;
    loop {
      if lines.peek().is_none() {
        return Ok(());
      }
      let mut evaluated = String::new();
      let mut continued = false;
      let quiet_line = lines.peek().map_or(false, |line| line.is_quiet());
      let infallible_line = lines.peek().map_or(false, |line| line.is_infallible());

      let comment_line =
        context.settings.ignore_comments && lines.peek().map_or(false, |line| line.is_comment());

      loop {
        if lines.peek().is_none() {
          break;
        }
        let line = lines.next().unwrap();
        line_number += 1;
        if !comment_line {
          evaluated += &evaluator.evaluate_line(line, continued)?;
        }
        if line.is_continuation() && !comment_line {
          continued = true;
          evaluated.pop();
        } else {
          break;
        }
      }

      if comment_line {
        continue;
      }

      let mut command = evaluated.as_str();

      let sigils = usize::from(infallible_line) + usize::from(quiet_line);

      command = &command[sigils..];

      if command.is_empty() {
        continue;
      }

      if config.dry_run
        || config.verbosity.loquacious()
        || !((quiet_line ^ self.quiet)
          || (context.settings.quiet && !self.no_quiet())
          || config.verbosity.quiet())
      {
        let color = if config.highlight {
          config.color.command(config.command_color)
        } else {
          config.color
        };
        eprintln!("{}", color.stderr().paint(command));
      }

      if config.dry_run {
        continue;
      }

      let mut cmd = context.settings.shell_command(config);

      if let Some(working_directory) = self.working_directory(context.search) {
        cmd.current_dir(working_directory);
      }

      cmd.arg(command);

      if context.settings.positional_arguments {
        cmd.arg(self.name.lexeme());
        cmd.args(positional);
      }

      if config.verbosity.quiet() {
        cmd.stderr(Stdio::null());
        cmd.stdout(Stdio::null());
      }

      cmd.export(context.settings, dotenv, scope);

      match InterruptHandler::guard(|| cmd.status()) {
        Ok(exit_status) => {
          if let Some(code) = exit_status.code() {
            if code != 0 && !infallible_line {
              return Err(Error::Code {
                recipe: self.name(),
                line_number: Some(line_number),
                code,
                print_message: self.print_exit_message(),
              });
            }
          } else {
            return Err(error_from_signal(
              self.name(),
              Some(line_number),
              exit_status,
            ));
          }
        }
        Err(io_error) => {
          return Err(Error::Io {
            recipe: self.name(),
            io_error,
          });
        }
      };
    }
  }

  pub(crate) fn run_shebang<'run>(
    &self,
    context: &RecipeContext<'src, 'run>,
    dotenv: &BTreeMap<String, String>,
    scope: &Scope<'src, 'run>,
    positional: &[String],
    config: &Config,
    mut evaluator: Evaluator<'src, 'run>,
  ) -> RunResult<'src, ()> {
    let mut evaluated_lines = vec![];
    for line in &self.body {
      evaluated_lines.push(evaluator.evaluate_line(line, false)?);
    }

    if config.verbosity.loud() && (config.dry_run || self.quiet) {
      for line in &evaluated_lines {
        eprintln!("{line}");
      }
    }

    if config.dry_run {
      return Ok(());
    }

    let shebang_line = evaluated_lines.first().ok_or_else(|| Error::Internal {
      message: "evaluated_lines was empty".to_owned(),
    })?;

    let shebang = Shebang::new(shebang_line).ok_or_else(|| Error::Internal {
      message: format!("bad shebang line: {shebang_line}"),
    })?;

    let mut tempdir_builder = tempfile::Builder::new();
    tempdir_builder.prefix("just-");
    let tempdir = match &context.settings.tempdir {
      Some(tempdir) => tempdir_builder.tempdir_in(context.search.working_directory.join(tempdir)),
      None => tempdir_builder.tempdir(),
    }
    .map_err(|error| Error::TmpdirIo {
      recipe: self.name(),
      io_error: error,
    })?;
    let mut path = tempdir.path().to_path_buf();
    path.push(shebang.script_filename(self.name()));

    {
      let mut f = fs::File::create(&path).map_err(|error| Error::TmpdirIo {
        recipe: self.name(),
        io_error: error,
      })?;
      let mut text = String::new();

      if shebang.include_shebang_line() {
        text += &evaluated_lines[0];
      } else {
        text += "\n";
      }

      text += "\n";
      // add blank lines so that lines in the generated script have the same line
      // number as the corresponding lines in the justfile
      for _ in 1..(self.line_number() + 2) {
        text += "\n";
      }
      for line in &evaluated_lines[1..] {
        text += line;
        text += "\n";
      }

      if config.verbosity.grandiloquent() {
        eprintln!("{}", config.color.doc().stderr().paint(&text));
      }

      f.write_all(text.as_bytes())
        .map_err(|error| Error::TmpdirIo {
          recipe: self.name(),
          io_error: error,
        })?;
    }

    // make script executable
    Platform::set_execute_permission(&path).map_err(|error| Error::TmpdirIo {
      recipe: self.name(),
      io_error: error,
    })?;

    // create command to run script
    let mut command =
      Platform::make_shebang_command(&path, self.working_directory(context.search), shebang)
        .map_err(|output_error| Error::Cygpath {
          recipe: self.name(),
          output_error,
        })?;

    if context.settings.positional_arguments {
      command.args(positional);
    }

    command.export(context.settings, dotenv, scope);

    // run it!
    match InterruptHandler::guard(|| command.status()) {
      Ok(exit_status) => exit_status.code().map_or_else(
        || Err(error_from_signal(self.name(), None, exit_status)),
        |code| {
          if code == 0 {
            Ok(())
          } else {
            Err(Error::Code {
              recipe: self.name(),
              line_number: None,
              code,
              print_message: self.print_exit_message(),
            })
          }
        },
      ),
      Err(io_error) => Err(Error::Shebang {
        recipe: self.name(),
        command: shebang.interpreter.to_owned(),
        argument: shebang.argument.map(String::from),
        io_error,
      }),
    }
  }
}

impl<'src, D: Display> ColorDisplay for Recipe<'src, D> {
  fn fmt(&self, f: &mut Formatter, color: Color) -> Result<(), fmt::Error> {
    if let Some(doc) = self.doc {
      writeln!(f, "# {doc}")?;
    }

    for attribute in &self.attributes {
      writeln!(f, "[{attribute}]")?;
    }

    if self.quiet {
      write!(f, "@{}", self.name)?;
    } else {
      write!(f, "{}", self.name)?;
    }

    for parameter in &self.parameters {
      write!(f, " {}", parameter.color_display(color))?;
    }
    write!(f, ":")?;

    for (i, dependency) in self.dependencies.iter().enumerate() {
      if i == self.priors {
        write!(f, " &&")?;
      }

      write!(f, " {dependency}")?;
    }

    for (i, line) in self.body.iter().enumerate() {
      if i == 0 {
        writeln!(f)?;
      }
      for (j, fragment) in line.fragments.iter().enumerate() {
        if j == 0 {
          write!(f, "    ")?;
        }
        match fragment {
          Fragment::Text { token } => write!(f, "{}", token.lexeme())?,
          Fragment::Interpolation { expression, .. } => write!(f, "{{{{ {expression} }}}}")?,
        }
      }
      if i + 1 < self.body.len() {
        writeln!(f)?;
      }
    }
    Ok(())
  }
}

impl<'src, D> Keyed<'src> for Recipe<'src, D> {
  fn key(&self) -> &'src str {
    self.name.lexeme()
  }
}
