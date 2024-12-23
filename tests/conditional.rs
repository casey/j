use super::*;

test! {
  name: then_branch_unevaluated,
  justfile: "
    foo:
      echo {{ if 'a' == 'b' { `exit 1` } else { 'otherwise' } }}
  ",
  stdout: "otherwise\n",
  stderr: "echo otherwise\n",
}

test! {
  name: otherwise_branch_unevaluated,
  justfile: "
    foo:
      echo {{ if 'a' == 'a' { 'then' } else { `exit 1` } }}
  ",
  stdout: "then\n",
  stderr: "echo then\n",
}

test! {
  name: otherwise_branch_unevaluated_inverted,
  justfile: "
    foo:
      echo {{ if 'a' != 'b' { 'then' } else { `exit 1` } }}
  ",
  stdout: "then\n",
  stderr: "echo then\n",
}

test! {
  name: then_branch_unevaluated_inverted,
  justfile: "
    foo:
      echo {{ if 'a' != 'a' { `exit 1` } else { 'otherwise' } }}
  ",
  stdout: "otherwise\n",
  stderr: "echo otherwise\n",
}

test! {
  name: complex_expressions,
  justfile: "
    foo:
      echo {{ if 'a' + 'b' == `echo ab` { 'c' + 'd' } else { 'e' + 'f' } }}
  ",
  stdout: "cd\n",
  stderr: "echo cd\n",
}

test! {
  name: undefined_lhs,
  justfile: "
    a := if b == '' { '' } else { '' }

    foo:
      echo {{ a }}
  ",
  stdout: "",
  stderr: "
    error: Variable `b` not defined
     ——▶ justfile:1:9
      │
    1 │ a := if b == '' { '' } else { '' }
      │         ^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: undefined_rhs,
  justfile: "
    a := if '' == b { '' } else { '' }

    foo:
      echo {{ a }}
  ",
  stdout: "",
  stderr: "
    error: Variable `b` not defined
     ——▶ justfile:1:15
      │
    1 │ a := if '' == b { '' } else { '' }
      │               ^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: undefined_then,
  justfile: "
    a := if '' == '' { b } else { '' }

    foo:
      echo {{ a }}
  ",
  stdout: "",
  stderr: "
    error: Variable `b` not defined
     ——▶ justfile:1:20
      │
    1 │ a := if '' == '' { b } else { '' }
      │                    ^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: undefined_otherwise,
  justfile: "
    a := if '' == '' { '' } else { b }

    foo:
      echo {{ a }}
  ",
  stdout: "",
  stderr: "
    error: Variable `b` not defined
     ——▶ justfile:1:32
      │
    1 │ a := if '' == '' { '' } else { b }
      │                                ^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: unexpected_op,
  justfile: "
    a := if '' a '' { '' } else { b }

    foo:
      echo {{ a }}
  ",
  stdout: "",
  stderr: "
    error: Expected '&&', '!=', '!~', '||', '==', '=~', '+', or '/', but found identifier
     ——▶ justfile:1:12
      │
    1 │ a := if '' a '' { '' } else { b }
      │            ^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: dump,
  justfile: "
    a := if '' == '' { '' } else { '' }

    foo:
      echo {{ a }}
  ",
  args: ("--dump"),
  stdout: "
    a := if '' == '' { '' } else { '' }

    foo:
        echo {{ a }}
  ",
}

test! {
  name: if_else,
  justfile: "
    x := if '0' == '1' { 'a' } else if '0' == '0' { 'b' } else { 'c' }

    foo:
      echo {{ x }}
  ",
  stdout: "b\n",
  stderr: "echo b\n",
}

test! {
  name: missing_else,
  justfile: "
  TEST := if path_exists('/bin/bash') == 'true' {'yes'}
  ",
  stdout: "",
  stderr: "
    error: Expected keyword `else` but found `end of line`
     ——▶ justfile:1:54
      │
    1 │ TEST := if path_exists('/bin/bash') == 'true' {'yes'}
      │                                                      ^
  ",
  status: EXIT_FAILURE,
}

test! {
  name: incorrect_else_identifier,
  justfile: "
  TEST := if path_exists('/bin/bash') == 'true' {'yes'} els {'no'}
  ",
  stdout: "",
  stderr: "
    error: Expected keyword `else` but found identifier `els`
     ——▶ justfile:1:55
      │
    1 │ TEST := if path_exists('/bin/bash') == 'true' {'yes'} els {'no'}
      │                                                       ^^^
  ",
  status: EXIT_FAILURE,
}
