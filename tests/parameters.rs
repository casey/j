use super::*;

#[test]
fn parameter_default_values_may_use_earlier_parameters() {
  Test::new()
    .justfile(
      "
        @foo a b=a:
          echo {{ b }}
      ",
    )
    .args(["foo", "bar"])
    .stdout("bar\n")
    .run();
}

#[test]
fn parameter_default_values_may_not_use_later_parameters() {
  Test::new()
    .justfile(
      "
        @foo a b=c c='':
          echo {{ b }}
      ",
    )
    .args(["foo", "bar"])
    .stderr(
      "
        error: Variable `c` not defined
         ——▶ justfile:1:10
          │
        1 │ @foo a b=c c='':
          │          ^
      ",
    )
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn parameter_variadic_default_may_use_earlier_parameters() {
  Test::new()
    .justfile(
      "
        @foo a *b=a:
          echo {{ b }}
      ",
    )
    .args(["foo", "bar"])
    .stdout("bar\n")
    .run();
}
