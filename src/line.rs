use super::*;

/// A single line in a recipe body, consisting of any number of `Fragment`s.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub(crate) struct Line<'src> {
  pub(crate) fragments: Vec<Fragment<'src>>,
  #[serde(skip)]
  pub(crate) number: usize,
}

impl Line<'_> {
  fn first(&self) -> Option<&str> {
    if let Fragment::Text { token } = self.fragments.first()? {
      Some(token.lexeme())
    } else {
      None
    }
  }

  pub(crate) fn is_comment(&self) -> bool {
    self.first().is_some_and(|text| text.starts_with('#'))
  }

  pub(crate) fn is_continuation(&self) -> bool {
    matches!(
      self.fragments.last(),
      Some(Fragment::Text { token }) if token.lexeme().ends_with('\\'),
    )
  }

  pub(crate) fn is_empty(&self) -> bool {
    self.fragments.is_empty()
  }

  pub(crate) fn is_infallible(&self) -> bool {
    self
      .first()
      .is_some_and(|text| text.starts_with('-') || text.starts_with("@-"))
  }

  pub(crate) fn is_quiet(&self) -> bool {
    self
      .first()
      .is_some_and(|text| text.starts_with('@') || text.starts_with("-@"))
  }

  pub(crate) fn is_shebang(&self) -> bool {
    self.first().is_some_and(|text| text.starts_with("#!"))
  }
}
