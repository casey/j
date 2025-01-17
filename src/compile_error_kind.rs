use super::*;

#[derive(Debug, PartialEq)]
pub(crate) enum CompileErrorKind<'src> {
  AttributeArgumentCountMismatch {
    attribute: &'src str,
    found: usize,
    min: usize,
    max: usize,
  },
  BacktickShebang,
  CircularRecipeDependency {
    recipe: &'src str,
    circle: Vec<&'src str>,
  },
  CircularVariableDependency {
    variable: &'src str,
    circle: Vec<&'src str>,
  },
  DependencyArgumentCountMismatch {
    dependency: &'src str,
    found: usize,
    min: usize,
    max: usize,
  },
  Redefinition {
    first: usize,
    first_type: &'static str,
    name: &'src str,
    second_type: &'static str,
  },
  DuplicateAttribute {
    attribute: &'src str,
    first: usize,
  },
  DuplicateParameter {
    recipe: &'src str,
    parameter: &'src str,
  },
  DuplicateSet {
    setting: &'src str,
    first: usize,
  },
  DuplicateVariable {
    variable: &'src str,
  },
  DuplicateUnexport {
    variable: &'src str,
  },
  ExitMessageAndNoExitMessageAttribute {
    recipe: &'src str,
  },
  ExpectedKeyword {
    expected: Vec<Keyword>,
    found: Token<'src>,
  },
  ExportUnexported {
    variable: &'src str,
  },
  ExtraLeadingWhitespace,
  ExtraneousAttributes {
    count: usize,
  },
  FunctionArgumentCountMismatch {
    function: &'src str,
    found: usize,
    expected: RangeInclusive<usize>,
  },
  Include,
  InconsistentLeadingWhitespace {
    expected: &'src str,
    found: &'src str,
  },
  Internal {
    message: String,
  },
  InvalidAttribute {
    item_kind: &'static str,
    item_name: &'src str,
    attribute: Attribute<'src>,
  },
  InvalidEscapeSequence {
    character: char,
  },
  MismatchedClosingDelimiter {
    close: Delimiter,
    open: Delimiter,
    open_line: usize,
  },
  MixedLeadingWhitespace {
    whitespace: &'src str,
  },
  NoCdAndWorkingDirectoryAttribute {
    recipe: &'src str,
  },
  ParameterFollowsVariadicParameter {
    parameter: &'src str,
  },
  ParsingRecursionDepthExceeded,
  RequiredParameterFollowsDefaultParameter {
    parameter: &'src str,
  },
  ShebangAndScriptAttribute {
    recipe: &'src str,
  },
  ShellExpansion {
    err: shellexpand::LookupError<env::VarError>,
  },
  UndefinedVariable {
    variable: &'src str,
  },
  UnexpectedCharacter {
    expected: Vec<char>,
  },
  UnexpectedClosingDelimiter {
    close: Delimiter,
  },
  UnexpectedEndOfToken {
    expected: Vec<char>,
  },
  UnexpectedToken {
    expected: Vec<TokenKind>,
    found: TokenKind,
  },
  UnicodeEscapeCharacter {
    character: char,
  },
  UnicodeEscapeDelimiter {
    character: char,
  },
  UnicodeEscapeEmpty,
  UnicodeEscapeLength {
    hex: String,
  },
  UnicodeEscapeRange {
    hex: String,
  },
  UnicodeEscapeUnterminated,
  UnknownAliasTarget {
    alias: &'src str,
    target: &'src str,
  },
  UnknownAttribute {
    attribute: &'src str,
  },
  UnknownDependency {
    recipe: &'src str,
    unknown: &'src str,
  },
  UnknownFunction {
    function: &'src str,
  },
  UnknownSetting {
    setting: &'src str,
  },
  UnknownStartOfToken,
  UnpairedCarriageReturn,
  UnterminatedBacktick,
  UnterminatedInterpolation,
  UnterminatedString,
}
