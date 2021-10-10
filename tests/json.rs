use crate::common::*;

use serde_json::{json, Value};

fn test(justfile: &str, value: Value) {
  let json = serde_json::to_string(&value).unwrap();
  Test::new()
    .justfile(justfile)
    .args(&["--json"])
    .stdout(format!("{}\n", json))
    .run();
}

#[test]
fn alias() {
  test(
    "
      alias f := foo

      foo:
    ",
    json!({
      "aliases": {
        "f": {
          "name": "f",
          "target": "foo",
        }
      },
      "recipes": {
        "foo": {
          "dependencies": [],
          "doc": null,
          "name": "foo",
          "priors": 0,
          "private": false,
          "quiet": false,
          "shebang": false,
        }
      }
    }),
  );
}

#[test]
fn dependencies() {
  test(
    "
      foo:
      bar: foo
    ",
    json!({
      "aliases": {},
      "recipes": {
        "bar": {
          "doc": null,
          "name": "bar",
          "dependencies": [{
            "arguments": [],
            "name": "foo"
          }],
          "priors": 1,
          "private": false,
          "quiet": false,
          "shebang": false,
        },
        "foo": {
          "dependencies": [],
          "doc": null,
          "name": "foo",
          "priors": 0,
          "private": false,
          "quiet": false,
          "shebang": false,
        }
      }
    }),
  );
}

#[test]
fn doc_comment() {
  test(
    "# hello\nfoo:",
    json!({
      "aliases": {},
      "recipes": {
        "foo": {
          "dependencies": [],
          "doc": "hello",
          "name": "foo",
          "priors": 0,
          "private": false,
          "quiet": false,
          "shebang": false,
        }
      }
    }),
  );
}

#[test]
fn empty_justfile() {
  test(
    "",
    json!({
      "aliases": {},
      "recipes": {}
    }),
  );
}

#[test]
fn priors() {
  test(
    "
      a:
      b: a && c
      c:
    ",
    json!({
      "aliases": {},
      "recipes": {
        "a": {
          "dependencies": [],
          "doc": null,
          "name": "a",
          "priors": 0,
          "private": false,
          "quiet": false,
          "shebang": false,
        },
        "b": {
          "dependencies": [
            {
              "arguments": [],
              "name": "a",
            },
            {
              "arguments": [],
              "name": "c",
            }
          ],
          "doc": null,
          "name": "b",
          "private": false,
          "quiet": false,
          "shebang": false,
          "priors": 1,
        },
        "c": {
          "dependencies": [],
          "doc": null,
          "name": "c",
          "priors": 0,
          "private": false,
          "quiet": false,
          "shebang": false,
          "priors": 0,
        },
      }
    }),
  );
}

#[test]
fn private() {
  test(
    "_foo:",
    json!({
      "aliases": {},
      "recipes": {
        "_foo": {
          "dependencies": [],
          "doc": null,
          "name": "_foo",
          "priors": 0,
          "private": true,
          "quiet": false,
          "shebang": false,
        }
      }
    }),
  );
}

#[test]
fn quiet() {
  test(
    "@foo:",
    json!({
      "aliases": {},
      "recipes": {
        "foo": {
          "dependencies": [],
          "doc": null,
          "name": "foo",
          "priors": 0,
          "private": false,
          "quiet": true,
          "shebang": false,
        }
      }
    }),
  );
}

#[test]
fn shebang() {
  test(
    "
      foo:
        #!bar
    ",
    json!({
      "aliases": {},
      "recipes": {
        "foo": {
          "dependencies": [],
          "doc": null,
          "name": "foo",
          "priors": 0,
          "private": false,
          "quiet": false,
          "shebang": true,
        }
      }
    }),
  );
}

#[test]
fn simple() {
  test(
    "foo:",
    json!({
      "aliases": {},
      "recipes": {
        "foo": {
          "dependencies": [],
          "doc": null,
          "name": "foo",
          "priors": 0,
          "private": false,
          "quiet": false,
          "shebang": false,
        }
      }
    }),
  );
}
