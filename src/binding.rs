use super::*;

/// A binding of `name` to `value`
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Binding<'src, V = String> {
  /// Module depth where binding appears
  pub(crate) depth: u32,
  /// Export binding as an environment variable to child processes
  pub(crate) export: bool,
  /// Binding name
  pub(crate) name: Name<'src>,
  /// Binding value
  pub(crate) value: V,
}

impl<'src, V> Keyed<'src> for Binding<'src, V> {
  fn key(&self) -> &'src str {
    self.name.lexeme()
  }
}
