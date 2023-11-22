use std::sync::Arc;
use {super::*, CompileErrorKind::*};

pub(crate) struct RecipeResolver<'src: 'run, 'run> {
  unresolved_recipes: Table<'src, UnresolvedRecipe<'src>>,
  resolved_recipes: Table<'src, Arc<Recipe<'src>>>,
  assignments: &'run Table<'src, Assignment<'src>>,
}

impl<'src: 'run, 'run> RecipeResolver<'src, 'run> {
  pub(crate) fn resolve_recipes(
    unresolved_recipes: Table<'src, UnresolvedRecipe<'src>>,
    assignments: &Table<'src, Assignment<'src>>,
  ) -> CompileResult<'src, Table<'src, Arc<Recipe<'src>>>> {
    let mut resolver = RecipeResolver {
      resolved_recipes: Table::new(),
      unresolved_recipes,
      assignments,
    };

    while let Some(unresolved) = resolver.unresolved_recipes.pop() {
      resolver.resolve_recipe(&mut Vec::new(), unresolved)?;
    }

    for recipe in resolver.resolved_recipes.values() {
      for parameter in &recipe.parameters {
        if let Some(expression) = &parameter.default {
          for variable in expression.variables() {
            resolver.resolve_variable(&variable, &[])?;
          }
        }
      }

      for dependency in &recipe.dependencies {
        for argument in &dependency.arguments {
          for variable in argument.variables() {
            resolver.resolve_variable(&variable, &recipe.parameters)?;
          }
        }
      }

      for line in &recipe.body {
        for fragment in &line.fragments {
          if let Fragment::Interpolation { expression, .. } = fragment {
            for variable in expression.variables() {
              resolver.resolve_variable(&variable, &recipe.parameters)?;
            }
          }
        }
      }
    }

    Ok(resolver.resolved_recipes)
  }

  fn resolve_variable(
    &self,
    variable: &Token<'src>,
    parameters: &[Parameter],
  ) -> CompileResult<'src, ()> {
    let name = variable.lexeme();
    let undefined =
      !self.assignments.contains_key(name) && !parameters.iter().any(|p| p.name.lexeme() == name);

    if undefined {
      return Err(variable.error(UndefinedVariable { variable: name }));
    }

    Ok(())
  }

  fn resolve_recipe(
    &mut self,
    stack: &mut Vec<&'src str>,
    recipe: UnresolvedRecipe<'src>,
  ) -> CompileResult<'src, Arc<Recipe<'src>>> {
    if let Some(resolved) = self.resolved_recipes.get(recipe.name()) {
      return Ok(Arc::clone(resolved));
    }

    stack.push(recipe.name());

    let mut dependencies: Vec<Arc<Recipe>> = Vec::new();
    for dependency in &recipe.dependencies {
      let name = dependency.recipe.lexeme();

      if let Some(resolved) = self.resolved_recipes.get(name) {
        // dependency already resolved
        dependencies.push(Arc::clone(resolved));
      } else if stack.contains(&name) {
        let first = stack[0];
        stack.push(first);
        return Err(
          dependency.recipe.error(CircularRecipeDependency {
            recipe: recipe.name(),
            circle: stack
              .iter()
              .skip_while(|name| **name != dependency.recipe.lexeme())
              .copied()
              .collect(),
          }),
        );
      } else if let Some(unresolved) = self.unresolved_recipes.remove(name) {
        // resolve unresolved dependency
        dependencies.push(self.resolve_recipe(stack, unresolved)?);
      } else {
        // dependency is unknown
        return Err(dependency.recipe.error(UnknownDependency {
          recipe: recipe.name(),
          unknown: name,
        }));
      }
    }

    stack.pop();

    let resolved = Arc::new(recipe.resolve(dependencies)?);
    self.resolved_recipes.insert(Arc::clone(&resolved));
    Ok(resolved)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  analysis_error! {
    name:   circular_recipe_dependency,
    input:  "a: b\nb: a",
    offset: 8,
    line:   1,
    column: 3,
    width:  1,
    kind:   CircularRecipeDependency{recipe: "b", circle: vec!["a", "b", "a"]},
  }

  analysis_error! {
    name:   self_recipe_dependency,
    input:  "a: a",
    offset: 3,
    line:   0,
    column: 3,
    width:  1,
    kind:   CircularRecipeDependency{recipe: "a", circle: vec!["a", "a"]},
  }

  analysis_error! {
    name:   unknown_dependency,
    input:  "a: b",
    offset: 3,
    line:   0,
    column: 3,
    width:  1,
    kind:   UnknownDependency{recipe: "a", unknown: "b"},
  }

  analysis_error! {
    name:   unknown_interpolation_variable,
    input:  "x:\n {{   hello}}",
    offset: 9,
    line:   1,
    column: 6,
    width:  5,
    kind:   UndefinedVariable{variable: "hello"},
  }

  analysis_error! {
    name:   unknown_second_interpolation_variable,
    input:  "wtf:=\"x\"\nx:\n echo\n foo {{wtf}} {{ lol }}",
    offset: 34,
    line:   3,
    column: 16,
    width:  3,
    kind:   UndefinedVariable{variable: "lol"},
  }

  analysis_error! {
    name:   unknown_variable_in_default,
    input:  "a f=foo:",
    offset: 4,
    line:   0,
    column: 4,
    width:  3,
    kind:   UndefinedVariable{variable: "foo"},
  }

  analysis_error! {
    name:   unknown_variable_in_dependency_argument,
    input:  "bar x:\nfoo: (bar baz)",
    offset: 17,
    line:   1,
    column: 10,
    width:  3,
    kind:   UndefinedVariable{variable: "baz"},
  }
}
