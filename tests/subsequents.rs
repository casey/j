use super::*;

test! {
  name: success,
  justfile: "
    foo: && bar
      echo foo

    bar:
      echo bar
  ",
  stdout: "
    foo
    bar
  ",
  stderr: "
    echo foo
    echo bar
  ",
}

test! {
  name: failure,
  justfile: "
    foo: && bar
      echo foo
      false

    bar:
      echo bar
  ",
  stdout: "
    foo
  ",
  stderr: "
    echo foo
    false
    error: Recipe `foo` failed on line 3 with exit code 1
  ",
  status: EXIT_FAILURE,
}

test! {
  name: circular_dependency,
  justfile: "
    foo: && foo
  ",
  stderr:
"Error: Recipe `foo` depends on itself
   ╭─[justfile:1:9]
   │
 1 │ foo: && foo
───╯
",
  status: EXIT_FAILURE,
}

test! {
  name: unknown,
  justfile: "
    foo: && bar
  ",
  stderr: "
    error: Recipe `foo` has unknown dependency `bar`
     ——▶ justfile:1:9
      │
    1 │ foo: && bar
      │         ^^^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: unknown_argument,
  justfile: "
    bar x:

    foo: && (bar y)
  ",
  stderr: "
    error: Variable `y` not defined
     ——▶ justfile:3:14
      │
    3 │ foo: && (bar y)
      │              ^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: argument,
  justfile: "
    foo: && (bar 'hello')

    bar x:
      echo {{ x }}
  ",
  stdout: "
    hello
  ",
  stderr: "
    echo hello
  ",
}

test! {
  name: duplicate_subsequents_dont_run,
  justfile: "
    a: && b c
      echo a

    b: d
      echo b

    c: d
      echo c

    d:
      echo d
  ",
  stdout: "
    a
    d
    b
    c
  ",
  stderr: "
    echo a
    echo d
    echo b
    echo c
  ",
}

test! {
  name: subsequents_run_even_if_already_ran_as_prior,
  justfile: "
    a: b && b
      echo a

    b:
      echo b
  ",
  stdout: "
    b
    a
    b
  ",
  stderr: "
    echo b
    echo a
    echo b
  ",
}
