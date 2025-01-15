use super::*;

#[derive(Debug)]
pub(crate) struct Scope<'src: 'run, 'run> {
  parent: Option<&'run Self>,
  bindings: Table<'src, Binding<'src, Val>>,
}

impl<'src, 'run> Scope<'src, 'run> {
  pub(crate) fn child(&'run self) -> Self {
    Self {
      parent: Some(self),
      bindings: Table::new(),
    }
  }

  pub(crate) fn root() -> Self {
    let mut root = Self {
      parent: None,
      bindings: Table::new(),
    };

    for (key, value) in constants() {
      root.bind(Binding {
        constant: true,
        export: false,
        file_depth: 0,
        name: Name {
          token: Token {
            column: 0,
            kind: TokenKind::Identifier,
            length: key.len(),
            line: 0,
            offset: 0,
            path: Path::new("PRELUDE"),
            src: key,
          },
        },
        private: false,
        value: (*value).into(),
      });
    }

    root
  }

  pub(crate) fn bind(&mut self, binding: Binding<'src>) {
    self.bindings.insert(binding);
  }

  pub(crate) fn bound(&self, name: &str) -> bool {
    self.bindings.contains_key(name)
  }

  pub(crate) fn value(&self, name: &str) -> Option<&Val> {
    if let Some(binding) = self.bindings.get(name) {
      Some(&binding.value)
    } else {
      self.parent?.value(name)
    }
  }

  pub(crate) fn bindings(&self) -> impl Iterator<Item = &Binding<Val>> {
    self.bindings.values()
  }

  pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
    self.bindings.keys().copied()
  }

  pub(crate) fn parent(&self) -> Option<&'run Self> {
    self.parent
  }
}
