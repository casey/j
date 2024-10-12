use {super::*, serde::Serialize};

#[derive(Debug)]
struct Invocation<'src: 'run, 'run> {
  arguments: Vec<&'run str>,
  module: &'run Justfile<'src>,
  recipe: &'run Recipe<'src>,
  scope: &'run Scope<'src, 'run>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Justfile<'src> {
  pub(crate) aliases: Table<'src, Alias<'src>>,
  pub(crate) assignments: Table<'src, Assignment<'src>>,
  pub(crate) doc: Option<String>,
  #[serde(rename = "first", serialize_with = "keyed::serialize_option")]
  pub(crate) default: Option<Rc<Recipe<'src>>>,
  #[serde(skip)]
  pub(crate) loaded: Vec<PathBuf>,
  pub(crate) groups: Vec<String>,
  pub(crate) modules: Table<'src, Justfile<'src>>,
  #[serde(skip)]
  pub(crate) name: Option<Name<'src>>,
  pub(crate) recipes: Table<'src, Rc<Recipe<'src>>>,
  pub(crate) settings: Settings<'src>,
  #[serde(skip)]
  pub(crate) source: PathBuf,
  pub(crate) unexports: HashSet<String>,
  #[serde(skip)]
  pub(crate) unstable_features: BTreeSet<UnstableFeature>,
  pub(crate) warnings: Vec<Warning>,
  #[serde(skip)]
  pub(crate) working_directory: PathBuf,
}

impl<'src> Justfile<'src> {
  fn find_suggestion(
    input: &str,
    candidates: impl Iterator<Item = Suggestion<'src>>,
  ) -> Option<Suggestion<'src>> {
    candidates
      .map(|suggestion| (edit_distance(input, suggestion.name), suggestion))
      .filter(|(distance, _suggestion)| *distance < 3)
      .min_by_key(|(distance, _suggestion)| *distance)
      .map(|(_distance, suggestion)| suggestion)
  }

  pub(crate) fn suggest_recipe(&self, input: &str) -> Option<Suggestion<'src>> {
    Self::find_suggestion(
      input,
      self
        .recipes
        .keys()
        .map(|name| Suggestion { name, target: None })
        .chain(self.aliases.iter().map(|(name, alias)| Suggestion {
          name,
          target: Some(alias.target.name.lexeme()),
        })),
    )
  }

  pub(crate) fn suggest_variable(&self, input: &str) -> Option<Suggestion<'src>> {
    Self::find_suggestion(
      input,
      self
        .assignments
        .keys()
        .map(|name| Suggestion { name, target: None }),
    )
  }

  pub(crate) fn run(
    &self,
    config: &Config,
    search: &Search,
    overrides: &BTreeMap<String, String>,
    arguments: &[String],
  ) -> RunResult<'src> {
    let unknown_overrides = overrides
      .keys()
      .filter(|name| !self.assignments.contains_key(name.as_str()))
      .cloned()
      .collect::<Vec<String>>();

    if !unknown_overrides.is_empty() {
      return Err(Error::UnknownOverrides {
        overrides: unknown_overrides,
      });
    }

    let dotenv = if config.load_dotenv {
      load_dotenv(config, &self.settings, &search.working_directory)?
    } else {
      BTreeMap::new()
    };

    let root = Scope::root();

    let scope = Evaluator::evaluate_assignments(config, &dotenv, self, overrides, &root, search)?;

    match &config.subcommand {
      Subcommand::Command {
        binary, arguments, ..
      } => {
        let mut command = if config.shell_command {
          let mut command = self.settings.shell_command(config);
          command.arg(binary);
          command
        } else {
          Command::new(binary)
        };

        command.args(arguments);

        command.current_dir(&search.working_directory);

        let scope = scope.child();

        command.export(&self.settings, &dotenv, &scope, &self.unexports);

        let status = InterruptHandler::guard(|| command.status()).map_err(|io_error| {
          Error::CommandInvoke {
            binary: binary.clone(),
            arguments: arguments.clone(),
            io_error,
          }
        })?;

        if !status.success() {
          return Err(Error::CommandStatus {
            binary: binary.clone(),
            arguments: arguments.clone(),
            status,
          });
        };

        return Ok(());
      }
      Subcommand::Evaluate { variable, .. } => {
        if let Some(variable) = variable {
          if let Some(value) = scope.value(variable) {
            print!("{value}");
          } else {
            return Err(Error::EvalUnknownVariable {
              suggestion: self.suggest_variable(variable),
              variable: variable.clone(),
            });
          }
        } else {
          let width = scope.names().fold(0, |max, name| name.len().max(max));

          for binding in scope.bindings() {
            if !binding.private {
              println!(
                "{0:1$} := \"{2}\"",
                binding.name.lexeme(),
                width,
                binding.value
              );
            }
          }
        }

        return Ok(());
      }
      _ => {}
    }

    let arguments = arguments.iter().map(String::as_str).collect::<Vec<&str>>();

    let groups = ArgumentParser::parse_arguments(self, &arguments)?;

    let arena: Arena<Scope> = Arena::new();
    let mut invocations = Vec::<Invocation>::new();
    let mut scopes = BTreeMap::new();

    for group in &groups {
      invocations.push(self.invocation(
        &arena,
        &group.arguments,
        config,
        &dotenv,
        &scope,
        &group.path,
        0,
        &mut scopes,
        search,
      )?);
    }

    if config.one && invocations.len() > 1 {
      return Err(Error::ExcessInvocations {
        invocations: invocations.len(),
      });
    }

    let mut ran = Ran::default();
    for invocation in invocations {
      let context = ExecutionContext {
        config,
        dotenv: &dotenv,
        module: invocation.module,
        scope: invocation.scope,
        search,
      };

      Self::run_recipe(
        &invocation
          .arguments
          .iter()
          .copied()
          .map(str::to_string)
          .collect::<Vec<String>>(),
        &context,
        &mut ran,
        invocation.recipe,
        false,
      )?;
    }

    Ok(())
  }

  pub(crate) fn check_unstable(&self, config: &Config) -> RunResult<'src> {
    if let Some(&unstable_feature) = self.unstable_features.iter().next() {
      config.require_unstable(self, unstable_feature)?;
    }

    for module in self.modules.values() {
      module.check_unstable(config)?;
    }

    Ok(())
  }

  pub(crate) fn get_alias(&self, name: &str) -> Option<&Alias<'src>> {
    self.aliases.get(name)
  }

  pub(crate) fn get_recipe(&self, name: &str) -> Option<&Recipe<'src>> {
    self
      .recipes
      .get(name)
      .map(Rc::as_ref)
      .or_else(|| self.aliases.get(name).map(|alias| alias.target.as_ref()))
  }

  fn invocation<'run>(
    &'run self,
    arena: &'run Arena<Scope<'src, 'run>>,
    arguments: &[&'run str],
    config: &'run Config,
    dotenv: &'run BTreeMap<String, String>,
    parent: &'run Scope<'src, 'run>,
    path: &'run [String],
    position: usize,
    scopes: &mut BTreeMap<&'run [String], &'run Scope<'src, 'run>>,
    search: &'run Search,
  ) -> RunResult<'src, Invocation<'src, 'run>> {
    if position + 1 == path.len() {
      let recipe = self.get_recipe(&path[position]).unwrap();
      Ok(Invocation {
        arguments: arguments.into(),
        module: self,
        recipe,
        scope: parent,
      })
    } else {
      let module = self.modules.get(&path[position]).unwrap();

      let scope = if let Some(scope) = scopes.get(&path[..position]) {
        scope
      } else {
        let scope = Evaluator::evaluate_assignments(
          config,
          dotenv,
          module,
          &BTreeMap::new(),
          parent,
          search,
        )?;
        let scope = arena.alloc(scope);
        scopes.insert(path, scope);
        scopes.get(path).unwrap()
      };

      module.invocation(
        arena,
        arguments,
        config,
        dotenv,
        scope,
        path,
        position + 1,
        scopes,
        search,
      )
    }
  }

  pub(crate) fn is_submodule(&self) -> bool {
    self.name.is_some()
  }

  pub(crate) fn name(&self) -> &'src str {
    self.name.map(|name| name.lexeme()).unwrap_or_default()
  }

  fn run_recipe(
    arguments: &[String],
    context: &ExecutionContext<'src, '_>,
    ran: &mut Ran<'src>,
    recipe: &Recipe<'src>,
    is_dependency: bool,
  ) -> RunResult<'src> {
    if ran.has_run(&recipe.namepath, arguments) {
      return Ok(());
    }

    if !context.config.yes && !recipe.confirm()? {
      return Err(Error::NotConfirmed {
        recipe: recipe.name(),
      });
    }

    let (outer, positional) =
      Evaluator::evaluate_parameters(context, is_dependency, arguments, &recipe.parameters)?;

    let scope = outer.child();

    let mut evaluator = Evaluator::new(context, true, &scope);

    if !context.config.no_dependencies {
      for Dependency { recipe, arguments } in recipe.dependencies.iter().take(recipe.priors) {
        let arguments = arguments
          .iter()
          .map(|argument| evaluator.evaluate_expression(argument))
          .collect::<RunResult<Vec<String>>>()?;

        Self::run_recipe(&arguments, context, ran, recipe, true)?;
      }
    }

    recipe.run(context, &scope, &positional, is_dependency)?;

    if !context.config.no_dependencies {
      let mut ran = Ran::default();

      for Dependency { recipe, arguments } in recipe.subsequents() {
        let mut evaluated = Vec::new();

        for argument in arguments {
          evaluated.push(evaluator.evaluate_expression(argument)?);
        }

        Self::run_recipe(&evaluated, context, &mut ran, recipe, true)?;
      }
    }

    ran.ran(&recipe.namepath, arguments.to_vec());

    Ok(())
  }

  pub(crate) fn modules(&self, config: &Config) -> Vec<&Justfile> {
    let mut modules = self.modules.values().collect::<Vec<&Justfile>>();

    if config.unsorted {
      modules.sort_by_key(|module| {
        module
          .name
          .map(|name| name.token.offset)
          .unwrap_or_default()
      });
    }

    modules
  }

  pub(crate) fn public_recipes(&self, config: &Config) -> Vec<&Recipe> {
    let mut recipes = self
      .recipes
      .values()
      .map(AsRef::as_ref)
      .filter(|recipe| recipe.is_public())
      .collect::<Vec<&Recipe>>();

    if config.unsorted {
      recipes.sort_by_key(|recipe| (&recipe.import_offsets, recipe.name.offset));
    }

    recipes
  }

  pub(crate) fn groups(&self) -> &[String] {
    &self.groups
  }

  pub(crate) fn public_groups(&self, config: &Config) -> Vec<String> {
    let mut groups = Vec::new();

    for recipe in self.recipes.values() {
      if recipe.is_public() {
        for group in recipe.groups() {
          groups.push((recipe.import_offsets.as_slice(), recipe.name.offset, group));
        }
      }
    }

    for submodule in self.modules.values() {
      for group in submodule.groups() {
        groups.push((&[], submodule.name.unwrap().offset, group.to_string()));
      }
    }

    if config.unsorted {
      groups.sort();
    } else {
      groups.sort_by(|(_, _, a), (_, _, b)| a.cmp(b));
    }

    let mut seen = HashSet::new();

    groups.retain(|(_, _, group)| seen.insert(group.clone()));

    groups.into_iter().map(|(_, _, group)| group).collect()
  }
}

impl<'src> ColorDisplay for Justfile<'src> {
  fn fmt(&self, f: &mut Formatter, color: Color) -> fmt::Result {
    let mut items = self.recipes.len() + self.assignments.len() + self.aliases.len();
    for (name, assignment) in &self.assignments {
      if assignment.export {
        write!(f, "export ")?;
      }
      write!(f, "{name} := {}", assignment.value)?;
      items -= 1;
      if items != 0 {
        write!(f, "\n\n")?;
      }
    }
    for alias in self.aliases.values() {
      write!(f, "{alias}")?;
      items -= 1;
      if items != 0 {
        write!(f, "\n\n")?;
      }
    }
    for recipe in self.recipes.values() {
      write!(f, "{}", recipe.color_display(color))?;
      items -= 1;
      if items != 0 {
        write!(f, "\n\n")?;
      }
    }
    Ok(())
  }
}

impl<'src> Keyed<'src> for Justfile<'src> {
  fn key(&self) -> &'src str {
    self.name()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use testing::compile;
  use Error::*;

  run_error! {
    name: unknown_recipe_no_suggestion,
    src: "a:\nb:\nc:",
    args: ["a", "xyz", "y", "z"],
    error: UnknownRecipe {
      recipe,
      suggestion,
    },
    check: {
      assert_eq!(recipe, "xyz");
      assert_eq!(suggestion, None);
    }
  }

  run_error! {
    name: unknown_recipe_with_suggestion,
    src: "a:\nb:\nc:",
    args: ["a", "x", "y", "z"],
    error: UnknownRecipe {
      recipe,
      suggestion,
    },
    check: {
      assert_eq!(recipe, "x");
      assert_eq!(suggestion, Some(Suggestion {
        name: "a",
        target: None,
      }));
    }
  }

  run_error! {
    name: unknown_recipe_show_alias_suggestion,
    src: "
      foo:
        echo foo

      alias z := foo
    ",
    args: ["zz"],
    error: UnknownRecipe {
      recipe,
      suggestion,
    },
    check: {
      assert_eq!(recipe, "zz");
      assert_eq!(suggestion, Some(Suggestion {
        name: "z",
        target: Some("foo"),
      }
    ));
    }
  }

  run_error! {
    name: code_error,
    src: "
      fail:
        @exit 100
    ",
    args: ["fail"],
    error: Code {
      recipe,
      line_number,
      code,
      print_message,
    },
    check: {
      assert_eq!(recipe, "fail");
      assert_eq!(code, 100);
      assert_eq!(line_number, Some(2));
      assert!(print_message);
    }
  }

  run_error! {
    name: run_args,
    src: r#"
      a return code:
        @x() { {{return}} {{code + "0"}}; }; x
    "#,
    args: ["a", "return", "15"],
    error: Code {
      recipe,
      line_number,
      code,
      print_message,
    },
    check: {
      assert_eq!(recipe, "a");
      assert_eq!(code, 150);
      assert_eq!(line_number, Some(2));
      assert!(print_message);
    }
  }

  run_error! {
    name: missing_some_arguments,
    src: "a b c d:",
    args: ["a", "b", "c"],
    error: ArgumentCountMismatch {
      recipe,
      parameters,
      found,
      min,
      max,
    },
    check: {
      let param_names = parameters
        .iter()
        .map(|p| p.name.lexeme())
        .collect::<Vec<&str>>();
      assert_eq!(recipe, "a");
      assert_eq!(param_names, ["b", "c", "d"]);
      assert_eq!(found, 2);
      assert_eq!(min, 3);
      assert_eq!(max, 3);
    }
  }

  run_error! {
    name: missing_some_arguments_variadic,
    src: "a b c +d:",
    args: ["a", "B", "C"],
    error: ArgumentCountMismatch {
      recipe,
      parameters,
      found,
      min,
      max,
    },
    check: {
      let param_names = parameters
        .iter()
        .map(|p| p.name.lexeme())
        .collect::<Vec<&str>>();
      assert_eq!(recipe, "a");
      assert_eq!(param_names, ["b", "c", "d"]);
      assert_eq!(found, 2);
      assert_eq!(min, 3);
      assert_eq!(max, usize::MAX - 1);
    }
  }

  run_error! {
    name: missing_all_arguments,
    src: "a b c d:\n echo {{b}}{{c}}{{d}}",
    args: ["a"],
    error: ArgumentCountMismatch {
      recipe,
      parameters,
      found,
      min,
      max,
    },
    check: {
      let param_names = parameters
        .iter()
        .map(|p| p.name.lexeme())
        .collect::<Vec<&str>>();
      assert_eq!(recipe, "a");
      assert_eq!(param_names, ["b", "c", "d"]);
      assert_eq!(found, 0);
      assert_eq!(min, 3);
      assert_eq!(max, 3);
    }
  }

  run_error! {
    name: missing_some_defaults,
    src: "a b c d='hello':",
    args: ["a", "b"],
    error: ArgumentCountMismatch {
      recipe,
      parameters,
      found,
      min,
      max,
    },
    check: {
      let param_names = parameters
        .iter()
        .map(|p| p.name.lexeme())
        .collect::<Vec<&str>>();
      assert_eq!(recipe, "a");
      assert_eq!(param_names, ["b", "c", "d"]);
      assert_eq!(found, 1);
      assert_eq!(min, 2);
      assert_eq!(max, 3);
    }
  }

  run_error! {
    name: missing_all_defaults,
    src: "a b c='r' d='h':",
    args: ["a"],
    error: ArgumentCountMismatch {
      recipe,
      parameters,
      found,
      min,
      max,
    },
    check: {
      let param_names = parameters
        .iter()
        .map(|p| p.name.lexeme())
        .collect::<Vec<&str>>();
      assert_eq!(recipe, "a");
      assert_eq!(param_names, ["b", "c", "d"]);
      assert_eq!(found, 0);
      assert_eq!(min, 1);
      assert_eq!(max, 3);
    }
  }

  run_error! {
    name: unknown_overrides,
    src: "
      a:
       echo {{`f() { return 100; }; f`}}
    ",
    args: ["foo=bar", "baz=bob", "a"],
    error: UnknownOverrides { overrides },
    check: {
      assert_eq!(overrides, &["baz", "foo"]);
    }
  }

  run_error! {
    name: export_failure,
    src: r#"
      export foo := "a"
      baz := "c"
      export bar := "b"
      export abc := foo + bar + baz

      wut:
        echo $foo $bar $baz
    "#,
    args: ["--quiet", "wut"],
    error: Code {
      recipe,
      line_number,
      print_message,
      ..
    },
    check: {
      assert_eq!(recipe, "wut");
      assert_eq!(line_number, Some(7));
      assert!(print_message);
    }
  }

  fn case(input: &str, expected: &str) {
    let justfile = compile(input);
    let actual = format!("{}", justfile.color_display(Color::never()));
    assert_eq!(actual, expected);
    println!("Re-parsing...");
    let reparsed = compile(&actual);
    let redumped = format!("{}", reparsed.color_display(Color::never()));
    assert_eq!(redumped, actual);
  }

  #[test]
  fn parse_empty() {
    case(
      "

# hello


    ",
      "",
    );
  }

  #[test]
  fn parse_string_default() {
    case(
      r#"

foo a="b\t":


  "#,
      r#"foo a="b\t":"#,
    );
  }

  #[test]
  fn parse_multiple() {
    case(
      r"
a:
b:
", r"a:

b:",
    );
  }

  #[test]
  fn parse_variadic() {
    case(
      r"

foo +a:


  ",
      r"foo +a:",
    );
  }

  #[test]
  fn parse_variadic_string_default() {
    case(
      r#"

foo +a="Hello":


  "#,
      r#"foo +a="Hello":"#,
    );
  }

  #[test]
  fn parse_raw_string_default() {
    case(
      r"

foo a='b\t':


  ",
      r"foo a='b\t':",
    );
  }

  #[test]
  fn parse_export() {
    case(
      r#"
export a := "hello"

  "#,
      r#"export a := "hello""#,
    );
  }

  #[test]
  fn parse_alias_after_target() {
    case(
      r"
foo:
  echo a
alias f := foo
",
      r"alias f := foo

foo:
    echo a",
    );
  }

  #[test]
  fn parse_alias_before_target() {
    case(
      r"
alias f := foo
foo:
  echo a
",
      r"alias f := foo

foo:
    echo a",
    );
  }

  #[test]
  fn parse_alias_with_comment() {
    case(
      r"
alias f := foo #comment
foo:
  echo a
",
      r"alias f := foo

foo:
    echo a",
    );
  }

  #[test]
  fn parse_complex() {
    case(
      "
x:
y:
z:
foo := \"xx\"
bar := foo
goodbye := \"y\"
hello a b    c   : x y    z #hello
  #! blah
  #blarg
  {{ foo + bar}}abc{{ goodbye\t  + \"x\" }}xyz
  1
  2
  3
",
      "bar := foo

foo := \"xx\"

goodbye := \"y\"

hello a b c: x y z
    #! blah
    #blarg
    {{ foo + bar }}abc{{ goodbye + \"x\" }}xyz
    1
    2
    3

x:

y:

z:",
    );
  }

  #[test]
  fn parse_shebang() {
    case(
      "
practicum := 'hello'
install:
\t#!/bin/sh
\tif [[ -f {{practicum}} ]]; then
\t\treturn
\tfi
",
      "practicum := 'hello'

install:
    #!/bin/sh
    if [[ -f {{ practicum }} ]]; then
    \treturn
    fi",
    );
  }

  #[test]
  fn parse_simple_shebang() {
    case("a:\n #!\n  print(1)", "a:\n    #!\n     print(1)");
  }

  #[test]
  fn parse_assignments() {
    case(
      r#"a := "0"
c := a + b + a + b
b := "1"
"#,
      r#"a := "0"

b := "1"

c := a + b + a + b"#,
    );
  }

  #[test]
  fn parse_assignment_backticks() {
    case(
      "a := `echo hello`
c := a + b + a + b
b := `echo goodbye`",
      "a := `echo hello`

b := `echo goodbye`

c := a + b + a + b",
    );
  }

  #[test]
  fn parse_interpolation_backticks() {
    case(
      r#"a:
  echo {{  `echo hello` + "blarg"   }} {{   `echo bob`   }}"#,
      r#"a:
    echo {{ `echo hello` + "blarg" }} {{ `echo bob` }}"#,
    );
  }

  #[test]
  fn eof_test() {
    case("x:\ny:\nz:\na b c: x y z", "a b c: x y z\n\nx:\n\ny:\n\nz:");
  }

  #[test]
  fn string_quote_escape() {
    case(r#"a := "hello\"""#, r#"a := "hello\"""#);
  }

  #[test]
  fn string_escapes() {
    case(r#"a := "\n\t\r\"\\""#, r#"a := "\n\t\r\"\\""#);
  }

  #[test]
  fn parameters() {
    case(
      "a b c:
  {{b}} {{c}}",
      "a b c:
    {{ b }} {{ c }}",
    );
  }

  #[test]
  fn unary_functions() {
    case(
      "
x := arch()

a:
  {{os()}} {{os_family()}} {{num_cpus()}}",
      "x := arch()

a:
    {{ os() }} {{ os_family() }} {{ num_cpus() }}",
    );
  }

  #[test]
  fn env_functions() {
    case(
      r#"
x := env_var('foo',)

a:
  {{env_var_or_default('foo' + 'bar', 'baz',)}} {{env_var(env_var("baz"))}}"#,
      r#"x := env_var('foo')

a:
    {{ env_var_or_default('foo' + 'bar', 'baz') }} {{ env_var(env_var("baz")) }}"#,
    );
  }

  #[test]
  fn parameter_default_string() {
    case(
      r#"
f x="abc":
"#,
      r#"f x="abc":"#,
    );
  }

  #[test]
  fn parameter_default_raw_string() {
    case(
      r"
f x='abc':
",
      r"f x='abc':",
    );
  }

  #[test]
  fn parameter_default_backtick() {
    case(
      r"
f x=`echo hello`:
",
      r"f x=`echo hello`:",
    );
  }

  #[test]
  fn parameter_default_concatenation_string() {
    case(
      r#"
f x=(`echo hello` + "foo"):
"#,
      r#"f x=(`echo hello` + "foo"):"#,
    );
  }

  #[test]
  fn parameter_default_concatenation_variable() {
    case(
      r#"
x := "10"
f y=(`echo hello` + x) +z="foo":
"#,
      r#"x := "10"

f y=(`echo hello` + x) +z="foo":"#,
    );
  }

  #[test]
  fn parameter_default_multiple() {
    case(
      r#"
x := "10"
f y=(`echo hello` + x) +z=("foo" + "bar"):
"#,
      r#"x := "10"

f y=(`echo hello` + x) +z=("foo" + "bar"):"#,
    );
  }

  #[test]
  fn concatenation_in_group() {
    case("x := ('0' + '1')", "x := ('0' + '1')");
  }

  #[test]
  fn string_in_group() {
    case("x := ('0'   )", "x := ('0')");
  }

  #[rustfmt::skip]
  #[test]
  fn escaped_dos_newlines() {
    case("@spam:\r
\t{ \\\r
\t\tfiglet test; \\\r
\t\tcargo build --color always 2>&1; \\\r
\t\tcargo test  --color always -- --color always 2>&1; \\\r
\t} | less\r
",
"@spam:
    { \\
    \tfiglet test; \\
    \tcargo build --color always 2>&1; \\
    \tcargo test  --color always -- --color always 2>&1; \\
    } | less");
  }
}
