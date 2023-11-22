use super::*;
use std::sync::Arc;

pub(crate) type UnresolvedRecipe<'src> = Recipe<'src, UnresolvedDependency<'src>>;

impl<'src> UnresolvedRecipe<'src> {
  pub(crate) fn resolve(
    self,
    resolved: Vec<Arc<Recipe<'src>>>,
  ) -> CompileResult<'src, Recipe<'src>> {
    assert_eq!(
      self.dependencies.len(),
      resolved.len(),
      "UnresolvedRecipe::resolve: dependency count not equal to resolved count: {} != {}",
      self.dependencies.len(),
      resolved.len()
    );

    for (unresolved, resolved) in self.dependencies.iter().zip(&resolved) {
      assert_eq!(unresolved.recipe.lexeme(), resolved.name.lexeme());
      if !resolved
        .argument_range()
        .contains(&unresolved.arguments.len())
      {
        return Err(
          unresolved
            .recipe
            .error(CompileErrorKind::DependencyArgumentCountMismatch {
              dependency: unresolved.recipe.lexeme(),
              found: unresolved.arguments.len(),
              min: resolved.min_arguments(),
              max: resolved.max_arguments(),
            }),
        );
      }
    }

    let dependencies = self
      .dependencies
      .into_iter()
      .zip(resolved)
      .map(|(unresolved, resolved)| Dependency {
        recipe: resolved,
        arguments: unresolved.arguments,
      })
      .collect();

    Ok(Recipe {
      body: self.body,
      doc: self.doc,
      name: self.name,
      parameters: self.parameters,
      private: self.private,
      quiet: self.quiet,
      shebang: self.shebang,
      priors: self.priors,
      attributes: self.attributes,
      dependencies,
    })
  }
}
