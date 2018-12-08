use common::*;

#[derive(PartialEq, Debug)]
pub struct CookedString<'a> {
  pub raw: &'a str,
  pub cooked: String,
}

impl<'a> CookedString<'a> {
  pub fn new(token: &Token<'a>) -> CompilationResult<'a, CookedString<'a>> {
    let raw = &token.lexeme[1..token.lexeme.len() - 1];

    if let TokenKind::RawString = token.kind {
      Ok(CookedString {
        cooked: raw.to_string(),
        raw,
      })
    } else if let TokenKind::StringToken = token.kind {
      let mut cooked = String::new();
      let mut escape = false;
      for c in raw.chars() {
        if escape {
          match c {
            'n' => cooked.push('\n'),
            'r' => cooked.push('\r'),
            't' => cooked.push('\t'),
            '\\' => cooked.push('\\'),
            '"' => cooked.push('"'),
            other => {
              return Err(
                token.error(CompilationErrorKind::InvalidEscapeSequence { character: other }),
              )
            }
          }
          escape = false;
          continue;
        }
        if c == '\\' {
          escape = true;
          continue;
        }
        cooked.push(c);
      }
      Ok(CookedString { raw, cooked })
    } else {
      Err(token.error(CompilationErrorKind::Internal {
        message: "cook_string() called on non-string token".to_string(),
      }))
    }
  }
}
