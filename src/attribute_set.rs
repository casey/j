use {super::*, std::collections};

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub(crate) struct AttributeSet<'src>(BTreeSet<Attribute<'src>>);

impl<'src> AttributeSet<'src> {
  pub(crate) fn len(&self) -> usize {
    self.0.len()
  }

  pub(crate) fn contains(&self, target: AttributeDiscriminant) -> bool {
    self.0.iter().any(|attr| attr.discriminant() == target)
  }

  pub(crate) fn contains_invertible(&self, target: AttributeDiscriminant) -> Option<bool> {
    self.get(target).map(|attr| match attr {
      Attribute::Linux { enabled }
      | Attribute::Macos { enabled }
      | Attribute::Openbsd { enabled }
      | Attribute::Unix { enabled }
      | Attribute::Windows { enabled } => *enabled,
      _ => panic!("contains_invertible called with non-invertible attribute"),
    })
  }

  pub(crate) fn get(&self, discriminant: AttributeDiscriminant) -> Option<&Attribute<'src>> {
    self
      .0
      .iter()
      .find(|attr| discriminant == attr.discriminant())
  }

  pub(crate) fn iter<'a>(&'a self) -> collections::btree_set::Iter<'a, Attribute<'src>> {
    self.0.iter()
  }

  pub(crate) fn ensure_valid_attributes(
    &self,
    item_kind: &'static str,
    item_token: Token<'src>,
    valid: &[AttributeDiscriminant],
  ) -> Result<(), CompileError<'src>> {
    for attribute in &self.0 {
      let discriminant = attribute.discriminant();
      if !valid.contains(&discriminant) {
        return Err(item_token.error(CompileErrorKind::InvalidAttribute {
          item_kind,
          item_name: item_token.lexeme(),
          attribute: attribute.clone(),
        }));
      }
    }
    Ok(())
  }
}

impl<'src> FromIterator<Attribute<'src>> for AttributeSet<'src> {
  fn from_iter<T: IntoIterator<Item = attribute::Attribute<'src>>>(iter: T) -> Self {
    Self(iter.into_iter().collect())
  }
}

impl<'src, 'a> IntoIterator for &'a AttributeSet<'src> {
  type Item = &'a Attribute<'src>;

  type IntoIter = collections::btree_set::Iter<'a, Attribute<'src>>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<'src> IntoIterator for AttributeSet<'src> {
  type Item = Attribute<'src>;

  type IntoIter = collections::btree_set::IntoIter<Attribute<'src>>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}
