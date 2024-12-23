use super::*;

#[test]
fn match_succeeds_evaluates_to_first_branch() {
  Test::new()
    .justfile(
      "
      foo := if 'abbbc' =~ 'ab+c' {
        'yes'
      } else {
        'no'
      }

      default:
        echo {{ foo }}
    ",
    )
    .stderr("echo yes\n")
    .stdout("yes\n")
    .run();
}

#[test]
fn match_fails_evaluates_to_second_branch() {
  Test::new()
    .justfile(
      "
      foo := if 'abbbc' =~ 'ab{4}c' {
        'yes'
      } else {
        'no'
      }

      default:
        echo {{ foo }}
    ",
    )
    .stderr("echo no\n")
    .stdout("no\n")
    .run();
}

#[test]
fn bad_regex_fails_at_runtime() {
  Test::new()
    .justfile(
      "
        default:
          echo before
          echo {{ if '' =~ '(' { 'a' } else { 'b' } }}
          echo after
      ",
    )
    .stderr(
      "
        echo before
        error: regex parse error:
            (
            ^
        error: unclosed group
      ",
    )
    .stdout("before\n")
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn mismatch() {
  Test::new()
    .justfile(
      "
      foo := if 'Foo' !~ '^ab+c' {
        'mismatch'
      } else {
        'match'
      }

      bar := if 'Foo' !~ 'Foo' {
        'mismatch'
      } else {
        'match'
      }

      @default:
        echo {{ foo }} {{ bar }}
    ",
    )
    .stdout("mismatch match\n")
    .run();
}
