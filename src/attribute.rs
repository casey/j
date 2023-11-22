use super::*;

#[derive(
  EnumString, PartialEq, Debug, Copy, Clone, Serialize, Ord, PartialOrd, Eq, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Attribute {
  Confirm,
  Linux,
  Macos,
  NoCd,
  NoExitMessage,
  Private,
  Parallel,
  Unix,
  Windows,
}

impl Attribute {
  pub(crate) fn from_name(name: Name) -> Option<Attribute> {
    name.lexeme().parse().ok()
  }

  pub(crate) fn to_str(self) -> &'static str {
    self.into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn to_str() {
    assert_eq!(Attribute::NoExitMessage.to_str(), "no-exit-message");
  }
}
