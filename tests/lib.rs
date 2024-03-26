pub(crate) use {
  crate::{
    assert_stdout::assert_stdout,
    assert_success::assert_success,
    tempdir::tempdir,
    test::{Output, Test},
  },
  cradle::input::Input,
  executable_path::executable_path,
  just::unindent,
  libc::{EXIT_FAILURE, EXIT_SUCCESS},
  pretty_assertions::Comparison,
  regex::Regex,
  serde_json::{json, Value},
  std::{
    collections::BTreeMap,
    env::{self, consts::EXE_SUFFIX},
    error::Error,
    fmt::Debug,
    fs,
    io::Write,
    iter,
    path::{Path, PathBuf, MAIN_SEPARATOR},
    process::{Command, Stdio},
    str,
  },
  tempfile::TempDir,
  temptree::{temptree, tree, Tree},
  which::which,
};

#[macro_use]
mod test;

mod allow_duplicate_recipes;
mod assert_stdout;
mod assert_success;
mod attributes;
mod byte_order_mark;
mod cached_recipes;
mod changelog;
mod choose;
mod command;
mod completions;
mod conditional;
mod confirm;
mod delimiters;
mod directories;
mod dotenv;
mod edit;
mod equals;
mod error_messages;
mod evaluate;
mod examples;
mod export;
mod fallback;
mod fmt;
mod functions;
mod ignore_comments;
mod imports;
mod init;
#[cfg(unix)]
mod interrupts;
mod invocation_directory;
mod json;
mod line_prefixes;
mod misc;
mod modules;
mod multibyte_char;
mod newline_escape;
mod no_aliases;
mod no_cd;
mod no_dependencies;
mod no_exit_message;
mod os_attributes;
mod parser;
mod positional_arguments;
mod private;
mod quiet;
mod quote;
mod readme;
mod recursion_limit;
mod regexes;
mod run;
mod search;
mod search_arguments;
mod shadowing_parameters;
mod shebang;
mod shell;
mod show;
mod slash_operator;
mod string;
mod subsequents;
mod summary;
mod tempdir;
mod undefined_variables;
mod unstable;
#[cfg(target_family = "windows")]
mod windows_shell;
mod working_directory;

fn path(s: &str) -> String {
  if cfg!(windows) {
    s.replace('/', "\\")
  } else {
    s.into()
  }
}

fn path_for_regex(s: &str) -> String {
  if cfg!(windows) {
    s.replace('/', "\\\\")
  } else {
    s.into()
  }
}
