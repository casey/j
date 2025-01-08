use {super::*, CompileErrorKind::*};

#[derive(Default)]
pub(crate) struct Analyzer<'run, 'src> {
  aliases: Table<'src, Alias<'src, Name<'src>>>,
  assignments: Vec<&'run Binding<'src, Expression<'src>>>,
  modules: Table<'src, Justfile<'src>>,
  recipes: Vec<&'run Recipe<'src, UnresolvedDependency<'src>>>,
  sets: Table<'src, Set<'src>>,
  unexports: HashSet<String>,
  warnings: Vec<Warning>,
}

impl<'run, 'src> Analyzer<'run, 'src> {
  pub(crate) fn analyze(
    asts: &'run HashMap<PathBuf, Ast<'src>>,
    doc: Option<String>,
    groups: &[String],
    loaded: &[PathBuf],
    name: Option<Name<'src>>,
    paths: &HashMap<PathBuf, PathBuf>,
    root: &Path,
  ) -> CompileResult<'src, Justfile<'src>> {
    Self::default().justfile(asts, doc, groups, loaded, name, paths, root)
  }

  fn justfile(
    mut self,
    asts: &'run HashMap<PathBuf, Ast<'src>>,
    doc: Option<String>,
    groups: &[String],
    loaded: &[PathBuf],
    name: Option<Name<'src>>,
    paths: &HashMap<PathBuf, PathBuf>,
    root: &Path,
  ) -> CompileResult<'src, Justfile<'src>> {
    let mut definitions = HashMap::new();
    let mut imports = HashSet::new();
    let mut unstable_features = BTreeSet::new();

    let mut stack = Vec::new();
    let ast = asts.get(root).unwrap();
    stack.push(ast);

    while let Some(ast) = stack.pop() {
      unstable_features.extend(&ast.unstable_features);

      for item in &ast.items {
        match item {
          Item::Alias(alias) => {
            Self::define(&mut definitions, alias.name, "alias", false)?;
            self.aliases.insert(alias.clone());
          }
          Item::Assignment(assignment) => {
            self.assignments.push(assignment);
          }
          Item::Comment(_) => (),
          Item::Import { absolute, .. } => {
            if let Some(absolute) = absolute {
              if imports.insert(absolute) {
                stack.push(asts.get(absolute).unwrap());
              }
            }
          }
          Item::Module {
            absolute,
            doc,
            groups,
            name,
            ..
          } => {
            if let Some(absolute) = absolute {
              Self::define(&mut definitions, *name, "module", false)?;
              self.modules.insert(Self::analyze(
                asts,
                doc.clone(),
                groups.as_slice(),
                loaded,
                Some(*name),
                paths,
                absolute,
              )?);
            }
          }
          Item::Recipe(recipe) => {
            if recipe.enabled() {
              Self::analyze_recipe(recipe)?;
              self.recipes.push(recipe);
            }
          }
          Item::Set(set) => {
            self.analyze_set(set)?;
            self.sets.insert(set.clone());
          }
          Item::Unexport { name } => {
            if !self.unexports.insert(name.lexeme().to_string()) {
              return Err(name.token.error(DuplicateUnexport {
                variable: name.lexeme(),
              }));
            }
          }
        }
      }

      self.warnings.extend(ast.warnings.iter().cloned());
    }

    let settings = Settings::from_table(self.sets);

    let mut assignments: Table<'src, Assignment<'src>> = Table::default();
    for assignment in self.assignments {
      let variable = assignment.name.lexeme();

      if !settings.allow_duplicate_variables && assignments.contains_key(variable) {
        return Err(assignment.name.token.error(DuplicateVariable { variable }));
      }

      if assignments.get(variable).map_or(true, |original| {
        assignment.file_depth <= original.file_depth
      }) {
        assignments.insert(assignment.clone());
      }

      if self.unexports.contains(variable) {
        return Err(assignment.name.token.error(ExportUnexported { variable }));
      }
    }

    AssignmentResolver::resolve_assignments(&assignments)?;

    let mut deduplicated_recipes = Table::<'src, UnresolvedRecipe<'src>>::default();
    for recipe in self.recipes {
      Self::define(
        &mut definitions,
        recipe.name,
        "recipe",
        settings.allow_duplicate_recipes,
      )?;

      if deduplicated_recipes
        .get(recipe.name.lexeme())
        .map_or(true, |original| recipe.file_depth <= original.file_depth)
      {
        deduplicated_recipes.insert(recipe.clone());
      }
    }

    let recipes = RecipeResolver::resolve_recipes(&assignments, &settings, deduplicated_recipes)?;

    let mut aliases = Table::new();
    while let Some(alias) = self.aliases.pop() {
      aliases.insert(Self::resolve_alias(&recipes, alias)?);
    }

    for recipe in recipes.values() {
      if recipe.attributes.contains(AttributeDiscriminant::Script) {
        unstable_features.insert(UnstableFeature::ScriptAttribute);
        break;
      }
    }

    if settings.script_interpreter.is_some() {
      unstable_features.insert(UnstableFeature::ScriptInterpreterSetting);
    }

    let source = root.to_owned();
    let root = paths.get(root).unwrap();

    Ok(Justfile {
      aliases,
      assignments,
      default: recipes
        .values()
        .filter(|recipe| recipe.name.path == root)
        .fold(None, |accumulator, next| match accumulator {
          None => Some(Rc::clone(next)),
          Some(previous) => Some(if previous.line_number() < next.line_number() {
            previous
          } else {
            Rc::clone(next)
          }),
        }),
      doc: doc.filter(|doc| !doc.is_empty()),
      groups: groups.into(),
      loaded: loaded.into(),
      modules: self.modules,
      name,
      recipes,
      settings,
      source,
      unexports: self.unexports,
      unstable_features,
      warnings: self.warnings,
      working_directory: ast.working_directory.clone(),
    })
  }

  fn define(
    definitions: &mut HashMap<&'src str, (&'static str, Name<'src>)>,
    name: Name<'src>,
    second_type: &'static str,
    duplicates_allowed: bool,
  ) -> CompileResult<'src> {
    if let Some((first_type, original)) = definitions.get(name.lexeme()) {
      if !(*first_type == second_type && duplicates_allowed) {
        let ((first_type, second_type), (original, redefinition)) = if name.line < original.line {
          ((second_type, *first_type), (name, *original))
        } else {
          ((*first_type, second_type), (*original, name))
        };

        return Err(redefinition.token.error(Redefinition {
          first_type,
          second_type,
          name: name.lexeme(),
          first: original.line,
        }));
      }
    }

    definitions.insert(name.lexeme(), (second_type, name));

    Ok(())
  }

  fn analyze_recipe(recipe: &UnresolvedRecipe<'src>) -> CompileResult<'src> {
    let mut parameters = BTreeSet::new();
    let mut passed_default = false;

    for parameter in &recipe.parameters {
      if parameters.contains(parameter.name.lexeme()) {
        return Err(parameter.name.token.error(DuplicateParameter {
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
            .token
            .error(RequiredParameterFollowsDefaultParameter {
              parameter: parameter.name.lexeme(),
            }),
        );
      }
    }

    let mut continued = false;
    for line in &recipe.body {
      if !recipe.is_script() && !continued {
        if let Some(Fragment::Text { token }) = line.fragments.first() {
          let text = token.lexeme();

          if text.starts_with(' ') || text.starts_with('\t') {
            return Err(token.error(ExtraLeadingWhitespace));
          }
        }
      }

      continued = line.is_continuation();
    }

    if !recipe.is_script() {
      if let Some(attribute) = recipe.attributes.get(AttributeDiscriminant::Extension) {
        return Err(recipe.name.error(InvalidAttribute {
          item_kind: "Recipe",
          item_name: recipe.name.lexeme(),
          attribute: attribute.clone(),
        }));
      }
    }

    Ok(())
  }

  fn analyze_set(&self, set: &Set<'src>) -> CompileResult<'src> {
    if let Some(original) = self.sets.get(set.name.lexeme()) {
      return Err(set.name.error(DuplicateSet {
        setting: original.name.lexeme(),
        first: original.name.line,
      }));
    }

    Ok(())
  }

  fn resolve_alias(
    recipes: &Table<'src, Rc<Recipe<'src>>>,
    alias: Alias<'src, Name<'src>>,
  ) -> CompileResult<'src, Alias<'src>> {
    // Make sure the target recipe exists
    match recipes.get(alias.target.lexeme()) {
      Some(target) => Ok(alias.resolve(Rc::clone(target))),
      None => Err(alias.name.token.error(UnknownAliasTarget {
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
    kind: Redefinition { first_type: "alias", second_type: "alias", name: "foo", first: 0 },
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
    offset: 34,
    line: 3,
    column: 0,
    width: 3,
    kind: Redefinition { first_type: "alias", second_type: "recipe", name: "foo", first: 2 },
  }

  analysis_error! {
    name: alias_shadows_recipe_after,
    input: "foo:\n  echo foo\nalias foo := bar\nbar:\n  echo bar",
    offset: 22,
    line: 2,
    column: 6,
    width: 3,
    kind: Redefinition { first_type: "recipe", second_type: "alias", name: "foo", first: 0 },
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
    kind:   Redefinition { first_type: "recipe", second_type: "recipe", name: "a", first: 0 },
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
