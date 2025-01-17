use super::*;

#[derive(Debug, PartialEq)]
pub(crate) struct CompileError<'src> {
  pub(crate) token: Token<'src>,
  pub(crate) kind: Box<CompileErrorKind<'src>>,
}

impl<'src> CompileError<'src> {
  pub(crate) fn context(&self) -> Token<'src> {
    self.token
  }

  pub(crate) fn new(token: Token<'src>, kind: CompileErrorKind<'src>) -> CompileError<'src> {
    Self {
      token,
      kind: kind.into(),
    }
  }
}

fn capitalize(s: &str) -> String {
  let mut chars = s.chars();
  match chars.next() {
    None => String::new(),
    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
  }
}

impl Display for CompileError<'_> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use CompileErrorKind::*;

    match &*self.kind {
      AttributeArgumentCountMismatch {
        attribute,
        found,
        min,
        max,
      } => {
        write!(
          f,
          "Attribute `{attribute}` got {found} {} but takes ",
          Count("argument", *found),
        )?;

        if min == max {
          let expected = min;
          write!(f, "{expected} {}", Count("argument", *expected))
        } else if found < min {
          write!(f, "at least {min} {}", Count("argument", *min))
        } else {
          write!(f, "at most {max} {}", Count("argument", *max))
        }
      }
      BacktickShebang => write!(f, "Backticks may not start with `#!`"),
      CircularRecipeDependency { recipe, ref circle } => {
        if circle.len() == 2 {
          write!(f, "Recipe `{recipe}` depends on itself")
        } else {
          write!(
            f,
            "Recipe `{recipe}` has circular dependency `{}`",
            circle.join(" -> ")
          )
        }
      }
      CircularVariableDependency {
        variable,
        ref circle,
      } => {
        if circle.len() == 2 {
          write!(f, "Variable `{variable}` is defined in terms of itself")
        } else {
          write!(
            f,
            "Variable `{variable}` depends on its own value: `{}`",
            circle.join(" -> "),
          )
        }
      }
      DependencyArgumentCountMismatch {
        dependency,
        found,
        min,
        max,
      } => {
        write!(
          f,
          "Dependency `{dependency}` got {found} {} but takes ",
          Count("argument", *found),
        )?;

        if min == max {
          let expected = min;
          write!(f, "{expected} {}", Count("argument", *expected))
        } else if found < min {
          write!(f, "at least {min} {}", Count("argument", *min))
        } else {
          write!(f, "at most {max} {}", Count("argument", *max))
        }
      }
      DuplicateAttribute { attribute, first } => write!(
        f,
        "Recipe attribute `{attribute}` first used on line {} is duplicated on line {}",
        first.ordinal(),
        self.token.line.ordinal(),
      ),
      DuplicateParameter { recipe, parameter } => {
        write!(f, "Recipe `{recipe}` has duplicate parameter `{parameter}`")
      }
      DuplicateSet { setting, first } => write!(
        f,
        "Setting `{setting}` first set on line {} is redefined on line {}",
        first.ordinal(),
        self.token.line.ordinal(),
      ),
      DuplicateVariable { variable } => {
        write!(f, "Variable `{variable}` has multiple definitions")
      }
      DuplicateUnexport { variable } => {
        write!(f, "Variable `{variable}` is unexported multiple times")
      },
      ExitMessageAndNoExitMessageAttribute { recipe } =>write!(
        f,
        "Recipe `{recipe}` has both `[exit-message]` and `[no-exit-message]` attributes"
      ),
      ExpectedKeyword { expected, found } => {
        let expected = List::or_ticked(expected);
        if found.kind == TokenKind::Identifier {
          write!(
            f,
            "Expected keyword {expected} but found identifier `{}`",
            found.lexeme()
          )
        } else {
          write!(f, "Expected keyword {expected} but found `{}`", found.kind)
        }
      }
      ExportUnexported { variable } => {
        write!(f, "Variable {variable} is both exported and unexported")
      }
      ExtraLeadingWhitespace => write!(f, "Recipe line has extra leading whitespace"),
      ExtraneousAttributes { count } => {
        write!(f, "Extraneous {}", Count("attribute", *count))
      }
      FunctionArgumentCountMismatch {
        function,
        found,
        expected,
      } => write!(
        f,
        "Function `{function}` called with {found} {} but takes {}",
        Count("argument", *found),
        expected.display(),
      ),
      Include => write!(
        f,
        "The `!include` directive has been stabilized as `import`"
      ),
      InconsistentLeadingWhitespace { expected, found } => write!(
        f,
        "Recipe line has inconsistent leading whitespace. Recipe started with `{}` but found \
           line with `{}`",
        ShowWhitespace(expected),
        ShowWhitespace(found)
      ),
      Internal { ref message } => write!(
        f,
        "Internal error, this may indicate a bug in just: {message}\n\
           consider filing an issue: https://github.com/casey/just/issues/new"
      ),
      InvalidAttribute {
        item_name,
        item_kind,
        attribute,
      } => write!(
        f,
        "{item_kind} `{item_name}` has invalid attribute `{}`",
        attribute.name(),
      ),
      InvalidEscapeSequence { character } => write!(
        f,
        "`\\{}` is not a valid escape sequence",
        match character {
          '`' => r"\`".to_owned(),
          '\\' => r"\".to_owned(),
          '\'' => r"'".to_owned(),
          '"' => r#"""#.to_owned(),
          _ => character.escape_default().collect(),
        }
      ),
      MismatchedClosingDelimiter {
        open,
        open_line,
        close,
      } => write!(
        f,
        "Mismatched closing delimiter `{}`. (Did you mean to close the `{}` on line {}?)",
        close.close(),
        open.open(),
        open_line.ordinal(),
      ),
      MixedLeadingWhitespace { whitespace } => write!(
        f,
        "Found a mix of tabs and spaces in leading whitespace: `{}`\nLeading whitespace may \
           consist of tabs or spaces, but not both",
        ShowWhitespace(whitespace)
      ),
      NoCdAndWorkingDirectoryAttribute { recipe } => write!(
        f,
        "Recipe `{recipe}` has both `[no-cd]` and `[working-directory]` attributes"
      ),
      ParameterFollowsVariadicParameter { parameter } => {
        write!(f, "Parameter `{parameter}` follows variadic parameter")
      }
      ParsingRecursionDepthExceeded => write!(f, "Parsing recursion depth exceeded"),
      Redefinition {
        first,
        first_type,
        name,
        second_type,
      } => {
        if first_type == second_type {
          write!(
            f,
            "{} `{name}` first defined on line {} is redefined on line {}",
            capitalize(first_type),
            first.ordinal(),
            self.token.line.ordinal(),
          )
        } else {
          write!(
            f,
            "{} `{name}` defined on line {} is redefined as {} {second_type} on line {}",
            capitalize(first_type),
            first.ordinal(),
            if *second_type == "alias" { "an" } else { "a" },
            self.token.line.ordinal(),
          )
        }
      }
      ShebangAndScriptAttribute { recipe } => write!(
        f,
        "Recipe `{recipe}` has both shebang line and `[script]` attribute"
      ),
      ShellExpansion { err } => write!(f, "Shell expansion failed: {err}"),
      RequiredParameterFollowsDefaultParameter { parameter } => write!(
        f,
        "Non-default parameter `{parameter}` follows default parameter"
      ),
      UndefinedVariable { variable } => write!(f, "Variable `{variable}` not defined"),
      UnexpectedCharacter { expected } => {
        write!(f, "Expected character {}", List::or_ticked(expected))
      }
      UnexpectedClosingDelimiter { close } => {
        write!(f, "Unexpected closing delimiter `{}`", close.close())
      }
      UnexpectedEndOfToken { expected } => {
        write!(
          f,
          "Expected character {} but found end-of-file",
          List::or_ticked(expected),
        )
      }
      UnexpectedToken {
        ref expected,
        found,
      } => write!(f, "Expected {}, but found {found}", List::or(expected)),
      UnicodeEscapeCharacter { character } => {
        write!(f, "expected hex digit [0-9A-Fa-f] but found `{character}`")
      }
      UnicodeEscapeDelimiter { character } => write!(
        f,
        "expected unicode escape sequence delimiter `{{` but found `{character}`"
      ),
      UnicodeEscapeEmpty => write!(f, "unicode escape sequences must not be empty"),
      UnicodeEscapeLength { hex } => write!(
        f,
        "unicode escape sequence starting with `\\u{{{hex}` longer than six hex digits"
      ),
      UnicodeEscapeRange { hex } => {
        write!(
          f,
          "unicode escape sequence value `{hex}` greater than maximum valid code point `10FFFF`",
        )
      }
      UnicodeEscapeUnterminated => write!(f, "unterminated unicode escape sequence"),
      UnknownAliasTarget { alias, target } => {
        write!(f, "Alias `{alias}` has an unknown target `{target}`")
      }
      UnknownAttribute { attribute } => write!(f, "Unknown attribute `{attribute}`"),
      UnknownDependency { recipe, unknown } => {
        write!(f, "Recipe `{recipe}` has unknown dependency `{unknown}`")
      }
      UnknownFunction { function } => write!(f, "Call to unknown function `{function}`"),
      UnknownSetting { setting } => write!(f, "Unknown setting `{setting}`"),
      UnknownStartOfToken => write!(f, "Unknown start of token:"),
      UnpairedCarriageReturn => write!(f, "Unpaired carriage return"),
      UnterminatedBacktick => write!(f, "Unterminated backtick"),
      UnterminatedInterpolation => write!(f, "Unterminated interpolation"),
      UnterminatedString => write!(f, "Unterminated string"),
    }
  }
}
