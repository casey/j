use super::*;
use std::sync::Arc;

#[derive(PartialEq, Debug, Serialize)]
pub(crate) struct Dependency<'src> {
  pub(crate) arguments: Vec<Expression<'src>>,
  #[serde(serialize_with = "keyed::serialize")]
  pub(crate) recipe: Arc<Recipe<'src>>,
}

impl<'src> Display for Dependency<'src> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    if self.arguments.is_empty() {
      write!(f, "{}", self.recipe.name())
    } else {
      write!(f, "({}", self.recipe.name())?;

      for argument in &self.arguments {
        write!(f, " {argument}")?;
      }

      write!(f, ")")
    }
  }
}
