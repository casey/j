use std::collections::{self};

use super::*;

#[derive(
  EnumDiscriminants, PartialEq, Debug, Clone, Serialize, Ord, PartialOrd, Eq, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
#[strum_discriminants(name(AttributeDiscriminant))]
#[strum_discriminants(derive(EnumString))]
#[strum_discriminants(strum(serialize_all = "kebab-case"))]
pub(crate) enum Attribute<'src> {
  Confirm(Option<StringLiteral<'src>>),
  Doc(Option<StringLiteral<'src>>),
  Extension(StringLiteral<'src>),
  Group(StringLiteral<'src>),
  Linux,
  Macos,
  NoCd,
  NoExitMessage,
  NoQuiet,
  PositionalArguments,
  Private,
  Script(Option<Interpreter<'src>>),
  Unix,
  Windows,
}

impl AttributeDiscriminant {
  fn argument_range(self) -> RangeInclusive<usize> {
    match self {
      Self::Confirm | Self::Doc => 0..=1,
      Self::Group | Self::Extension => 1..=1,
      Self::Linux
      | Self::Macos
      | Self::NoCd
      | Self::NoExitMessage
      | Self::NoQuiet
      | Self::PositionalArguments
      | Self::Private
      | Self::Unix
      | Self::Windows => 0..=0,
      Self::Script => 0..=usize::MAX,
    }
  }
}

impl<'src> Attribute<'src> {
  pub(crate) fn new(
    name: Name<'src>,
    arguments: Vec<StringLiteral<'src>>,
  ) -> CompileResult<'src, Self> {
    let discriminant = name
      .lexeme()
      .parse::<AttributeDiscriminant>()
      .ok()
      .ok_or_else(|| {
        name.error(CompileErrorKind::UnknownAttribute {
          attribute: name.lexeme(),
        })
      })?;

    let found = arguments.len();
    let range = discriminant.argument_range();
    if !range.contains(&found) {
      return Err(
        name.error(CompileErrorKind::AttributeArgumentCountMismatch {
          attribute: name.lexeme(),
          found,
          min: *range.start(),
          max: *range.end(),
        }),
      );
    }

    Ok(match discriminant {
      AttributeDiscriminant::Confirm => Self::Confirm(arguments.into_iter().next()),
      AttributeDiscriminant::Doc => Self::Doc(arguments.into_iter().next()),
      AttributeDiscriminant::Extension => Self::Extension(arguments.into_iter().next().unwrap()),
      AttributeDiscriminant::Group => Self::Group(arguments.into_iter().next().unwrap()),
      AttributeDiscriminant::Linux => Self::Linux,
      AttributeDiscriminant::Macos => Self::Macos,
      AttributeDiscriminant::NoCd => Self::NoCd,
      AttributeDiscriminant::NoExitMessage => Self::NoExitMessage,
      AttributeDiscriminant::NoQuiet => Self::NoQuiet,
      AttributeDiscriminant::PositionalArguments => Self::PositionalArguments,
      AttributeDiscriminant::Private => Self::Private,
      AttributeDiscriminant::Script => Self::Script({
        let mut arguments = arguments.into_iter();
        arguments.next().map(|command| Interpreter {
          command,
          arguments: arguments.collect(),
        })
      }),
      AttributeDiscriminant::Unix => Self::Unix,
      AttributeDiscriminant::Windows => Self::Windows,
    })
  }

  pub(crate) fn discriminant(&self) -> AttributeDiscriminant {
    self.into()
  }

  pub(crate) fn name(&self) -> &'static str {
    self.into()
  }
}

impl<'src> Display for Attribute<'src> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.name())?;

    match self {
      Self::Confirm(Some(argument))
      | Self::Doc(Some(argument))
      | Self::Extension(argument)
      | Self::Group(argument) => write!(f, "({argument})")?,
      Self::Script(Some(shell)) => write!(f, "({shell})")?,
      Self::Confirm(None)
      | Self::Doc(None)
      | Self::Linux
      | Self::Macos
      | Self::NoCd
      | Self::NoExitMessage
      | Self::NoQuiet
      | Self::PositionalArguments
      | Self::Private
      | Self::Script(None)
      | Self::Unix
      | Self::Windows => {}
    }

    Ok(())
  }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub(crate) struct AttributeSet<'src>(BTreeSet<Attribute<'src>>);

impl<'src, 'a> IntoIterator for &'a AttributeSet<'src> {
  type Item = &'a Attribute<'src>;

  type IntoIter = collections::btree_set::Iter<'a, Attribute<'src>>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<'src, 'a> FromIterator<Attribute<'src>> for AttributeSet<'src> {
  fn from_iter<T: IntoIterator<Item = attribute::Attribute<'src>>>(iter: T) -> Self {
    Self(iter.into_iter().collect())
  }
}

impl<'src> AttributeSet<'src> {
  pub(crate) fn len(&self) -> usize {
    self.0.len()
  }

  pub(crate) fn contains(&self, target: AttributeDiscriminant) -> bool {
    self.0.iter().any(|attr| attr.discriminant() == target)
  }

  pub(crate) fn get(&self, discriminant: AttributeDiscriminant) -> Option<&Attribute<'src>> {
    self
      .0
      .iter()
      .find(|attr| discriminant == attr.discriminant())
  }

  pub(crate) fn iter(&self) -> impl Iterator<Item = &Attribute<'src>> {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn name() {
    assert_eq!(Attribute::NoExitMessage.name(), "no-exit-message");
  }
}
