use std::sync::Arc;
use {super::*, CompileErrorKind::*};

#[derive(Default)]
pub(crate) struct Analyzer<'src> {
  assignments: Table<'src, Assignment<'src>>,
  aliases: Table<'src, Alias<'src, Name<'src>>>,
  sets: Table<'src, Set<'src>>,
}

impl<'src> Analyzer<'src> {
  pub(crate) fn analyze(
    asts: &HashMap<PathBuf, Ast<'src>>,
    root: &Path,
  ) -> CompileResult<'src, Justfile<'src>> {
    Analyzer::default().justfile(asts, root)
  }

  fn justfile(
    mut self,
    asts: &HashMap<PathBuf, Ast<'src>>,
    root: &Path,
  ) -> CompileResult<'src, Justfile<'src>> {
    let mut recipes = Vec::new();

    let mut stack = Vec::new();
    stack.push(asts.get(root).unwrap());

    let mut warnings = Vec::new();

    while let Some(ast) = stack.pop() {
      for item in &ast.items {
        match item {
          Item::Alias(alias) => {
            self.analyze_alias(alias)?;
            self.aliases.insert(alias.clone());
          }
          Item::Assignment(assignment) => {
            self.analyze_assignment(assignment)?;
            self.assignments.insert(assignment.clone());
          }
          Item::Comment(_) => (),
          Item::Recipe(recipe) => {
            if recipe.enabled() {
              Self::analyze_recipe(recipe)?;
              recipes.push(recipe);
            }
          }
          Item::Set(set) => {
            self.analyze_set(set)?;
            self.sets.insert(set.clone());
          }
          Item::Include { absolute, .. } => {
            stack.push(asts.get(absolute.as_ref().unwrap()).unwrap());
          }
        }
      }

      warnings.extend(ast.warnings.iter().cloned());
    }

    let settings = Settings::from_setting_iter(self.sets.into_iter().map(|(_, set)| set.value));

    let mut recipe_table: Table<'src, UnresolvedRecipe<'src>> = Table::default();

    AssignmentResolver::resolve_assignments(&self.assignments)?;

    for recipe in recipes {
      if let Some(original) = recipe_table.get(recipe.name.lexeme()) {
        if !settings.allow_duplicate_recipes {
          return Err(recipe.name.token().error(DuplicateRecipe {
            recipe: original.name(),
            first: original.line_number(),
          }));
        }
      }
      recipe_table.insert(recipe.clone());
    }

    let recipes = RecipeResolver::resolve_recipes(recipe_table, &self.assignments)?;

    let mut aliases = Table::new();
    while let Some(alias) = self.aliases.pop() {
      aliases.insert(Self::resolve_alias(&recipes, alias)?);
    }

    Ok(Justfile {
      first: recipes
        .values()
        .fold(None, |accumulator, next| match accumulator {
          None => Some(Arc::clone(next)),
          Some(previous) => Some(if previous.line_number() < next.line_number() {
            previous
          } else {
            Arc::clone(next)
          }),
        }),
      aliases,
      assignments: self.assignments,
      recipes,
      settings,
      warnings,
    })
  }

  fn analyze_recipe(recipe: &UnresolvedRecipe<'src>) -> CompileResult<'src, ()> {
    let mut parameters = BTreeSet::new();
    let mut passed_default = false;

    for parameter in &recipe.parameters {
      if parameters.contains(parameter.name.lexeme()) {
        return Err(parameter.name.token().error(DuplicateParameter {
          recipe: recipe.name.lexeme(),
          parameter: parameter.name.lexeme(),
        }));
      }
      parameters.insert(parameter.name.lexeme());

      if parameter.default.is_some() {
        passed_default = true;
      } else if passed_default {
        return Err(
          parameter
            .name
            .token()
            .error(RequiredParameterFollowsDefaultParameter {
              parameter: parameter.name.lexeme(),
            }),
        );
      }
    }

    let mut continued = false;
    for line in &recipe.body {
      if !recipe.shebang && !continued {
        if let Some(Fragment::Text { token }) = line.fragments.first() {
          let text = token.lexeme();

          if text.starts_with(' ') || text.starts_with('\t') {
            return Err(token.error(ExtraLeadingWhitespace));
          }
        }
      }

      continued = line.is_continuation();
    }

    Ok(())
  }

  fn analyze_assignment(&self, assignment: &Assignment<'src>) -> CompileResult<'src, ()> {
    if self.assignments.contains_key(assignment.name.lexeme()) {
      return Err(assignment.name.token().error(DuplicateVariable {
        variable: assignment.name.lexeme(),
      }));
    }
    Ok(())
  }

  fn analyze_alias(&self, alias: &Alias<'src, Name<'src>>) -> CompileResult<'src, ()> {
    let name = alias.name.lexeme();

    if let Some(original) = self.aliases.get(name) {
      return Err(alias.name.token().error(DuplicateAlias {
        alias: name,
        first: original.line_number(),
      }));
    }

    for attr in &alias.attributes {
      if *attr != Attribute::Private {
        return Err(alias.name.token().error(AliasInvalidAttribute {
          alias: name,
          attr: *attr,
        }));
      }
    }

    Ok(())
  }

  fn analyze_set(&self, set: &Set<'src>) -> CompileResult<'src, ()> {
    if let Some(original) = self.sets.get(set.name.lexeme()) {
      return Err(set.name.error(DuplicateSet {
        setting: original.name.lexeme(),
        first: original.name.line,
      }));
    }

    Ok(())
  }

  fn resolve_alias(
    recipes: &Table<'src, Arc<Recipe<'src>>>,
    alias: Alias<'src, Name<'src>>,
  ) -> CompileResult<'src, Alias<'src>> {
    let token = alias.name.token();
    // Make sure the alias doesn't conflict with any recipe
    if let Some(recipe) = recipes.get(alias.name.lexeme()) {
      return Err(token.error(AliasShadowsRecipe {
        alias: alias.name.lexeme(),
        recipe_line: recipe.line_number(),
      }));
    }

    // Make sure the target recipe exists
    match recipes.get(alias.target.lexeme()) {
      Some(target) => Ok(alias.resolve(Arc::clone(target))),
      None => Err(token.error(UnknownAliasTarget {
        alias: alias.name.lexeme(),
        target: alias.target.lexeme(),
      })),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  analysis_error! {
    name: duplicate_alias,
    input: "alias foo := bar\nalias foo := baz",
    offset: 23,
    line: 1,
    column: 6,
    width: 3,
    kind: DuplicateAlias { alias: "foo", first: 0 },
  }

  analysis_error! {
    name: unknown_alias_target,
    input: "alias foo := bar\n",
    offset: 6,
    line: 0,
    column: 6,
    width: 3,
    kind: UnknownAliasTarget {alias: "foo", target: "bar"},
  }

  analysis_error! {
    name: alias_shadows_recipe_before,
    input: "bar: \n  echo bar\nalias foo := bar\nfoo:\n  echo foo",
    offset: 23,
    line: 2,
    column: 6,
    width: 3,
    kind: AliasShadowsRecipe {alias: "foo", recipe_line: 3},
  }

  analysis_error! {
    name: alias_shadows_recipe_after,
    input: "foo:\n  echo foo\nalias foo := bar\nbar:\n  echo bar",
    offset: 22,
    line: 2,
    column: 6,
    width: 3,
    kind: AliasShadowsRecipe { alias: "foo", recipe_line: 0 },
  }

  analysis_error! {
    name:   required_after_default,
    input:  "hello arg='foo' bar:",
    offset:  16,
    line:   0,
    column: 16,
    width:  3,
    kind:   RequiredParameterFollowsDefaultParameter{parameter: "bar"},
  }

  analysis_error! {
    name:   duplicate_parameter,
    input:  "a b b:",
    offset:  4,
    line:   0,
    column: 4,
    width:  1,
    kind:   DuplicateParameter{recipe: "a", parameter: "b"},
  }

  analysis_error! {
    name:   duplicate_variadic_parameter,
    input:  "a b +b:",
    offset: 5,
    line:   0,
    column: 5,
    width:  1,
    kind:   DuplicateParameter{recipe: "a", parameter: "b"},
  }

  analysis_error! {
    name:   duplicate_recipe,
    input:  "a:\nb:\na:",
    offset:  6,
    line:   2,
    column: 0,
    width:  1,
    kind:   DuplicateRecipe{recipe: "a", first: 0},
  }

  analysis_error! {
    name:   duplicate_variable,
    input:  "a := \"0\"\na := \"0\"",
    offset: 9,
    line:   1,
    column: 0,
    width:  1,
    kind:   DuplicateVariable{variable: "a"},
  }

  analysis_error! {
    name:   extra_whitespace,
    input:  "a:\n blah\n  blarg",
    offset:  10,
    line:   2,
    column: 1,
    width:  6,
    kind:   ExtraLeadingWhitespace,
  }
}
