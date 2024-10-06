use super::*;

test! {
  name: set_export_parse_error,
  justfile: "
    set export := fals
  ",
  stdout: "",
  stderr: "
    error: Expected keyword `true` or `false` but found identifier `fals`
     ——▶ justfile:1:15
      │
    1 │ set export := fals
      │               ^^^^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: set_export_parse_error_eol,
  justfile: "
    set export :=
  ",
  stdout: "",
  stderr: "
    error: Expected identifier, but found end of line
     ——▶ justfile:1:14
      │
    1 │ set export :=
      │              ^
  ",
  status: EXIT_FAILURE,
}

#[test]
fn invalid_attributes_are_an_error() {
  Test::new()
    .justfile(
      "
        [group: 'bar']
        x := 'foo'
      ",
    )
    .args(["--evaluate", "x"])
    .stderr(
      "
        error: Assignment `x` has invalid attribute `group`
         ——▶ justfile:2:1
          │
        2 │ x := 'foo'
          │ ^
      ",
    )
    .status(EXIT_FAILURE)
    .run();
}
