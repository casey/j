use super::*;

#[test]
fn environment_variable_set() {
  Test::new()
    .justfile(
      r#"
      export BAR := 'baz'

      @foo:
        '{{just_executable()}}' --request '{"environment-variable": "BAR"}'
    "#,
    )
    .response(Response::EnvironmentVariable(Some("baz".into())))
    .run();
}

#[test]
fn environment_variable_missing() {
  Test::new()
    .justfile(
      r#"
      @foo:
        '{{just_executable()}}' --request '{"environment-variable": "FOO_BAR_BAZ"}'
    "#,
    )
    .response(Response::EnvironmentVariable(None))
    .run();
}
