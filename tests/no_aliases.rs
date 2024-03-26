use super::*;

#[test]
fn skip_alias() {
  Test::new()
    .justfile(
      "
      alias t := test1

      test1:
        @echo 'test1'

      test2:
        @echo 'test2'
      ",
    )
    .args(["--no-aliases", "--list"])
    .stdout("Available recipes:\n    test1\n    test2\n")
    .run();
}
