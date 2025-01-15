use std::rc::Rc;

// Design notes:
// =============
//
// The "list of strings" data structure can be represented in three different reasonable ways:
//
// 1. As a vec of strings (currently implemented).
// This is simple and easy to debug, but makes zero-copy stuff difficult.
// At least can make copying cheap with an Rc<T>.
//
// 2. As a pre-joined string + vec of indices.
// This allows a `&str` ref to the pre-joined string to be acquired,
// at the cost of bounds checking when the parts are accessed.
// Would probably be the most compact and efficient implementation.
//
// 3. As a value that contains both a pre-joined variant and list variant.
// This trades memory for the ability to borrow refs to both variants.
//
// The current implementation focuses on simplicity,
// but tries to preserve flexibility for future optimizations
// by mostly only dealing with owned data at the API boundaries (no borrowed views).

/// The internal data type of Just, an immutable list of strings.
///
/// It can be viewed as a "joined string", or as the individual "parts".
#[derive(Debug, Clone, Default)]
pub(crate) struct Val {
  parts: Rc<[Box<str>]>,
}

impl Val {
  /// Create an empty `Val`
  pub fn new() -> Self {
    Self::default()
  }

  /// Construct a `Val` consisting of a single string.
  pub fn from_str<S: AsRef<str>>(s: S) -> Self {
    Self {
      parts: Rc::new([s.as_ref().into()]),
    }
  }

  /// Construct a `Val` consisting of multiple parts.
  pub fn from_parts<I, S>(parts: I) -> Self
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    Self {
      parts: parts.into_iter().map(|s| s.as_ref().into()).collect(),
    }
  }

  /// Convert to a single joined string.
  pub fn to_joined(&self) -> String {
    self.parts.join(" ")
  }

  /// Convert to individual parts.
  pub fn to_parts(&self) -> Vec<Box<str>> {
    self.parts.to_vec()
  }
}

// Also provides a `to_string()` method
impl std::fmt::Display for Val {
  /// When formatted as a string, a `Val` is joined by whitespace.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.to_joined())
  }
}

// convenience conversion
impl From<&str> for Val {
  fn from(value: &str) -> Self {
    Self::from_str(value)
  }
}

// convenience conversion
impl From<String> for Val {
  fn from(value: String) -> Self {
    Self::from_str(value)
  }
}

impl<A: AsRef<str>> FromIterator<A> for Val {
  fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
    Self::from_parts(iter)
  }
}

#[cfg(test)]
fn bs(s: &str) -> Box<str> {
  s.to_owned().into_boxed_str()
}

#[test]
fn can_construct_vals() {
  assert_eq!(Val::from("foo bar").to_parts(), vec![bs("foo bar")]);
  assert_eq!(Val::from("x y z".to_string()).to_parts(), vec![bs("x y z")]);
  assert_eq!(Val::from_parts(["a b"]).to_parts(), vec![bs("a b")]);
  assert_eq!(
    Val::from_parts(["a b", "c d"]).to_parts(),
    vec![bs("a b"), bs("c d")]
  );
}

#[test]
fn empty_representations() {
  let empty_string = Val::from("");
  let empty_parts = Val::from_parts::<_, &str>([]);

  // the two empty string representations have different parts
  assert_eq!(empty_string.to_parts(), vec![bs("")]);
  assert_eq!(empty_parts.to_parts(), vec![]);

  // but they have the same joined representation
  assert_eq!(empty_string.to_joined(), empty_parts.to_joined());
}

#[test]
fn joins_by_space_without_quoting() {
  let val = Val::from_parts(["a", "b c", "d"]);
  assert_eq!(val.to_joined(), "a b c d");
}
