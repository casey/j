use super::*;

#[derive(
  EnumDiscriminants, PartialEq, Debug, Clone, Serialize, Ord, PartialOrd, Eq, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
#[strum_discriminants(name(AttributeDiscriminant))]
#[strum_discriminants(derive(EnumString, Ord, PartialOrd))]
#[strum_discriminants(strum(serialize_all = "kebab-case"))]
pub(crate) enum Attribute<'src> {
  Confirm(Option<StringLiteral<'src>>),
  Doc(Option<StringLiteral<'src>>),
  Extension(StringLiteral<'src>),
  Group(StringLiteral<'src>),
  Linux { enabled: bool },
  Macos { enabled: bool },
  NoCd,
  NoExitMessage,
  NoQuiet,
  Openbsd { enabled: bool },
  PositionalArguments,
  Private,
  Script(Option<Interpreter<'src>>),
  Unix { enabled: bool },
  Windows { enabled: bool },
  WorkingDirectory(StringLiteral<'src>),
}

impl AttributeDiscriminant {
  fn argument_range(self) -> RangeInclusive<usize> {
    match self {
      Self::Confirm | Self::Doc => 0..=1,
      Self::Group | Self::Extension | Self::WorkingDirectory => 1..=1,
      Self::Linux
      | Self::Macos
      | Self::NoCd
      | Self::NoExitMessage
      | Self::NoQuiet
      | Self::Openbsd
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
    enabled: bool,
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

    Ok(match (enabled, discriminant) {
      (enabled, AttributeDiscriminant::Linux) => Self::Linux { enabled },
      (enabled, AttributeDiscriminant::Macos) => Self::Macos { enabled },
      (enabled, AttributeDiscriminant::Unix) => Self::Unix { enabled },
      (enabled, AttributeDiscriminant::Windows) => Self::Windows { enabled },
      (enabled, AttributeDiscriminant::Openbsd) => Self::Openbsd { enabled },

      (false, _attr) => {
        return Err(name.error(CompileErrorKind::InvalidInvertedAttribute {
          attr_name: name.lexeme(),
        }))
      }

      (true, AttributeDiscriminant::Confirm) => Self::Confirm(arguments.into_iter().next()),
      (true, AttributeDiscriminant::Doc) => Self::Doc(arguments.into_iter().next()),
      (true, AttributeDiscriminant::Extension) => {
        Self::Extension(arguments.into_iter().next().unwrap())
      }
      (true, AttributeDiscriminant::Group) => Self::Group(arguments.into_iter().next().unwrap()),
      (true, AttributeDiscriminant::NoCd) => Self::NoCd,
      (true, AttributeDiscriminant::NoExitMessage) => Self::NoExitMessage,
      (true, AttributeDiscriminant::NoQuiet) => Self::NoQuiet,
      (true, AttributeDiscriminant::PositionalArguments) => Self::PositionalArguments,
      (true, AttributeDiscriminant::Private) => Self::Private,
      (true, AttributeDiscriminant::Script) => Self::Script({
        let mut arguments = arguments.into_iter();
        arguments.next().map(|command| Interpreter {
          command,
          arguments: arguments.collect(),
        })
      }),
      (true, AttributeDiscriminant::WorkingDirectory) => {
        Self::WorkingDirectory(arguments.into_iter().next().unwrap())
      }
    })
  }

  pub(crate) fn discriminant(&self) -> AttributeDiscriminant {
    self.into()
  }

  pub(crate) fn name(&self) -> &'static str {
    self.into()
  }

  pub(crate) fn repeatable(&self) -> bool {
    matches!(self, Attribute::Group(_))
  }
}

impl Display for Attribute<'_> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let name = self.name();

    match self {
      Self::Confirm(Some(argument))
      | Self::Doc(Some(argument))
      | Self::Extension(argument)
      | Self::Group(argument)
      | Self::WorkingDirectory(argument) => write!(f, "{name}({argument})")?,
      Self::Script(Some(shell)) => write!(f, "{name}({shell})")?,
      Self::Linux { enabled }
      | Self::Macos { enabled }
      | Self::Unix { enabled }
      | Self::Openbsd { enabled }
      | Self::Windows { enabled } => {
        if *enabled {
          write!(f, "{name}")?;
        } else {
          write!(f, "not({name})")?;
        }
      }
      Self::Confirm(None)
      | Self::Doc(None)
      | Self::NoCd
      | Self::NoExitMessage
      | Self::NoQuiet
      | Self::PositionalArguments
      | Self::Private
      | Self::Script(None) => write!(f, "{name}")?,
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
