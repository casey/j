use super::*;

#[test]
fn ignore_comments_in_recipe() {
  Test::new()
    .justfile(
      "
      set ignore-comments

      some_recipe:
        # A recipe-internal comment
        echo something-useful
    ",
    )
    .stdout("something-useful\n")
    .stderr("echo something-useful\n")
    .run();
}

#[test]
fn dont_ignore_comments_in_recipe_by_default() {
  Test::new()
    .justfile(
      "
      some_recipe:
        # A recipe-internal comment
        echo something-useful
    ",
    )
    .stdout("something-useful\n")
    .stderr("# A recipe-internal comment\necho something-useful\n")
    .run();
}

#[test]
fn ignore_recipe_comments_with_shell_setting() {
  Test::new()
    .justfile(
      "
      set shell := ['echo', '-n']
      set ignore-comments

      some_recipe:
        # Alternate shells still ignore comments
        echo something-useful
    ",
    )
    .stdout("something-useful\n")
    .stderr("echo something-useful\n")
    .run();
}

#[test]
fn continuations_with_echo_comments_false() {
  Test::new()
    .justfile(
      "
      set ignore-comments

      some_recipe:
        # Comment lines ignore line continuations \\
        echo something-useful
    ",
    )
    .stdout("something-useful\n")
    .stderr("echo something-useful\n")
    .run();
}

#[test]
fn continuations_with_echo_comments_true() {
  Test::new()
    .justfile(
      "
      set ignore-comments := false

      some_recipe:
        # comment lines can be continued \\
        echo something-useful
    ",
    )
    .stdout("")
    .stderr("# comment lines can be continued echo something-useful\n")
    .run();
}

#[test]
fn dont_evaluate_comments() {
  Test::new()
    .justfile(
      "
      set ignore-comments

      some_recipe:
        # {{ error('foo') }}
    ",
    )
    .run();
}
