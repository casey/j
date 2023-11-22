use parallel::Ran;
use std::sync::Arc;
use {super::*, serde::Serialize};

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Justfile<'src> {
  pub(crate) aliases: Table<'src, Alias<'src>>,
  pub(crate) assignments: Table<'src, Assignment<'src>>,
  #[serde(serialize_with = "keyed::serialize_option")]
  pub(crate) first: Option<Arc<Recipe<'src>>>,
  pub(crate) recipes: Table<'src, Arc<Recipe<'src>>>,
  pub(crate) settings: Settings<'src>,
  pub(crate) warnings: Vec<Warning>,
}

impl<'src> Justfile<'src> {
  pub(crate) fn count(&self) -> usize {
    self.recipes.len()
  }

  pub(crate) fn suggest_recipe(&self, input: &str) -> Option<Suggestion<'src>> {
    let mut suggestions = self
      .recipes
      .keys()
      .map(|name| {
        (
          edit_distance(name, input),
          Suggestion { name, target: None },
        )
      })
      .chain(self.aliases.iter().map(|(name, alias)| {
        (
          edit_distance(name, input),
          Suggestion {
            name,
            target: Some(alias.target.name.lexeme()),
          },
        )
      }))
      .filter(|(distance, _suggestion)| distance < &3)
      .collect::<Vec<(usize, Suggestion)>>();
    suggestions.sort_by_key(|(distance, _suggestion)| *distance);

    suggestions
      .into_iter()
      .map(|(_distance, suggestion)| suggestion)
      .next()
  }

  pub(crate) fn suggest_variable(&self, input: &str) -> Option<Suggestion<'src>> {
    let mut suggestions = self
      .assignments
      .keys()
      .map(|name| {
        (
          edit_distance(name, input),
          Suggestion { name, target: None },
        )
      })
      .filter(|(distance, _suggestion)| distance < &3)
      .collect::<Vec<(usize, Suggestion)>>();
    suggestions.sort_by_key(|(distance, _suggestion)| *distance);

    suggestions
      .into_iter()
      .map(|(_distance, suggestion)| suggestion)
      .next()
  }

  pub(crate) fn run(
    &self,
    config: &Config,
    search: &Search,
    overrides: &BTreeMap<String, String>,
    arguments: &[String],
  ) -> RunResult<'src, ()> {
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

    let scope = {
      let mut scope = Scope::new();
      let mut unknown_overrides = Vec::new();

      for (name, value) in overrides {
        if let Some(assignment) = self.assignments.get(name) {
          scope.bind(assignment.export, assignment.name, value.clone());
        } else {
          unknown_overrides.push(name.clone());
        }
      }

      if !unknown_overrides.is_empty() {
        return Err(Error::UnknownOverrides {
          overrides: unknown_overrides,
        });
      }

      Evaluator::evaluate_assignments(
        &self.assignments,
        config,
        &dotenv,
        scope,
        &self.settings,
        search,
      )?
    };

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

        command.export(&self.settings, &dotenv, &scope);

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
          let mut width = 0;

          for name in scope.names() {
            width = cmp::max(name.len(), width);
          }

          for binding in scope.bindings() {
            println!(
              "{0:1$} := \"{2}\"",
              binding.name.lexeme(),
              width,
              binding.value
            );
          }
        }

        return Ok(());
      }
      _ => {}
    }

    let argvec: Vec<&str> = if !arguments.is_empty() {
      arguments.iter().map(String::as_str).collect()
    } else if let Some(recipe) = &self.first {
      let min_arguments = recipe.min_arguments();
      if min_arguments > 0 {
        return Err(Error::DefaultRecipeRequiresArguments {
          recipe: recipe.name.lexeme(),
          min_arguments,
        });
      }
      vec![recipe.name()]
    } else {
      return Err(Error::NoRecipes);
    };

    let arguments = argvec.as_slice();

    let mut missing = vec![];
    let mut grouped = vec![];
    let mut rest = arguments;

    while let Some((argument, mut tail)) = rest.split_first() {
      if let Some(recipe) = self.get_recipe(argument) {
        if recipe.parameters.is_empty() {
          grouped.push((recipe, &[][..]));
        } else {
          let argument_range = recipe.argument_range();
          let argument_count = cmp::min(tail.len(), recipe.max_arguments());
          if !argument_range.range_contains(&argument_count) {
            return Err(Error::ArgumentCountMismatch {
              recipe: recipe.name(),
              parameters: recipe.parameters.clone(),
              found: tail.len(),
              min: recipe.min_arguments(),
              max: recipe.max_arguments(),
            });
          }
          grouped.push((recipe, &tail[0..argument_count]));
          tail = &tail[argument_count..];
        }
      } else {
        missing.push((*argument).to_owned());
      }
      rest = tail;
    }

    if !missing.is_empty() {
      let suggestion = if missing.len() == 1 {
        self.suggest_recipe(missing.first().unwrap())
      } else {
        None
      };
      return Err(Error::UnknownRecipes {
        recipes: missing,
        suggestion,
      });
    }

    let context = RecipeContext {
      settings: &self.settings,
      config,
      scope,
      search,
    };

    let ran = Ran::new();
    parallel::task_scope(config.parallel, |scope| {
      for (recipe, arguments) in grouped {
        scope.run(|| Self::run_recipe(&context, recipe, arguments, &dotenv, search, &ran))?;
      }
      Ok(())
    })?;

    Ok(())
  }

  pub(crate) fn get_alias(&self, name: &str) -> Option<&Alias<'src>> {
    self.aliases.get(name)
  }

  pub(crate) fn get_recipe(&self, name: &str) -> Option<&Recipe<'src>> {
    self
      .recipes
      .get(name)
      .map(Arc::as_ref)
      .or_else(|| self.aliases.get(name).map(|alias| alias.target.as_ref()))
  }

  fn run_recipe(
    context: &RecipeContext<'src, '_>,
    recipe: &Recipe<'src>,
    arguments: &[&str],
    dotenv: &BTreeMap<String, String>,
    search: &Search,
    ran: &Ran,
  ) -> RunResult<'src, ()> {
    let mut invocation = vec![recipe.name().to_owned()];
    for argument in arguments {
      invocation.push((*argument).to_string());
    }

    if ran.contains(&invocation) {
      return Ok(());
    }

    if !context.config.yes && !recipe.confirm()? {
      return Err(Error::NotConfirmed {
        recipe: recipe.name(),
      });
    }

    let (outer, positional) = Evaluator::evaluate_parameters(
      context.config,
      dotenv,
      &recipe.parameters,
      arguments,
      &context.scope,
      context.settings,
      search,
    )?;

    let scope = outer.child();

    let mut evaluator =
      Evaluator::recipe_evaluator(context.config, dotenv, &scope, context.settings, search);

    let run_dependencies_in_parallel = recipe.attributes.contains(&Attribute::Parallel);
    parallel::task_scope(run_dependencies_in_parallel, |scope| {
      for Dependency { recipe, arguments } in recipe.dependencies.iter().take(recipe.priors) {
        let arguments = arguments
          .iter()
          .map(|argument| evaluator.evaluate_expression(argument))
          .collect::<RunResult<Vec<String>>>()?;

        scope.run(move || {
          Self::run_recipe(
            context,
            recipe,
            &arguments.iter().map(String::as_ref).collect::<Vec<&str>>(),
            dotenv,
            search,
            ran,
          )
        })?;
      }
      Ok(())
    })?;

    recipe.run(context, dotenv, scope.child(), search, &positional)?;

    {
      let ran = Ran::new();

      parallel::task_scope(run_dependencies_in_parallel, |scope| {
        for Dependency { recipe, arguments } in recipe.dependencies.iter().skip(recipe.priors) {
          let mut evaluated = Vec::new();

          for argument in arguments {
            evaluated.push(
              evaluator
                .evaluate_expression(argument)
                .expect("error evaluating expression"),
            );
          }

          scope.run({
            let ran = ran.clone();
            move || {
              Self::run_recipe(
                context,
                recipe,
                &evaluated.iter().map(String::as_ref).collect::<Vec<&str>>(),
                dotenv,
                search,
                &ran,
              )
            }
          })?;
        }
        Ok(())
      })?;
    }

    ran.insert(invocation);
    Ok(())
  }

  pub(crate) fn public_recipes(&self, source_order: bool) -> Vec<&Recipe<'src, Dependency>> {
    let mut recipes = self
      .recipes
      .values()
      .map(AsRef::as_ref)
      .filter(|recipe| recipe.public())
      .collect::<Vec<&Recipe<Dependency>>>();

    if source_order {
      recipes.sort_by_key(|recipe| recipe.name.offset);
    }

    recipes
  }
}

impl<'src> ColorDisplay for Justfile<'src> {
  fn fmt(&self, f: &mut Formatter, color: Color) -> Result<(), fmt::Error> {
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

#[cfg(test)]
mod tests {
  use super::*;

  use testing::compile;
  use Error::*;

  run_error! {
    name: unknown_recipes,
    src: "a:\nb:\nc:",
    args: ["a", "x", "y", "z"],
    error: UnknownRecipes {
      recipes,
      suggestion,
    },
    check: {
      assert_eq!(recipes, &["x", "y", "z"]);
      assert_eq!(suggestion, None);
    }
  }

  run_error! {
    name: unknown_recipes_show_alias_suggestion,
    src: "
      foo:
        echo foo

      alias z := foo
    ",
    args: ["zz"],
    error: UnknownRecipes {
      recipes,
      suggestion,
    },
    check: {
      assert_eq!(recipes, &["zz"]);
      assert_eq!(suggestion, Some(Suggestion {
        name: "z",
        target: Some("foo"),
      }
    ));
    }
  }

  // This test exists to make sure that shebang recipes run correctly.  Although
  // this script is still executed by a shell its behavior depends on the value of
  // a variable and continuing even though a command fails, whereas in plain
  // recipes variables are not available in subsequent lines and execution stops
  // when a line fails.
  run_error! {
    name: run_shebang,
    src: "
      a:
        #!/usr/bin/env sh
        code=200
          x() { return $code; }
            x
              x
    ",
    args: ["a"],
    error: Code {
      recipe,
      line_number,
      code,
      print_message,
    },
    check: {
      assert_eq!(recipe, "a");
      assert_eq!(code, 200);
      assert_eq!(line_number, None);
      assert!(print_message);
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
