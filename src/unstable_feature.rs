use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Ord, Eq, PartialOrd)]
pub(crate) enum UnstableFeature {
  FormatSubcommand,
  GlobImport,
  ScriptAttribute,
  ScriptInterpreterSetting,
}

impl Display for UnstableFeature {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::FormatSubcommand => write!(f, "The `--fmt` command is currently unstable."),
      Self::GlobImport => write!(f, "Glob imports are currently unstable"),
      Self::ScriptAttribute => write!(f, "The `[script]` attribute is currently unstable."),
      Self::ScriptInterpreterSetting => {
        write!(f, "The `script-interpreter` setting is currently unstable.")
      }
    }
  }
}
