#![deny(clippy::all, clippy::pedantic)]
#![allow(
  clippy::enum_glob_use,
  clippy::let_underscore_untyped,
  clippy::needless_pass_by_value,
  clippy::too_many_lines,
  clippy::unnecessary_wraps,
  clippy::wildcard_imports
)]

pub(crate) use {
  crate::{
    alias::Alias, analyzer::Analyzer, assignment::Assignment,
    assignment_resolver::AssignmentResolver, ast::Ast, attribute::Attribute, binding::Binding,
    color::Color, color_display::ColorDisplay, command_ext::CommandExt, compilation::Compilation,
    compile_error::CompileError, compile_error_kind::CompileErrorKind, compiler::Compiler,
    conditional_operator::ConditionalOperator, config::Config, config_error::ConfigError,
    count::Count, delimiter::Delimiter, dependency::Dependency, dump_format::DumpFormat,
    enclosure::Enclosure, error::Error, evaluator::Evaluator, expression::Expression,
    fragment::Fragment, function::Function, function_context::FunctionContext,
    interrupt_guard::InterruptGuard, interrupt_handler::InterruptHandler, item::Item,
    justfile::Justfile, keyed::Keyed, keyword::Keyword, lexer::Lexer, line::Line, list::List,
    load_dotenv::load_dotenv, loader::Loader, name::Name, ordinal::Ordinal, output::output,
    output_error::OutputError, parameter::Parameter, parameter_kind::ParameterKind, parser::Parser,
    platform::Platform, platform_interface::PlatformInterface, position::Position,
    positional::Positional, range_ext::RangeExt, recipe::Recipe, recipe_context::RecipeContext,
    recipe_resolver::RecipeResolver, scope::Scope, search::Search, search_config::SearchConfig,
    search_error::SearchError, set::Set, setting::Setting, settings::Settings, shebang::Shebang,
    shell::Shell, show_whitespace::ShowWhitespace, string_kind::StringKind,
    string_literal::StringLiteral, subcommand::Subcommand, suggestion::Suggestion, table::Table,
    thunk::Thunk, token::Token, token_kind::TokenKind, unresolved_dependency::UnresolvedDependency,
    unresolved_recipe::UnresolvedRecipe, use_color::UseColor, variables::Variables,
    verbosity::Verbosity, warning::Warning,
  },
  std::{
    cmp,
    collections::{BTreeMap, BTreeSet, HashMap},
    env,
    ffi::{OsStr, OsString},
    fmt::{self, Debug, Display, Formatter},
    fs,
    io::{self, Cursor, Write},
    iter::{self, FromIterator},
    mem,
    ops::{Index, Range, RangeInclusive},
    path::{self, Path, PathBuf},
    process::{self, Command, ExitStatus, Stdio},
    str::{self, Chars},
    sync::{Mutex, MutexGuard},
    vec,
  },
  {
    camino::Utf8Path,
    derivative::Derivative,
    edit_distance::edit_distance,
    lexiclean::Lexiclean,
    libc::EXIT_FAILURE,
    log::{info, warn},
    regex::Regex,
    serde::{
      ser::{SerializeMap, SerializeSeq},
      Serialize, Serializer,
    },
    snafu::{ResultExt, Snafu},
    strum::{Display, EnumString, IntoStaticStr},
    typed_arena::Arena,
    unicode_width::{UnicodeWidthChar, UnicodeWidthStr},
  },
};

#[cfg(test)]
pub(crate) use crate::{node::Node, tree::Tree};

pub use crate::run::run;

// Used in integration tests.
#[doc(hidden)]
pub use unindent::unindent;

pub(crate) type CompileResult<'a, T> = Result<T, CompileError<'a>>;
pub(crate) type ConfigResult<T> = Result<T, ConfigError>;
pub(crate) type RunResult<'a, T> = Result<T, Error<'a>>;
pub(crate) type SearchResult<T> = Result<T, SearchError>;

#[cfg(test)]
#[macro_use]
pub mod testing;

#[cfg(test)]
#[macro_use]
pub mod tree;

#[cfg(test)]
pub mod node;

#[cfg(fuzzing)]
pub mod fuzzing;

// Used by Janus, https://github.com/casey/janus, a tool
// that analyses all public justfiles on GitHub to avoid
// breaking changes.
#[doc(hidden)]
pub mod summary;

mod alias;
mod analyzer;
mod assignment;
mod assignment_resolver;
mod ast;
mod attribute;
mod binding;
mod color;
mod color_display;
mod command_ext;
mod compilation;
mod compile_error;
mod compile_error_kind;
mod compiler;
mod completions;
mod conditional_operator;
mod config;
mod config_error;
mod count;
mod delimiter;
mod dependency;
mod dump_format;
mod enclosure;
mod error;
mod evaluator;
mod expression;
mod fragment;
mod function;
mod function_context;
mod interrupt_guard;
mod interrupt_handler;
mod item;
mod justfile;
mod keyed;
mod keyword;
mod lexer;
mod line;
mod list;
mod load_dotenv;
mod loader;
mod name;
mod ordinal;
mod output;
mod output_error;
mod parallel;
mod parameter;
mod parameter_kind;
mod parser;
mod platform;
mod platform_interface;
mod position;
mod positional;
mod range_ext;
mod recipe;
mod recipe_context;
mod recipe_resolver;
mod run;
mod scope;
mod search;
mod search_config;
mod search_error;
mod set;
mod setting;
mod settings;
mod shebang;
mod shell;
mod show_whitespace;
mod string_kind;
mod string_literal;
mod subcommand;
mod suggestion;
mod table;
mod thunk;
mod token;
mod token_kind;
mod unindent;
mod unresolved_dependency;
mod unresolved_recipe;
mod use_color;
mod variables;
mod verbosity;
mod warning;
