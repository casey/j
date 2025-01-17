use super::*;

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Alias<'a> {
  attributes: Vec<&'a str>,
  name: &'a str,
  target: &'a str,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Assignment<'a> {
  export: bool,
  name: &'a str,
  private: bool,
  value: &'a str,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Dependency<'a> {
  arguments: Vec<Value>,
  recipe: &'a str,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Interpreter<'a> {
  arguments: Vec<&'a str>,
  command: &'a str,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Module<'a> {
  aliases: BTreeMap<&'a str, Alias<'a>>,
  assignments: BTreeMap<&'a str, Assignment<'a>>,
  doc: Option<&'a str>,
  first: Option<&'a str>,
  groups: Vec<&'a str>,
  modules: BTreeMap<&'a str, Module<'a>>,
  recipes: BTreeMap<&'a str, Recipe<'a>>,
  settings: Settings<'a>,
  unexports: Vec<&'a str>,
  warnings: Vec<&'a str>,
  source: PathBuf,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Parameter<'a> {
  default: Option<&'a str>,
  export: bool,
  kind: &'a str,
  name: &'a str,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Recipe<'a> {
  attributes: Vec<Value>,
  body: Vec<Value>,
  dependencies: Vec<Dependency<'a>>,
  doc: Option<&'a str>,
  name: &'a str,
  namepath: &'a str,
  parameters: Vec<Parameter<'a>>,
  priors: u32,
  private: bool,
  quiet: bool,
  shebang: bool,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Settings<'a> {
  allow_duplicate_recipes: bool,
  allow_duplicate_variables: bool,
  dotenv_filename: Option<&'a str>,
  dotenv_load: bool,
  dotenv_path: Option<&'a str>,
  dotenv_required: bool,
  export: bool,
  fallback: bool,
  ignore_comments: bool,
  no_exit_message: bool,
  positional_arguments: bool,
  quiet: bool,
  shell: Option<Interpreter<'a>>,
  tempdir: Option<&'a str>,
  unstable: bool,
  windows_powershell: bool,
  windows_shell: Option<&'a str>,
  working_directory: Option<&'a str>,
}

#[track_caller]
fn case(justfile: &str, expected: Module) {
  case_with_submodule(justfile, None, expected);
}

fn fix_source(dir: &Path, module: &mut Module) {
  let filename = if module.source.as_os_str().is_empty() {
    Path::new("justfile")
  } else {
    &module.source
  };

  module.source = if cfg!(target_os = "macos") {
    dir.canonicalize().unwrap().join(filename)
  } else {
    dir.join(filename)
  };

  for module in module.modules.values_mut() {
    fix_source(dir, module);
  }
}

#[track_caller]
fn case_with_submodule(justfile: &str, submodule: Option<(&str, &str)>, mut expected: Module) {
  let mut test = Test::new()
    .justfile(justfile)
    .args(["--dump", "--dump-format", "json"])
    .stdout_regex(".*");

  if let Some((path, source)) = submodule {
    test = test.write(path, source);
  }

  fix_source(test.tempdir.path(), &mut expected);

  let actual = test.run().stdout;

  let actual: Module = serde_json::from_str(actual.as_str()).unwrap();
  pretty_assertions::assert_eq!(actual, expected);
}

#[test]
fn alias() {
  case(
    "
      alias f := foo

      foo:
    ",
    Module {
      aliases: [(
        "f",
        Alias {
          name: "f",
          target: "foo",
          ..default()
        },
      )]
      .into(),
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          name: "foo",
          namepath: "foo",
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn assignment() {
  case(
    "foo := 'bar'",
    Module {
      assignments: [(
        "foo",
        Assignment {
          name: "foo",
          value: "bar",
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn private_assignment() {
  case(
    "
      _foo := 'foo'
      [private]
      bar := 'bar'
    ",
    Module {
      assignments: [
        (
          "_foo",
          Assignment {
            name: "_foo",
            value: "foo",
            private: true,
            ..default()
          },
        ),
        (
          "bar",
          Assignment {
            name: "bar",
            value: "bar",
            private: true,
            ..default()
          },
        ),
      ]
      .into(),
      ..default()
    },
  );
}

#[test]
fn body() {
  case(
    "
      foo:
        bar
        abc{{ 'xyz' }}def
    ",
    Module {
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          name: "foo",
          namepath: "foo",
          body: [json!(["bar"]), json!(["abc", ["xyz"], "def"])].into(),
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn dependencies() {
  case(
    "
      foo:
      bar: foo
    ",
    Module {
      first: Some("foo"),
      recipes: [
        (
          "foo",
          Recipe {
            name: "foo",
            namepath: "foo",
            ..default()
          },
        ),
        (
          "bar",
          Recipe {
            name: "bar",
            namepath: "bar",
            dependencies: [Dependency {
              recipe: "foo",
              ..default()
            }]
            .into(),
            priors: 1,
            ..default()
          },
        ),
      ]
      .into(),
      ..default()
    },
  );
}

#[test]
fn dependency_argument() {
  case(
    "
      x := 'foo'
      foo *args:
      bar: (
        foo
        'baz'
        ('baz')
        ('a' + 'b')
        `echo`
        x
        if 'a' == 'b' { 'c' } else { 'd' }
        arch()
        env_var('foo')
        join('a', 'b')
        replace('a', 'b', 'c')
      )
    ",
    Module {
      assignments: [(
        "x",
        Assignment {
          name: "x",
          value: "foo",
          ..default()
        },
      )]
      .into(),
      first: Some("foo"),
      recipes: [
        (
          "foo",
          Recipe {
            name: "foo",
            namepath: "foo",
            parameters: [Parameter {
              kind: "star",
              name: "args",
              ..default()
            }]
            .into(),
            ..default()
          },
        ),
        (
          "bar",
          Recipe {
            name: "bar",
            namepath: "bar",
            dependencies: [Dependency {
              recipe: "foo",
              arguments: [
                json!("baz"),
                json!("baz"),
                json!(["concatenate", "a", "b"]),
                json!(["evaluate", "echo"]),
                json!(["variable", "x"]),
                json!(["if", ["==", "a", "b"], "c", "d"]),
                json!(["call", "arch"]),
                json!(["call", "env_var", "foo"]),
                json!(["call", "join", "a", "b"]),
                json!(["call", "replace", "a", "b", "c"]),
              ]
              .into(),
            }]
            .into(),
            priors: 1,
            ..default()
          },
        ),
      ]
      .into(),
      ..default()
    },
  );
}

#[test]
fn duplicate_recipes() {
  case(
    "
      set allow-duplicate-recipes
      alias f := foo

      foo:
      foo bar:
    ",
    Module {
      aliases: [(
        "f",
        Alias {
          name: "f",
          target: "foo",
          ..default()
        },
      )]
      .into(),
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          name: "foo",
          namepath: "foo",
          parameters: [Parameter {
            kind: "singular",
            name: "bar",
            ..default()
          }]
          .into(),
          ..default()
        },
      )]
      .into(),
      settings: Settings {
        allow_duplicate_recipes: true,
        ..default()
      },
      ..default()
    },
  );
}

#[test]
fn duplicate_variables() {
  case(
    "
      set allow-duplicate-variables
      x := 'foo'
      x := 'bar'
    ",
    Module {
      assignments: [(
        "x",
        Assignment {
          name: "x",
          value: "bar",
          ..default()
        },
      )]
      .into(),
      settings: Settings {
        allow_duplicate_variables: true,
        ..default()
      },
      ..default()
    },
  );
}

#[test]
fn doc_comment() {
  case(
    "# hello\nfoo:",
    Module {
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          doc: Some("hello"),
          name: "foo",
          namepath: "foo",
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn empty_justfile() {
  case("", Module::default());
}

#[test]
fn parameters() {
  case(
    "
      a:
      b x:
      c x='y':
      d +x:
      e *x:
      f $x:
    ",
    Module {
      first: Some("a"),
      recipes: [
        (
          "a",
          Recipe {
            name: "a",
            namepath: "a",
            ..default()
          },
        ),
        (
          "b",
          Recipe {
            name: "b",
            namepath: "b",
            parameters: [Parameter {
              kind: "singular",
              name: "x",
              ..default()
            }]
            .into(),
            ..default()
          },
        ),
        (
          "c",
          Recipe {
            name: "c",
            namepath: "c",
            parameters: [Parameter {
              default: Some("y"),
              kind: "singular",
              name: "x",
              ..default()
            }]
            .into(),
            ..default()
          },
        ),
        (
          "d",
          Recipe {
            name: "d",
            namepath: "d",
            parameters: [Parameter {
              kind: "plus",
              name: "x",
              ..default()
            }]
            .into(),
            ..default()
          },
        ),
        (
          "e",
          Recipe {
            name: "e",
            namepath: "e",
            parameters: [Parameter {
              kind: "star",
              name: "x",
              ..default()
            }]
            .into(),
            ..default()
          },
        ),
        (
          "f",
          Recipe {
            name: "f",
            namepath: "f",
            parameters: [Parameter {
              export: true,
              kind: "singular",
              name: "x",
              ..default()
            }]
            .into(),
            ..default()
          },
        ),
      ]
      .into(),
      ..default()
    },
  );
}

#[test]
fn priors() {
  case(
    "
      a:
      b: a && c
      c:
    ",
    Module {
      first: Some("a"),
      recipes: [
        (
          "a",
          Recipe {
            name: "a",
            namepath: "a",
            ..default()
          },
        ),
        (
          "b",
          Recipe {
            dependencies: [
              Dependency {
                recipe: "a",
                ..default()
              },
              Dependency {
                recipe: "c",
                ..default()
              },
            ]
            .into(),
            name: "b",
            namepath: "b",
            priors: 1,
            ..default()
          },
        ),
        (
          "c",
          Recipe {
            name: "c",
            namepath: "c",
            ..default()
          },
        ),
      ]
      .into(),
      ..default()
    },
  );
}

#[test]
fn private() {
  case(
    "_foo:",
    Module {
      first: Some("_foo"),
      recipes: [(
        "_foo",
        Recipe {
          name: "_foo",
          namepath: "_foo",
          private: true,
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn quiet() {
  case(
    "@foo:",
    Module {
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          name: "foo",
          namepath: "foo",
          quiet: true,
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn settings() {
  case(
    "
      set allow-duplicate-recipes
      set dotenv-filename := \"filename\"
      set dotenv-load
      set dotenv-path := \"path\"
      set export
      set fallback
      set ignore-comments
      set positional-arguments
      set quiet
      set shell := ['a', 'b', 'c']
      foo:
        #!bar
    ",
    Module {
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          name: "foo",
          namepath: "foo",
          shebang: true,
          body: [json!(["#!bar"])].into(),
          ..default()
        },
      )]
      .into(),
      settings: Settings {
        allow_duplicate_recipes: true,
        dotenv_filename: Some("filename"),
        dotenv_path: Some("path"),
        dotenv_load: true,
        export: true,
        fallback: true,
        ignore_comments: true,
        positional_arguments: true,
        quiet: true,
        shell: Some(Interpreter {
          arguments: ["b", "c"].into(),
          command: "a",
        }),
        ..default()
      },
      ..default()
    },
  );
}

#[test]
fn shebang() {
  case(
    "
      foo:
        #!bar
    ",
    Module {
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          name: "foo",
          namepath: "foo",
          shebang: true,
          body: [json!(["#!bar"])].into(),
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn simple() {
  case(
    "foo:",
    Module {
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          name: "foo",
          namepath: "foo",
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn attribute() {
  case(
    "
      [no-exit-message]
      foo:
    ",
    Module {
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          attributes: [json!("no-exit-message")].into(),
          name: "foo",
          namepath: "foo",
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn module() {
  case_with_submodule(
    "
      # hello
      mod foo
    ",
    Some(("foo.just", "bar:")),
    Module {
      modules: [(
        "foo",
        Module {
          doc: Some("hello"),
          first: Some("bar"),
          source: "foo.just".into(),
          recipes: [(
            "bar",
            Recipe {
              name: "bar",
              namepath: "foo::bar",
              ..default()
            },
          )]
          .into(),
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn module_group() {
  case_with_submodule(
    "
      [group('alpha')]
      mod foo
    ",
    Some(("foo.just", "bar:")),
    Module {
      modules: [(
        "foo",
        Module {
          first: Some("bar"),
          groups: ["alpha"].into(),
          source: "foo.just".into(),
          recipes: [(
            "bar",
            Recipe {
              name: "bar",
              namepath: "foo::bar",
              ..default()
            },
          )]
          .into(),
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn recipes_with_private_attribute_are_private() {
  case(
    "
      [private]
      foo:
    ",
    Module {
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          attributes: [json!("private")].into(),
          name: "foo",
          namepath: "foo",
          private: true,
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}

#[test]
fn doc_attribute_overrides_comment() {
  case(
    "
      # COMMENT
      [doc('ATTRIBUTE')]
      foo:
    ",
    Module {
      first: Some("foo"),
      recipes: [(
        "foo",
        Recipe {
          attributes: [json!({"doc": "ATTRIBUTE"})].into(),
          doc: Some("ATTRIBUTE"),
          name: "foo",
          namepath: "foo",
          ..default()
        },
      )]
      .into(),
      ..default()
    },
  );
}
