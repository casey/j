use super::*;

#[derive(Debug, Eq, PartialEq, IntoStaticStr, Display, Copy, Clone, EnumString)]
#[strum(serialize_all = "kebab_case")]
pub(crate) enum Keyword {
  Alias,
  AllowDuplicateRecipes,
  AllowDuplicateVariables,
  Assert,
  DotenvFilename,
  DotenvLoad,
  DotenvPath,
  DotenvRequired,
  Else,
  Export,
  Fallback,
  False,
  If,
  IgnoreComments,
  Import,
  Mod,
  NoExitMessage,
  PositionalArguments,
  Quiet,
  ScriptInterpreter,
  Set,
  Shell,
  Tempdir,
  True,
  Unexport,
  Unstable,
  WindowsPowershell,
  WindowsShell,
  WorkingDirectory,
  X,
}

impl Keyword {
  pub(crate) fn from_lexeme(lexeme: &str) -> Option<Keyword> {
    lexeme.parse().ok()
  }

  pub(crate) fn lexeme(self) -> &'static str {
    self.into()
  }
}

impl<'a> PartialEq<&'a str> for Keyword {
  fn eq(&self, other: &&'a str) -> bool {
    self.lexeme() == *other
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keyword_case() {
    assert_eq!(Keyword::X.lexeme(), "x");
    assert_eq!(Keyword::IgnoreComments.lexeme(), "ignore-comments");
  }
}
