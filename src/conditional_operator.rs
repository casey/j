use super::*;

/// A conditional expression operator.
#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum ConditionalOperator {
  /// `==`
  Equality,
  /// `!=`
  Inequality,
  /// `=~`
  RegexMatch,
  /// `!~`
  RegexMismatch,
}

impl Display for ConditionalOperator {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Equality => write!(f, "=="),
      Self::Inequality => write!(f, "!="),
      Self::RegexMatch => write!(f, "=~"),
      Self::RegexMismatch => write!(f, "!~"),
    }
  }
}
