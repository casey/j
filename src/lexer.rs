use {super::*, CompileErrorKind::*, TokenKind::*};

/// Just language lexer
///
/// The lexer proceeds character-by-character, as opposed to using regular
/// expressions to lex tokens or semi-tokens at a time. As a result, it is
/// verbose and straightforward. Just used to have a regex-based lexer, which
/// was slower and generally godawful.  However, this should not be taken as a
/// slight against regular expressions, the lexer was just idiosyncratically
/// bad.
pub(crate) struct Lexer<'src> {
  /// Char iterator
  chars: Chars<'src>,
  /// Indentation stack
  indentation: Vec<&'src str>,
  /// Interpolation token start stack
  interpolation_stack: Vec<Token<'src>>,
  /// Next character to be lexed
  next: Option<char>,
  /// Current open delimiters
  open_delimiters: Vec<(Delimiter, usize)>,
  /// Path to source file
  path: &'src Path,
  /// Inside recipe body
  recipe_body: bool,
  /// Next indent will start a recipe body
  recipe_body_pending: bool,
  /// Source text
  src: &'src str,
  /// Tokens
  tokens: Vec<Token<'src>>,
  /// Current token end
  token_end: Position,
  /// Current token start
  token_start: Position,
}

impl<'src> Lexer<'src> {
  /// Lex `src`
  pub(crate) fn lex(path: &'src Path, src: &'src str) -> CompileResult<'src, Vec<Token<'src>>> {
    Self::new(path, src).tokenize()
  }

  #[cfg(test)]
  pub(crate) fn test_lex(src: &'src str) -> CompileResult<'src, Vec<Token<'src>>> {
    Self::new("justfile".as_ref(), src).tokenize()
  }

  /// Create a new Lexer to lex `src`
  fn new(path: &'src Path, src: &'src str) -> Self {
    let mut chars = src.chars();
    let next = chars.next();

    let start = Position {
      offset: 0,
      column: 0,
      line: 0,
    };

    Self {
      indentation: vec![""],
      tokens: Vec::new(),
      token_start: start,
      token_end: start,
      recipe_body_pending: false,
      recipe_body: false,
      interpolation_stack: Vec::new(),
      open_delimiters: Vec::new(),
      chars,
      next,
      src,
      path,
    }
  }

  /// Advance over the character in `self.next`, updating `self.token_end`
  /// accordingly.
  fn advance(&mut self) -> CompileResult<'src> {
    match self.next {
      Some(c) => {
        let len_utf8 = c.len_utf8();

        self.token_end.offset += len_utf8;
        self.token_end.column += len_utf8;

        if c == '\n' {
          self.token_end.column = 0;
          self.token_end.line += 1;
        }

        self.next = self.chars.next();

        Ok(())
      }
      None => Err(self.internal_error("Lexer advanced past end of text")),
    }
  }

  /// Advance over N characters.
  fn skip(&mut self, n: usize) -> CompileResult<'src> {
    for _ in 0..n {
      self.advance()?;
    }

    Ok(())
  }

  /// Lexeme of in-progress token
  fn lexeme(&self) -> &'src str {
    &self.src[self.token_start.offset..self.token_end.offset]
  }

  /// Length of current token
  fn current_token_length(&self) -> usize {
    self.token_end.offset - self.token_start.offset
  }

  fn accepted(&mut self, c: char) -> CompileResult<'src, bool> {
    if self.next_is(c) {
      self.advance()?;
      Ok(true)
    } else {
      Ok(false)
    }
  }

  fn presume(&mut self, c: char) -> CompileResult<'src> {
    if !self.next_is(c) {
      return Err(self.internal_error(format!("Lexer presumed character `{c}`")));
    }

    self.advance()?;

    Ok(())
  }

  fn presume_str(&mut self, s: &str) -> CompileResult<'src> {
    for c in s.chars() {
      self.presume(c)?;
    }

    Ok(())
  }

  /// Is next character c?
  fn next_is(&self, c: char) -> bool {
    self.next == Some(c)
  }

  /// Is next character ' ' or '\t'?
  fn next_is_whitespace(&self) -> bool {
    self.next_is(' ') || self.next_is('\t')
  }

  /// Un-lexed text
  fn rest(&self) -> &'src str {
    &self.src[self.token_end.offset..]
  }

  /// Check if unlexed text begins with prefix
  fn rest_starts_with(&self, prefix: &str) -> bool {
    self.rest().starts_with(prefix)
  }

  /// Does rest start with "\n" or "\r\n"?
  fn at_eol(&self) -> bool {
    self.next_is('\n') || self.rest_starts_with("\r\n")
  }

  /// Are we at end-of-file?
  fn at_eof(&self) -> bool {
    self.rest().is_empty()
  }

  /// Are we at end-of-line or end-of-file?
  fn at_eol_or_eof(&self) -> bool {
    self.at_eol() || self.at_eof()
  }

  /// Get current indentation
  fn indentation(&self) -> &'src str {
    self.indentation.last().unwrap()
  }

  /// Are we currently indented
  fn indented(&self) -> bool {
    !self.indentation().is_empty()
  }

  /// Create a new token with `kind` whose lexeme is between `self.token_start`
  /// and `self.token_end`
  fn token(&mut self, kind: TokenKind) {
    self.tokens.push(Token {
      offset: self.token_start.offset,
      column: self.token_start.column,
      line: self.token_start.line,
      src: self.src,
      length: self.token_end.offset - self.token_start.offset,
      kind,
      path: self.path,
    });

    // Set `token_start` to point after the lexed token
    self.token_start = self.token_end;
  }

  /// Create an internal error with `message`
  fn internal_error(&self, message: impl Into<String>) -> CompileError<'src> {
    // Use `self.token_end` as the location of the error
    let token = Token {
      src: self.src,
      offset: self.token_end.offset,
      line: self.token_end.line,
      column: self.token_end.column,
      length: 0,
      kind: Unspecified,
      path: self.path,
    };
    CompileError::new(
      token,
      Internal {
        message: message.into(),
      },
    )
  }

  /// Create a compilation error with `kind`
  fn error(&self, kind: CompileErrorKind<'src>) -> CompileError<'src> {
    // Use the in-progress token span as the location of the error.

    // The width of the error site to highlight depends on the kind of error:
    let length = match kind {
      UnterminatedString | UnterminatedBacktick => {
        let Some(kind) = StringKind::from_token_start(self.lexeme()) else {
          return self.internal_error("Lexer::error: expected string or backtick token start");
        };
        kind.delimiter().len()
      }
      // highlight the full token
      _ => self.lexeme().len(),
    };

    let token = Token {
      kind: Unspecified,
      src: self.src,
      offset: self.token_start.offset,
      line: self.token_start.line,
      column: self.token_start.column,
      length,
      path: self.path,
    };

    CompileError::new(token, kind)
  }

  fn unterminated_interpolation_error(interpolation_start: Token<'src>) -> CompileError<'src> {
    CompileError::new(interpolation_start, UnterminatedInterpolation)
  }

  /// True if `text` could be an identifier
  pub(crate) fn is_identifier(text: &str) -> bool {
    if !text.chars().next().map_or(false, Self::is_identifier_start) {
      return false;
    }

    for c in text.chars().skip(1) {
      if !Self::is_identifier_continue(c) {
        return false;
      }
    }

    true
  }

  /// True if `c` can be the first character of an identifier
  pub(crate) fn is_identifier_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
  }

  /// True if `c` can be a continuation character of an identifier
  pub(crate) fn is_identifier_continue(c: char) -> bool {
    Self::is_identifier_start(c) || matches!(c, '0'..='9' | '-')
  }

  /// Consume the text and produce a series of tokens
  fn tokenize(mut self) -> CompileResult<'src, Vec<Token<'src>>> {
    loop {
      if self.token_start.column == 0 {
        self.lex_line_start()?;
      }

      match self.next {
        Some(first) => {
          if let Some(&interpolation_start) = self.interpolation_stack.last() {
            self.lex_interpolation(interpolation_start, first)?;
          } else if self.recipe_body {
            self.lex_body()?;
          } else {
            self.lex_normal(first)?;
          };
        }
        None => break,
      }
    }

    if let Some(&interpolation_start) = self.interpolation_stack.last() {
      return Err(Self::unterminated_interpolation_error(interpolation_start));
    }

    while self.indented() {
      self.lex_dedent();
    }

    self.token(Eof);

    assert_eq!(self.token_start.offset, self.token_end.offset);
    assert_eq!(self.token_start.offset, self.src.len());
    assert_eq!(self.indentation.len(), 1);

    Ok(self.tokens)
  }

  /// Handle blank lines and indentation
  fn lex_line_start(&mut self) -> CompileResult<'src> {
    enum Indentation<'src> {
      // Line only contains whitespace
      Blank,
      // Indentation continues
      Continue,
      // Indentation decreases
      Decrease,
      // Indentation isn't consistent
      Inconsistent,
      // Indentation increases
      Increase,
      // Indentation mixes spaces and tabs
      Mixed { whitespace: &'src str },
    }

    use Indentation::*;

    let nonblank_index = self
      .rest()
      .char_indices()
      .skip_while(|&(_, c)| c == ' ' || c == '\t')
      .map(|(i, _)| i)
      .next()
      .unwrap_or_else(|| self.rest().len());

    let rest = &self.rest()[nonblank_index..];

    let whitespace = &self.rest()[..nonblank_index];

    let body_whitespace = &whitespace[..whitespace
      .char_indices()
      .take(self.indentation().chars().count())
      .map(|(i, _c)| i)
      .next()
      .unwrap_or(0)];

    let spaces = whitespace.chars().any(|c| c == ' ');
    let tabs = whitespace.chars().any(|c| c == '\t');

    let body_spaces = body_whitespace.chars().any(|c| c == ' ');
    let body_tabs = body_whitespace.chars().any(|c| c == '\t');

    #[allow(clippy::if_same_then_else)]
    let indentation = if rest.starts_with('\n') || rest.starts_with("\r\n") || rest.is_empty() {
      Blank
    } else if whitespace == self.indentation() {
      Continue
    } else if self.indentation.contains(&whitespace) {
      Decrease
    } else if self.recipe_body && whitespace.starts_with(self.indentation()) {
      Continue
    } else if self.recipe_body && body_spaces && body_tabs {
      Mixed {
        whitespace: body_whitespace,
      }
    } else if !self.recipe_body && spaces && tabs {
      Mixed { whitespace }
    } else if whitespace.len() < self.indentation().len() {
      Inconsistent
    } else if self.recipe_body
      && body_whitespace.len() >= self.indentation().len()
      && !body_whitespace.starts_with(self.indentation())
    {
      Inconsistent
    } else if whitespace.len() >= self.indentation().len()
      && !whitespace.starts_with(self.indentation())
    {
      Inconsistent
    } else {
      Increase
    };

    match indentation {
      Blank => {
        if !whitespace.is_empty() {
          while self.next_is_whitespace() {
            self.advance()?;
          }

          self.token(Whitespace);
        };

        Ok(())
      }
      Continue => {
        if !self.indentation().is_empty() {
          for _ in self.indentation().chars() {
            self.advance()?;
          }

          self.token(Whitespace);
        }

        Ok(())
      }
      Decrease => {
        while self.indentation() != whitespace {
          self.lex_dedent();
        }

        if !whitespace.is_empty() {
          while self.next_is_whitespace() {
            self.advance()?;
          }

          self.token(Whitespace);
        }

        Ok(())
      }
      Mixed { whitespace } => {
        for _ in whitespace.chars() {
          self.advance()?;
        }

        Err(self.error(MixedLeadingWhitespace { whitespace }))
      }
      Inconsistent => {
        for _ in whitespace.chars() {
          self.advance()?;
        }

        Err(self.error(InconsistentLeadingWhitespace {
          expected: self.indentation(),
          found: whitespace,
        }))
      }
      Increase => {
        while self.next_is_whitespace() {
          self.advance()?;
        }

        if self.open_delimiters() {
          self.token(Whitespace);
        } else {
          let indentation = self.lexeme();
          self.indentation.push(indentation);
          self.token(Indent);
          if self.recipe_body_pending {
            self.recipe_body = true;
          }
        }

        Ok(())
      }
    }
  }

  /// Lex token beginning with `start` outside of a recipe body
  fn lex_normal(&mut self, start: char) -> CompileResult<'src> {
    match start {
      ' ' | '\t' => self.lex_whitespace(),
      '!' if self.rest().starts_with("!include") => Err(self.error(Include)),
      '!' => self.lex_choices('!', &[('=', BangEquals), ('~', BangTilde)], None),
      '#' => self.lex_comment(),
      '$' => self.lex_single(Dollar),
      '&' => self.lex_digraph('&', '&', AmpersandAmpersand),
      '(' => self.lex_delimiter(ParenL),
      ')' => self.lex_delimiter(ParenR),
      '*' => self.lex_single(Asterisk),
      '+' => self.lex_single(Plus),
      ',' => self.lex_single(Comma),
      '/' => self.lex_single(Slash),
      ':' => self.lex_colon(),
      '=' => self.lex_choices(
        '=',
        &[('=', EqualsEquals), ('~', EqualsTilde)],
        Some(Equals),
      ),
      '?' => self.lex_single(QuestionMark),
      '@' => self.lex_single(At),
      '[' => self.lex_delimiter(BracketL),
      '\\' => self.lex_escape(),
      '\n' | '\r' => self.lex_eol(),
      '\u{feff}' => self.lex_single(ByteOrderMark),
      ']' => self.lex_delimiter(BracketR),
      '`' | '"' | '\'' => self.lex_string(),
      '{' => self.lex_delimiter(BraceL),
      '|' => self.lex_digraph('|', '|', BarBar),
      '}' => self.lex_delimiter(BraceR),
      _ if Self::is_identifier_start(start) => self.lex_identifier(),
      _ => {
        self.advance()?;
        Err(self.error(UnknownStartOfToken))
      }
    }
  }

  /// Lex token beginning with `start` inside an interpolation
  fn lex_interpolation(
    &mut self,
    interpolation_start: Token<'src>,
    start: char,
  ) -> CompileResult<'src> {
    if self.rest_starts_with("}}") {
      // end current interpolation
      if self.interpolation_stack.pop().is_none() {
        self.advance()?;
        self.advance()?;
        return Err(self.internal_error(
          "Lexer::lex_interpolation found `}}` but was called with empty interpolation stack.",
        ));
      }
      // Emit interpolation end token
      self.lex_double(InterpolationEnd)
    } else if self.at_eol_or_eof() {
      // Return unterminated interpolation error that highlights the opening
      // {{
      Err(Self::unterminated_interpolation_error(interpolation_start))
    } else {
      // Otherwise lex as per normal
      self.lex_normal(start)
    }
  }

  /// Lex token while in recipe body
  fn lex_body(&mut self) -> CompileResult<'src> {
    enum Terminator {
      Newline,
      NewlineCarriageReturn,
      Interpolation,
      EndOfFile,
    }

    use Terminator::*;

    let terminator = loop {
      if self.rest_starts_with("{{{{") {
        self.skip(4)?;
        continue;
      }

      if self.rest_starts_with("\n") {
        break Newline;
      }

      if self.rest_starts_with("\r\n") {
        break NewlineCarriageReturn;
      }

      if self.rest_starts_with("{{") {
        break Interpolation;
      }

      if self.at_eof() {
        break EndOfFile;
      }

      self.advance()?;
    };

    // emit text token containing text so far
    if self.current_token_length() > 0 {
      self.token(Text);
    }

    match terminator {
      Newline => self.lex_single(Eol),
      NewlineCarriageReturn => self.lex_double(Eol),
      Interpolation => {
        self.lex_double(InterpolationStart)?;
        self
          .interpolation_stack
          .push(self.tokens[self.tokens.len() - 1]);
        Ok(())
      }
      EndOfFile => Ok(()),
    }
  }

  fn lex_dedent(&mut self) {
    assert_eq!(self.current_token_length(), 0);
    self.token(Dedent);
    self.indentation.pop();
    self.recipe_body_pending = false;
    self.recipe_body = false;
  }

  /// Lex a single-character token
  fn lex_single(&mut self, kind: TokenKind) -> CompileResult<'src> {
    self.advance()?;
    self.token(kind);
    Ok(())
  }

  /// Lex a double-character token
  fn lex_double(&mut self, kind: TokenKind) -> CompileResult<'src> {
    self.advance()?;
    self.advance()?;
    self.token(kind);
    Ok(())
  }

  /// Lex a double-character token of kind `then` if the second character of
  /// that token would be `second`, otherwise lex a single-character token of
  /// kind `otherwise`
  fn lex_choices(
    &mut self,
    first: char,
    choices: &[(char, TokenKind)],
    otherwise: Option<TokenKind>,
  ) -> CompileResult<'src> {
    self.presume(first)?;

    for (second, then) in choices {
      if self.accepted(*second)? {
        self.token(*then);
        return Ok(());
      }
    }

    if let Some(token) = otherwise {
      self.token(token);
    } else {
      // Emit an unspecified token to consume the current character,
      self.token(Unspecified);

      let expected = choices.iter().map(|choice| choice.0).collect();

      if self.at_eof() {
        return Err(self.error(UnexpectedEndOfToken { expected }));
      }

      // …and advance past another character,
      self.advance()?;

      // …so that the error we produce highlights the unexpected character.
      return Err(self.error(UnexpectedCharacter { expected }));
    }

    Ok(())
  }

  /// Lex an opening or closing delimiter
  fn lex_delimiter(&mut self, kind: TokenKind) -> CompileResult<'src> {
    use Delimiter::*;

    match kind {
      BraceL => self.open_delimiter(Brace),
      BraceR => self.close_delimiter(Brace)?,
      BracketL => self.open_delimiter(Bracket),
      BracketR => self.close_delimiter(Bracket)?,
      ParenL => self.open_delimiter(Paren),
      ParenR => self.close_delimiter(Paren)?,
      _ => {
        return Err(self.internal_error(format!(
          "Lexer::lex_delimiter called with non-delimiter token: `{kind}`",
        )))
      }
    }

    // Emit the delimiter token
    self.lex_single(kind)
  }

  /// Push a delimiter onto the open delimiter stack
  fn open_delimiter(&mut self, delimiter: Delimiter) {
    self
      .open_delimiters
      .push((delimiter, self.token_start.line));
  }

  /// Pop a delimiter from the open delimiter stack and error if incorrect type
  fn close_delimiter(&mut self, close: Delimiter) -> CompileResult<'src> {
    match self.open_delimiters.pop() {
      Some((open, _)) if open == close => Ok(()),
      Some((open, open_line)) => Err(self.error(MismatchedClosingDelimiter {
        open,
        close,
        open_line,
      })),
      None => Err(self.error(UnexpectedClosingDelimiter { close })),
    }
  }

  /// Return true if there are any unclosed delimiters
  fn open_delimiters(&self) -> bool {
    !self.open_delimiters.is_empty()
  }

  /// Lex a two-character digraph
  fn lex_digraph(&mut self, left: char, right: char, token: TokenKind) -> CompileResult<'src> {
    self.presume(left)?;

    if self.accepted(right)? {
      self.token(token);
      Ok(())
    } else {
      // Emit an unspecified token to consume the current character,
      self.token(Unspecified);

      if self.at_eof() {
        return Err(self.error(UnexpectedEndOfToken {
          expected: vec![right],
        }));
      }

      // …and advance past another character,
      self.advance()?;

      // …so that the error we produce highlights the unexpected character.
      Err(self.error(UnexpectedCharacter {
        expected: vec![right],
      }))
    }
  }

  /// Lex a token starting with ':'
  fn lex_colon(&mut self) -> CompileResult<'src> {
    self.presume(':')?;

    if self.accepted('=')? {
      self.token(ColonEquals);
    } else {
      self.token(Colon);
      self.recipe_body_pending = true;
    }

    Ok(())
  }

  /// Lex an token starting with '\' escape
  fn lex_escape(&mut self) -> CompileResult<'src> {
    self.presume('\\')?;

    // Treat newline escaped with \ as whitespace
    if self.accepted('\n')? {
      while self.next_is_whitespace() {
        self.advance()?;
      }
      self.token(Whitespace);
    } else if self.accepted('\r')? {
      if !self.accepted('\n')? {
        return Err(self.error(UnpairedCarriageReturn));
      }
      while self.next_is_whitespace() {
        self.advance()?;
      }
      self.token(Whitespace);
    } else if let Some(character) = self.next {
      return Err(self.error(InvalidEscapeSequence { character }));
    }

    Ok(())
  }

  /// Lex a carriage return and line feed
  fn lex_eol(&mut self) -> CompileResult<'src> {
    if self.accepted('\r')? {
      if !self.accepted('\n')? {
        return Err(self.error(UnpairedCarriageReturn));
      }
    } else {
      self.presume('\n')?;
    }

    // Emit an eol if there are no open delimiters, otherwise emit a whitespace
    // token.
    if self.open_delimiters() {
      self.token(Whitespace);
    } else {
      self.token(Eol);
    }

    Ok(())
  }

  /// Lex name: [a-zA-Z_][a-zA-Z0-9_]*
  fn lex_identifier(&mut self) -> CompileResult<'src> {
    self.advance()?;

    while let Some(c) = self.next {
      if !Self::is_identifier_continue(c) {
        break;
      }

      self.advance()?;
    }

    self.token(Identifier);

    Ok(())
  }

  /// Lex comment: #[^\r\n]
  fn lex_comment(&mut self) -> CompileResult<'src> {
    self.presume('#')?;

    while !self.at_eol_or_eof() {
      self.advance()?;
    }

    self.token(Comment);

    Ok(())
  }

  /// Lex whitespace: [ \t]+
  fn lex_whitespace(&mut self) -> CompileResult<'src> {
    while self.next_is_whitespace() {
      self.advance()?;
    }

    self.token(Whitespace);

    Ok(())
  }

  /// Lex a backtick, cooked string, or raw string.
  ///
  /// Backtick:      ``[^`]*``
  /// Cooked string: "[^"]*" # also processes escape sequences
  /// Raw string:    '[^']*'
  fn lex_string(&mut self) -> CompileResult<'src> {
    let Some(kind) = StringKind::from_token_start(self.rest()) else {
      self.advance()?;
      return Err(self.internal_error("Lexer::lex_string: invalid string start"));
    };

    self.presume_str(kind.delimiter())?;

    let mut escape = false;

    loop {
      if self.next.is_none() {
        return Err(self.error(kind.unterminated_error_kind()));
      } else if kind.processes_escape_sequences() && self.next_is('\\') && !escape {
        escape = true;
      } else if self.rest_starts_with(kind.delimiter()) && !escape {
        break;
      } else {
        escape = false;
      }

      self.advance()?;
    }

    self.presume_str(kind.delimiter())?;
    self.token(kind.token_kind());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  macro_rules! test {
    {
      name:     $name:ident,
      text:     $text:expr,
      tokens:   ($($kind:ident $(: $lexeme:literal)?),* $(,)?)$(,)?
    } => {
      #[test]
      fn $name() {
        let kinds: &[TokenKind] = &[$($kind,)* Eof];

        let lexemes: &[&str] = &[$(lexeme!($kind $(, $lexeme)?),)* ""];

        test($text, true, kinds, lexemes);
      }
    };
    {
      name:     $name:ident,
      text:     $text:expr,
      tokens:   ($($kind:ident $(: $lexeme:literal)?),* $(,)?)$(,)?
      unindent: $unindent:expr,
    } => {
      #[test]
      fn $name() {
        let kinds: &[TokenKind] = &[$($kind,)* Eof];

        let lexemes: &[&str] = &[$(lexeme!($kind $(, $lexeme)?),)* ""];

        test($text, $unindent, kinds, lexemes);
      }
    }
  }

  macro_rules! lexeme {
    {
      $kind:ident, $lexeme:literal
    } => {
      $lexeme
    };
    {
      $kind:ident
    } => {
      default_lexeme($kind)
    }
  }

  fn test(text: &str, unindent_text: bool, want_kinds: &[TokenKind], want_lexemes: &[&str]) {
    let text = if unindent_text {
      unindent(text)
    } else {
      text.to_owned()
    };

    let have = Lexer::test_lex(&text).unwrap();

    let have_kinds = have
      .iter()
      .map(|token| token.kind)
      .collect::<Vec<TokenKind>>();

    let have_lexemes = have.iter().map(Token::lexeme).collect::<Vec<&str>>();

    assert_eq!(have_kinds, want_kinds, "Token kind mismatch");
    assert_eq!(have_lexemes, want_lexemes, "Token lexeme mismatch");

    let mut roundtrip = String::new();

    for lexeme in have_lexemes {
      roundtrip.push_str(lexeme);
    }

    assert_eq!(roundtrip, text, "Roundtrip mismatch");

    let mut offset = 0;
    let mut line = 0;
    let mut column = 0;

    for token in have {
      assert_eq!(token.offset, offset);
      assert_eq!(token.line, line);
      assert_eq!(token.lexeme().len(), token.length);
      assert_eq!(token.column, column);

      for c in token.lexeme().chars() {
        if c == '\n' {
          line += 1;
          column = 0;
        } else {
          column += c.len_utf8();
        }
      }

      offset += token.length;
    }
  }

  fn default_lexeme(kind: TokenKind) -> &'static str {
    match kind {
      // Fixed lexemes
      AmpersandAmpersand => "&&",
      Asterisk => "*",
      At => "@",
      BangEquals => "!=",
      BangTilde => "!~",
      BarBar => "||",
      BraceL => "{",
      BraceR => "}",
      BracketL => "[",
      BracketR => "]",
      ByteOrderMark => "\u{feff}",
      Colon => ":",
      ColonEquals => ":=",
      Comma => ",",
      Dollar => "$",
      Eol => "\n",
      Equals => "=",
      EqualsEquals => "==",
      EqualsTilde => "=~",
      Indent => "  ",
      InterpolationEnd => "}}",
      InterpolationStart => "{{",
      ParenL => "(",
      ParenR => ")",
      Plus => "+",
      QuestionMark => "?",
      Slash => "/",
      Whitespace => " ",

      // Empty lexemes
      Dedent | Eof => "",

      // Variable lexemes
      Text | StringToken | Backtick | Identifier | Comment | Unspecified => {
        panic!("Token {kind:?} has no default lexeme")
      }
    }
  }

  macro_rules! error {
    (
      name:   $name:ident,
      input:  $input:expr,
      offset: $offset:expr,
      line:   $line:expr,
      column: $column:expr,
      width:  $width:expr,
      kind:   $kind:expr,
    ) => {
      #[test]
      fn $name() {
        error($input, $offset, $line, $column, $width, $kind);
      }
    };
  }

  fn error(
    src: &str,
    offset: usize,
    line: usize,
    column: usize,
    length: usize,
    kind: CompileErrorKind,
  ) {
    match Lexer::test_lex(src) {
      Ok(_) => panic!("Lexing succeeded but expected"),
      Err(have) => {
        let want = CompileError {
          token: Token {
            kind: have.token.kind,
            src,
            offset,
            line,
            column,
            length,
            path: "justfile".as_ref(),
          },
          kind: kind.into(),
        };
        assert_eq!(have, want);
      }
    }
  }

  test! {
    name:   name_new,
    text:   "foo",
    tokens: (Identifier:"foo"),
  }

  test! {
    name:   comment,
    text:   "# hello",
    tokens: (Comment:"# hello"),
  }

  test! {
    name:   backtick,
    text:   "`echo`",
    tokens: (Backtick:"`echo`"),
  }

  test! {
    name:   backtick_multi_line,
    text:   "`echo\necho`",
    tokens: (Backtick:"`echo\necho`"),
  }

  test! {
    name:   raw_string,
    text:   "'hello'",
    tokens: (StringToken:"'hello'"),
  }

  test! {
    name:   raw_string_multi_line,
    text:   "'hello\ngoodbye'",
    tokens: (StringToken:"'hello\ngoodbye'"),
  }

  test! {
    name:   cooked_string,
    text:   "\"hello\"",
    tokens: (StringToken:"\"hello\""),
  }

  test! {
    name:   cooked_string_multi_line,
    text:   "\"hello\ngoodbye\"",
    tokens: (StringToken:"\"hello\ngoodbye\""),
  }

  test! {
    name:   cooked_multiline_string,
    text:   "\"\"\"hello\ngoodbye\"\"\"",
    tokens: (StringToken:"\"\"\"hello\ngoodbye\"\"\""),
  }

  test! {
    name:   ampersand_ampersand,
    text:   "&&",
    tokens: (AmpersandAmpersand),
  }

  test! {
    name:   equals,
    text:   "=",
    tokens: (Equals),
  }

  test! {
    name:   equals_equals,
    text:   "==",
    tokens: (EqualsEquals),
  }

  test! {
    name:   bang_equals,
    text:   "!=",
    tokens: (BangEquals),
  }

  test! {
    name:   brace_l,
    text:   "{",
    tokens: (BraceL),
  }

  test! {
    name:   brace_r,
    text:   "{}",
    tokens: (BraceL, BraceR),
  }

  test! {
    name:   brace_lll,
    text:   "{{{",
    tokens: (BraceL, BraceL, BraceL),
  }

  test! {
    name:   brace_rrr,
    text:   "{{{}}}",
    tokens: (BraceL, BraceL, BraceL, BraceR, BraceR, BraceR),
  }

  test! {
    name:   dollar,
    text:   "$",
    tokens: (Dollar),
  }

  test! {
    name:   export_concatenation,
    text:   "export foo = 'foo' + 'bar'",
    tokens: (
      Identifier:"export",
      Whitespace,
      Identifier:"foo",
      Whitespace,
      Equals,
      Whitespace,
      StringToken:"'foo'",
      Whitespace,
      Plus,
      Whitespace,
      StringToken:"'bar'",
    )
  }

  test! {
    name: export_complex,
    text: "export foo = ('foo' + 'bar') + `baz`",
    tokens: (
      Identifier:"export",
      Whitespace,
      Identifier:"foo",
      Whitespace,
      Equals,
      Whitespace,
      ParenL,
      StringToken:"'foo'",
      Whitespace,
      Plus,
      Whitespace,
      StringToken:"'bar'",
      ParenR,
      Whitespace,
      Plus,
      Whitespace,
      Backtick:"`baz`",
    ),
  }

  test! {
    name:     eol_linefeed,
    text:     "\n",
    tokens:   (Eol),
    unindent: false,
  }

  test! {
    name:     eol_carriage_return_linefeed,
    text:     "\r\n",
    tokens:   (Eol:"\r\n"),
    unindent: false,
  }

  test! {
    name:   indented_line,
    text:   "foo:\n a",
    tokens: (Identifier:"foo", Colon, Eol, Indent:" ", Text:"a", Dedent),
  }

  test! {
    name:   indented_normal,
    text:   "
      a
        b
        c
    ",
    tokens: (
      Identifier:"a",
      Eol,
      Indent:"  ",
      Identifier:"b",
      Eol,
      Whitespace:"  ",
      Identifier:"c",
      Eol,
      Dedent,
    ),
  }

  test! {
    name:   indented_normal_nonempty_blank,
    text:   "a\n  b\n\t\t\n  c\n",
    tokens: (
      Identifier:"a",
      Eol,
      Indent:"  ",
      Identifier:"b",
      Eol,
      Whitespace:"\t\t",
      Eol,
      Whitespace:"  ",
      Identifier:"c",
      Eol,
      Dedent,
    ),
    unindent: false,
  }

  test! {
    name:   indented_normal_multiple,
    text:   "
      a
        b
          c
    ",
    tokens: (
      Identifier:"a",
      Eol,
      Indent:"  ",
      Identifier:"b",
      Eol,
      Indent:"    ",
      Identifier:"c",
      Eol,
      Dedent,
      Dedent,
    ),
  }

  test! {
    name:   indent_indent_dedent_indent,
    text:   "
      a
        b
          c
        d
          e
    ",
    tokens: (
      Identifier:"a",
      Eol,
      Indent:"  ",
        Identifier:"b",
        Eol,
        Indent:"    ",
          Identifier:"c",
          Eol,
        Dedent,
        Whitespace:"  ",
        Identifier:"d",
        Eol,
        Indent:"    ",
          Identifier:"e",
          Eol,
        Dedent,
      Dedent,
    ),
  }

  test! {
    name:   indent_recipe_dedent_indent,
    text:   "
      a
        b:
          c
        d
          e
    ",
    tokens: (
      Identifier:"a",
      Eol,
      Indent:"  ",
        Identifier:"b",
        Colon,
        Eol,
        Indent:"    ",
          Text:"c",
          Eol,
        Dedent,
        Whitespace:"  ",
        Identifier:"d",
        Eol,
        Indent:"    ",
          Identifier:"e",
          Eol,
        Dedent,
      Dedent,
    ),
  }

  test! {
    name: indented_block,
    text: "
      foo:
        a
        b
        c
    ",
    tokens: (
      Identifier:"foo",
      Colon,
      Eol,
      Indent,
      Text:"a",
      Eol,
      Whitespace:"  ",
      Text:"b",
      Eol,
      Whitespace:"  ",
      Text:"c",
      Eol,
      Dedent,
    )
  }

  test! {
    name: brace_escape,
    text: "
      foo:
        {{{{
    ",
    tokens: (
      Identifier:"foo",
      Colon,
      Eol,
      Indent,
      Text:"{{{{",
      Eol,
      Dedent,
    )
  }

  test! {
    name: indented_block_followed_by_item,
    text: "
      foo:
        a
      b:
    ",
    tokens: (
      Identifier:"foo",
      Colon,
      Eol,
      Indent,
      Text:"a",
      Eol,
      Dedent,
      Identifier:"b",
      Colon,
      Eol,
    )
  }

  test! {
    name: indented_block_followed_by_blank,
    text: "
      foo:
          a

      b:
    ",
    tokens: (
      Identifier:"foo",
      Colon,
      Eol,
      Indent:"    ",
      Text:"a",
      Eol,
      Eol,
      Dedent,
      Identifier:"b",
      Colon,
      Eol,
    ),
  }

  test! {
    name: indented_line_containing_unpaired_carriage_return,
    text: "foo:\n \r \n",
    tokens: (
      Identifier:"foo",
      Colon,
      Eol,
      Indent:" ",
      Text:"\r ",
      Eol,
      Dedent,
    ),
    unindent: false,
  }

  test! {
    name: indented_blocks,
    text: "
      b: a
        @mv a b

      a:
        @touch F
        @touch a

      d: c
        @rm c

      c: b
        @mv b c
    ",
    tokens: (
      Identifier:"b",
      Colon,
      Whitespace,
      Identifier:"a",
      Eol,
      Indent,
      Text:"@mv a b",
      Eol,
      Eol,
      Dedent,
      Identifier:"a",
      Colon,
      Eol,
      Indent,
      Text:"@touch F",
      Eol,
      Whitespace:"  ",
      Text:"@touch a",
      Eol,
      Eol,
      Dedent,
      Identifier:"d",
      Colon,
      Whitespace,
      Identifier:"c",
      Eol,
      Indent,
      Text:"@rm c",
      Eol,
      Eol,
      Dedent,
      Identifier:"c",
      Colon,
      Whitespace,
      Identifier:"b",
      Eol,
      Indent,
      Text:"@mv b c",
      Eol,
      Dedent
    ),
  }

  test! {
    name: interpolation_empty,
    text: "hello:\n echo {{}}",
    tokens: (
      Identifier:"hello",
      Colon,
      Eol,
      Indent:" ",
      Text:"echo ",
      InterpolationStart,
      InterpolationEnd,
      Dedent,
    ),
  }

  test! {
    name: interpolation_expression,
    text: "hello:\n echo {{`echo hello` + `echo goodbye`}}",
    tokens: (
      Identifier:"hello",
      Colon,
      Eol,
      Indent:" ",
      Text:"echo ",
      InterpolationStart,
      Backtick:"`echo hello`",
      Whitespace,
      Plus,
      Whitespace,
      Backtick:"`echo goodbye`",
      InterpolationEnd,
      Dedent,
    ),
  }

  test! {
    name: interpolation_raw_multiline_string,
    text: "hello:\n echo {{'\n'}}",
    tokens: (
      Identifier:"hello",
      Colon,
      Eol,
      Indent:" ",
      Text:"echo ",
      InterpolationStart,
      StringToken:"'\n'",
      InterpolationEnd,
      Dedent,
    ),
  }

  test! {
    name: tokenize_names,
    text: "
      foo
      bar-bob
      b-bob_asdfAAAA
      test123
    ",
    tokens: (
      Identifier:"foo",
      Eol,
      Identifier:"bar-bob",
      Eol,
      Identifier:"b-bob_asdfAAAA",
      Eol,
      Identifier:"test123",
      Eol,
    ),
  }

  test! {
    name: tokenize_indented_line,
    text: "foo:\n a",
    tokens: (
      Identifier:"foo",
      Colon,
      Eol,
      Indent:" ",
      Text:"a",
      Dedent,
    ),
  }

  test! {
    name: tokenize_indented_block,
    text: "
      foo:
        a
        b
        c
    ",
    tokens: (
      Identifier:"foo",
      Colon,
      Eol,
      Indent,
      Text:"a",
      Eol,
      Whitespace:"  ",
      Text:"b",
      Eol,
      Whitespace:"  ",
      Text:"c",
      Eol,
      Dedent,
    ),
  }

  test! {
    name: tokenize_strings,
    text: r#"a = "'a'" + '"b"' + "'c'" + '"d"'#echo hello"#,
    tokens: (
      Identifier:"a",
      Whitespace,
      Equals,
      Whitespace,
      StringToken:"\"'a'\"",
      Whitespace,
      Plus,
      Whitespace,
      StringToken:"'\"b\"'",
      Whitespace,
      Plus,
      Whitespace,
      StringToken:"\"'c'\"",
      Whitespace,
      Plus,
      Whitespace,
      StringToken:"'\"d\"'",
      Comment:"#echo hello",
    )
  }

  test! {
    name: tokenize_recipe_interpolation_eol,
    text: "
      foo: # some comment
       {{hello}}
    ",
    tokens: (
      Identifier:"foo",
      Colon,
      Whitespace,
      Comment:"# some comment",
      Eol,
      Indent:" ",
      InterpolationStart,
      Identifier:"hello",
      InterpolationEnd,
      Eol,
      Dedent
    ),
  }

  test! {
    name: tokenize_recipe_interpolation_eof,
    text: "foo: # more comments
 {{hello}}
# another comment
",
    tokens: (
      Identifier:"foo",
      Colon,
      Whitespace,
      Comment:"# more comments",
      Eol,
      Indent:" ",
      InterpolationStart,
      Identifier:"hello",
      InterpolationEnd,
      Eol,
      Dedent,
      Comment:"# another comment",
      Eol,
    ),
  }

  test! {
    name: tokenize_recipe_complex_interpolation_expression,
    text: "foo: #lol\n {{a + b + \"z\" + blarg}}",
    tokens: (
      Identifier:"foo",
      Colon,
      Whitespace:" ",
      Comment:"#lol",
      Eol,
      Indent:" ",
      InterpolationStart,
      Identifier:"a",
      Whitespace,
      Plus,
      Whitespace,
      Identifier:"b",
      Whitespace,
      Plus,
      Whitespace,
      StringToken:"\"z\"",
      Whitespace,
      Plus,
      Whitespace,
      Identifier:"blarg",
      InterpolationEnd,
      Dedent,
    ),
  }

  test! {
    name: tokenize_recipe_multiple_interpolations,
    text: "foo:,#ok\n {{a}}0{{b}}1{{c}}",
    tokens: (
      Identifier:"foo",
      Colon,
      Comma,
      Comment:"#ok",
      Eol,
      Indent:" ",
      InterpolationStart,
      Identifier:"a",
      InterpolationEnd,
      Text:"0",
      InterpolationStart,
      Identifier:"b",
      InterpolationEnd,
      Text:"1",
      InterpolationStart,
      Identifier:"c",
      InterpolationEnd,
      Dedent,

    ),
  }

  test! {
    name: tokenize_junk,
    text: "
      bob

      hello blah blah blah : a b c #whatever
    ",
    tokens: (
      Identifier:"bob",
      Eol,
      Eol,
      Identifier:"hello",
      Whitespace,
      Identifier:"blah",
      Whitespace,
      Identifier:"blah",
      Whitespace,
      Identifier:"blah",
      Whitespace,
      Colon,
      Whitespace,
      Identifier:"a",
      Whitespace,
      Identifier:"b",
      Whitespace,
      Identifier:"c",
      Whitespace,
      Comment:"#whatever",
      Eol,
    )
  }

  test! {
    name: tokenize_empty_lines,
    text: "

      # this does something
      hello:
        asdf
        bsdf

        csdf

        dsdf # whatever

      # yolo
    ",
    tokens: (
      Eol,
      Comment:"# this does something",
      Eol,
      Identifier:"hello",
      Colon,
      Eol,
      Indent,
      Text:"asdf",
      Eol,
      Whitespace:"  ",
      Text:"bsdf",
      Eol,
      Eol,
      Whitespace:"  ",
      Text:"csdf",
      Eol,
      Eol,
      Whitespace:"  ",
      Text:"dsdf # whatever",
      Eol,
      Eol,
      Dedent,
      Comment:"# yolo",
      Eol,
    ),
  }

  test! {
    name: tokenize_comment_before_variable,
    text: "
      #
      A='1'
      echo:
        echo {{A}}
    ",
    tokens: (
      Comment:"#",
      Eol,
      Identifier:"A",
      Equals,
      StringToken:"'1'",
      Eol,
      Identifier:"echo",
      Colon,
      Eol,
      Indent,
      Text:"echo ",
      InterpolationStart,
      Identifier:"A",
      InterpolationEnd,
      Eol,
      Dedent,
    ),
  }

  test! {
    name: tokenize_interpolation_backticks,
    text: "hello:\n echo {{`echo hello` + `echo goodbye`}}",
    tokens: (
      Identifier:"hello",
      Colon,
      Eol,
      Indent:" ",
      Text:"echo ",
      InterpolationStart,
      Backtick:"`echo hello`",
      Whitespace,
      Plus,
      Whitespace,
      Backtick:"`echo goodbye`",
      InterpolationEnd,
      Dedent
    ),
  }

  test! {
    name: tokenize_empty_interpolation,
    text: "hello:\n echo {{}}",
    tokens: (
      Identifier:"hello",
      Colon,
      Eol,
      Indent:" ",
      Text:"echo ",
      InterpolationStart,
      InterpolationEnd,
      Dedent,
    ),
  }

  test! {
    name: tokenize_assignment_backticks,
    text: "a = `echo hello` + `echo goodbye`",
    tokens: (
      Identifier:"a",
      Whitespace,
      Equals,
      Whitespace,
      Backtick:"`echo hello`",
      Whitespace,
      Plus,
      Whitespace,
      Backtick:"`echo goodbye`",
    ),
  }

  test! {
    name: tokenize_multiple,
    text: "

      hello:
        a
        b

        c

        d

      # hello
      bob:
        frank
       \t
    ",
    tokens: (
      Eol,
      Identifier:"hello",
      Colon,
      Eol,
      Indent,
      Text:"a",
      Eol,
      Whitespace:"  ",
      Text:"b",
      Eol,
      Eol,
      Whitespace:"  ",
      Text:"c",
      Eol,
      Eol,
      Whitespace:"  ",
      Text:"d",
      Eol,
      Eol,
      Dedent,
      Comment:"# hello",
      Eol,
      Identifier:"bob",
      Colon,
      Eol,
      Indent:"  ",
      Text:"frank",
      Eol,
      Eol,
      Dedent,
    ),
  }

  test! {
    name: tokenize_comment,
    text: "a:=#",
    tokens: (
      Identifier:"a",
      ColonEquals,
      Comment:"#",
    ),
  }

  test! {
    name: tokenize_comment_with_bang,
    text: "a:=#foo!",
    tokens: (
      Identifier:"a",
      ColonEquals,
      Comment:"#foo!",
    ),
  }

  test! {
    name: tokenize_order,
    text: "
      b: a
        @mv a b

      a:
        @touch F
        @touch a

      d: c
        @rm c

      c: b
        @mv b c
    ",
    tokens: (
      Identifier:"b",
      Colon,
      Whitespace,
      Identifier:"a",
      Eol,
      Indent,
      Text:"@mv a b",
      Eol,
      Eol,
      Dedent,
      Identifier:"a",
      Colon,
      Eol,
      Indent,
      Text:"@touch F",
      Eol,
      Whitespace:"  ",
      Text:"@touch a",
      Eol,
      Eol,
      Dedent,
      Identifier:"d",
      Colon,
      Whitespace,
      Identifier:"c",
      Eol,
      Indent,
      Text:"@rm c",
      Eol,
      Eol,
      Dedent,
      Identifier:"c",
      Colon,
      Whitespace,
      Identifier:"b",
      Eol,
      Indent,
      Text:"@mv b c",
      Eol,
      Dedent,
    ),
  }

  test! {
    name: tokenize_parens,
    text: "((())) ()abc(+",
    tokens: (
      ParenL,
      ParenL,
      ParenL,
      ParenR,
      ParenR,
      ParenR,
      Whitespace,
      ParenL,
      ParenR,
      Identifier:"abc",
      ParenL,
      Plus,
    ),
  }

  test! {
    name: crlf_newline,
    text: "#\r\n#asdf\r\n",
    tokens: (
      Comment:"#",
      Eol:"\r\n",
      Comment:"#asdf",
      Eol:"\r\n",
    ),
  }

  test! {
    name: multiple_recipes,
    text: "a:\n  foo\nb:",
    tokens: (
      Identifier:"a",
      Colon,
      Eol,
      Indent:"  ",
      Text:"foo",
      Eol,
      Dedent,
      Identifier:"b",
      Colon,
    ),
  }

  test! {
    name:   brackets,
    text:   "[][]",
    tokens: (BracketL, BracketR, BracketL, BracketR),
  }

  test! {
    name:   open_delimiter_eol,
    text:   "[\n](\n){\n}",
    tokens: (
      BracketL, Whitespace:"\n", BracketR,
      ParenL, Whitespace:"\n", ParenR,
      BraceL, Whitespace:"\n", BraceR
    ),
  }

  error! {
    name:  tokenize_space_then_tab,
    input: "a:
 0
 1
\t2
",
    offset: 9,
    line:   3,
    column: 0,
    width:  1,
    kind:   InconsistentLeadingWhitespace{expected: " ", found: "\t"},
  }

  error! {
    name:  tokenize_tabs_then_tab_space,
    input: "a:
\t\t0
\t\t 1
\t  2
",
    offset: 12,
    line:   3,
    column: 0,
    width:  3,
    kind:   InconsistentLeadingWhitespace{expected: "\t\t", found: "\t  "},
  }

  error! {
    name:   tokenize_unknown,
    input:  "%",
    offset: 0,
    line:   0,
    column: 0,
    width:  1,
    kind:   UnknownStartOfToken,
  }

  error! {
    name:   unterminated_string_with_escapes,
    input:  r#"a = "\n\t\r\"\\"#,
    offset: 4,
    line:   0,
    column: 4,
    width:  1,
    kind:   UnterminatedString,
  }

  error! {
    name:   unterminated_raw_string,
    input:  "r a='asdf",
    offset: 4,
    line:   0,
    column: 4,
    width:  1,
    kind:   UnterminatedString,
  }

  error! {
    name:   unterminated_interpolation,
    input:  "foo:\n echo {{
  ",
    offset: 11,
    line:   1,
    column: 6,
    width:  2,
    kind:   UnterminatedInterpolation,
  }

  error! {
    name:   unterminated_backtick,
    input:  "`echo",
    offset: 0,
    line:   0,
    column: 0,
    width:  1,
    kind:   UnterminatedBacktick,
  }

  error! {
    name:   unpaired_carriage_return,
    input:  "foo\rbar",
    offset: 3,
    line:   0,
    column: 3,
    width:  1,
    kind:   UnpairedCarriageReturn,
  }

  error! {
    name:   invalid_name_start_dash,
    input:  "-foo",
    offset: 0,
    line:   0,
    column: 0,
    width:  1,
    kind:   UnknownStartOfToken,
  }

  error! {
    name:   invalid_name_start_digit,
    input:  "0foo",
    offset: 0,
    line:   0,
    column: 0,
    width:  1,
    kind:   UnknownStartOfToken,
  }

  error! {
    name:   unterminated_string,
    input:  r#"a = ""#,
    offset: 4,
    line:   0,
    column: 4,
    width:  1,
    kind:   UnterminatedString,
  }

  error! {
    name:   mixed_leading_whitespace_recipe,
    input:  "a:\n\t echo hello",
    offset: 3,
    line:   1,
    column: 0,
    width:  2,
    kind:   MixedLeadingWhitespace{whitespace: "\t "},
  }

  error! {
    name:   mixed_leading_whitespace_normal,
    input:  "a\n\t echo hello",
    offset: 2,
    line:   1,
    column: 0,
    width:  2,
    kind:   MixedLeadingWhitespace{whitespace: "\t "},
  }

  error! {
    name:   mixed_leading_whitespace_indent,
    input:  "a\n foo\n \tbar",
    offset: 7,
    line:   2,
    column: 0,
    width:  2,
    kind:   MixedLeadingWhitespace{whitespace: " \t"},
  }

  error! {
    name:   bad_dedent,
    input:  "a\n foo\n   bar\n  baz",
    offset: 14,
    line:   3,
    column: 0,
    width:  2,
    kind:   InconsistentLeadingWhitespace{expected: "   ", found: "  "},
  }

  error! {
    name:   unclosed_interpolation_delimiter,
    input:  "a:\n echo {{ foo",
    offset: 9,
    line:   1,
    column: 6,
    width:  2,
    kind:   UnterminatedInterpolation,
  }

  error! {
    name:   unexpected_character_after_at,
    input:  "@%",
    offset: 1,
    line:   0,
    column: 1,
    width:  1,
    kind:   UnknownStartOfToken,
  }

  error! {
    name:   mismatched_closing_brace,
    input:  "(]",
    offset: 1,
    line:   0,
    column: 1,
    width:  0,
    kind:   MismatchedClosingDelimiter {
      open:      Delimiter::Paren,
      close:     Delimiter::Bracket,
      open_line: 0,
    },
  }

  error! {
    name:   ampersand_eof,
    input:  "&",
    offset: 1,
    line:   0,
    column: 1,
    width:  0,
    kind:   UnexpectedEndOfToken {
      expected: vec!['&'],
    },
  }

  error! {
    name:   ampersand_unexpected,
    input:  "&%",
    offset: 1,
    line:   0,
    column: 1,
    width:  1,
    kind:   UnexpectedCharacter {
      expected: vec!['&'],
    },
  }

  error! {
    name:   bang_eof,
    input:  "!",
    offset: 1,
    line:   0,
    column: 1,
    width:  0,
    kind:   UnexpectedEndOfToken {
      expected: vec!['=', '~'],
    },
  }

  #[test]
  fn presume_error() {
    let compile_error = Lexer::new("justfile".as_ref(), "!")
      .presume('-')
      .unwrap_err();
    assert_matches!(
      compile_error.token,
      Token {
        offset: 0,
        line: 0,
        column: 0,
        length: 0,
        src: "!",
        kind: Unspecified,
        path: _,
      }
    );
    assert_matches!(&*compile_error.kind,
        Internal { ref message }
        if message == "Lexer presumed character `-`"
    );

    assert_eq!(
      Error::Compile { compile_error }
        .color_display(Color::never())
        .to_string(),
      "error: Internal error, this may indicate a bug in just: Lexer presumed character `-`
consider filing an issue: https://github.com/casey/just/issues/new
 ——▶ justfile:1:1
  │
1 │ !
  │ ^"
    );
  }
}
