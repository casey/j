use super::*;

test! {
  name: alias_listing,
  justfile: "
    foo:
      echo foo

    alias f := foo
  ",
  args: ("--list"),
  stdout: "
    Available recipes:
        foo # [alias: f]
  ",
}

#[test]
fn alias_listing_with_doc() {
  Test::new()
    .justfile(
      "
        # foo command
        foo:
          echo foo

        alias f := foo
      ",
    )
    .arg("--list")
    .stdout(
      "
      Available recipes:
          foo # foo command [alias: f]
    ",
    )
    .run();
}

test! {
  name: alias_listing_multiple_aliases,
  justfile: "foo:\n  echo foo\nalias f := foo\nalias fo := foo",
  args: ("--list"),
  stdout: "
    Available recipes:
        foo # [aliases: f, fo]
  ",
}

test! {
  name: alias_listing_parameters,
  justfile: "foo PARAM='foo':\n  echo {{PARAM}}\nalias f := foo",
  args: ("--list"),
  stdout: "
    Available recipes:
        foo PARAM='foo' # [alias: f]
  ",
}

test! {
  name: alias_listing_private,
  justfile: "foo PARAM='foo':\n  echo {{PARAM}}\nalias _f := foo",
  args: ("--list"),
  stdout: "
    Available recipes:
        foo PARAM='foo'
  ",
}

test! {
  name: alias,
  justfile: "foo:\n  echo foo\nalias f := foo",
  args: ("f"),
  stdout: "foo\n",
  stderr: "echo foo\n",
}

test! {
  name: alias_with_parameters,
  justfile: "foo value='foo':\n  echo {{value}}\nalias f := foo",
  args: ("f", "bar"),
  stdout: "bar\n",
  stderr: "echo bar\n",
}

test! {
  name: bad_setting,
  justfile: "
    set foo
  ",
  stderr: "
  error: Unknown setting `foo`
   ——▶ justfile:1:5
    │
  1 │ set foo
    │     ^^^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: bad_setting_with_keyword_name,
  justfile: "
    set if := 'foo'
  ",
  stderr: "
  error: Unknown setting `if`
   ——▶ justfile:1:5
    │
  1 │ set if := 'foo'
    │     ^^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: alias_with_dependencies,
  justfile: "foo:\n  echo foo\nbar: foo\nalias b := bar",
  args: ("b"),
  stdout: "foo\n",
  stderr: "echo foo\n",
}

test! {
  name: duplicate_alias,
  justfile: "alias foo := bar\nalias foo := baz\n",
  stderr: "
    error: Alias `foo` first defined on line 1 is redefined on line 2
     ——▶ justfile:2:7
      │
    2 │ alias foo := baz
      │       ^^^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: unknown_alias_target,
  justfile: "alias foo := bar\n",
  stderr: "
    error: Alias `foo` has an unknown target `bar`
     ——▶ justfile:1:7
      │
    1 │ alias foo := bar
      │       ^^^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: alias_shadows_recipe,
  justfile: "bar:\n  echo bar\nalias foo := bar\nfoo:\n  echo foo",
  stderr: "
    error: Alias `foo` defined on line 3 is redefined as a recipe on line 4
     ——▶ justfile:4:1
      │
    4 │ foo:
      │ ^^^
  ",
  status: EXIT_FAILURE,
}

test! {
  name:     default,
  justfile: "default:\n echo hello\nother: \n echo bar",
  stdout:   "hello\n",
  stderr:   "echo hello\n",
}

test! {
  name:     quiet,
  justfile: "default:\n @echo hello",
  stdout:   "hello\n",
}

test! {
  name:     verbose,
  justfile: "default:\n @echo hello",
  args:     ("--verbose"),
  stdout:   "hello\n",
  stderr:   "===> Running recipe `default`...\necho hello\n",
}

test! {
  name:     order,
  justfile: "
b: a
  echo b
  @mv a b

a:
  echo a
  @touch F
  @touch a

d: c
  echo d
  @rm c

c: b
  echo c
  @mv b c",
  args:     ("a", "d"),
  stdout:   "a\nb\nc\nd\n",
  stderr:   "echo a\necho b\necho c\necho d\n",
}

test! {
  name:     select,
  justfile: "b:
  @echo b
a:
  @echo a
d:
  @echo d
c:
  @echo c",
  args:     ("d", "c"),
  stdout:   "d\nc\n",
}

test! {
  name:     print,
  justfile: "b:
  echo b
a:
  echo a
d:
  echo d
c:
  echo c",
  args:     ("d", "c"),
  stdout:   "d\nc\n",
  stderr:   "echo d\necho c\n",
}

test! {
  name:     status_passthrough,
  justfile: "

hello:

recipe:
  @exit 100",
  args:     ("recipe"),
  stderr:   "error: Recipe `recipe` failed on line 5 with exit code 100\n",
  status:   100,
}

test! {
  name:     unknown_dependency,
  justfile: "bar:\nhello:\nfoo: bar baaaaaaaz hello",
  stderr:   "
    error: Recipe `foo` has unknown dependency `baaaaaaaz`
     ——▶ justfile:3:10
      │
    3 │ foo: bar baaaaaaaz hello
      │          ^^^^^^^^^
  ",
  status:   EXIT_FAILURE,
}

test! {
  name:     backtick_success,
  justfile: "a := `printf Hello,`\nbar:\n printf '{{a + `printf ' world.'`}}'",
  stdout:   "Hello, world.",
  stderr:   "printf 'Hello, world.'\n",
}

test! {
  name:     backtick_trimming,
  justfile: "a := `echo Hello,`\nbar:\n echo '{{a + `echo ' world.'`}}'",
  stdout:   "Hello, world.\n",
  stderr:   "echo 'Hello, world.'\n",
}

test! {
  name:     backtick_code_assignment,
  justfile: "b := a\na := `exit 100`\nbar:\n echo '{{`exit 200`}}'",
  stderr:   "
    error: Backtick failed with exit code 100
     ——▶ justfile:2:6
      │
    2 │ a := `exit 100`
      │      ^^^^^^^^^^
  ",
  status:   100,
}

test! {
  name:     backtick_code_interpolation,
  justfile: "b := a\na := `echo hello`\nbar:\n echo '{{`exit 200`}}'",
  stderr:   "
    error: Backtick failed with exit code 200
     ——▶ justfile:4:10
      │
    4 │  echo '{{`exit 200`}}'
      │          ^^^^^^^^^^
  ",
  status:   200,
}

test! {
  name:     backtick_code_interpolation_mod,
  justfile: "f:\n 無{{`exit 200`}}",
  stderr:   "
    error: Backtick failed with exit code 200
     ——▶ justfile:2:7
      │
    2 │  無{{`exit 200`}}
      │      ^^^^^^^^^^
  ",
  status:   200,
}

test! {
  name:     backtick_code_interpolation_tab,
  justfile: "
    backtick-fail:
    \techo {{`exit 200`}}
  ",
  stderr:   "    error: Backtick failed with exit code 200
     ——▶ justfile:2:9
      │
    2 │     echo {{`exit 200`}}
      │            ^^^^^^^^^^
  ",
  status:   200,
}

test! {
  name:     backtick_code_interpolation_tabs,
  justfile: "
    backtick-fail:
    \techo {{\t`exit 200`}}
  ",
  stderr:   "error: Backtick failed with exit code 200
 ——▶ justfile:2:10
  │
2 │     echo {{    `exit 200`}}
  │                ^^^^^^^^^^
",
  status:   200,
}

test! {
  name:     backtick_code_interpolation_inner_tab,
  justfile: "
    backtick-fail:
    \techo {{\t`exit\t\t200`}}
  ",
  stderr:   "
    error: Backtick failed with exit code 200
     ——▶ justfile:2:10
      │
    2 │     echo {{    `exit        200`}}
      │                ^^^^^^^^^^^^^^^^^
  ",
  status:   200,
}

test! {
  name:     backtick_code_interpolation_leading_emoji,
  justfile: "
    backtick-fail:
    \techo 😬{{`exit 200`}}
  ",
  stderr: "
    error: Backtick failed with exit code 200
     ——▶ justfile:2:13
      │
    2 │     echo 😬{{`exit 200`}}
      │              ^^^^^^^^^^
  ",
  status:   200,
}

test! {
  name:     backtick_code_interpolation_unicode_hell,
  justfile: "
    backtick-fail:
    \techo \t\t\t😬鎌鼬{{\t\t`exit 200 # \t\t\tabc`}}\t\t\t😬鎌鼬
  ",
  stderr: "
    error: Backtick failed with exit code 200
     ——▶ justfile:2:24
      │
    2 │     echo             😬鎌鼬{{        `exit 200 #             abc`}}            😬鎌鼬
      │                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ",
  status:   200,
}

test! {
  name:     backtick_code_long,
  justfile: "






    b := a
    a := `echo hello`
    bar:
     echo '{{`exit 200`}}'
  ",
  stderr:   "
    error: Backtick failed with exit code 200
      ——▶ justfile:10:10
       │
    10 │  echo '{{`exit 200`}}'
       │          ^^^^^^^^^^
  ",
  status:   200,
}

test! {
  name:     shebang_backtick_failure,
  justfile: "foo:
 #!/bin/sh
 echo hello
 echo {{`exit 123`}}",
  stdout:   "",
  stderr:   "
    error: Backtick failed with exit code 123
     ——▶ justfile:4:9
      │
    4 │  echo {{`exit 123`}}
      │         ^^^^^^^^^^
  ",
  status:   123,
}

test! {
  name:     command_backtick_failure,
  justfile: "foo:
 echo hello
 echo {{`exit 123`}}",
  stdout:   "hello\n",
  stderr:   "
    echo hello
    error: Backtick failed with exit code 123
     ——▶ justfile:3:9
      │
    3 │  echo {{`exit 123`}}
      │         ^^^^^^^^^^
  ",
  status:   123,
}

test! {
  name:     assignment_backtick_failure,
  justfile: "foo:
 echo hello
 echo {{`exit 111`}}
a := `exit 222`",
  stdout:   "",
  stderr:   "
    error: Backtick failed with exit code 222
     ——▶ justfile:4:6
      │
    4 │ a := `exit 222`
      │      ^^^^^^^^^^
  ",
  status:   222,
}

test! {
  name:     unknown_override_options,
  justfile: "foo:
 echo hello
 echo {{`exit 111`}}
a := `exit 222`",
  args:     ("--set", "foo", "bar", "--set", "baz", "bob", "--set", "a", "b", "a", "b"),
  stderr:   "error: Variables `baz` and `foo` overridden on the command line but not present \
    in justfile\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     unknown_override_args,
  justfile: "foo:
 echo hello
 echo {{`exit 111`}}
a := `exit 222`",
  args:     ("foo=bar", "baz=bob", "a=b", "a", "b"),
  stderr:   "error: Variables `baz` and `foo` overridden on the command line but not present \
    in justfile\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     unknown_override_arg,
  justfile: "foo:
 echo hello
 echo {{`exit 111`}}
a := `exit 222`",
  args:     ("foo=bar", "a=b", "a", "b"),
  stderr:   "error: Variable `foo` overridden on the command line but not present in justfile\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     overrides_first,
  justfile: r#"
foo := "foo"
a := "a"
baz := "baz"

recipe arg:
 echo arg={{arg}}
 echo {{foo + a + baz}}"#,
  args:     ("foo=bar", "a=b", "recipe", "baz=bar"),
  stdout:   "arg=baz=bar\nbarbbaz\n",
  stderr:   "echo arg=baz=bar\necho barbbaz\n",
}

test! {
  name:     overrides_not_evaluated,
  justfile: r#"
foo := `exit 1`
a := "a"
baz := "baz"

recipe arg:
 echo arg={{arg}}
 echo {{foo + a + baz}}"#,
  args:     ("foo=bar", "a=b", "recipe", "baz=bar"),
  stdout:   "arg=baz=bar\nbarbbaz\n",
  stderr:   "echo arg=baz=bar\necho barbbaz\n",
}

test! {
  name:     dry_run,
  justfile: r"
var := `echo stderr 1>&2; echo backtick`

command:
  @touch /this/is/not/a/file
  {{var}}
  echo {{`echo command interpolation`}}

shebang:
  #!/bin/sh
  touch /this/is/not/a/file
  {{var}}
  echo {{`echo shebang interpolation`}}",
  args:     ("--dry-run", "shebang", "command"),
  stdout:   "",
  stderr:   "#!/bin/sh
touch /this/is/not/a/file
`echo stderr 1>&2; echo backtick`
echo `echo shebang interpolation`
touch /this/is/not/a/file
`echo stderr 1>&2; echo backtick`
echo `echo command interpolation`
",
}

test! {
  name:     line_error_spacing,
  justfile: r"









^^^
",
  stdout:   "",
  stderr:   "error: Unknown start of token:
  ——▶ justfile:10:1
   │
10 │ ^^^
   │ ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     argument_single,
  justfile: "
foo A:
  echo {{A}}
    ",
  args:     ("foo", "ARGUMENT"),
  stdout:   "ARGUMENT\n",
  stderr:   "echo ARGUMENT\n",
}

test! {
  name:     argument_multiple,
  justfile: "
foo A B:
  echo A:{{A}} B:{{B}}
    ",
  args:     ("foo", "ONE", "TWO"),
  stdout:   "A:ONE B:TWO\n",
  stderr:   "echo A:ONE B:TWO\n",
}

test! {
  name:     argument_mismatch_more,
  justfile: "
foo A B:
  echo A:{{A}} B:{{B}}
    ",
  args:     ("foo", "ONE", "TWO", "THREE"),
  stdout:   "",
  stderr:   "error: Justfile does not contain recipe `THREE`\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     argument_mismatch_fewer,
  justfile: "
foo A B:
  echo A:{{A}} B:{{B}}
    ",
  args:     ("foo", "ONE"),
  stdout:   "",
  stderr:   "error: Recipe `foo` got 1 argument but takes 2\nusage:\n    just foo A B\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     argument_mismatch_more_with_default,
  justfile: "
foo A B='B':
  echo A:{{A}} B:{{B}}
    ",
  args:     ("foo", "ONE", "TWO", "THREE"),
  stdout:   "",
  stderr:   "error: Justfile does not contain recipe `THREE`\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     argument_mismatch_fewer_with_default,
  justfile: "
foo A B C='C':
  echo A:{{A}} B:{{B}} C:{{C}}
    ",
  args:     ("foo", "bar"),
  stdout:   "",
  stderr:   "
    error: Recipe `foo` got 1 argument but takes at least 2
    usage:
        just foo A B C='C'
  ",
  status:   EXIT_FAILURE,
}

test! {
  name:     unknown_recipe,
  justfile: "hello:",
  args:     ("foo"),
  stdout:   "",
  stderr:   "error: Justfile does not contain recipe `foo`\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     unknown_recipes,
  justfile: "hello:",
  args:     ("foo", "bar"),
  stdout:   "",
  stderr:   "error: Justfile does not contain recipe `foo`\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     color_always,
  justfile: "b := a\na := `exit 100`\nbar:\n echo '{{`exit 200`}}'",
  args:     ("--color", "always"),
  stdout:   "",
  stderr:   "\u{1b}[1;31merror\u{1b}[0m: \u{1b}[1mBacktick failed with exit code 100\u{1b}[0m\n \u{1b}[1;34m——▶\u{1b}[0m justfile:2:6\n  \u{1b}[1;34m│\u{1b}[0m\n\u{1b}[1;34m2 │\u{1b}[0m a := `exit 100`\n  \u{1b}[1;34m│\u{1b}[0m      \u{1b}[1;31m^^^^^^^^^^\u{1b}[0m\n",
  status:   100,
}

test! {
  name:     color_never,
  justfile: "b := a\na := `exit 100`\nbar:\n echo '{{`exit 200`}}'",
  args:     ("--color", "never"),
  stdout:   "",
  stderr:   "error: Backtick failed with exit code 100
 ——▶ justfile:2:6
  │
2 │ a := `exit 100`
  │      ^^^^^^^^^^
",
  status:   100,
}

test! {
  name:     color_auto,
  justfile: "b := a\na := `exit 100`\nbar:\n echo '{{`exit 200`}}'",
  args:     ("--color", "auto"),
  stdout:   "",
  stderr:   "error: Backtick failed with exit code 100
 ——▶ justfile:2:6
  │
2 │ a := `exit 100`
  │      ^^^^^^^^^^
",
  status:   100,
}

test! {
  name:     colors_no_context,
  justfile: "
recipe:
  @exit 100",
  args:     ("--color=always"),
  stdout:   "",
  stderr:   "\u{1b}[1;31merror\u{1b}[0m: \u{1b}[1m\
Recipe `recipe` failed on line 2 with exit code 100\u{1b}[0m\n",
  status:   100,
}

test! {
  name:     dump,
  justfile: r"
# this recipe does something
recipe a b +d:
 @exit 100",
  args:     ("--dump"),
  stdout:   "# this recipe does something
recipe a b +d:
    @exit 100
",
}

test! {
  name:     mixed_whitespace,
  justfile: "bar:\n\t echo hello",
  stdout:   "",
  stderr:   "error: Found a mix of tabs and spaces in leading whitespace: `␉␠`
Leading whitespace may consist of tabs or spaces, but not both
 ——▶ justfile:2:1
  │
2 │      echo hello
  │ ^^^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     extra_leading_whitespace,
  justfile: "bar:\n\t\techo hello\n\t\t\techo goodbye",
  stdout:   "",
  stderr:   "error: Recipe line has extra leading whitespace
 ——▶ justfile:3:3
  │
3 │             echo goodbye
  │         ^^^^^^^^^^^^^^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     inconsistent_leading_whitespace,
  justfile: "bar:\n\t\techo hello\n\t echo goodbye",
  stdout:   "",
  stderr:   "error: Recipe line has inconsistent leading whitespace. \
            Recipe started with `␉␉` but found line with `␉␠`
 ——▶ justfile:3:1
  │
3 │      echo goodbye
  │ ^^^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     required_after_default,
  justfile: "bar:\nhello baz arg='foo' bar:",
  stdout:   "",
  stderr:   "error: Non-default parameter `bar` follows default parameter
 ——▶ justfile:2:21
  │
2 │ hello baz arg='foo' bar:
  │                     ^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     required_after_plus_variadic,
  justfile: "bar:\nhello baz +arg bar:",
  stdout:   "",
  stderr:   "error: Parameter `bar` follows variadic parameter
 ——▶ justfile:2:16
  │
2 │ hello baz +arg bar:
  │                ^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     required_after_star_variadic,
  justfile: "bar:\nhello baz *arg bar:",
  stdout:   "",
  stderr:   "error: Parameter `bar` follows variadic parameter
 ——▶ justfile:2:16
  │
2 │ hello baz *arg bar:
  │                ^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     use_string_default,
  justfile: r#"
bar:
hello baz arg="XYZ\t\"	":
  echo '{{baz}}...{{arg}}'
"#,
  args:     ("hello", "ABC"),
  stdout:   "ABC...XYZ\t\"\t\n",
  stderr:   "echo 'ABC...XYZ\t\"\t'\n",
}

test! {
  name:     use_raw_string_default,
  justfile: r#"
bar:
hello baz arg='XYZ"	':
  printf '{{baz}}...{{arg}}'
"#,
  args:     ("hello", "ABC"),
  stdout:   "ABC...XYZ\"\t",
  stderr:   "printf 'ABC...XYZ\"\t'\n",
}

test! {
  name:     supply_use_default,
  justfile: r"
hello a b='B' c='C':
  echo {{a}} {{b}} {{c}}
",
  args:     ("hello", "0", "1"),
  stdout:   "0 1 C\n",
  stderr:   "echo 0 1 C\n",
}

test! {
  name:     supply_defaults,
  justfile: r"
hello a b='B' c='C':
  echo {{a}} {{b}} {{c}}
",
  args:     ("hello", "0", "1", "2"),
  stdout:   "0 1 2\n",
  stderr:   "echo 0 1 2\n",
}

test! {
  name:     list,
  justfile: r#"

# this does a thing
hello a b='B	' c='C':
  echo {{a}} {{b}} {{c}}

# this comment will be ignored

a Z="\t z":

# this recipe will not appear
_private-recipe:
"#,
  args:     ("--list"),
  stdout:   r#"
    Available recipes:
        a Z="\t z"
        hello a b='B	' c='C' # this does a thing
  "#,
}

test! {
  name:     list_alignment,
  justfile: r#"

# this does a thing
hello a b='B	' c='C':
  echo {{a}} {{b}} {{c}}

# something else
a Z="\t z":

# this recipe will not appear
_private-recipe:
"#,
  args:     ("--list"),
  stdout:   r#"
    Available recipes:
        a Z="\t z"           # something else
        hello a b='B	' c='C' # this does a thing
  "#,
}

test! {
  name:     list_alignment_long,
  justfile: r#"

# this does a thing
hello a b='B	' c='C':
  echo {{a}} {{b}} {{c}}

# this does another thing
x a b='B	' c='C':
  echo {{a}} {{b}} {{c}}

# something else
this-recipe-is-very-very-very-very-very-very-very-very-important Z="\t z":

# this recipe will not appear
_private-recipe:
"#,
  args:     ("--list"),
  stdout:   r#"
    Available recipes:
        hello a b='B	' c='C' # this does a thing
        this-recipe-is-very-very-very-very-very-very-very-very-important Z="\t z" # something else
        x a b='B	' c='C'     # this does another thing
  "#,
}

test! {
  name:     list_sorted,
  justfile: r"
alias c := b
b:
a:
",
  args:     ("--list"),
  stdout:   r"
    Available recipes:
        a
        b # [alias: c]
  ",
}

test! {
  name:     list_unsorted,
  justfile: r"
alias c := b
b:
a:
",
  args:     ("--list", "--unsorted"),
  stdout:   r"
    Available recipes:
        b # [alias: c]
        a
  ",
}

test! {
  name:     list_heading,
  justfile: r"
a:
b:
",
  args:     ("--list", "--list-heading", "Cool stuff…\n"),
  stdout:   r"
    Cool stuff…
        a
        b
  ",
}

test! {
  name:     list_prefix,
  justfile: r"
a:
b:
",
  args:     ("--list", "--list-prefix", "····"),
  stdout:   r"
    Available recipes:
    ····a
    ····b
  ",
}

test! {
  name:     list_empty_prefix_and_heading,
  justfile: r"
a:
b:
",
  args:     ("--list", "--list-heading", "", "--list-prefix", ""),
  stdout:   r"
    a
    b
  ",
}

test! {
  name:     run_suggestion,
  justfile: r#"
hello a b='B	' c='C':
  echo {{a}} {{b}} {{c}}

a Z="\t z":
"#,
  args:     ("hell"),
  stdout:   "",
  stderr:   "error: Justfile does not contain recipe `hell`\nDid you mean `hello`?\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     line_continuation_with_space,
  justfile: r"
foo:
  echo a\
         b  \
             c
",
  stdout:   "ab c\n",
  stderr:   "echo ab  c\n",
}

test! {
  name:     line_continuation_with_quoted_space,
  justfile: r"
foo:
  echo 'a\
         b  \
             c'
",
  stdout:   "ab  c\n",
  stderr:   "echo 'ab  c'\n",
}

test! {
  name:     line_continuation_no_space,
  justfile: r"
foo:
  echo a\
  b\
  c
",
  stdout:   "abc\n",
  stderr:   "echo abc\n",
}

test! {
  name: infallible_command,
  justfile: r"
infallible:
  -exit 101
",
  stderr: "exit 101\n",
  status: EXIT_SUCCESS,
}

test! {
  name: infallible_with_failing,
  justfile: r"
infallible:
  -exit 101
  exit 202
",
  stderr: r"exit 101
exit 202
error: Recipe `infallible` failed on line 3 with exit code 202
",
  status: 202,
}

test! {
  name:     quiet_recipe,
  justfile: r"
@quiet:
  # a
  # b
  @echo c
",
  stdout:   "c\n",
  stderr:   "echo c\n",
}

test! {
  name:     quiet_shebang_recipe,
  justfile: r"
@quiet:
  #!/bin/sh
  echo hello
",
  stdout:   "hello\n",
  stderr:   "#!/bin/sh\necho hello\n",
}

test! {
  name:     complex_dependencies,
  justfile: r"
a: b
b:
c: b a
",
  args:     ("b"),
  stdout:   "",
}

test! {
  name:     unknown_function_in_assignment,
  justfile: r#"foo := foo() + "hello"
bar:"#,
  args:     ("bar"),
  stdout:   "",
  stderr:   r#"error: Call to unknown function `foo`
 ——▶ justfile:1:8
  │
1 │ foo := foo() + "hello"
  │        ^^^
"#,
  status:   EXIT_FAILURE,
}

test! {
  name:     dependency_takes_arguments_exact,
  justfile: "
    a FOO:
    b: a
  ",
  args:     ("b"),
  stdout:   "",
  stderr:   "error: Dependency `a` got 0 arguments but takes 1 argument
 ——▶ justfile:2:4
  │
2 │ b: a
  │    ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     dependency_takes_arguments_at_least,
  justfile: "
    a FOO LUZ='hello':
    b: a
  ",
  args:     ("b"),
  stdout:   "",
  stderr:   "error: Dependency `a` got 0 arguments but takes at least 1 argument
 ——▶ justfile:2:4
  │
2 │ b: a
  │    ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     dependency_takes_arguments_at_most,
  justfile: "
    a FOO LUZ='hello':
    b: (a '0' '1' '2')
  ",
  args:     ("b"),
  stdout:   "",
  stderr:   "error: Dependency `a` got 3 arguments but takes at most 2 arguments
 ——▶ justfile:2:5
  │
2 │ b: (a '0' '1' '2')
  │     ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     duplicate_parameter,
  justfile: "a foo foo:",
  args:     ("a"),
  stdout:   "",
  stderr:   "error: Recipe `a` has duplicate parameter `foo`
 ——▶ justfile:1:7
  │
1 │ a foo foo:
  │       ^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     duplicate_recipe,
  justfile: "b:\nb:",
  args:     ("b"),
  stdout:   "",
  stderr:   "error: Recipe `b` first defined on line 1 is redefined on line 2
 ——▶ justfile:2:1
  │
2 │ b:
  │ ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     duplicate_variable,
  justfile: "a := 'hello'\na := 'hello'\nfoo:",
  args:     ("foo"),
  stdout:   "",
  stderr:   "error: Variable `a` has multiple definitions
 ——▶ justfile:2:1
  │
2 │ a := 'hello'
  │ ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     unexpected_token_in_dependency_position,
  justfile: "foo: 'bar'",
  args:     ("foo"),
  stdout:   "",
  stderr:   "error: Expected '&&', comment, end of file, end of line, \
    identifier, or '(', but found string
 ——▶ justfile:1:6
  │
1 │ foo: 'bar'
  │      ^^^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     unexpected_token_after_name,
  justfile: "foo 'bar'",
  args:     ("foo"),
  stdout:   "",
  stderr:   "error: Expected '*', ':', '$', identifier, or '+', but found string
 ——▶ justfile:1:5
  │
1 │ foo 'bar'
  │     ^^^^^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     self_dependency,
  justfile: "a: a",
  args:     ("a"),
  stdout:   "",
  stderr:   "error: Recipe `a` depends on itself
 ——▶ justfile:1:4
  │
1 │ a: a
  │    ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     long_circular_recipe_dependency,
  justfile: "a: b\nb: c\nc: d\nd: a",
  args:     ("a"),
  stdout:   "",
  stderr:   "error: Recipe `d` has circular dependency `a -> b -> c -> d -> a`
 ——▶ justfile:4:4
  │
4 │ d: a
  │    ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     variable_self_dependency,
  justfile: "z := z\na:",
  args:     ("a"),
  stdout:   "",
  stderr:   "error: Variable `z` is defined in terms of itself
 ——▶ justfile:1:1
  │
1 │ z := z
  │ ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     variable_circular_dependency,
  justfile: "x := y\ny := z\nz := x\na:",
  args:     ("a"),
  stdout:   "",
  stderr:   "error: Variable `x` depends on its own value: `x -> y -> z -> x`
 ——▶ justfile:1:1
  │
1 │ x := y
  │ ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     variable_circular_dependency_with_additional_variable,
  justfile: "
    a := ''
    x := y
    y := x

    a:
  ",
  args:     ("a"),
  stdout:   "",
  stderr:   "error: Variable `x` depends on its own value: `x -> y -> x`
 ——▶ justfile:2:1
  │
2 │ x := y
  │ ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     plus_variadic_recipe,
  justfile: "
a x y +z:
  echo {{x}} {{y}} {{z}}
",
  args:     ("a", "0", "1", "2", "3", " 4 "),
  stdout:   "0 1 2 3 4\n",
  stderr:   "echo 0 1 2 3  4 \n",
}

test! {
  name:     plus_variadic_ignore_default,
  justfile: "
a x y +z='HELLO':
  echo {{x}} {{y}} {{z}}
",
  args:     ("a", "0", "1", "2", "3", " 4 "),
  stdout:   "0 1 2 3 4\n",
  stderr:   "echo 0 1 2 3  4 \n",
}

test! {
  name:     plus_variadic_use_default,
  justfile: "
a x y +z='HELLO':
  echo {{x}} {{y}} {{z}}
",
  args:     ("a", "0", "1"),
  stdout:   "0 1 HELLO\n",
  stderr:   "echo 0 1 HELLO\n",
}

test! {
  name:     plus_variadic_too_few,
  justfile: "
a x y +z:
  echo {{x}} {{y}} {{z}}
",
  args:     ("a", "0", "1"),
  stdout:   "",
  stderr:   "error: Recipe `a` got 2 arguments but takes at least 3\nusage:\n    just a x y +z\n",
  status:   EXIT_FAILURE,
}

test! {
  name:     star_variadic_recipe,
  justfile: "
a x y *z:
  echo {{x}} {{y}} {{z}}
",
  args:     ("a", "0", "1", "2", "3", " 4 "),
  stdout:   "0 1 2 3 4\n",
  stderr:   "echo 0 1 2 3  4 \n",
}

test! {
  name:     star_variadic_none,
  justfile: "
a x y *z:
  echo {{x}} {{y}} {{z}}
",
  args:     ("a", "0", "1"),
  stdout:   "0 1\n",
  stderr:   "echo 0 1 \n",
}

test! {
  name:     star_variadic_ignore_default,
  justfile: "
a x y *z='HELLO':
  echo {{x}} {{y}} {{z}}
",
  args:     ("a", "0", "1", "2", "3", " 4 "),
  stdout:   "0 1 2 3 4\n",
  stderr:   "echo 0 1 2 3  4 \n",
}

test! {
  name:     star_variadic_use_default,
  justfile: "
a x y *z='HELLO':
  echo {{x}} {{y}} {{z}}
",
  args:     ("a", "0", "1"),
  stdout:   "0 1 HELLO\n",
  stderr:   "echo 0 1 HELLO\n",
}

test! {
  name:     star_then_plus_variadic,
  justfile: "
foo *a +b:
  echo {{a}} {{b}}
",
  stdout:   "",
  stderr:   "error: Expected \':\' or \'=\', but found \'+\'
 ——▶ justfile:1:8
  │
1 │ foo *a +b:
  │        ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     plus_then_star_variadic,
  justfile: "
foo +a *b:
  echo {{a}} {{b}}
",
  stdout:   "",
  stderr:   "error: Expected \':\' or \'=\', but found \'*\'
 ——▶ justfile:1:8
  │
1 │ foo +a *b:
  │        ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     argument_grouping,
  justfile: "
FOO A B='blarg':
  echo foo: {{A}} {{B}}

BAR X:
  echo bar: {{X}}

BAZ +Z:
  echo baz: {{Z}}
",
  args:     ("BAR", "0", "FOO", "1", "2", "BAZ", "3", "4", "5"),
  stdout:   "bar: 0\nfoo: 1 2\nbaz: 3 4 5\n",
  stderr:   "echo bar: 0\necho foo: 1 2\necho baz: 3 4 5\n",
}

test! {
  name:     missing_second_dependency,
  justfile: "
x:

a: x y
",
  stdout:   "",
  stderr:   "error: Recipe `a` has unknown dependency `y`
 ——▶ justfile:3:6
  │
3 │ a: x y
  │      ^
",
  status:   EXIT_FAILURE,
}

test! {
  name:     list_colors,
  justfile: "
# comment
a B C +D='hello':
  echo {{B}} {{C}} {{D}}
",
  args:     ("--color", "always", "--list"),
  stdout:   "
    Available recipes:
        a \
    \u{1b}[36mB\u{1b}[0m \u{1b}[36mC\u{1b}[0m \u{1b}[35m+\
    \u{1b}[0m\u{1b}[36mD\u{1b}[0m=\u{1b}[32m'hello'\u{1b}[0m \
     \u{1b}[34m#\u{1b}[0m \u{1b}[34mcomment\u{1b}[0m
  ",
}

test! {
  name:     run_colors,
  justfile: "
# comment
a:
  echo hi
",
  args:     ("--color", "always", "--highlight", "--verbose"),
  stdout:   "hi\n",
  stderr:   "\u{1b}[1;36m===> Running recipe `a`...\u{1b}[0m\n\u{1b}[1mecho hi\u{1b}[0m\n",
}

test! {
  name:     no_highlight,
  justfile: "
# comment
a:
  echo hi
",
  args:     ("--color", "always", "--highlight", "--no-highlight", "--verbose"),
  stdout:   "hi\n",
  stderr:   "\u{1b}[1;36m===> Running recipe `a`...\u{1b}[0m\necho hi\n",
}

test! {
  name:     trailing_flags,
  justfile: "
echo A B C:
  echo {{A}} {{B}} {{C}}
",
  args:     ("echo", "--some", "--awesome", "--flags"),
  stdout:   "--some --awesome --flags\n",
  stderr:   "echo --some --awesome --flags\n",
}

test! {
   name:     comment_before_variable,
   justfile: "
#
A:='1'
echo:
  echo {{A}}
 ",
   args:     ("echo"),
   stdout:   "1\n",
   stderr:   "echo 1\n",
}

test! {
   name:     invalid_escape_sequence_message,
   justfile: r#"
X := "\'"
"#,
   stdout:   "",
   stderr:   r#"error: `\'` is not a valid escape sequence
 ——▶ justfile:1:6
  │
1 │ X := "\'"
  │      ^^^^
"#,
   status:   EXIT_FAILURE,
}

test! {
   name:     unknown_variable_in_default,
   justfile: "
     foo x=bar:
   ",
   stdout:   "",
   stderr:   r"error: Variable `bar` not defined
 ——▶ justfile:1:7
  │
1 │ foo x=bar:
  │       ^^^
",
   status:   EXIT_FAILURE,
}

test! {
   name:     unknown_function_in_default,
   justfile: "
foo x=bar():
",
   stdout:   "",
   stderr:   r"error: Call to unknown function `bar`
 ——▶ justfile:1:7
  │
1 │ foo x=bar():
  │       ^^^
",
   status:   EXIT_FAILURE,
}

test! {
   name:     default_string,
   justfile: "
foo x='bar':
  echo {{x}}
",
   stdout:   "bar\n",
   stderr:   "echo bar\n",
}

test! {
   name:     default_concatenation,
   justfile: "
foo x=(`echo foo` + 'bar'):
  echo {{x}}
",
   stdout:   "foobar\n",
   stderr:   "echo foobar\n",
}

test! {
   name:     default_backtick,
   justfile: "
foo x=`echo foo`:
  echo {{x}}
",
   stdout:   "foo\n",
   stderr:   "echo foo\n",
}

test! {
   name:     default_variable,
   justfile: "
y := 'foo'
foo x=y:
  echo {{x}}
",
   stdout:   "foo\n",
   stderr:   "echo foo\n",
}

test! {
  name:     unterminated_interpolation_eol,
  justfile: "
    foo:
      echo {{
  ",
  stderr:   r"
    error: Unterminated interpolation
     ——▶ justfile:2:8
      │
    2 │   echo {{
      │        ^^
  ",
  status:   EXIT_FAILURE,
}

test! {
  name:     unterminated_interpolation_eof,
  justfile: "
    foo:
      echo {{
  ",
  stderr:   r"
    error: Unterminated interpolation
     ——▶ justfile:2:8
      │
    2 │   echo {{
      │        ^^
  ",
  status:   EXIT_FAILURE,
}

test! {
  name:     unknown_start_of_token,
  justfile: "
assembly_source_files = %(wildcard src/arch/$(arch)/*.s)
",
  stderr:   r"
    error: Unknown start of token:
     ——▶ justfile:1:25
      │
    1 │ assembly_source_files = %(wildcard src/arch/$(arch)/*.s)
      │                         ^
  ",
   status:   EXIT_FAILURE,
}

test! {
  name:     backtick_variable_cat,
  justfile: "
stdin := `cat`

default:
  echo {{stdin}}
",
  stdin:    "STDIN",
  stdout:   "STDIN\n",
  stderr:   "echo STDIN\n",
}

test! {
   name:     backtick_default_cat_stdin,
   justfile: "
default stdin = `cat`:
  echo {{stdin}}
",
   stdin:    "STDIN",
   stdout:   "STDIN\n",
   stderr:   "echo STDIN\n",
}

test! {
  name:     backtick_default_cat_justfile,
  justfile: "
    default stdin = `cat justfile`:
      echo '{{stdin}}'
  ",
  stdout:   "
    default stdin = `cat justfile`:
      echo {{stdin}}
  ",
  stderr:   "
    echo 'default stdin = `cat justfile`:
      echo '{{stdin}}''
  ",
}

test! {
   name:     backtick_variable_read_single,
   justfile: "
password := `read PW && echo $PW`

default:
  echo {{password}}
",
   stdin:    "foobar\n",
   stdout:   "foobar\n",
   stderr:   "echo foobar\n",
}

test! {
   name:     backtick_variable_read_multiple,
   justfile: "
a := `read A && echo $A`
b := `read B && echo $B`

default:
  echo {{a}}
  echo {{b}}
",
   stdin:    "foo\nbar\n",
   stdout:   "foo\nbar\n",
   stderr:   "echo foo\necho bar\n",
}

test! {
   name:     backtick_default_read_multiple,
   justfile: "

default a=`read A && echo $A` b=`read B && echo $B`:
  echo {{a}}
  echo {{b}}
",
   stdin:    "foo\nbar\n",
   stdout:   "foo\nbar\n",
   stderr:   "echo foo\necho bar\n",
}

test! {
  name: old_equals_assignment_syntax_produces_error,
  justfile: "
    foo = 'bar'

    default:
      echo {{foo}}
  ",
  stderr: "
    error: Expected '*', ':', '$', identifier, or '+', but found '='
     ——▶ justfile:1:5
      │
    1 │ foo = 'bar'
      │     ^
    ",
  status: EXIT_FAILURE,
}

test! {
  name: dependency_argument_string,
  justfile: "
    release: (build 'foo') (build 'bar')

    build target:
      echo 'Building {{target}}...'
  ",
  args: (),
  stdout: "Building foo...\nBuilding bar...\n",
  stderr: "echo 'Building foo...'\necho 'Building bar...'\n",
  shell: false,
}

test! {
  name: dependency_argument_parameter,
  justfile: "
    default: (release '1.0')

    release version: (build 'foo' version) (build 'bar' version)

    build target version:
      echo 'Building {{target}}@{{version}}...'
  ",
  args: (),
  stdout: "Building foo@1.0...\nBuilding bar@1.0...\n",
  stderr: "echo 'Building foo@1.0...'\necho 'Building bar@1.0...'\n",
  shell: false,
}

test! {
  name: dependency_argument_function,
  justfile: "
    foo: (bar env_var_or_default('x', 'y'))

    bar arg:
      echo {{arg}}
  ",
  args: (),
  stdout: "y\n",
  stderr: "echo y\n",
  shell: false,
}

test! {
  name: env_function_as_env_var,
  justfile: "
    foo: (bar env('x'))

    bar arg:
      echo {{arg}}
  ",
  args: (),
  env: { "x": "z", },
  stdout: "z\n",
  stderr: "echo z\n",
  shell: false,
}

test! {
  name: env_function_as_env_var_or_default,
  justfile: "
    foo: (bar env('x', 'y'))

    bar arg:
      echo {{arg}}
  ",
  args: (),
  env: { "x": "z", },
  stdout: "z\n",
  stderr: "echo z\n",
  shell: false,
}

test! {
  name: env_function_as_env_var_with_existing_env_var,
  justfile: "
    foo: (bar env('x'))

    bar arg:
      echo {{arg}}
  ",
  args: (),
  env: { "x": "z", },
  stdout: "z\n",
  stderr: "echo z\n",
  shell: false,
}

test! {
  name: env_function_as_env_var_or_default_with_existing_env_var,
  justfile: "
    foo: (bar env('x', 'y'))

    bar arg:
      echo {{arg}}
  ",
  args: (),
  env: { "x": "z", },
  stdout: "z\n",
  stderr: "echo z\n",
  shell: false,
}

test! {
  name: dependency_argument_backtick,
  justfile: "
    export X := 'X'

    foo: (bar `echo $X`)

    bar arg:
      echo {{arg}}
      echo $X
  ",
  args: (),
  stdout: "X\nX\n",
  stderr: "echo X\necho $X\n",
  shell: false,
}

test! {
  name: dependency_argument_assignment,
  justfile: "
    v := '1.0'

    default: (release v)

    release version:
      echo Release {{version}}...
  ",
  args: (),
  stdout: "Release 1.0...\n",
  stderr: "echo Release 1.0...\n",
  shell: false,
}

test! {
  name: dependency_argument_plus_variadic,
  justfile: "
    foo: (bar 'A' 'B' 'C')

    bar +args:
      echo {{args}}
  ",
  args: (),
  stdout: "A B C\n",
  stderr: "echo A B C\n",
  shell: false,
}

test! {
  name: duplicate_dependency_no_args,
  justfile: "
    foo: bar bar bar bar

    bar:
      echo BAR
  ",
  args: (),
  stdout: "BAR\n",
  stderr: "echo BAR\n",
  shell: false,
}

test! {
  name: duplicate_dependency_argument,
  justfile: "
    foo: (bar 'BAR') (bar `echo BAR`)

    bar bar:
      echo {{bar}}
  ",
  args: (),
  stdout: "BAR\n",
  stderr: "echo BAR\n",
  shell: false,
}

#[cfg(windows)]
test! {
  name: pwsh_invocation_directory,
  justfile: r#"
    set shell := ["pwsh", "-NoProfile", "-c"]

    pwd:
      @Test-Path {{invocation_directory()}} > result.txt
  "#,
  args: (),
  stdout: "",
  stderr: "",
  status: EXIT_SUCCESS,
  shell: false,
}

test! {
  name: variables,
  justfile: "
    z := 'a'
    a := 'z'
  ",
  args: ("--variables"),
  stdout: "a z\n",
  stderr: "",
  shell: false,
}

test! {
  name: interpolation_evaluation_ignore_quiet,
  justfile: r#"
    foo:
      {{"@echo foo 2>/dev/null"}}
  "#,
  args: (),
  stdout: "",
  stderr: "
    @echo foo 2>/dev/null
    error: Recipe `foo` failed on line 2 with exit code 127
  ",
  status: 127,
  shell: false,
}

test! {
  name: interpolation_evaluation_ignore_quiet_continuation,
  justfile: r#"
    foo:
      {{""}}\
      @echo foo 2>/dev/null
  "#,
  args: (),
  stdout: "",
  stderr: "
    @echo foo 2>/dev/null
    error: Recipe `foo` failed on line 3 with exit code 127
  ",
  status: 127,
  shell: false,
}

test! {
  name: brace_escape,
  justfile: "
    foo:
      echo '{{{{'
  ",
  stdout: "{{\n",
  stderr: "
    echo '{{'
  ",
}

test! {
  name: brace_escape_extra,
  justfile: "
    foo:
      echo '{{{{{'
  ",
  stdout: "{{{\n",
  stderr: "
    echo '{{{'
  ",
}

test! {
  name: multi_line_string_in_interpolation,
  justfile: "
    foo:
      echo {{'a
      echo b
      echo c'}}z
      echo baz
  ",
  stdout: "a\nb\ncz\nbaz\n",
  stderr: "echo a\n  echo b\n  echo cz\necho baz\n",
}

#[cfg(windows)]
test! {
  name: windows_interpreter_path_no_base,
  justfile: r#"
    foo:
      #!powershell

      exit 0
  "#,
  args: (),
}
