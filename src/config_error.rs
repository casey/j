use super::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)), context(suffix(Context)))]
pub(crate) enum ConfigError {
  #[snafu(display("Failed to get current directory: {}", source))]
  CurrentDir { source: io::Error },
  #[snafu(display(
    "Internal config error, this may indicate a bug in just: {message} \
     consider filing an issue: https://github.com/casey/just/issues/new",
  ))]
  Internal { message: String },
  #[snafu(display("Invalid module path `{}`", path.join(" ")))]
  ModulePath { path: Vec<String> },
  #[snafu(display("Failed to parse request: {source}"))]
  RequestParse { source: serde_json::Error },
  #[snafu(display(
    "Path-prefixed recipes may not be used with `--working-directory` or `--justfile`."
  ))]
  SearchDirConflict,
  #[snafu(display(
    "`--{}` used with unexpected {}: {}",
    subcommand.to_lowercase(),
    Count("argument", arguments.len()),
    List::and_ticked(arguments)
  ))]
  SubcommandArguments {
    subcommand: &'static str,
    arguments: Vec<String>,
  },
  #[snafu(display(
      "`--{}` used with unexpected overrides: {}",
      subcommand.to_lowercase(),
      List::and_ticked(overrides.iter().map(|(key, value)| format!("{key}={value}"))),
  ))]
  SubcommandOverrides {
    subcommand: &'static str,
    overrides: BTreeMap<String, String>,
  },
  #[snafu(display(
      "`--{}` used with unexpected overrides: {}; and arguments: {}",
      subcommand.to_lowercase(),
      List::and_ticked(overrides.iter().map(|(key, value)| format!("{key}={value}"))),
      List::and_ticked(arguments)))
  ]
  SubcommandOverridesAndArguments {
    subcommand: &'static str,
    overrides: BTreeMap<String, String>,
    arguments: Vec<String>,
  },
}

impl ConfigError {
  pub(crate) fn internal(message: impl Into<String>) -> Self {
    Self::Internal {
      message: message.into(),
    }
  }
}
