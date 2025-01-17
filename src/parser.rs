use {super::*, TokenKind::*};

/// Just language parser
///
/// The parser is a (hopefully) straightforward recursive descent parser.
///
/// It uses a few tokens of lookahead to disambiguate different constructs.
///
/// The `expect_*` and `presume_`* methods are similar in that they assert the
/// type of unparsed tokens and consume them. However, upon encountering an
/// unexpected token, the `expect_*` methods return an unexpected token error,
/// whereas the `presume_*` tokens return an internal error.
///
/// The `presume_*` methods are used when the token stream has been inspected in
/// some other way, and thus encountering an unexpected token is a bug in Just,
/// and not a syntax error.
///
/// All methods starting with `parse_*` parse and return a language construct.
///
/// The parser tracks an expected set of tokens as it parses. This set contains
/// all tokens which would have been accepted at the current point in the
/// parse. Whenever the parser tests for a token that would be accepted, but
/// does not find it, it adds that token to the set. When the parser accepts a
/// token, the set is cleared. If the parser finds a token which is unexpected,
/// the elements of the set are printed in the resultant error message.
pub(crate) struct Parser<'run, 'src> {
  expected_tokens: BTreeSet<TokenKind>,
  file_depth: u32,
  import_offsets: Vec<usize>,
  module_namepath: &'run Namepath<'src>,
  next_token: usize,
  recursion_depth: usize,
  tokens: &'run [Token<'src>],
  unstable_features: BTreeSet<UnstableFeature>,
  working_directory: &'run Path,
}

impl<'run, 'src> Parser<'run, 'src> {
  /// Parse `tokens` into an `Ast`
  pub(crate) fn parse(
    file_depth: u32,
    import_offsets: &[usize],
    module_namepath: &'run Namepath<'src>,
    tokens: &'run [Token<'src>],
    working_directory: &'run Path,
  ) -> CompileResult<'src, Ast<'src>> {
    Self {
      expected_tokens: BTreeSet::new(),
      file_depth,
      import_offsets: import_offsets.to_vec(),
      module_namepath,
      next_token: 0,
      recursion_depth: 0,
      tokens,
      unstable_features: BTreeSet::new(),
      working_directory,
    }
    .parse_ast()
  }

  fn error(&self, kind: CompileErrorKind<'src>) -> CompileResult<'src, CompileError<'src>> {
    Ok(self.next()?.error(kind))
  }

  /// Construct an unexpected token error with the token returned by
  /// `Parser::next`
  fn unexpected_token(&self) -> CompileResult<'src, CompileError<'src>> {
    self.error(CompileErrorKind::UnexpectedToken {
      expected: self
        .expected_tokens
        .iter()
        .copied()
        .filter(|kind| *kind != ByteOrderMark)
        .collect::<Vec<TokenKind>>(),
      found: self.next()?.kind,
    })
  }

  fn internal_error(&self, message: impl Into<String>) -> CompileResult<'src, CompileError<'src>> {
    self.error(CompileErrorKind::Internal {
      message: message.into(),
    })
  }

  /// An iterator over the remaining significant tokens
  fn rest(&self) -> impl Iterator<Item = Token<'src>> + 'run {
    self.tokens[self.next_token..]
      .iter()
      .copied()
      .filter(|token| token.kind != Whitespace)
  }

  /// The next significant token
  fn next(&self) -> CompileResult<'src, Token<'src>> {
    if let Some(token) = self.rest().next() {
      Ok(token)
    } else {
      Err(self.internal_error("`Parser::next()` called after end of token stream")?)
    }
  }

  /// Check if the next significant token is of kind `kind`
  fn next_is(&mut self, kind: TokenKind) -> bool {
    self.next_are(&[kind])
  }

  /// Check if the next significant tokens are of kinds `kinds`
  ///
  /// The first token in `kinds` will be added to the expected token set.
  fn next_are(&mut self, kinds: &[TokenKind]) -> bool {
    if let Some(&kind) = kinds.first() {
      self.expected_tokens.insert(kind);
    }

    let mut rest = self.rest();
    for kind in kinds {
      match rest.next() {
        Some(token) => {
          if token.kind != *kind {
            return false;
          }
        }
        None => return false,
      }
    }
    true
  }

  /// Advance past one significant token, clearing the expected token set.
  fn advance(&mut self) -> CompileResult<'src, Token<'src>> {
    self.expected_tokens.clear();

    for skipped in &self.tokens[self.next_token..] {
      self.next_token += 1;

      if skipped.kind != Whitespace {
        return Ok(*skipped);
      }
    }

    Err(self.internal_error("`Parser::advance()` advanced past end of token stream")?)
  }

  /// Return the next token if it is of kind `expected`, otherwise, return an
  /// unexpected token error
  fn expect(&mut self, expected: TokenKind) -> CompileResult<'src, Token<'src>> {
    if let Some(token) = self.accept(expected)? {
      Ok(token)
    } else {
      Err(self.unexpected_token()?)
    }
  }

  /// Return an unexpected token error if the next token is not an EOL
  fn expect_eol(&mut self) -> CompileResult<'src> {
    self.accept(Comment)?;

    if self.next_is(Eof) {
      return Ok(());
    }

    self.expect(Eol).map(|_| ())
  }

  fn expect_keyword(&mut self, expected: Keyword) -> CompileResult<'src> {
    let found = self.advance()?;

    if found.kind == Identifier && expected == found.lexeme() {
      Ok(())
    } else {
      Err(found.error(CompileErrorKind::ExpectedKeyword {
        expected: vec![expected],
        found,
      }))
    }
  }

  /// Return an internal error if the next token is not of kind `Identifier`
  /// with lexeme `lexeme`.
  fn presume_keyword(&mut self, keyword: Keyword) -> CompileResult<'src> {
    let next = self.advance()?;

    if next.kind != Identifier {
      Err(self.internal_error(format!(
        "Presumed next token would have kind {Identifier}, but found {}",
        next.kind
      ))?)
    } else if keyword == next.lexeme() {
      Ok(())
    } else {
      Err(self.internal_error(format!(
        "Presumed next token would have lexeme \"{keyword}\", but found \"{}\"",
        next.lexeme(),
      ))?)
    }
  }

  /// Return an internal error if the next token is not of kind `kind`.
  fn presume(&mut self, kind: TokenKind) -> CompileResult<'src, Token<'src>> {
    let next = self.advance()?;

    if next.kind == kind {
      Ok(next)
    } else {
      Err(self.internal_error(format!(
        "Presumed next token would have kind {kind:?}, but found {:?}",
        next.kind
      ))?)
    }
  }

  /// Return an internal error if the next token is not one of kinds `kinds`.
  fn presume_any(&mut self, kinds: &[TokenKind]) -> CompileResult<'src, Token<'src>> {
    let next = self.advance()?;
    if kinds.contains(&next.kind) {
      Ok(next)
    } else {
      Err(self.internal_error(format!(
        "Presumed next token would be {}, but found {}",
        List::or(kinds),
        next.kind
      ))?)
    }
  }

  /// Accept and return a token of kind `kind`
  fn accept(&mut self, kind: TokenKind) -> CompileResult<'src, Option<Token<'src>>> {
    if self.next_is(kind) {
      Ok(Some(self.advance()?))
    } else {
      Ok(None)
    }
  }

  /// Return an error if the next token is of kind `forbidden`
  fn forbid<F>(&self, forbidden: TokenKind, error: F) -> CompileResult<'src>
  where
    F: FnOnce(Token) -> CompileError,
  {
    let next = self.next()?;

    if next.kind == forbidden {
      Err(error(next))
    } else {
      Ok(())
    }
  }

  /// Accept a token of kind `Identifier` and parse into a `Name`
  fn accept_name(&mut self) -> CompileResult<'src, Option<Name<'src>>> {
    if self.next_is(Identifier) {
      Ok(Some(self.parse_name()?))
    } else {
      Ok(None)
    }
  }

  fn accepted_keyword(&mut self, keyword: Keyword) -> CompileResult<'src, bool> {
    let next = self.next()?;

    if next.kind == Identifier && next.lexeme() == keyword.lexeme() {
      self.advance()?;
      Ok(true)
    } else {
      Ok(false)
    }
  }

  /// Accept a dependency
  fn accept_dependency(&mut self) -> CompileResult<'src, Option<UnresolvedDependency<'src>>> {
    if let Some(recipe) = self.accept_name()? {
      Ok(Some(UnresolvedDependency {
        arguments: Vec::new(),
        recipe,
      }))
    } else if self.accepted(ParenL)? {
      let recipe = self.parse_name()?;

      let mut arguments = Vec::new();

      while !self.accepted(ParenR)? {
        arguments.push(self.parse_expression()?);
      }

      Ok(Some(UnresolvedDependency { recipe, arguments }))
    } else {
      Ok(None)
    }
  }

  /// Accept and return `true` if next token is of kind `kind`
  fn accepted(&mut self, kind: TokenKind) -> CompileResult<'src, bool> {
    Ok(self.accept(kind)?.is_some())
  }

  /// Parse a justfile, consumes self
  fn parse_ast(mut self) -> CompileResult<'src, Ast<'src>> {
    fn pop_doc_comment<'src>(
      items: &mut Vec<Item<'src>>,
      eol_since_last_comment: bool,
    ) -> Option<&'src str> {
      if !eol_since_last_comment {
        if let Some(Item::Comment(contents)) = items.last() {
          let doc = Some(contents[1..].trim_start());
          items.pop();
          return doc;
        }
      }

      None
    }

    let mut items = Vec::new();

    let mut eol_since_last_comment = false;

    self.accept(ByteOrderMark)?;

    loop {
      let mut attributes = self.parse_attributes()?;
      let mut take_attributes = || {
        attributes
          .take()
          .map(|(_token, attributes)| attributes)
          .unwrap_or_default()
      };

      let next = self.next()?;

      if let Some(comment) = self.accept(Comment)? {
        items.push(Item::Comment(comment.lexeme().trim_end()));
        self.expect_eol()?;
        eol_since_last_comment = false;
      } else if self.accepted(Eol)? {
        eol_since_last_comment = true;
      } else if self.accepted(Eof)? {
        break;
      } else if self.next_is(Identifier) {
        match Keyword::from_lexeme(next.lexeme()) {
          Some(Keyword::Alias) if self.next_are(&[Identifier, Identifier, ColonEquals]) => {
            items.push(Item::Alias(self.parse_alias(take_attributes())?));
          }
          Some(Keyword::Export) if self.next_are(&[Identifier, Identifier, ColonEquals]) => {
            self.presume_keyword(Keyword::Export)?;
            items.push(Item::Assignment(
              self.parse_assignment(true, take_attributes())?,
            ));
          }
          Some(Keyword::Unexport)
            if self.next_are(&[Identifier, Identifier, Eof])
              || self.next_are(&[Identifier, Identifier, Eol]) =>
          {
            self.presume_keyword(Keyword::Unexport)?;
            let name = self.parse_name()?;
            self.expect_eol()?;
            items.push(Item::Unexport { name });
          }
          Some(Keyword::Import)
            if self.next_are(&[Identifier, StringToken])
              || self.next_are(&[Identifier, Identifier, StringToken])
              || self.next_are(&[Identifier, QuestionMark]) =>
          {
            self.presume_keyword(Keyword::Import)?;
            let optional = self.accepted(QuestionMark)?;
            let (path, relative) = self.parse_string_literal_token()?;
            items.push(Item::Import {
              absolute: None,
              optional,
              path,
              relative,
            });
          }
          Some(Keyword::Mod)
            if self.next_are(&[Identifier, Identifier, Comment])
              || self.next_are(&[Identifier, Identifier, Eof])
              || self.next_are(&[Identifier, Identifier, Eol])
              || self.next_are(&[Identifier, Identifier, Identifier, StringToken])
              || self.next_are(&[Identifier, Identifier, StringToken])
              || self.next_are(&[Identifier, QuestionMark]) =>
          {
            let doc = pop_doc_comment(&mut items, eol_since_last_comment);

            self.presume_keyword(Keyword::Mod)?;

            let optional = self.accepted(QuestionMark)?;

            let name = self.parse_name()?;

            let relative = if self.next_is(StringToken) || self.next_are(&[Identifier, StringToken])
            {
              Some(self.parse_string_literal()?)
            } else {
              None
            };

            let attributes = take_attributes();

            attributes.ensure_valid_attributes(
              "Module",
              *name,
              &[AttributeDiscriminant::Doc, AttributeDiscriminant::Group],
            )?;

            let doc = match attributes.get(AttributeDiscriminant::Doc) {
              Some(Attribute::Doc(Some(doc))) => Some(doc.cooked.clone()),
              Some(Attribute::Doc(None)) => None,
              None => doc.map(ToOwned::to_owned),
              _ => unreachable!(),
            };

            let mut groups = Vec::new();
            for attribute in attributes {
              if let Attribute::Group(group) = attribute {
                groups.push(group.cooked);
              }
            }

            items.push(Item::Module {
              groups,
              absolute: None,
              doc,
              name,
              optional,
              relative,
            });
          }
          Some(Keyword::Set)
            if self.next_are(&[Identifier, Identifier, ColonEquals])
              || self.next_are(&[Identifier, Identifier, Comment, Eof])
              || self.next_are(&[Identifier, Identifier, Comment, Eol])
              || self.next_are(&[Identifier, Identifier, Eof])
              || self.next_are(&[Identifier, Identifier, Eol]) =>
          {
            items.push(Item::Set(self.parse_set()?));
          }
          _ => {
            if self.next_are(&[Identifier, ColonEquals]) {
              items.push(Item::Assignment(
                self.parse_assignment(false, take_attributes())?,
              ));
            } else {
              let doc = pop_doc_comment(&mut items, eol_since_last_comment);
              items.push(Item::Recipe(self.parse_recipe(
                doc,
                false,
                take_attributes(),
              )?));
            }
          }
        }
      } else if self.accepted(At)? {
        let doc = pop_doc_comment(&mut items, eol_since_last_comment);
        items.push(Item::Recipe(self.parse_recipe(
          doc,
          true,
          take_attributes(),
        )?));
      } else {
        return Err(self.unexpected_token()?);
      }

      if let Some((token, attributes)) = attributes {
        return Err(token.error(CompileErrorKind::ExtraneousAttributes {
          count: attributes.len(),
        }));
      }
    }

    if self.next_token != self.tokens.len() {
      return Err(self.internal_error(format!(
        "Parse completed with {} unparsed tokens",
        self.tokens.len() - self.next_token,
      ))?);
    }

    Ok(Ast {
      items,
      unstable_features: self.unstable_features,
      warnings: Vec::new(),
      working_directory: self.working_directory.into(),
    })
  }

  /// Parse an alias, e.g `alias name := target`
  fn parse_alias(
    &mut self,
    attributes: AttributeSet<'src>,
  ) -> CompileResult<'src, Alias<'src, Name<'src>>> {
    self.presume_keyword(Keyword::Alias)?;
    let name = self.parse_name()?;
    self.presume_any(&[Equals, ColonEquals])?;
    let target = self.parse_name()?;
    self.expect_eol()?;

    attributes.ensure_valid_attributes("Alias", *name, &[AttributeDiscriminant::Private])?;

    Ok(Alias {
      attributes,
      name,
      target,
    })
  }

  /// Parse an assignment, e.g. `foo := bar`
  fn parse_assignment(
    &mut self,
    export: bool,
    attributes: AttributeSet<'src>,
  ) -> CompileResult<'src, Assignment<'src>> {
    let name = self.parse_name()?;
    self.presume(ColonEquals)?;
    let value = self.parse_expression()?;
    self.expect_eol()?;

    let private = attributes.contains(AttributeDiscriminant::Private);

    attributes.ensure_valid_attributes("Assignment", *name, &[AttributeDiscriminant::Private])?;

    Ok(Assignment {
      constant: false,
      export,
      file_depth: self.file_depth,
      name,
      private: private || name.lexeme().starts_with('_'),
      value,
    })
  }

  /// Parse an expression, e.g. `1 + 2`
  fn parse_expression(&mut self) -> CompileResult<'src, Expression<'src>> {
    if self.recursion_depth == if cfg!(windows) { 48 } else { 256 } {
      let token = self.next()?;
      return Err(CompileError::new(
        token,
        CompileErrorKind::ParsingRecursionDepthExceeded,
      ));
    }

    self.recursion_depth += 1;

    let disjunct = self.parse_disjunct()?;

    let expression = if self.accepted(BarBar)? {
      self
        .unstable_features
        .insert(UnstableFeature::LogicalOperators);
      let lhs = disjunct.into();
      let rhs = self.parse_expression()?.into();
      Expression::Or { lhs, rhs }
    } else {
      disjunct
    };

    self.recursion_depth -= 1;

    Ok(expression)
  }

  fn parse_disjunct(&mut self) -> CompileResult<'src, Expression<'src>> {
    let conjunct = self.parse_conjunct()?;

    let disjunct = if self.accepted(AmpersandAmpersand)? {
      self
        .unstable_features
        .insert(UnstableFeature::LogicalOperators);
      let lhs = conjunct.into();
      let rhs = self.parse_disjunct()?.into();
      Expression::And { lhs, rhs }
    } else {
      conjunct
    };

    Ok(disjunct)
  }

  fn parse_conjunct(&mut self) -> CompileResult<'src, Expression<'src>> {
    if self.accepted_keyword(Keyword::If)? {
      self.parse_conditional()
    } else if self.accepted(Slash)? {
      let lhs = None;
      let rhs = self.parse_conjunct()?.into();
      Ok(Expression::Join { lhs, rhs })
    } else {
      let value = self.parse_value()?;

      if self.accepted(Slash)? {
        let lhs = Some(Box::new(value));
        let rhs = self.parse_conjunct()?.into();
        Ok(Expression::Join { lhs, rhs })
      } else if self.accepted(Plus)? {
        let lhs = value.into();
        let rhs = self.parse_conjunct()?.into();
        Ok(Expression::Concatenation { lhs, rhs })
      } else {
        Ok(value)
      }
    }
  }

  /// Parse a conditional, e.g. `if a == b { "foo" } else { "bar" }`
  fn parse_conditional(&mut self) -> CompileResult<'src, Expression<'src>> {
    let condition = self.parse_condition()?;

    self.expect(BraceL)?;

    let then = self.parse_expression()?;

    self.expect(BraceR)?;

    self.expect_keyword(Keyword::Else)?;

    let otherwise = if self.accepted_keyword(Keyword::If)? {
      self.parse_conditional()?
    } else {
      self.expect(BraceL)?;
      let otherwise = self.parse_expression()?;
      self.expect(BraceR)?;
      otherwise
    };

    Ok(Expression::Conditional {
      condition,
      then: then.into(),
      otherwise: otherwise.into(),
    })
  }

  fn parse_condition(&mut self) -> CompileResult<'src, Condition<'src>> {
    let lhs = self.parse_expression()?;
    let operator = if self.accepted(BangEquals)? {
      ConditionalOperator::Inequality
    } else if self.accepted(EqualsTilde)? {
      ConditionalOperator::RegexMatch
    } else if self.accepted(BangTilde)? {
      ConditionalOperator::RegexMismatch
    } else {
      self.expect(EqualsEquals)?;
      ConditionalOperator::Equality
    };
    let rhs = self.parse_expression()?;
    Ok(Condition {
      lhs: lhs.into(),
      rhs: rhs.into(),
      operator,
    })
  }

  // Check if the next tokens are a shell-expanded string, i.e., `x"foo"`.
  //
  // This function skips initial whitespace tokens, but thereafter is
  // whitespace-sensitive, so `x"foo"` is a shell-expanded string, whereas `x
  // "foo"` is not.
  fn next_is_shell_expanded_string(&self) -> bool {
    let mut tokens = self
      .tokens
      .iter()
      .skip(self.next_token)
      .skip_while(|token| token.kind == Whitespace);

    tokens
      .next()
      .is_some_and(|token| token.kind == Identifier && token.lexeme() == "x")
      && tokens.next().is_some_and(|token| token.kind == StringToken)
  }

  /// Parse a value, e.g. `(bar)`
  fn parse_value(&mut self) -> CompileResult<'src, Expression<'src>> {
    if self.next_is(StringToken) || self.next_is_shell_expanded_string() {
      Ok(Expression::StringLiteral {
        string_literal: self.parse_string_literal()?,
      })
    } else if self.next_is(Backtick) {
      let next = self.next()?;
      let kind = StringKind::from_string_or_backtick(next)?;
      let contents =
        &next.lexeme()[kind.delimiter_len()..next.lexeme().len() - kind.delimiter_len()];
      let token = self.advance()?;
      let contents = if kind.indented() {
        unindent(contents)
      } else {
        contents.to_owned()
      };

      if contents.starts_with("#!") {
        return Err(next.error(CompileErrorKind::BacktickShebang));
      }
      Ok(Expression::Backtick { contents, token })
    } else if self.next_is(Identifier) {
      if self.accepted_keyword(Keyword::Assert)? {
        self.expect(ParenL)?;
        let condition = self.parse_condition()?;
        self.expect(Comma)?;
        let error = Box::new(self.parse_expression()?);
        self.expect(ParenR)?;
        Ok(Expression::Assert { condition, error })
      } else {
        let name = self.parse_name()?;

        if self.next_is(ParenL) {
          let arguments = self.parse_sequence()?;
          if name.lexeme() == "which" {
            self.unstable_features.insert(UnstableFeature::WhichFunction);
          }
          Ok(Expression::Call {
            thunk: Thunk::resolve(name, arguments)?,
          })
        } else {
          Ok(Expression::Variable { name })
        }
      }
    } else if self.next_is(ParenL) {
      self.presume(ParenL)?;
      let contents = self.parse_expression()?.into();
      self.expect(ParenR)?;
      Ok(Expression::Group { contents })
    } else {
      Err(self.unexpected_token()?)
    }
  }

  /// Parse a string literal, e.g. `"FOO"`, returning the string literal and the string token
  fn parse_string_literal_token(
    &mut self,
  ) -> CompileResult<'src, (Token<'src>, StringLiteral<'src>)> {
    let expand = if self.next_is(Identifier) {
      self.expect_keyword(Keyword::X)?;
      true
    } else {
      false
    };

    let token = self.expect(StringToken)?;

    let kind = StringKind::from_string_or_backtick(token)?;

    let delimiter_len = kind.delimiter_len();

    let raw = &token.lexeme()[delimiter_len..token.lexeme().len() - delimiter_len];

    let unindented = if kind.indented() {
      unindent(raw)
    } else {
      raw.to_owned()
    };

    let cooked = if kind.processes_escape_sequences() {
      Self::cook_string(token, &unindented)?
    } else {
      unindented
    };

    let cooked = if expand {
      shellexpand::full(&cooked)
        .map_err(|err| token.error(CompileErrorKind::ShellExpansion { err }))?
        .into_owned()
    } else {
      cooked
    };

    Ok((
      token,
      StringLiteral {
        cooked,
        expand,
        kind,
        raw,
      },
    ))
  }

  // Transform escape sequences in from string literal `token` with content `text`
  fn cook_string(token: Token<'src>, text: &str) -> CompileResult<'src, String> {
    #[derive(PartialEq, Eq)]
    enum State {
      Initial,
      Backslash,
      Unicode,
      UnicodeValue { hex: String },
    }

    let mut cooked = String::new();

    let mut state = State::Initial;

    for c in text.chars() {
      match state {
        State::Initial => {
          if c == '\\' {
            state = State::Backslash;
          } else {
            cooked.push(c);
          }
        }
        State::Backslash if c == 'u' => {
          state = State::Unicode;
        }
        State::Backslash => {
          match c {
            'n' => cooked.push('\n'),
            'r' => cooked.push('\r'),
            't' => cooked.push('\t'),
            '\\' => cooked.push('\\'),
            '\n' => {}
            '"' => cooked.push('"'),
            character => {
              return Err(token.error(CompileErrorKind::InvalidEscapeSequence { character }))
            }
          }
          state = State::Initial;
        }
        State::Unicode => match c {
          '{' => {
            state = State::UnicodeValue { hex: String::new() };
          }
          character => {
            return Err(token.error(CompileErrorKind::UnicodeEscapeDelimiter { character }));
          }
        },
        State::UnicodeValue { ref mut hex } => match c {
          '}' => {
            if hex.is_empty() {
              return Err(token.error(CompileErrorKind::UnicodeEscapeEmpty));
            }

            let codepoint = u32::from_str_radix(hex, 16).unwrap();

            cooked.push(char::from_u32(codepoint).ok_or_else(|| {
              token.error(CompileErrorKind::UnicodeEscapeRange { hex: hex.clone() })
            })?);

            state = State::Initial;
          }
          '0'..='9' | 'A'..='F' | 'a'..='f' => {
            hex.push(c);
            if hex.len() > 6 {
              return Err(token.error(CompileErrorKind::UnicodeEscapeLength { hex: hex.clone() }));
            }
          }
          _ => {
            return Err(token.error(CompileErrorKind::UnicodeEscapeCharacter { character: c }));
          }
        },
      }
    }

    if state != State::Initial {
      return Err(token.error(CompileErrorKind::UnicodeEscapeUnterminated));
    }

    Ok(cooked)
  }

  /// Parse a string literal, e.g. `"FOO"`
  fn parse_string_literal(&mut self) -> CompileResult<'src, StringLiteral<'src>> {
    let (_token, string_literal) = self.parse_string_literal_token()?;
    Ok(string_literal)
  }

  /// Parse a name from an identifier token
  fn parse_name(&mut self) -> CompileResult<'src, Name<'src>> {
    self.expect(Identifier).map(Name::from_identifier)
  }

  /// Parse sequence of comma-separated expressions
  fn parse_sequence(&mut self) -> CompileResult<'src, Vec<Expression<'src>>> {
    self.presume(ParenL)?;

    let mut elements = Vec::new();

    while !self.next_is(ParenR) {
      elements.push(self.parse_expression()?);

      if !self.accepted(Comma)? {
        break;
      }
    }

    self.expect(ParenR)?;

    Ok(elements)
  }

  /// Parse a recipe
  fn parse_recipe(
    &mut self,
    doc: Option<&'src str>,
    quiet: bool,
    attributes: AttributeSet<'src>,
  ) -> CompileResult<'src, UnresolvedRecipe<'src>> {
    let name = self.parse_name()?;

    let mut positional = Vec::new();

    while self.next_is(Identifier) || self.next_is(Dollar) {
      positional.push(self.parse_parameter(ParameterKind::Singular)?);
    }

    let kind = if self.accepted(Plus)? {
      ParameterKind::Plus
    } else if self.accepted(Asterisk)? {
      ParameterKind::Star
    } else {
      ParameterKind::Singular
    };

    let variadic = if kind.is_variadic() {
      let variadic = self.parse_parameter(kind)?;

      self.forbid(Identifier, |token| {
        token.error(CompileErrorKind::ParameterFollowsVariadicParameter {
          parameter: token.lexeme(),
        })
      })?;

      Some(variadic)
    } else {
      None
    };

    self.expect(Colon)?;

    let mut dependencies = Vec::new();

    while let Some(dependency) = self.accept_dependency()? {
      dependencies.push(dependency);
    }

    let priors = dependencies.len();

    if self.accepted(AmpersandAmpersand)? {
      let mut subsequents = Vec::new();

      while let Some(subsequent) = self.accept_dependency()? {
        subsequents.push(subsequent);
      }

      if subsequents.is_empty() {
        return Err(self.unexpected_token()?);
      }

      dependencies.append(&mut subsequents);
    }

    self.expect_eol()?;

    let body = self.parse_body()?;

    let shebang = body.first().is_some_and(Line::is_shebang);
    let script = attributes.contains(AttributeDiscriminant::Script);

    if shebang && script {
      return Err(name.error(CompileErrorKind::ShebangAndScriptAttribute {
        recipe: name.lexeme(),
      }));
    }

    let working_directory = attributes.contains(AttributeDiscriminant::WorkingDirectory);

    if working_directory && attributes.contains(AttributeDiscriminant::NoCd) {
      return Err(
        name.error(CompileErrorKind::NoCdAndWorkingDirectoryAttribute {
          recipe: name.lexeme(),
        }),
      );
    }

    let private =
      name.lexeme().starts_with('_') || attributes.contains(AttributeDiscriminant::Private);

    let mut doc = doc.map(ToOwned::to_owned);

    for attribute in &attributes {
      if let Attribute::Doc(attribute_doc) = attribute {
        doc = attribute_doc.as_ref().map(|doc| doc.cooked.clone());
      }
    }

    Ok(Recipe {
      shebang: shebang || script,
      attributes,
      body,
      dependencies,
      doc: doc.filter(|doc| !doc.is_empty()),
      file_depth: self.file_depth,
      import_offsets: self.import_offsets.clone(),
      name,
      namepath: self.module_namepath.join(name),
      parameters: positional.into_iter().chain(variadic).collect(),
      priors,
      private,
      quiet,
    })
  }

  /// Parse a recipe parameter
  fn parse_parameter(&mut self, kind: ParameterKind) -> CompileResult<'src, Parameter<'src>> {
    let export = self.accepted(Dollar)?;

    let name = self.parse_name()?;

    let default = if self.accepted(Equals)? {
      Some(self.parse_value()?)
    } else {
      None
    };

    Ok(Parameter {
      default,
      export,
      kind,
      name,
    })
  }

  /// Parse the body of a recipe
  fn parse_body(&mut self) -> CompileResult<'src, Vec<Line<'src>>> {
    let mut lines = Vec::new();

    if self.accepted(Indent)? {
      while !self.accepted(Dedent)? {
        let mut fragments = Vec::new();
        let number = self
          .tokens
          .get(self.next_token)
          .map(|token| token.line)
          .unwrap_or_default();

        if !self.accepted(Eol)? {
          while !(self.accepted(Eol)? || self.next_is(Dedent)) {
            if let Some(token) = self.accept(Text)? {
              fragments.push(Fragment::Text { token });
            } else if self.accepted(InterpolationStart)? {
              fragments.push(Fragment::Interpolation {
                expression: self.parse_expression()?,
              });
              self.expect(InterpolationEnd)?;
            } else {
              return Err(self.unexpected_token()?);
            }
          }
        };

        lines.push(Line { fragments, number });
      }
    }

    while lines.last().is_some_and(Line::is_empty) {
      lines.pop();
    }

    Ok(lines)
  }

  /// Parse a boolean setting value
  fn parse_set_bool(&mut self) -> CompileResult<'src, bool> {
    if !self.accepted(ColonEquals)? {
      return Ok(true);
    }

    let identifier = self.expect(Identifier)?;

    let value = if Keyword::True == identifier.lexeme() {
      true
    } else if Keyword::False == identifier.lexeme() {
      false
    } else {
      return Err(identifier.error(CompileErrorKind::ExpectedKeyword {
        expected: vec![Keyword::True, Keyword::False],
        found: identifier,
      }));
    };

    Ok(value)
  }

  /// Parse a setting
  fn parse_set(&mut self) -> CompileResult<'src, Set<'src>> {
    self.presume_keyword(Keyword::Set)?;
    let name = Name::from_identifier(self.presume(Identifier)?);
    let lexeme = name.lexeme();
    let Some(keyword) = Keyword::from_lexeme(lexeme) else {
      return Err(name.error(CompileErrorKind::UnknownSetting {
        setting: name.lexeme(),
      }));
    };

    let set_bool = match keyword {
      Keyword::AllowDuplicateRecipes => {
        Some(Setting::AllowDuplicateRecipes(self.parse_set_bool()?))
      }
      Keyword::AllowDuplicateVariables => {
        Some(Setting::AllowDuplicateVariables(self.parse_set_bool()?))
      }
      Keyword::DotenvLoad => Some(Setting::DotenvLoad(self.parse_set_bool()?)),
      Keyword::DotenvRequired => Some(Setting::DotenvRequired(self.parse_set_bool()?)),
      Keyword::Export => Some(Setting::Export(self.parse_set_bool()?)),
      Keyword::Fallback => Some(Setting::Fallback(self.parse_set_bool()?)),
      Keyword::IgnoreComments => Some(Setting::IgnoreComments(self.parse_set_bool()?)),
      Keyword::PositionalArguments => Some(Setting::PositionalArguments(self.parse_set_bool()?)),
      Keyword::Quiet => Some(Setting::Quiet(self.parse_set_bool()?)),
      Keyword::Unstable => Some(Setting::Unstable(self.parse_set_bool()?)),
      Keyword::WindowsPowershell => Some(Setting::WindowsPowerShell(self.parse_set_bool()?)),
      _ => None,
    };

    if let Some(value) = set_bool {
      return Ok(Set { name, value });
    }

    self.expect(ColonEquals)?;

    let set_value = match keyword {
      Keyword::DotenvFilename => Some(Setting::DotenvFilename(self.parse_string_literal()?)),
      Keyword::DotenvPath => Some(Setting::DotenvPath(self.parse_string_literal()?)),
      Keyword::ScriptInterpreter => Some(Setting::ScriptInterpreter(self.parse_interpreter()?)),
      Keyword::Shell => Some(Setting::Shell(self.parse_interpreter()?)),
      Keyword::Tempdir => Some(Setting::Tempdir(self.parse_string_literal()?)),
      Keyword::WindowsShell => Some(Setting::WindowsShell(self.parse_interpreter()?)),
      Keyword::WorkingDirectory => Some(Setting::WorkingDirectory(self.parse_string_literal()?)),
      _ => None,
    };

    if let Some(value) = set_value {
      return Ok(Set { name, value });
    }

    Err(name.error(CompileErrorKind::UnknownSetting {
      setting: name.lexeme(),
    }))
  }

  /// Parse interpreter setting value, i.e., `['sh', '-eu']`
  fn parse_interpreter(&mut self) -> CompileResult<'src, Interpreter<'src>> {
    self.expect(BracketL)?;

    let command = self.parse_string_literal()?;

    let mut arguments = Vec::new();

    if self.accepted(Comma)? {
      while !self.next_is(BracketR) {
        arguments.push(self.parse_string_literal()?);

        if !self.accepted(Comma)? {
          break;
        }
      }
    }

    self.expect(BracketR)?;

    Ok(Interpreter { arguments, command })
  }

  /// Item attributes, i.e., `[macos]` or `[confirm: "warning!"]`
  fn parse_attributes(&mut self) -> CompileResult<'src, Option<(Token<'src>, AttributeSet<'src>)>> {
    let mut attributes = BTreeMap::new();
    let mut discriminants = BTreeMap::new();

    let mut token = None;

    while let Some(bracket) = self.accept(BracketL)? {
      token.get_or_insert(bracket);

      loop {
        let name = self.parse_name()?;

        let mut arguments = Vec::new();

        if self.accepted(Colon)? {
          arguments.push(self.parse_string_literal()?);
        } else if self.accepted(ParenL)? {
          loop {
            arguments.push(self.parse_string_literal()?);

            if !self.accepted(Comma)? {
              break;
            }
          }
          self.expect(ParenR)?;
        }

        let attribute = Attribute::new(name, arguments)?;

        let first = attributes.get(&attribute).or_else(|| {
          if attribute.repeatable() {
            None
          } else {
            discriminants.get(&attribute.discriminant())
          }
        });

        if let Some(&first) = first {
          return Err(name.error(CompileErrorKind::DuplicateAttribute {
            attribute: name.lexeme(),
            first,
          }));
        }

        discriminants.insert(attribute.discriminant(), name.line);

        attributes.insert(attribute, name.line);

        if !self.accepted(Comma)? {
          break;
        }
      }
      self.expect(BracketR)?;
      self.expect_eol()?;
    }

    if attributes.is_empty() {
      Ok(None)
    } else {
      Ok(Some((token.unwrap(), attributes.into_keys().collect())))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;
  use CompileErrorKind::*;

  macro_rules! test {
    {
      name: $name:ident,
      text: $text:expr,
      tree: $tree:tt,
    } => {
      #[test]
      fn $name() {
        let text: String = $text.into();
        let want = tree!($tree);
        test(&text, want);
      }
    }
  }

  fn test(text: &str, want: Tree) {
    let unindented = unindent(text);
    let tokens = Lexer::test_lex(&unindented).expect("lexing failed");
    let justfile = Parser::parse(0, &[], &Namepath::default(), &tokens, &PathBuf::new())
      .expect("parsing failed");
    let have = justfile.tree();
    if have != want {
      println!("parsed text: {unindented}");
      println!("expected:    {want}");
      println!("but got:     {have}");
      println!("tokens:      {tokens:?}");
      panic!();
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
    let tokens = Lexer::test_lex(src).expect("Lexing failed in parse test...");

    match Parser::parse(0, &[], &Namepath::default(), &tokens, &PathBuf::new()) {
      Ok(_) => panic!("Parsing unexpectedly succeeded"),
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
    name: empty,
    text: "",
    tree: (justfile),
  }

  test! {
    name: empty_multiline,
    text: "





    ",
    tree: (justfile),
  }

  test! {
    name: whitespace,
    text: " ",
    tree: (justfile),
  }

  test! {
    name: alias_single,
    text: "alias t := test",
    tree: (justfile (alias t test)),
  }

  test! {
    name: alias_with_attribute,
    text: "[private]\nalias t := test",
    tree: (justfile (alias t test)),
  }

  test! {
    name: single_argument_attribute_shorthand,
    text: "[group: 'foo']\nbar:",
    tree: (justfile (recipe bar)),
  }

  test! {
    name: single_argument_attribute_shorthand_multiple_same_line,
    text: "[group: 'foo', group: 'bar']\nbaz:",
    tree: (justfile (recipe baz)),
  }

  test! {
    name: aliases_multiple,
    text: "alias t := test\nalias b := build",
    tree: (
      justfile
      (alias t test)
      (alias b build)
    ),
  }

  test! {
    name: alias_equals,
    text: "alias t := test",
    tree: (justfile
      (alias t test)
    ),
  }

  test! {
      name: recipe_named_alias,
      text: r"
      [private]
      alias:
        echo 'echoing alias'
          ",
    tree: (justfile
      (recipe alias (body ("echo 'echoing alias'")))
    ),
  }

  test! {
    name: export,
    text: r#"export x := "hello""#,
    tree: (justfile (assignment #export x "hello")),
  }

  test! {
    name: private_export,
    text: "
      [private]
      export x := 'hello'
    ",
    tree: (justfile (assignment #export x "hello")),
  }

  test! {
    name: export_equals,
    text: r#"export x := "hello""#,
    tree: (justfile
      (assignment #export x "hello")
    ),
  }

  test! {
    name: assignment,
    text: r#"x := "hello""#,
    tree: (justfile (assignment x "hello")),
  }

  test! {
    name: private_assignment,
    text: "
      [private]
      x := 'hello'
      ",
    tree: (justfile (assignment x "hello")),
  }

  test! {
    name: assignment_equals,
    text: r#"x := "hello""#,
    tree: (justfile
      (assignment x "hello")
    ),
  }

  test! {
    name: backtick,
    text: "x := `hello`",
    tree: (justfile (assignment x (backtick "hello"))),
  }

  test! {
    name: variable,
    text: "x := y",
    tree: (justfile (assignment x y)),
  }

  test! {
    name: group,
    text: "x := (y)",
    tree: (justfile (assignment x (y))),
  }

  test! {
    name: addition_single,
    text: "x := a + b",
    tree: (justfile (assignment x (+ a b))),
  }

  test! {
    name: addition_chained,
    text: "x := a + b + c",
    tree: (justfile (assignment x (+ a (+ b c)))),
  }

  test! {
    name: call_one_arg,
    text: "x := env_var(y)",
    tree: (justfile (assignment x (call env_var y))),
  }

  test! {
    name: call_multiple_args,
    text: "x := env_var_or_default(y, z)",
    tree: (justfile (assignment x (call env_var_or_default y z))),
  }

  test! {
    name: call_trailing_comma,
    text: "x := env_var(y,)",
    tree: (justfile (assignment x (call env_var y))),
  }

  test! {
    name: recipe,
    text: "foo:",
    tree: (justfile (recipe foo)),
  }

  test! {
    name: recipe_multiple,
    text: "
      foo:
      bar:
      baz:
    ",
    tree: (justfile (recipe foo) (recipe bar) (recipe baz)),
  }

  test! {
    name: recipe_quiet,
    text: "@foo:",
    tree: (justfile (recipe #quiet foo)),
  }

  test! {
    name: recipe_parameter_single,
    text: "foo bar:",
    tree: (justfile (recipe foo (params (bar)))),
  }

  test! {
    name: recipe_parameter_multiple,
    text: "foo bar baz:",
    tree: (justfile (recipe foo (params (bar) (baz)))),
  }

  test! {
    name: recipe_default_single,
    text: r#"foo bar="baz":"#,
    tree: (justfile (recipe foo (params (bar "baz")))),
  }

  test! {
    name: recipe_default_multiple,
    text: r#"foo bar="baz" bob="biz":"#,
    tree: (justfile (recipe foo (params (bar "baz") (bob "biz")))),
  }

  test! {
    name: recipe_plus_variadic,
    text: r"foo +bar:",
    tree: (justfile (recipe foo (params +(bar)))),
  }

  test! {
    name: recipe_star_variadic,
    text: r"foo *bar:",
    tree: (justfile (recipe foo (params *(bar)))),
  }

  test! {
    name: recipe_variadic_string_default,
    text: r#"foo +bar="baz":"#,
    tree: (justfile (recipe foo (params +(bar "baz")))),
  }

  test! {
    name: recipe_variadic_variable_default,
    text: r"foo +bar=baz:",
    tree: (justfile (recipe foo (params +(bar baz)))),
  }

  test! {
    name: recipe_variadic_addition_group_default,
    text: r"foo +bar=(baz + bob):",
    tree: (justfile (recipe foo (params +(bar ((+ baz bob)))))),
  }

  test! {
    name: recipe_dependency_single,
    text: "foo: bar",
    tree: (justfile (recipe foo (deps bar))),
  }

  test! {
    name: recipe_dependency_multiple,
    text: "foo: bar baz",
    tree: (justfile (recipe foo (deps bar baz))),
  }

  test! {
    name: recipe_dependency_parenthesis,
    text: "foo: (bar)",
    tree: (justfile (recipe foo (deps bar))),
  }

  test! {
    name: recipe_dependency_argument_string,
    text: "foo: (bar 'baz')",
    tree: (justfile (recipe foo (deps (bar "baz")))),
  }

  test! {
    name: recipe_dependency_argument_identifier,
    text: "foo: (bar baz)",
    tree: (justfile (recipe foo (deps (bar baz)))),
  }

  test! {
    name: recipe_dependency_argument_concatenation,
    text: "foo: (bar 'a' + 'b' 'c' + 'd')",
    tree: (justfile (recipe foo (deps (bar (+ 'a' 'b') (+ 'c' 'd'))))),
  }

  test! {
    name: recipe_subsequent,
    text: "foo: && bar",
    tree: (justfile (recipe foo (sups bar))),
  }

  test! {
    name: recipe_line_single,
    text: "foo:\n bar",
    tree: (justfile (recipe foo (body ("bar")))),
  }

  test! {
    name: recipe_line_multiple,
    text: "foo:\n bar\n baz\n {{\"bob\"}}biz",
    tree: (justfile (recipe foo (body ("bar") ("baz") (("bob") "biz")))),
  }

  test! {
    name: recipe_line_interpolation,
    text: "foo:\n bar{{\"bob\"}}biz",
    tree: (justfile (recipe foo (body ("bar" ("bob") "biz")))),
  }

  test! {
    name: comment,
    text: "# foo",
    tree: (justfile (comment "# foo")),
  }

  test! {
    name: comment_before_alias,
    text: "# foo\nalias x := y",
    tree: (justfile (comment "# foo") (alias x y)),
  }

  test! {
    name: comment_after_alias,
    text: "alias x := y # foo",
    tree: (justfile (alias x y)),
  }

  test! {
    name: comment_assignment,
    text: "x := y # foo",
    tree: (justfile (assignment x y)),
  }

  test! {
    name: comment_export,
    text: "export x := y # foo",
    tree: (justfile (assignment #export x y)),
  }

  test! {
    name: comment_recipe,
    text: "foo: # bar",
    tree: (justfile (recipe foo)),
  }

  test! {
    name: comment_recipe_dependencies,
    text: "foo: bar # baz",
    tree: (justfile (recipe foo (deps bar))),
  }

  test! {
    name: doc_comment_single,
    text: "
      # foo
      bar:
    ",
    tree: (justfile (recipe "foo" bar)),
  }

  test! {
    name: doc_comment_recipe_clear,
    text: "
      # foo
      bar:
      baz:
    ",
    tree: (justfile (recipe "foo" bar) (recipe baz)),
  }

  test! {
    name: doc_comment_middle,
    text: "
      bar:
      # foo
      baz:
    ",
    tree: (justfile (recipe bar) (recipe "foo" baz)),
  }

  test! {
    name: doc_comment_assignment_clear,
    text: "
      # foo
      x := y
      bar:
    ",
    tree: (justfile (comment "# foo") (assignment x y) (recipe bar)),
  }

  test! {
    name: doc_comment_empty_line_clear,
    text: "
      # foo

      bar:
    ",
    tree: (justfile (comment "# foo") (recipe bar)),
  }

  test! {
    name: string_escape_tab,
    text: r#"x := "foo\tbar""#,
    tree: (justfile (assignment x "foo\tbar")),
  }

  test! {
    name: string_escape_newline,
    text: r#"x := "foo\nbar""#,
    tree: (justfile (assignment x "foo\nbar")),
  }

  test! {
    name: string_escape_suppress_newline,
    text: r#"
      x := "foo\
      bar"
    "#,
    tree: (justfile (assignment x "foobar")),
  }

  test! {
    name: string_escape_carriage_return,
    text: r#"x := "foo\rbar""#,
    tree: (justfile (assignment x "foo\rbar")),
  }

  test! {
    name: string_escape_slash,
    text: r#"x := "foo\\bar""#,
    tree: (justfile (assignment x "foo\\bar")),
  }

  test! {
    name: string_escape_quote,
    text: r#"x := "foo\"bar""#,
    tree: (justfile (assignment x "foo\"bar")),
  }

  test! {
    name: indented_string_raw_with_dedent,
    text: "
      x := '''
        foo\\t
        bar\\n
      '''
    ",
    tree: (justfile (assignment x "foo\\t\nbar\\n\n")),
  }

  test! {
    name: indented_string_raw_no_dedent,
    text: "
      x := '''
      foo\\t
        bar\\n
      '''
    ",
    tree: (justfile (assignment x "foo\\t\n  bar\\n\n")),
  }

  test! {
    name: indented_string_cooked,
    text: r#"
      x := """
        \tfoo\t
        \tbar\n
      """
    "#,
    tree: (justfile (assignment x "\tfoo\t\n\tbar\n\n")),
  }

  test! {
    name: indented_string_cooked_no_dedent,
    text: r#"
      x := """
      \tfoo\t
        \tbar\n
      """
    "#,
    tree: (justfile (assignment x "\tfoo\t\n  \tbar\n\n")),
  }

  test! {
    name: indented_backtick,
    text: r"
      x := ```
        \tfoo\t
        \tbar\n
      ```
    ",
    tree: (justfile (assignment x (backtick "\\tfoo\\t\n\\tbar\\n\n"))),
  }

  test! {
    name: indented_backtick_no_dedent,
    text: r"
      x := ```
      \tfoo\t
        \tbar\n
      ```
    ",
    tree: (justfile (assignment x (backtick "\\tfoo\\t\n  \\tbar\\n\n"))),
  }

  test! {
    name: recipe_variadic_with_default_after_default,
    text: r"
      f a=b +c=d:
    ",
    tree: (justfile (recipe f (params (a b) +(c d)))),
  }

  test! {
    name: parameter_default_concatenation_variable,
    text: r#"
      x := "10"

      f y=(`echo hello` + x) +z="foo":
    "#,
    tree: (justfile
      (assignment x "10")
      (recipe f (params (y ((+ (backtick "echo hello") x))) +(z "foo")))
    ),
  }

  test! {
    name: parameter_default_multiple,
    text: r#"
      x := "10"
      f y=(`echo hello` + x) +z=("foo" + "bar"):
    "#,
    tree: (justfile
      (assignment x "10")
      (recipe f (params (y ((+ (backtick "echo hello") x))) +(z ((+ "foo" "bar")))))
    ),
  }

  test! {
    name: parse_raw_string_default,
    text: r"

      foo a='b\t':


    ",
    tree: (justfile (recipe foo (params (a "b\\t")))),
  }

  test! {
    name: parse_alias_after_target,
    text: r"
      foo:
        echo a
      alias f := foo
    ",
    tree: (justfile
      (recipe foo (body ("echo a")))
      (alias f foo)
    ),
  }

  test! {
    name: parse_alias_before_target,
    text: "
      alias f := foo
      foo:
        echo a
      ",
    tree: (justfile
      (alias f foo)
      (recipe foo (body ("echo a")))
    ),
  }

  test! {
    name: parse_alias_with_comment,
    text: "
      alias f := foo #comment
      foo:
        echo a
    ",
    tree: (justfile
      (alias f foo)
      (recipe foo (body ("echo a")))
    ),
  }

  test! {
    name: parse_assignment_with_comment,
    text: "
      f := foo #comment
      foo:
        echo a
    ",
    tree: (justfile
      (assignment f foo)
      (recipe foo (body ("echo a")))
    ),
  }

  test! {
    name: parse_complex,
    text: "
      x:
      y:
      z:
      foo := \"xx\"
      bar := foo
      goodbye := \"y\"
      hello a b    c   : x y    z #hello
        #! blah
        #blarg
        {{ foo + bar}}abc{{ goodbye\t  + \"x\" }}xyz
        1
        2
        3
    ",
    tree: (justfile
      (recipe x)
      (recipe y)
      (recipe z)
      (assignment foo "xx")
      (assignment bar foo)
      (assignment goodbye "y")
      (recipe hello
        (params (a) (b) (c))
        (deps x y z)
        (body
          ("#! blah")
          ("#blarg")
          (((+ foo bar)) "abc" ((+ goodbye "x")) "xyz")
          ("1")
          ("2")
          ("3")
        )
      )
    ),
  }

  test! {
    name: parse_shebang,
    text: "
      practicum := 'hello'
      install:
      \t#!/bin/sh
      \tif [[ -f {{practicum}} ]]; then
      \t\treturn
      \tfi
      ",
    tree: (justfile
      (assignment practicum "hello")
      (recipe install
        (body
         ("#!/bin/sh")
         ("if [[ -f " (practicum) " ]]; then")
         ("\treturn")
         ("fi")
        )
      )
    ),
  }

  test! {
    name: parse_simple_shebang,
    text: "a:\n #!\n  print(1)",
    tree: (justfile
      (recipe a (body ("#!") (" print(1)")))
    ),
  }

  test! {
    name: parse_assignments,
    text: r#"
      a := "0"
      c := a + b + a + b
      b := "1"
    "#,
    tree: (justfile
      (assignment a "0")
      (assignment c (+ a (+ b (+ a b))))
      (assignment b "1")
    ),
  }

  test! {
    name: parse_assignment_backticks,
    text: "
      a := `echo hello`
      c := a + b + a + b
      b := `echo goodbye`
    ",
    tree: (justfile
      (assignment a (backtick "echo hello"))
      (assignment c (+ a (+ b (+ a b))))
      (assignment b (backtick "echo goodbye"))
    ),
  }

  test! {
    name: parse_interpolation_backticks,
    text: r#"
      a:
        echo {{  `echo hello` + "blarg"   }} {{   `echo bob`   }}
    "#,
    tree: (justfile
      (recipe a
        (body ("echo " ((+ (backtick "echo hello") "blarg")) " " ((backtick "echo bob"))))
      )
    ),
  }

  test! {
    name: eof_test,
    text: "x:\ny:\nz:\na b c: x y z",
    tree: (justfile
      (recipe x)
      (recipe y)
      (recipe z)
      (recipe a (params (b) (c)) (deps x y z))
    ),
  }

  test! {
    name: string_quote_escape,
    text: r#"a := "hello\"""#,
    tree: (justfile
      (assignment a "hello\"")
    ),
  }

  test! {
    name: string_escapes,
    text: r#"a := "\n\t\r\"\\""#,
    tree: (justfile (assignment a "\n\t\r\"\\")),
  }

  test! {
    name: parameters,
    text: "
      a b c:
        {{b}} {{c}}
    ",
    tree: (justfile (recipe a (params (b) (c)) (body ((b) " " (c))))),
  }

  test! {
    name: unary_functions,
    text: "
      x := arch()

      a:
        {{os()}} {{os_family()}}
    ",
    tree: (justfile
      (assignment x (call arch))
      (recipe a (body (((call os)) " " ((call os_family)))))
    ),
  }

  test! {
    name: env_functions,
    text: r#"
      x := env_var('foo',)

      a:
        {{env_var_or_default('foo' + 'bar', 'baz',)}} {{env_var(env_var("baz"))}}
    "#,
    tree: (justfile
      (assignment x (call env_var "foo"))
      (recipe a
        (body
          (
            ((call env_var_or_default (+ "foo" "bar") "baz"))
            " "
            ((call env_var (call env_var "baz")))
          )
        )
      )
    ),
  }

  test! {
    name: parameter_default_string,
    text: r#"
      f x="abc":
    "#,
    tree: (justfile (recipe f (params (x "abc")))),
  }

  test! {
    name: parameter_default_raw_string,
    text: r"
      f x='abc':
    ",
    tree: (justfile (recipe f (params (x "abc")))),
  }

  test! {
    name: parameter_default_backtick,
    text: "
      f x=`echo hello`:
    ",
    tree: (justfile
      (recipe f (params (x (backtick "echo hello"))))
    ),
  }

  test! {
    name: parameter_default_concatenation_string,
    text: r#"
      f x=(`echo hello` + "foo"):
    "#,
    tree: (justfile (recipe f (params (x ((+ (backtick "echo hello") "foo")))))),
  }

  test! {
    name: concatenation_in_group,
    text: "x := ('0' + '1')",
    tree: (justfile (assignment x ((+ "0" "1")))),
  }

  test! {
    name: string_in_group,
    text: "x := ('0'   )",
    tree: (justfile (assignment x ("0"))),
  }

  test! {
    name: escaped_dos_newlines,
    text: "
      @spam:\r
      \t{ \\\r
      \t\tfiglet test; \\\r
      \t\tcargo build --color always 2>&1; \\\r
      \t\tcargo test  --color always -- --color always 2>&1; \\\r
      \t} | less\r
    ",
    tree: (justfile
      (recipe #quiet spam
        (body
         ("{ \\")
         ("\tfiglet test; \\")
         ("\tcargo build --color always 2>&1; \\")
         ("\tcargo test  --color always -- --color always 2>&1; \\")
         ("} | less")
        )
      )
    ),
  }

  test! {
    name: empty_body,
    text: "a:",
    tree: (justfile (recipe a)),
  }

  test! {
    name: single_line_body,
    text: "a:\n foo",
    tree: (justfile (recipe a (body ("foo")))),
  }

  test! {
    name: trimmed_body,
    text: "a:\n foo\n \n \n \nb:\n  ",
    tree: (justfile (recipe a (body ("foo"))) (recipe b)),
  }

  test! {
    name: set_export_implicit,
    text: "set export",
    tree: (justfile (set export true)),
  }

  test! {
    name: set_export_true,
    text: "set export := true",
    tree: (justfile (set export true)),
  }

  test! {
    name: set_export_false,
    text: "set export := false",
    tree: (justfile (set export false)),
  }

  test! {
    name: set_dotenv_load_implicit,
    text: "set dotenv-load",
    tree: (justfile (set dotenv_load true)),
  }

  test! {
    name: set_allow_duplicate_recipes_implicit,
    text: "set allow-duplicate-recipes",
    tree: (justfile (set allow_duplicate_recipes true)),
  }

  test! {
    name: set_allow_duplicate_variables_implicit,
    text: "set allow-duplicate-variables",
    tree: (justfile (set allow_duplicate_variables true)),
  }

  test! {
    name: set_dotenv_load_true,
    text: "set dotenv-load := true",
    tree: (justfile (set dotenv_load true)),
  }

  test! {
    name: set_dotenv_load_false,
    text: "set dotenv-load := false",
    tree: (justfile (set dotenv_load false)),
  }

  test! {
    name: set_positional_arguments_implicit,
    text: "set positional-arguments",
    tree: (justfile (set positional_arguments true)),
  }

  test! {
    name: set_positional_arguments_true,
    text: "set positional-arguments := true",
    tree: (justfile (set positional_arguments true)),
  }

  test! {
    name: set_quiet_implicit,
    text: "set quiet",
    tree: (justfile (set quiet true)),
  }

  test! {
    name: set_quiet_true,
    text: "set quiet := true",
    tree: (justfile (set quiet true)),
  }

  test! {
    name: set_quiet_false,
    text: "set quiet := false",
    tree: (justfile (set quiet false)),
  }

  test! {
    name: set_positional_arguments_false,
    text: "set positional-arguments := false",
    tree: (justfile (set positional_arguments false)),
  }

  test! {
    name: set_shell_no_arguments,
    text: "set shell := ['tclsh']",
    tree: (justfile (set shell "tclsh")),
  }

  test! {
    name: set_shell_no_arguments_cooked,
    text: "set shell := [\"tclsh\"]",
    tree: (justfile (set shell "tclsh")),
  }

  test! {
    name: set_shell_no_arguments_trailing_comma,
    text: "set shell := ['tclsh',]",
    tree: (justfile (set shell "tclsh")),
  }

  test! {
    name: set_shell_with_one_argument,
    text: "set shell := ['bash', '-cu']",
    tree: (justfile (set shell "bash" "-cu")),
  }

  test! {
    name: set_shell_with_one_argument_trailing_comma,
    text: "set shell := ['bash', '-cu',]",
    tree: (justfile (set shell "bash" "-cu")),
  }

  test! {
    name: set_shell_with_two_arguments,
    text: "set shell := ['bash', '-cu', '-l']",
    tree: (justfile (set shell "bash" "-cu" "-l")),
  }

  test! {
    name: set_windows_powershell_implicit,
    text: "set windows-powershell",
    tree: (justfile (set windows_powershell true)),
  }

  test! {
    name: set_windows_powershell_true,
    text: "set windows-powershell := true",
    tree: (justfile (set windows_powershell true)),
  }

  test! {
    name: set_windows_powershell_false,
    text: "set windows-powershell := false",
    tree: (justfile (set windows_powershell false)),
  }

  test! {
    name: set_working_directory,
    text: "set working-directory := 'foo'",
    tree: (justfile (set working_directory "foo")),
  }

  test! {
    name: conditional,
    text: "a := if b == c { d } else { e }",
    tree: (justfile (assignment a (if b == c d e))),
  }

  test! {
    name: conditional_inverted,
    text: "a := if b != c { d } else { e }",
    tree: (justfile (assignment a (if b != c d e))),
  }

  test! {
    name: conditional_concatenations,
    text: "a := if b0 + b1 == c0 + c1 { d0 + d1 } else { e0 + e1 }",
    tree: (justfile (assignment a (if (+ b0 b1) == (+ c0 c1) (+ d0 d1) (+ e0 e1)))),
  }

  test! {
    name: conditional_nested_lhs,
    text: "a := if if b == c { d } else { e } == c { d } else { e }",
    tree: (justfile (assignment a (if (if b == c d e) == c d e))),
  }

  test! {
    name: conditional_nested_rhs,
    text: "a := if c == if b == c { d } else { e } { d } else { e }",
    tree: (justfile (assignment a (if c == (if b == c d e) d e))),
  }

  test! {
    name: conditional_nested_then,
    text: "a := if b == c { if b == c { d } else { e } } else { e }",
    tree: (justfile (assignment a (if b == c (if b == c d e) e))),
  }

  test! {
    name: conditional_nested_otherwise,
    text: "a := if b == c { d } else { if b == c { d } else { e } }",
    tree: (justfile (assignment a (if b == c d (if b == c d e)))),
  }

  test! {
    name: import,
    text: "import \"some/file/path.txt\"     \n",
    tree: (justfile (import "some/file/path.txt")),
  }

  test! {
    name: optional_import,
    text: "import? \"some/file/path.txt\"     \n",
    tree: (justfile (import ? "some/file/path.txt")),
  }

  test! {
    name: module_with,
    text: "mod foo",
    tree: (justfile (mod foo )),
  }

  test! {
    name: optional_module,
    text: "mod? foo",
    tree: (justfile (mod ? foo)),
  }

  test! {
    name: module_with_path,
    text: "mod foo \"some/file/path.txt\"     \n",
    tree: (justfile (mod foo "some/file/path.txt")),
  }

  test! {
    name: optional_module_with_path,
    text: "mod? foo \"some/file/path.txt\"     \n",
    tree: (justfile (mod ? foo "some/file/path.txt")),
  }

  test! {
    name: assert,
    text: "a := assert(foo == \"bar\", \"error\")",
    tree: (justfile (assignment a (assert foo == "bar" "error"))),
  }

  test! {
    name: assert_conditional_condition,
    text: "foo := assert(if a != b { c } else { d } == \"abc\", \"error\")",
    tree: (justfile (assignment foo (assert (if a != b c d) == "abc" "error"))),
  }

  error! {
    name:   alias_syntax_multiple_rhs,
    input:  "alias foo := bar baz",
    offset: 17,
    line:   0,
    column: 17,
    width:  3,
    kind:   UnexpectedToken { expected: vec![Comment, Eof, Eol], found: Identifier },
  }

  error! {
    name:   alias_syntax_no_rhs,
    input:  "alias foo := \n",
    offset: 13,
    line:   0,
    column: 13,
    width:  1,
    kind:   UnexpectedToken {expected: vec![Identifier], found:Eol},
  }

  error! {
    name:   missing_colon,
    input:  "a b c\nd e f",
    offset:  5,
    line:   0,
    column: 5,
    width:  1,
    kind:   UnexpectedToken{
      expected: vec![Asterisk, Colon, Dollar, Equals, Identifier, Plus],
      found:    Eol
    },
  }

  error! {
    name:   missing_default_eol,
    input:  "hello arg=\n",
    offset:  10,
    line:   0,
    column: 10,
    width:  1,
    kind:   UnexpectedToken {
      expected: vec![
        Backtick,
        Identifier,
        ParenL,
        StringToken,
      ],
      found: Eol
    },
  }

  error! {
    name:   missing_default_eof,
    input:  "hello arg=",
    offset:  10,
    line:   0,
    column: 10,
    width:  0,
    kind:   UnexpectedToken {
      expected: vec![
        Backtick,
        Identifier,
        ParenL,
        StringToken,
      ],
      found: Eof,
    },
  }

  error! {
    name:   missing_eol,
    input:  "a b c: z =",
    offset:  9,
    line:    0,
    column:  9,
    width:   1,
    kind:    UnexpectedToken{
      expected: vec![AmpersandAmpersand, Comment, Eof, Eol, Identifier, ParenL],
      found: Equals
    },
  }

  error! {
    name:   unexpected_brace,
    input:  "{{",
    offset:  0,
    line:   0,
    column: 0,
    width:  1,
    kind: UnexpectedToken {
      expected: vec![At, BracketL, Comment, Eof, Eol, Identifier],
      found: BraceL,
    },
  }

  error! {
    name:   unclosed_parenthesis_in_expression,
    input:  "x := foo(",
    offset: 9,
    line:   0,
    column: 9,
    width:  0,
    kind: UnexpectedToken{
      expected: vec![
        Backtick,
        Identifier,
        ParenL,
        ParenR,
        Slash,
        StringToken,
      ],
      found: Eof,
    },
  }

  error! {
    name:   unclosed_parenthesis_in_interpolation,
    input:  "a:\n echo {{foo(}}",
    offset:  15,
    line:   1,
    column: 12,
    width:  2,
    kind:   UnexpectedToken{
      expected: vec![
        Backtick,
        Identifier,
        ParenL,
        ParenR,
        Slash,
        StringToken,
      ],
      found: InterpolationEnd,
    },
  }

  error! {
    name:   plus_following_parameter,
    input:  "a b c+:",
    offset: 6,
    line:   0,
    column: 6,
    width:  1,
    kind:   UnexpectedToken{expected: vec![Dollar, Identifier], found: Colon},
  }

  error! {
    name:   invalid_escape_sequence,
    input:  r#"foo := "\b""#,
    offset: 7,
    line:   0,
    column: 7,
    width:  4,
    kind:   InvalidEscapeSequence{character: 'b'},
  }

  error! {
    name:   bad_export,
    input:  "export a",
    offset:  8,
    line:   0,
    column: 8,
    width:  0,
    kind:   UnexpectedToken {
      expected: vec![Asterisk, Colon, Dollar, Equals, Identifier, Plus],
      found:    Eof
    },
  }

  error! {
    name:   parameter_follows_variadic_parameter,
    input:  "foo +a b:",
    offset: 7,
    line:   0,
    column: 7,
    width:  1,
    kind:   ParameterFollowsVariadicParameter{parameter: "b"},
  }

  error! {
    name:   parameter_after_variadic,
    input:  "foo +a bbb:",
    offset: 7,
    line:   0,
    column: 7,
    width:  3,
    kind:   ParameterFollowsVariadicParameter{parameter: "bbb"},
  }

  error! {
    name:   concatenation_in_default,
    input:  "foo a=c+d e:",
    offset: 10,
    line:   0,
    column: 10,
    width:  1,
    kind:   ParameterFollowsVariadicParameter{parameter: "e"},
  }

  error! {
    name:   set_shell_empty,
    input:  "set shell := []",
    offset: 14,
    line:   0,
    column: 14,
    width:  1,
    kind:   UnexpectedToken {
      expected: vec![
        Identifier,
        StringToken,
      ],
      found: BracketR,
    },
  }

  error! {
    name:   set_shell_non_literal_first,
    input:  "set shell := ['bar' + 'baz']",
    offset: 20,
    line:   0,
    column: 20,
    width:  1,
    kind:   UnexpectedToken {
      expected: vec![BracketR, Comma],
      found: Plus,
    },
  }

  error! {
    name:   set_shell_non_literal_second,
    input:  "set shell := ['biz', 'bar' + 'baz']",
    offset: 27,
    line:   0,
    column: 27,
    width:  1,
    kind:   UnexpectedToken {
      expected: vec![BracketR, Comma],
      found: Plus,
    },
  }

  error! {
    name:   set_shell_bad_comma,
    input:  "set shell := ['bash',",
    offset: 21,
    line:   0,
    column: 21,
    width:  0,
    kind:   UnexpectedToken {
      expected: vec![
        BracketR,
        Identifier,
        StringToken,
      ],
      found: Eof,
    },
  }

  error! {
    name:   set_shell_bad,
    input:  "set shell := ['bash'",
    offset: 20,
    line:   0,
    column: 20,
    width:  0,
    kind:   UnexpectedToken {
      expected: vec![BracketR, Comma],
      found: Eof,
    },
  }

  error! {
    name:   empty_attribute,
    input:  "[]\nsome_recipe:\n @exit 3",
    offset: 1,
    line:   0,
    column: 1,
    width:  1,
    kind:   UnexpectedToken {
      expected: vec![Identifier],
      found: BracketR,
    },
  }

  error! {
    name:   unknown_attribute,
    input:  "[unknown]\nsome_recipe:\n @exit 3",
    offset: 1,
    line:   0,
    column: 1,
    width:  7,
    kind:   UnknownAttribute { attribute: "unknown" },
  }

  error! {
    name:   set_unknown,
    input:  "set shall := []",
    offset: 4,
    line:   0,
    column: 4,
    width:  5,
    kind:   UnknownSetting {
      setting: "shall",
    },
  }

  error! {
    name:   set_shell_non_string,
    input:  "set shall := []",
    offset: 4,
    line:   0,
    column: 4,
    width:  5,
    kind:   UnknownSetting {
      setting: "shall",
    },
  }

  error! {
    name:   unknown_function,
    input:  "a := foo()",
    offset: 5,
    line:   0,
    column: 5,
    width:  3,
    kind:   UnknownFunction{function: "foo"},
  }

  error! {
    name:   unknown_function_in_interpolation,
    input:  "a:\n echo {{bar()}}",
    offset: 11,
    line:   1,
    column: 8,
    width:  3,
    kind:   UnknownFunction{function: "bar"},
  }

  error! {
    name:   unknown_function_in_default,
    input:  "a f=baz():",
    offset: 4,
    line:   0,
    column: 4,
    width:  3,
    kind:   UnknownFunction{function: "baz"},
  }

  error! {
    name: function_argument_count_nullary,
    input: "x := arch('foo')",
    offset: 5,
    line: 0,
    column: 5,
    width: 4,
    kind: FunctionArgumentCountMismatch {
      function: "arch",
      found: 1,
      expected: 0..=0,
    },
  }

  error! {
    name: function_argument_count_unary,
    input: "x := env_var()",
    offset: 5,
    line: 0,
    column: 5,
    width: 7,
    kind: FunctionArgumentCountMismatch {
      function: "env_var",
      found: 0,
      expected: 1..=1,
    },
  }

  error! {
    name: function_argument_count_too_high_unary_opt,
    input: "x := env('foo', 'foo', 'foo')",
    offset: 5,
    line: 0,
    column: 5,
    width: 3,
    kind: FunctionArgumentCountMismatch {
      function: "env",
      found: 3,
      expected: 1..=2,
    },
  }

  error! {
    name: function_argument_count_too_low_unary_opt,
    input: "x := env()",
    offset: 5,
    line: 0,
    column: 5,
    width: 3,
    kind: FunctionArgumentCountMismatch {
      function: "env",
      found: 0,
      expected: 1..=2,
    },
  }

  error! {
    name: function_argument_count_binary,
    input: "x := env_var_or_default('foo')",
    offset: 5,
    line: 0,
    column: 5,
    width: 18,
    kind: FunctionArgumentCountMismatch {
      function: "env_var_or_default",
      found: 1,
      expected: 2..=2,
    },
  }

  error! {
    name: function_argument_count_binary_plus,
    input: "x := join('foo')",
    offset: 5,
    line: 0,
    column: 5,
    width: 4,
    kind: FunctionArgumentCountMismatch {
      function: "join",
      found: 1,
      expected: 2..=usize::MAX,
    },
  }

  error! {
    name: function_argument_count_ternary,
    input: "x := replace('foo')",
    offset: 5,
    line: 0,
    column: 5,
    width: 7,
    kind: FunctionArgumentCountMismatch {
      function: "replace",
      found: 1,
      expected: 3..=3,
    },
  }
}
