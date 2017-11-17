use common::*;

use compile;

pub fn parse_success(text: &str) -> Justfile {
  match compile(text) {
    Ok(justfile) => justfile,
    Err(error) => panic!("Expected successful parse but got error:\n{}", error),
  }
}

pub fn parse_error(text: &str, expected: CompilationError) {
  if let Err(error) = compile(text) {
    assert_eq!(error.text,   expected.text);
    assert_eq!(error.index,  expected.index);
    assert_eq!(error.line,   expected.line);
    assert_eq!(error.column, expected.column);
    assert_eq!(error.kind,   expected.kind);
    assert_eq!(error.width,  expected.width);
    assert_eq!(error,        expected);
  } else {
    panic!("Expected {:?} but parse succeeded", expected.kind);
  }
}

