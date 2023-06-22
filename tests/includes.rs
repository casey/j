use super::*;

#[test]
fn include_fails_without_unstable() {
  Test::new()
    .justfile("!include ./include.justfile")
    .status(EXIT_FAILURE)
    .stderr("error: The !include directive is currently unstable. Invoke `just` with the `--unstable` flag to enable unstable features.\n")
    .run();
}

#[test]
fn include_succeeds_with_unstable() {
  Test::new()
    .tree(tree! {
      "include.justfile": "
        b:
          @echo B
      ",
    })
    .justfile(
      "
        !include ./include.justfile

        a: b
          @echo A
      ",
    )
    .arg("--unstable")
    .test_round_trip(false)
    .arg("a")
    .stdout("B\nA\n")
    .run();
}

#[test]
fn trailing_spaces_after_include_are_ignored() {
  Test::new()
    .tree(tree! {
      "include.justfile": "
        a:
          @echo A
      ",
    })
    .justfile("!include ./include.justfile\x20")
    .arg("--unstable")
    .test_round_trip(false)
    .stdout("A\n")
    .run();
}

#[test]
fn include_directive_with_no_path() {
  Test::new()
    .justfile("!include")
    .arg("--unstable")
    .status(EXIT_FAILURE)
    .stderr(
      "
error: !include directive has no argument
  |
1 | !include
  |         ^
     ",
    )
    .run();
}

#[test]
fn include_after_recipe() {
  Test::new()
    .tree(tree! {
      "include.justfile": "
        a:
          @echo A
      ",
    })
    .justfile(
      "
      b: a
      !include ./include.justfile
      ",
    )
    .arg("--unstable")
    .test_round_trip(false)
    .stdout("A\n")
    .run();
}

#[test]
fn circular_include() {
  Test::new()
    .justfile("!include a")
    .tree(tree! {
      a: "!include b",
      b: "!include a",
    })
    .arg("--unstable")
    .status(EXIT_FAILURE)
    .stderr_regex(path_for_regex(
      "error: Include `.*/a` in `.*/b` is a circular include\n",
    ))
    .run();
}