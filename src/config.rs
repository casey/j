use {
  super::*,
  clap::{
    builder::{
      styling::{AnsiColor, Effects},
      FalseyValueParser, Styles,
    },
    parser::ValuesRef,
    value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command,
  },
};

#[derive(Debug, PartialEq)]
pub(crate) struct Config {
  pub(crate) alias_style: AliasStyle,
  pub(crate) allow_missing: bool,
  pub(crate) check: bool,
  pub(crate) color: Color,
  pub(crate) command_color: Option<ansi_term::Color>,
  pub(crate) dotenv_filename: Option<String>,
  pub(crate) dotenv_path: Option<PathBuf>,
  pub(crate) dry_run: bool,
  pub(crate) dump_format: DumpFormat,
  pub(crate) explain: bool,
  pub(crate) highlight: bool,
  pub(crate) invocation_directory: PathBuf,
  pub(crate) list_heading: String,
  pub(crate) list_prefix: String,
  pub(crate) list_submodules: bool,
  pub(crate) load_dotenv: bool,
  pub(crate) no_aliases: bool,
  pub(crate) no_dependencies: bool,
  pub(crate) one: bool,
  pub(crate) search_config: SearchConfig,
  pub(crate) shell: Option<String>,
  pub(crate) shell_args: Option<Vec<String>>,
  pub(crate) shell_command: bool,
  pub(crate) subcommand: Subcommand,
  pub(crate) timestamp: bool,
  pub(crate) timestamp_format: String,
  pub(crate) unsorted: bool,
  pub(crate) unstable: bool,
  pub(crate) verbosity: Verbosity,
  pub(crate) yes: bool,
}

mod cmd {
  pub(crate) const CHANGELOG: &str = "CHANGELOG";
  pub(crate) const CHOOSE: &str = "CHOOSE";
  pub(crate) const COMMAND: &str = "COMMAND";
  pub(crate) const COMPLETIONS: &str = "COMPLETIONS";
  pub(crate) const DUMP: &str = "DUMP";
  pub(crate) const EDIT: &str = "EDIT";
  pub(crate) const EVALUATE: &str = "EVALUATE";
  pub(crate) const FORMAT: &str = "FORMAT";
  pub(crate) const GROUPS: &str = "GROUPS";
  pub(crate) const INIT: &str = "INIT";
  pub(crate) const LIST: &str = "LIST";
  pub(crate) const MAN: &str = "MAN";
  pub(crate) const REQUEST: &str = "REQUEST";
  pub(crate) const SHOW: &str = "SHOW";
  pub(crate) const SUMMARY: &str = "SUMMARY";
  pub(crate) const VARIABLES: &str = "VARIABLES";

  pub(crate) const ALL: &[&str] = &[
    CHANGELOG,
    CHOOSE,
    COMMAND,
    COMPLETIONS,
    DUMP,
    EDIT,
    EVALUATE,
    FORMAT,
    INIT,
    LIST,
    MAN,
    REQUEST,
    SHOW,
    SUMMARY,
    VARIABLES,
  ];

  pub(crate) const ARGLESS: &[&str] =
    &[CHANGELOG, DUMP, EDIT, FORMAT, INIT, MAN, SUMMARY, VARIABLES];

  pub(crate) const HEADING: &str = "Commands";
}

mod arg {
  pub(crate) const ALIAS_STYLE: &str = "ALIAS_STYLE";
  pub(crate) const ALLOW_MISSING: &str = "ALLOW-MISSING";
  pub(crate) const ARGUMENTS: &str = "ARGUMENTS";
  pub(crate) const CHECK: &str = "CHECK";
  pub(crate) const CHOOSER: &str = "CHOOSER";
  pub(crate) const CLEAR_SHELL_ARGS: &str = "CLEAR-SHELL-ARGS";
  pub(crate) const COLOR: &str = "COLOR";
  pub(crate) const COMMAND_COLOR: &str = "COMMAND-COLOR";
  pub(crate) const DOTENV_FILENAME: &str = "DOTENV-FILENAME";
  pub(crate) const DOTENV_PATH: &str = "DOTENV-PATH";
  pub(crate) const DRY_RUN: &str = "DRY-RUN";
  pub(crate) const DUMP_FORMAT: &str = "DUMP-FORMAT";
  pub(crate) const EXPLAIN: &str = "EXPLAIN";
  pub(crate) const GLOBAL_JUSTFILE: &str = "GLOBAL-JUSTFILE";
  pub(crate) const HIGHLIGHT: &str = "HIGHLIGHT";
  pub(crate) const JUSTFILE: &str = "JUSTFILE";
  pub(crate) const LIST_HEADING: &str = "LIST-HEADING";
  pub(crate) const LIST_PREFIX: &str = "LIST-PREFIX";
  pub(crate) const LIST_SUBMODULES: &str = "LIST-SUBMODULES";
  pub(crate) const NO_ALIASES: &str = "NO-ALIASES";
  pub(crate) const NO_DEPS: &str = "NO-DEPS";
  pub(crate) const NO_DOTENV: &str = "NO-DOTENV";
  pub(crate) const NO_HIGHLIGHT: &str = "NO-HIGHLIGHT";
  pub(crate) const ONE: &str = "ONE";
  pub(crate) const QUIET: &str = "QUIET";
  pub(crate) const SET: &str = "SET";
  pub(crate) const SHELL: &str = "SHELL";
  pub(crate) const SHELL_ARG: &str = "SHELL-ARG";
  pub(crate) const SHELL_COMMAND: &str = "SHELL-COMMAND";
  pub(crate) const TIMESTAMP: &str = "TIMESTAMP";
  pub(crate) const TIMESTAMP_FORMAT: &str = "TIMESTAMP-FORMAT";
  pub(crate) const UNSORTED: &str = "UNSORTED";
  pub(crate) const UNSTABLE: &str = "UNSTABLE";
  pub(crate) const VERBOSE: &str = "VERBOSE";
  pub(crate) const WORKING_DIRECTORY: &str = "WORKING-DIRECTORY";
  pub(crate) const YES: &str = "YES";
}

impl Config {
  pub(crate) fn app() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
      .bin_name(env!("CARGO_PKG_NAME"))
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about(concat!(
        env!("CARGO_PKG_DESCRIPTION"),
        " - ",
        env!("CARGO_PKG_HOMEPAGE")
      ))
      .trailing_var_arg(true)
      .styles(
        Styles::styled()
          .error(AnsiColor::Red.on_default() | Effects::BOLD)
          .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
          .invalid(AnsiColor::Red.on_default())
          .literal(AnsiColor::Green.on_default())
          .placeholder(AnsiColor::Cyan.on_default())
          .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
          .valid(AnsiColor::Green.on_default()),
      )
      .arg(
        Arg::new(arg::ALIAS_STYLE)
          .long("alias-style")
          .env("JUST_ALIAS_STYLE")
          .action(ArgAction::Set)
          .value_parser(clap::value_parser!(AliasStyle))
          .default_value("right")
          .help("Set list command alias display style")
          .conflicts_with(arg::NO_ALIASES),
      )
      .arg(
        Arg::new(arg::CHECK)
          .long("check")
          .action(ArgAction::SetTrue)
          .requires(cmd::FORMAT)
          .help(
            "Run `--fmt` in 'check' mode. Exits with 0 if justfile is formatted correctly. \
                 Exits with 1 and prints a diff if formatting is required.",
          ),
      )
      .arg(
        Arg::new(arg::CHOOSER)
          .long("chooser")
          .env("JUST_CHOOSER")
          .action(ArgAction::Set)
          .help("Override binary invoked by `--choose`"),
      )
      .arg(
        Arg::new(arg::CLEAR_SHELL_ARGS)
          .long("clear-shell-args")
          .action(ArgAction::SetTrue)
          .overrides_with(arg::SHELL_ARG)
          .help("Clear shell arguments"),
      )
      .arg(
        Arg::new(arg::COLOR)
          .long("color")
          .env("JUST_COLOR")
          .action(ArgAction::Set)
          .value_parser(clap::value_parser!(UseColor))
          .default_value("auto")
          .help("Print colorful output"),
      )
      .arg(
        Arg::new(arg::COMMAND_COLOR)
          .long("command-color")
          .env("JUST_COMMAND_COLOR")
          .action(ArgAction::Set)
          .value_parser(clap::value_parser!(CommandColor))
          .help("Echo recipe lines in <COMMAND-COLOR>"),
      )
      .arg(
        Arg::new(arg::DOTENV_FILENAME)
          .long("dotenv-filename")
          .action(ArgAction::Set)
          .help("Search for environment file named <DOTENV-FILENAME> instead of `.env`")
          .conflicts_with(arg::DOTENV_PATH),
      )
      .arg(
        Arg::new(arg::DOTENV_PATH)
          .short('E')
          .long("dotenv-path")
          .action(ArgAction::Set)
          .value_parser(value_parser!(PathBuf))
          .help("Load <DOTENV-PATH> as environment file instead of searching for one"),
      )
      .arg(
        Arg::new(arg::DRY_RUN)
          .short('n')
          .long("dry-run")
          .env("JUST_DRY_RUN")
          .action(ArgAction::SetTrue)
          .help("Print what just would do without doing it")
          .conflicts_with(arg::QUIET),
      )
      .arg(
        Arg::new(arg::DUMP_FORMAT)
          .long("dump-format")
          .env("JUST_DUMP_FORMAT")
          .action(ArgAction::Set)
          .value_parser(clap::value_parser!(DumpFormat))
          .default_value("just")
          .value_name("FORMAT")
          .help("Dump justfile as <FORMAT>"),
      )
      .arg(
        Arg::new(arg::EXPLAIN)
          .action(ArgAction::SetTrue)
          .long("explain")
          .env("JUST_EXPLAIN")
          .help("Print recipe doc comment before running it"),
      )
      .arg(
        Arg::new(arg::GLOBAL_JUSTFILE)
          .action(ArgAction::SetTrue)
          .long("global-justfile")
          .short('g')
          .conflicts_with(arg::JUSTFILE)
          .conflicts_with(arg::WORKING_DIRECTORY)
          .help("Use global justfile"),
      )
      .arg(
        Arg::new(arg::HIGHLIGHT)
          .long("highlight")
          .env("JUST_HIGHLIGHT")
          .action(ArgAction::SetTrue)
          .help("Highlight echoed recipe lines in bold")
          .overrides_with(arg::NO_HIGHLIGHT),
      )
      .arg(
        Arg::new(arg::JUSTFILE)
          .short('f')
          .long("justfile")
          .env("JUST_JUSTFILE")
          .action(ArgAction::Set)
          .value_parser(value_parser!(PathBuf))
          .help("Use <JUSTFILE> as justfile"),
      )
      .arg(
        Arg::new(arg::LIST_HEADING)
          .long("list-heading")
          .env("JUST_LIST_HEADING")
          .help("Print <TEXT> before list")
          .value_name("TEXT")
          .default_value("Available recipes:\n")
          .action(ArgAction::Set),
      )
      .arg(
        Arg::new(arg::LIST_PREFIX)
          .long("list-prefix")
          .env("JUST_LIST_PREFIX")
          .help("Print <TEXT> before each list item")
          .value_name("TEXT")
          .default_value("    ")
          .action(ArgAction::Set),
      )
      .arg(
        Arg::new(arg::LIST_SUBMODULES)
          .long("list-submodules")
          .env("JUST_LIST_SUBMODULES")
          .help("List recipes in submodules")
          .action(ArgAction::SetTrue)
          .env("JUST_LIST_SUBMODULES"),
      )
      .arg(
        Arg::new(arg::NO_ALIASES)
          .long("no-aliases")
          .env("JUST_NO_ALIASES")
          .action(ArgAction::SetTrue)
          .help("Don't show aliases in list"),
      )
      .arg(
        Arg::new(arg::NO_DEPS)
          .long("no-deps")
          .env("JUST_NO_DEPS")
          .alias("no-dependencies")
          .action(ArgAction::SetTrue)
          .help("Don't run recipe dependencies"),
      )
      .arg(
        Arg::new(arg::NO_DOTENV)
          .long("no-dotenv")
          .env("JUST_NO_DOTENV")
          .action(ArgAction::SetTrue)
          .help("Don't load `.env` file"),
      )
      .arg(
        Arg::new(arg::NO_HIGHLIGHT)
          .long("no-highlight")
          .env("JUST_NO_HIGHLIGHT")
          .action(ArgAction::SetTrue)
          .help("Don't highlight echoed recipe lines in bold")
          .overrides_with(arg::HIGHLIGHT),
      )
      .arg(
        Arg::new(arg::ONE)
          .long("one")
          .env("JUST_ONE")
          .action(ArgAction::SetTrue)
          .help("Forbid multiple recipes from being invoked on the command line"),
      )
      .arg(
        Arg::new(arg::QUIET)
          .short('q')
          .long("quiet")
          .env("JUST_QUIET")
          .action(ArgAction::SetTrue)
          .help("Suppress all output")
          .conflicts_with(arg::DRY_RUN),
      )
      .arg(
        Arg::new(arg::ALLOW_MISSING)
          .long("allow-missing")
          .env("JUST_ALLOW_MISSING")
          .action(ArgAction::SetTrue)
          .help("Ignore missing recipe and module errors"),
      )
      .arg(
        Arg::new(arg::SET)
          .long("set")
          .action(ArgAction::Append)
          .number_of_values(2)
          .value_names(["VARIABLE", "VALUE"])
          .help("Override <VARIABLE> with <VALUE>"),
      )
      .arg(
        Arg::new(arg::SHELL)
          .long("shell")
          .action(ArgAction::Set)
          .help("Invoke <SHELL> to run recipes"),
      )
      .arg(
        Arg::new(arg::SHELL_ARG)
          .long("shell-arg")
          .action(ArgAction::Append)
          .allow_hyphen_values(true)
          .overrides_with(arg::CLEAR_SHELL_ARGS)
          .help("Invoke shell with <SHELL-ARG> as an argument"),
      )
      .arg(
        Arg::new(arg::SHELL_COMMAND)
          .long("shell-command")
          .requires(cmd::COMMAND)
          .action(ArgAction::SetTrue)
          .help("Invoke <COMMAND> with the shell used to run recipe lines and backticks"),
      )
      .arg(
        Arg::new(arg::TIMESTAMP)
          .action(ArgAction::SetTrue)
          .long("timestamp")
          .env("JUST_TIMESTAMP")
          .help("Print recipe command timestamps"),
      )
      .arg(
        Arg::new(arg::TIMESTAMP_FORMAT)
          .action(ArgAction::Set)
          .long("timestamp-format")
          .env("JUST_TIMESTAMP_FORMAT")
          .default_value("%H:%M:%S")
          .help("Timestamp format string"),
      )
      .arg(
        Arg::new(arg::UNSORTED)
          .long("unsorted")
          .env("JUST_UNSORTED")
          .short('u')
          .action(ArgAction::SetTrue)
          .help("Return list and summary entries in source order"),
      )
      .arg(
        Arg::new(arg::UNSTABLE)
          .long("unstable")
          .env("JUST_UNSTABLE")
          .action(ArgAction::SetTrue)
          .value_parser(FalseyValueParser::new())
          .help("Enable unstable features"),
      )
      .arg(
        Arg::new(arg::VERBOSE)
          .short('v')
          .long("verbose")
          .env("JUST_VERBOSE")
          .action(ArgAction::Count)
          .help("Use verbose output"),
      )
      .arg(
        Arg::new(arg::WORKING_DIRECTORY)
          .short('d')
          .long("working-directory")
          .env("JUST_WORKING_DIRECTORY")
          .action(ArgAction::Set)
          .value_parser(value_parser!(PathBuf))
          .help("Use <WORKING-DIRECTORY> as working directory. --justfile must also be set")
          .requires(arg::JUSTFILE),
      )
      .arg(
        Arg::new(arg::YES)
          .long("yes")
          .env("JUST_YES")
          .action(ArgAction::SetTrue)
          .help("Automatically confirm all recipes."),
      )
      .arg(
        Arg::new(cmd::CHANGELOG)
          .long("changelog")
          .action(ArgAction::SetTrue)
          .help("Print changelog")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::CHOOSE)
          .long("choose")
          .action(ArgAction::SetTrue)
          .help(
            "Select one or more recipes to run using a binary chooser. If `--chooser` is not \
             passed the chooser defaults to the value of $JUST_CHOOSER, falling back to `fzf`",
          )
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::COMMAND)
          .long("command")
          .short('c')
          .num_args(1..)
          .allow_hyphen_values(true)
          .action(ArgAction::Append)
          .value_parser(value_parser!(std::ffi::OsString))
          .help(
            "Run an arbitrary command with the working directory, `.env`, overrides, and exports \
             set",
          )
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::COMPLETIONS)
          .long("completions")
          .action(ArgAction::Set)
          .value_name("SHELL")
          .value_parser(value_parser!(completions::Shell))
          .ignore_case(true)
          .help("Print shell completion script for <SHELL>")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::DUMP)
          .long("dump")
          .action(ArgAction::SetTrue)
          .help("Print justfile")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::EDIT)
          .short('e')
          .long("edit")
          .action(ArgAction::SetTrue)
          .help("Edit justfile with editor given by $VISUAL or $EDITOR, falling back to `vim`")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::EVALUATE)
          .long("evaluate")
          .alias("eval")
          .action(ArgAction::SetTrue)
          .help(
            "Evaluate and print all variables. If a variable name is given as an argument, only \
             print that variable's value.",
          )
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::FORMAT)
          .long("fmt")
          .alias("format")
          .action(ArgAction::SetTrue)
          .help("Format and overwrite justfile")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::GROUPS)
          .long("groups")
          .action(ArgAction::SetTrue)
          .help("List recipe groups")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::INIT)
          .long("init")
          .alias("initialize")
          .action(ArgAction::SetTrue)
          .help("Initialize new justfile in project root")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::LIST)
          .short('l')
          .long("list")
          .num_args(0..)
          .value_name("PATH")
          .action(ArgAction::Set)
          .conflicts_with(arg::ARGUMENTS)
          .help("List available recipes")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::MAN)
          .long("man")
          .action(ArgAction::SetTrue)
          .help("Print man page")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::REQUEST)
          .long("request")
          .action(ArgAction::Set)
          .hide(true)
          .help(
            "Execute <REQUEST>. For internal testing purposes only. May be changed or removed at \
            any time.",
          )
          .help_heading(cmd::REQUEST),
      )
      .arg(
        Arg::new(cmd::SHOW)
          .short('s')
          .long("show")
          .num_args(1..)
          .action(ArgAction::Set)
          .value_name("PATH")
          .conflicts_with(arg::ARGUMENTS)
          .help("Show recipe at <PATH>")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::SUMMARY)
          .long("summary")
          .action(ArgAction::SetTrue)
          .help("List names of available recipes")
          .help_heading(cmd::HEADING),
      )
      .arg(
        Arg::new(cmd::VARIABLES)
          .long("variables")
          .action(ArgAction::SetTrue)
          .help("List names of variables")
          .help_heading(cmd::HEADING),
      )
      .group(ArgGroup::new("SUBCOMMAND").args(cmd::ALL))
      .arg(
        Arg::new(arg::ARGUMENTS)
          .num_args(1..)
          .action(ArgAction::Append)
          .help("Overrides and recipe(s) to run, defaulting to the first recipe in the justfile"),
      )
  }

  fn parse_module_path(values: ValuesRef<String>) -> ConfigResult<ModulePath> {
    let path = values.clone().map(|s| (*s).as_str()).collect::<Vec<&str>>();

    let path = if path.len() == 1 && path[0].contains(' ') {
      path[0].split_whitespace().collect::<Vec<&str>>()
    } else {
      path
    };

    path
      .as_slice()
      .try_into()
      .map_err(|()| ConfigError::ModulePath {
        path: values.cloned().collect(),
      })
  }

  fn search_config(matches: &ArgMatches, positional: &Positional) -> ConfigResult<SearchConfig> {
    if matches.get_flag(arg::GLOBAL_JUSTFILE) {
      return Ok(SearchConfig::GlobalJustfile);
    }

    let justfile = matches.get_one::<PathBuf>(arg::JUSTFILE).map(Into::into);

    let working_directory = matches
      .get_one::<PathBuf>(arg::WORKING_DIRECTORY)
      .map(Into::into);

    if let Some(search_directory) = positional.search_directory.as_ref().map(PathBuf::from) {
      if justfile.is_some() || working_directory.is_some() {
        return Err(ConfigError::SearchDirConflict);
      }
      Ok(SearchConfig::FromSearchDirectory { search_directory })
    } else {
      match (justfile, working_directory) {
        (None, None) => Ok(SearchConfig::FromInvocationDirectory),
        (Some(justfile), None) => Ok(SearchConfig::WithJustfile { justfile }),
        (Some(justfile), Some(working_directory)) => {
          Ok(SearchConfig::WithJustfileAndWorkingDirectory {
            justfile,
            working_directory,
          })
        }
        (None, Some(_)) => Err(ConfigError::internal(
          "--working-directory set without --justfile",
        )),
      }
    }
  }

  pub(crate) fn from_matches(matches: &ArgMatches) -> ConfigResult<Self> {
    let mut overrides = BTreeMap::new();
    if let Some(mut values) = matches.get_many::<String>(arg::SET) {
      while let (Some(k), Some(v)) = (values.next(), values.next()) {
        overrides.insert(k.into(), v.into());
      }
    }

    let positional = Positional::from_values(
      matches
        .get_many::<String>(arg::ARGUMENTS)
        .map(|s| s.map(String::as_str)),
    );

    for (name, value) in &positional.overrides {
      overrides.insert(name.clone(), value.clone());
    }

    let search_config = Self::search_config(matches, &positional)?;

    for subcommand in cmd::ARGLESS {
      if matches.get_flag(subcommand) {
        match (!overrides.is_empty(), !positional.arguments.is_empty()) {
          (false, false) => {}
          (true, false) => {
            return Err(ConfigError::SubcommandOverrides {
              subcommand,
              overrides,
            });
          }
          (false, true) => {
            return Err(ConfigError::SubcommandArguments {
              arguments: positional.arguments,
              subcommand,
            });
          }
          (true, true) => {
            return Err(ConfigError::SubcommandOverridesAndArguments {
              arguments: positional.arguments,
              subcommand,
              overrides,
            });
          }
        }
      }
    }

    let subcommand = if matches.get_flag(cmd::CHANGELOG) {
      Subcommand::Changelog
    } else if matches.get_flag(cmd::CHOOSE) {
      Subcommand::Choose {
        chooser: matches.get_one::<String>(arg::CHOOSER).map(Into::into),
        overrides,
      }
    } else if let Some(values) = matches.get_many::<OsString>(cmd::COMMAND) {
      let mut arguments = values.map(Into::into).collect::<Vec<OsString>>();
      Subcommand::Command {
        binary: arguments.remove(0),
        arguments,
        overrides,
      }
    } else if let Some(&shell) = matches.get_one::<completions::Shell>(cmd::COMPLETIONS) {
      Subcommand::Completions { shell }
    } else if matches.get_flag(cmd::DUMP) {
      Subcommand::Dump
    } else if matches.get_flag(cmd::EDIT) {
      Subcommand::Edit
    } else if matches.get_flag(cmd::EVALUATE) {
      if positional.arguments.len() > 1 {
        return Err(ConfigError::SubcommandArguments {
          subcommand: cmd::EVALUATE,
          arguments: positional
            .arguments
            .into_iter()
            .skip(1)
            .collect::<Vec<String>>(),
        });
      }

      Subcommand::Evaluate {
        variable: positional.arguments.into_iter().next(),
        overrides,
      }
    } else if matches.get_flag(cmd::FORMAT) {
      Subcommand::Format
    } else if matches.get_flag(cmd::GROUPS) {
      Subcommand::Groups
    } else if matches.get_flag(cmd::INIT) {
      Subcommand::Init
    } else if let Some(path) = matches.get_many::<String>(cmd::LIST) {
      Subcommand::List {
        path: Self::parse_module_path(path)?,
      }
    } else if matches.get_flag(cmd::MAN) {
      Subcommand::Man
    } else if let Some(request) = matches.get_one::<String>(cmd::REQUEST) {
      Subcommand::Request {
        request: serde_json::from_str(request)
          .map_err(|source| ConfigError::RequestParse { source })?,
      }
    } else if let Some(path) = matches.get_many::<String>(cmd::SHOW) {
      Subcommand::Show {
        path: Self::parse_module_path(path)?,
      }
    } else if matches.get_flag(cmd::SUMMARY) {
      Subcommand::Summary
    } else if matches.get_flag(cmd::VARIABLES) {
      Subcommand::Variables
    } else {
      Subcommand::Run {
        arguments: positional.arguments,
        overrides,
      }
    };

    let unstable = matches.get_flag(arg::UNSTABLE) || subcommand == Subcommand::Summary;
    let explain = matches.get_flag(arg::EXPLAIN);

    Ok(Self {
      alias_style: matches
        .get_one::<AliasStyle>(arg::ALIAS_STYLE)
        .unwrap()
        .clone(),
      allow_missing: matches.get_flag(arg::ALLOW_MISSING),
      check: matches.get_flag(arg::CHECK),
      color: (*matches.get_one::<UseColor>(arg::COLOR).unwrap()).into(),
      command_color: matches
        .get_one::<CommandColor>(arg::COMMAND_COLOR)
        .copied()
        .map(CommandColor::into),
      dotenv_filename: matches
        .get_one::<String>(arg::DOTENV_FILENAME)
        .map(Into::into),
      dotenv_path: matches.get_one::<PathBuf>(arg::DOTENV_PATH).map(Into::into),
      dry_run: matches.get_flag(arg::DRY_RUN),
      dump_format: matches
        .get_one::<DumpFormat>(arg::DUMP_FORMAT)
        .unwrap()
        .clone(),
      explain,
      highlight: !matches.get_flag(arg::NO_HIGHLIGHT),
      invocation_directory: env::current_dir().context(config_error::CurrentDirContext)?,
      list_heading: matches.get_one::<String>(arg::LIST_HEADING).unwrap().into(),
      list_prefix: matches.get_one::<String>(arg::LIST_PREFIX).unwrap().into(),
      list_submodules: matches.get_flag(arg::LIST_SUBMODULES),
      load_dotenv: !matches.get_flag(arg::NO_DOTENV),
      no_aliases: matches.get_flag(arg::NO_ALIASES),
      no_dependencies: matches.get_flag(arg::NO_DEPS),
      one: matches.get_flag(arg::ONE),
      search_config,
      shell: matches.get_one::<String>(arg::SHELL).map(Into::into),
      shell_args: if matches.get_flag(arg::CLEAR_SHELL_ARGS) {
        Some(Vec::new())
      } else {
        matches
          .get_many::<String>(arg::SHELL_ARG)
          .map(|s| s.map(Into::into).collect())
      },
      shell_command: matches.get_flag(arg::SHELL_COMMAND),
      subcommand,
      timestamp: matches.get_flag(arg::TIMESTAMP),
      timestamp_format: matches
        .get_one::<String>(arg::TIMESTAMP_FORMAT)
        .unwrap()
        .into(),
      unsorted: matches.get_flag(arg::UNSORTED),
      unstable,
      verbosity: if matches.get_flag(arg::QUIET) {
        Verbosity::Quiet
      } else {
        Verbosity::from_flag_occurrences(matches.get_count(arg::VERBOSE))
      },
      yes: matches.get_flag(arg::YES),
    })
  }

  pub(crate) fn require_unstable(
    &self,
    justfile: &Justfile,
    unstable_feature: UnstableFeature,
  ) -> RunResult<'static> {
    if self.unstable || justfile.settings.unstable {
      Ok(())
    } else {
      Err(Error::UnstableFeature { unstable_feature })
    }
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    clap::error::{ContextKind, ContextValue},
    pretty_assertions::assert_eq,
  };

  macro_rules! test {
    {
      name: $name:ident,
      args: [$($arg:expr),*],
      $(color: $color:expr,)?
      $(dry_run: $dry_run:expr,)?
      $(dump_format: $dump_format:expr,)?
      $(highlight: $highlight:expr,)?
      $(no_dependencies: $no_dependencies:expr,)?
      $(search_config: $search_config:expr,)?
      $(shell: $shell:expr,)?
      $(shell_args: $shell_args:expr,)?
      $(subcommand: $subcommand:expr,)?
      $(unsorted: $unsorted:expr,)?
      $(unstable: $unstable:expr,)?
      $(verbosity: $verbosity:expr,)?
    } => {
      #[test]
      fn $name() {
        let arguments = &[
          "just",
          $($arg,)*
        ];

        let want = Config {
          $(color: $color,)?
          $(dry_run: $dry_run,)?
          $(dump_format: $dump_format,)?
          $(highlight: $highlight,)?
          $(no_dependencies: $no_dependencies,)?
          $(search_config: $search_config,)?
          $(shell: $shell,)?
          $(shell_args: $shell_args,)?
          $(subcommand: $subcommand,)?
          $(unsorted: $unsorted,)?
          $(unstable: $unstable,)?
          $(verbosity: $verbosity,)?
          ..testing::config(&[])
        };

        test(arguments, want);
      }
    }
  }

  #[track_caller]
  fn test(arguments: &[&str], want: Config) {
    let app = Config::app();
    let matches = app
      .try_get_matches_from(arguments)
      .expect("argument parsing failed");
    let have = Config::from_matches(&matches).expect("config parsing failed");
    assert_eq!(have, want);
  }

  macro_rules! error {
    {
      name: $name:ident,
      args: [$($arg:expr),*],
    } => {
      #[test]
      fn $name() {
        let arguments = &[
          "just",
          $($arg,)*
        ];

        let app = Config::app();

        app.try_get_matches_from(arguments).expect_err("Expected clap error");
      }
    };
    {
      name: $name:ident,
      args: [$($arg:expr),*],
      error: $error:pat,
      $(check: $check:block,)?
    } => {
      #[test]
      fn $name() {
        let arguments = &[
          "just",
          $($arg,)*
        ];

        let app = Config::app();

        let matches = app.try_get_matches_from(arguments).expect("Matching fails");

        match Config::from_matches(&matches).expect_err("config parsing succeeded") {
          $error => { $($check)? }
          other => panic!("Unexpected config error: {other}"),
        }
      }
    }
  }

  macro_rules! error_matches {
    (
      name: $name:ident,
      args: [$($arg:expr),*],
      error: $error:pat,
      $(check: $check:block,)?
    ) => {
      #[test]
      fn $name() {
        let arguments = &[
          "just",
          $($arg,)*
        ];

        let app = Config::app();

        match app.try_get_matches_from(arguments) {
          Err($error) => { $($check)? }
          other => panic!("Unexpected result from get matches: {other:?}")
        }
      }
    };
  }

  macro_rules! map {
    {} => {
      BTreeMap::new()
    };
    {
      $($key:literal : $value:literal),* $(,)?
    } => {
      {
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        $(
          map.insert($key.to_owned(), $value.to_owned());
        )*
        map
      }
    }
  }

  test! {
    name: default_config,
    args: [],
  }

  test! {
    name: color_default,
    args: [],
    color: Color::auto(),
  }

  test! {
    name: color_never,
    args: ["--color", "never"],
    color: Color::never(),
  }

  test! {
    name: color_always,
    args: ["--color", "always"],
    color: Color::always(),
  }

  test! {
    name: color_auto,
    args: ["--color", "auto"],
    color: Color::auto(),
  }

  error! {
    name: color_bad_value,
    args: ["--color", "foo"],
  }

  test! {
    name: dry_run_default,
    args: [],
    dry_run: false,
  }

  test! {
    name: dry_run_long,
    args: ["--dry-run"],
    dry_run: true,
  }

  test! {
    name: dry_run_short,
    args: ["-n"],
    dry_run: true,
  }

  error! {
    name: dry_run_quiet,
    args: ["--dry-run", "--quiet"],
  }

  test! {
    name: highlight_default,
    args: [],
    highlight: true,
  }

  test! {
    name: highlight_yes,
    args: ["--highlight"],
    highlight: true,
  }

  test! {
    name: highlight_no,
    args: ["--no-highlight"],
    highlight: false,
  }

  test! {
    name: highlight_no_yes,
    args: ["--no-highlight", "--highlight"],
    highlight: true,
  }

  test! {
    name: highlight_no_yes_no,
    args: ["--no-highlight", "--highlight", "--no-highlight"],
    highlight: false,
  }

  test! {
    name: highlight_yes_no,
    args: ["--highlight", "--no-highlight"],
    highlight: false,
  }

  test! {
    name: no_deps,
    args: ["--no-deps"],
    no_dependencies: true,
  }

  test! {
    name: no_dependencies,
    args: ["--no-dependencies"],
    no_dependencies: true,
  }

  test! {
    name: unsorted_default,
    args: [],
    unsorted: false,
  }

  test! {
    name: unsorted_long,
    args: ["--unsorted"],
    unsorted: true,
  }

  test! {
    name: unsorted_short,
    args: ["-u"],
    unsorted: true,
  }

  test! {
    name: quiet_default,
    args: [],
    verbosity: Verbosity::Taciturn,
  }

  test! {
    name: quiet_long,
    args: ["--quiet"],
    verbosity: Verbosity::Quiet,
  }

  test! {
    name: quiet_short,
    args: ["-q"],
    verbosity: Verbosity::Quiet,
  }

  error! {
    name: dotenv_both_filename_and_path,
    args: ["--dotenv-filename", "foo", "--dotenv-path", "bar"],
  }

  test! {
    name: set_default,
    args: [],
    subcommand: Subcommand::Run {
      arguments: Vec::new(),
      overrides: map!(),
    },
  }

  test! {
    name: set_one,
    args: ["--set", "foo", "bar"],
    subcommand: Subcommand::Run {
      arguments: Vec::new(),
      overrides: map!{"foo": "bar"},
    },
  }

  test! {
    name: set_empty,
    args: ["--set", "foo", ""],
    subcommand: Subcommand::Run {
      arguments: Vec::new(),
      overrides: map!{"foo": ""},
    },
  }

  test! {
    name: set_two,
    args: ["--set", "foo", "bar", "--set", "bar", "baz"],
    subcommand: Subcommand::Run {
      arguments: Vec::new(),
      overrides: map!{"foo": "bar", "bar": "baz"},
    },
  }

  test! {
    name: set_override,
    args: ["--set", "foo", "bar", "--set", "foo", "baz"],
    subcommand: Subcommand::Run {
      arguments: Vec::new(),
      overrides: map!{"foo": "baz"},
    },
  }

  error! {
    name: set_bad,
    args: ["--set", "foo"],
  }

  test! {
    name: shell_default,
    args: [],
    shell: None,
    shell_args: None,
  }

  test! {
    name: shell_set,
    args: ["--shell", "tclsh"],
    shell: Some("tclsh".to_owned()),
  }

  test! {
    name: shell_args_set,
    args: ["--shell-arg", "hello"],
    shell: None,
    shell_args: Some(vec!["hello".into()]),
  }

  test! {
    name: verbosity_default,
    args: [],
    verbosity: Verbosity::Taciturn,
  }

  test! {
    name: verbosity_long,
    args: ["--verbose"],
    verbosity: Verbosity::Loquacious,
  }

  test! {
    name: verbosity_loquacious,
    args: ["-v"],
    verbosity: Verbosity::Loquacious,
  }

  test! {
    name: verbosity_grandiloquent,
    args: ["-v", "-v"],
    verbosity: Verbosity::Grandiloquent,
  }

  test! {
    name: verbosity_great_grandiloquent,
    args: ["-v", "-v", "-v"],
    verbosity: Verbosity::Grandiloquent,
  }

  test! {
    name: subcommand_default,
    args: [],
    subcommand: Subcommand::Run {
      arguments: Vec::new(),
      overrides: map!{},
    },
  }

  error! {
    name: subcommand_conflict_changelog,
    args: ["--list", "--changelog"],
  }

  error! {
    name: subcommand_conflict_summary,
    args: ["--list", "--summary"],
  }

  error! {
    name: subcommand_conflict_dump,
    args: ["--list", "--dump"],
  }

  error! {
    name: subcommand_conflict_fmt,
    args: ["--list", "--fmt"],
  }

  error! {
    name: subcommand_conflict_init,
    args: ["--list", "--init"],
  }

  error! {
    name: subcommand_conflict_evaluate,
    args: ["--list", "--evaluate"],
  }

  error! {
    name: subcommand_conflict_show,
    args: ["--list", "--show"],
  }

  error! {
    name: subcommand_conflict_completions,
    args: ["--list", "--completions"],
  }

  error! {
    name: subcommand_conflict_variables,
    args: ["--list", "--variables"],
  }

  error! {
    name: subcommand_conflict_choose,
    args: ["--list", "--choose"],
  }

  test! {
    name: subcommand_completions,
    args: ["--completions", "bash"],
    subcommand: Subcommand::Completions{ shell: completions::Shell::Bash },
  }

  test! {
    name: subcommand_completions_uppercase,
    args: ["--completions", "BASH"],
    subcommand: Subcommand::Completions{ shell: completions::Shell::Bash },
  }

  error! {
    name: subcommand_completions_invalid,
    args: ["--completions", "monstersh"],
  }

  test! {
    name: subcommand_dump,
    args: ["--dump"],
    subcommand: Subcommand::Dump,
  }

  test! {
    name: dump_format,
    args: ["--dump-format", "json"],
    dump_format: DumpFormat::Json,
  }

  test! {
    name: subcommand_edit,
    args: ["--edit"],
    subcommand: Subcommand::Edit,
  }

  test! {
    name: subcommand_evaluate,
    args: ["--evaluate"],
    subcommand: Subcommand::Evaluate {
      overrides: map!{},
      variable: None,
    },
  }

  test! {
    name: subcommand_evaluate_overrides,
    args: ["--evaluate", "x=y"],
    subcommand: Subcommand::Evaluate {
      overrides: map!{"x": "y"},
      variable: None,
    },
  }

  test! {
    name: subcommand_evaluate_overrides_with_argument,
    args: ["--evaluate", "x=y", "foo"],
    subcommand: Subcommand::Evaluate {
      overrides: map!{"x": "y"},
      variable: Some("foo".to_owned()),
    },
  }

  test! {
    name: subcommand_list_long,
    args: ["--list"],
    subcommand: Subcommand::List{ path: ModulePath { path: Vec::new(), spaced: false } },
  }

  test! {
    name: subcommand_list_short,
    args: ["-l"],
    subcommand: Subcommand::List{ path: ModulePath { path: Vec::new(), spaced: false } },
  }

  test! {
    name: subcommand_list_arguments,
    args: ["--list", "bar"],
    subcommand: Subcommand::List{ path: ModulePath { path: vec!["bar".into()], spaced: false } },
  }

  test! {
    name: subcommand_show_long,
    args: ["--show", "build"],
    subcommand: Subcommand::Show { path: ModulePath { path: vec!["build".into()], spaced: false } },
  }

  test! {
    name: subcommand_show_short,
    args: ["-s", "build"],
    subcommand: Subcommand::Show { path: ModulePath { path: vec!["build".into()], spaced: false } },
  }

  test! {
    name: subcommand_show_multiple_args,
    args: ["--show", "foo", "bar"],
    subcommand: Subcommand::Show {
      path: ModulePath {
        path: vec!["foo".into(), "bar".into()],
        spaced: true,
      },
    },
  }

  test! {
    name: subcommand_summary,
    args: ["--summary"],
    subcommand: Subcommand::Summary,
    unstable: true,
  }

  test! {
    name: arguments,
    args: ["foo", "bar"],
    subcommand: Subcommand::Run {
      arguments: vec![String::from("foo"), String::from("bar")],
      overrides: map!{},
    },
  }

  test! {
    name: arguments_leading_equals,
    args: ["=foo"],
    subcommand: Subcommand::Run {
      arguments: vec!["=foo".to_owned()],
      overrides: map!{},
    },
  }

  test! {
    name: overrides,
    args: ["foo=bar", "bar=baz"],
    subcommand: Subcommand::Run {
      arguments: Vec::new(),
      overrides: map!{"foo": "bar", "bar": "baz"},
    },
  }

  test! {
    name: overrides_empty,
    args: ["foo=", "bar="],
    subcommand: Subcommand::Run {
      arguments: Vec::new(),
      overrides: map!{"foo": "", "bar": ""},
    },
  }

  test! {
    name: overrides_override_sets,
    args: ["--set", "foo", "0", "--set", "bar", "1", "foo=bar", "bar=baz"],
    subcommand: Subcommand::Run {
      arguments: Vec::new(),
      overrides: map!{"foo": "bar", "bar": "baz"},
    },
  }

  test! {
    name: shell_args_default,
    args: [],
  }

  test! {
    name: shell_args_set_hyphen,
    args: ["--shell-arg", "--foo"],
    shell_args: Some(vec!["--foo".to_owned()]),
  }

  test! {
    name: shell_args_set_word,
    args: ["--shell-arg", "foo"],
    shell_args: Some(vec!["foo".to_owned()]),
  }

  test! {
    name: shell_args_set_multiple,
    args: ["--shell-arg", "foo", "--shell-arg", "bar"],
    shell_args: Some(vec!["foo".to_owned(), "bar".to_owned()]),

  }

  test! {
    name: shell_args_clear,
    args: ["--clear-shell-args"],
    shell_args: Some(Vec::new()),

  }

  test! {
    name: shell_args_clear_and_set,
    args: ["--clear-shell-args", "--shell-arg", "bar"],
    shell_args: Some(vec!["bar".to_owned()]),

  }

  test! {
    name: shell_args_set_and_clear,
    args: ["--shell-arg", "bar", "--clear-shell-args"],
    shell_args: Some(Vec::new()),

  }

  test! {
    name: shell_args_set_multiple_and_clear,
    args: ["--shell-arg", "bar", "--shell-arg", "baz", "--clear-shell-args"],
    shell_args: Some(Vec::new()),

  }

  test! {
    name: search_config_default,
    args: [],
    search_config: SearchConfig::FromInvocationDirectory,
  }

  test! {
    name: search_config_from_working_directory_and_justfile,
    args: ["--working-directory", "foo", "--justfile", "bar"],
    search_config: SearchConfig::WithJustfileAndWorkingDirectory {
      justfile: PathBuf::from("bar"),
      working_directory: PathBuf::from("foo"),
    },
  }

  test! {
    name: search_config_justfile_long,
    args: ["--justfile", "foo"],
    search_config: SearchConfig::WithJustfile {
      justfile: PathBuf::from("foo"),
    },
  }

  test! {
    name: search_config_justfile_short,
    args: ["-f", "foo"],
    search_config: SearchConfig::WithJustfile {
      justfile: PathBuf::from("foo"),
    },
  }

  test! {
    name: search_directory_parent,
    args: ["../"],
    search_config: SearchConfig::FromSearchDirectory {
      search_directory: PathBuf::from(".."),
    },
  }

  test! {
    name: search_directory_parent_with_recipe,
    args: ["../build"],
    search_config: SearchConfig::FromSearchDirectory {
      search_directory: PathBuf::from(".."),
    },
    subcommand: Subcommand::Run { arguments: vec!["build".to_owned()], overrides: BTreeMap::new() },
  }

  test! {
    name: search_directory_child,
    args: ["foo/"],
    search_config: SearchConfig::FromSearchDirectory {
      search_directory: PathBuf::from("foo"),
    },
  }

  test! {
    name: search_directory_deep,
    args: ["foo/bar/"],
    search_config: SearchConfig::FromSearchDirectory {
      search_directory: PathBuf::from("foo/bar"),
    },
  }

  test! {
    name: search_directory_child_with_recipe,
    args: ["foo/build"],
    search_config: SearchConfig::FromSearchDirectory {
      search_directory: PathBuf::from("foo"),
    },
    subcommand: Subcommand::Run { arguments: vec!["build".to_owned()], overrides: BTreeMap::new() },
  }

  error! {
    name: search_directory_conflict_justfile,
    args: ["--justfile", "bar", "foo/build"],
    error: ConfigError::SearchDirConflict,
  }

  error! {
    name: search_directory_conflict_working_directory,
    args: ["--justfile", "bar", "--working-directory", "baz", "foo/build"],
    error: ConfigError::SearchDirConflict,
  }

  error_matches! {
    name: completions_argument,
    args: ["--completions", "foo"],
    error: error,
    check: {
      assert_eq!(error.kind(), clap::error::ErrorKind::InvalidValue);
      assert_eq!(error.context().collect::<Vec<_>>(), vec![
        (
          ContextKind::InvalidArg,
          &ContextValue::String("--completions <SHELL>".into())),
        (
          ContextKind::InvalidValue,
          &ContextValue::String("foo".into()),
        ),
        (
          ContextKind::ValidValue,
          &ContextValue::Strings([
            "bash".into(),
            "elvish".into(),
            "fish".into(),
            "nushell".into(),
            "powershell".into(),
            "zsh".into()].into()
          ),
        ),
      ]);
    },
  }

  error! {
    name: changelog_arguments,
    args: ["--changelog", "bar"],
    error: ConfigError::SubcommandArguments { subcommand, arguments },
    check: {
      assert_eq!(subcommand, cmd::CHANGELOG);
      assert_eq!(arguments, &["bar"]);
    },
  }

  error! {
    name: dump_arguments,
    args: ["--dump", "bar"],
    error: ConfigError::SubcommandArguments { subcommand, arguments },
    check: {
      assert_eq!(subcommand, cmd::DUMP);
      assert_eq!(arguments, &["bar"]);
    },
  }

  error! {
    name: edit_arguments,
    args: ["--edit", "bar"],
    error: ConfigError::SubcommandArguments { subcommand, arguments },
    check: {
      assert_eq!(subcommand, cmd::EDIT);
      assert_eq!(arguments, &["bar"]);
    },
  }

  error! {
    name: fmt_arguments,
    args: ["--fmt", "bar"],
    error: ConfigError::SubcommandArguments { subcommand, arguments },
    check: {
      assert_eq!(subcommand, cmd::FORMAT);
      assert_eq!(arguments, &["bar"]);
    },
  }

  error! {
    name: fmt_alias,
    args: ["--format", "bar"],
    error: ConfigError::SubcommandArguments { subcommand, arguments },
    check: {
      assert_eq!(subcommand, cmd::FORMAT);
      assert_eq!(arguments, &["bar"]);
    },
  }

  error! {
    name: init_arguments,
    args: ["--init", "bar"],
    error: ConfigError::SubcommandArguments { subcommand, arguments },
    check: {
      assert_eq!(subcommand, cmd::INIT);
      assert_eq!(arguments, &["bar"]);
    },
  }

  error! {
    name: init_alias,
    args: ["--initialize", "bar"],
    error: ConfigError::SubcommandArguments { subcommand, arguments },
    check: {
      assert_eq!(subcommand, cmd::INIT);
      assert_eq!(arguments, &["bar"]);
    },
  }

  error! {
    name: summary_arguments,
    args: ["--summary", "bar"],
    error: ConfigError::SubcommandArguments { subcommand, arguments },
    check: {
      assert_eq!(subcommand, cmd::SUMMARY);
      assert_eq!(arguments, &["bar"]);
    },
  }

  error! {
    name: subcommand_overrides_and_arguments,
    args: ["--summary", "bar=baz", "bar"],
    error: ConfigError::SubcommandOverridesAndArguments { subcommand, arguments, overrides },
    check: {
      assert_eq!(subcommand, cmd::SUMMARY);
      assert_eq!(overrides, map!{"bar": "baz"});
      assert_eq!(arguments, &["bar"]);
    },
  }

  error! {
    name: summary_overrides,
    args: ["--summary", "bar=baz"],
    error: ConfigError::SubcommandOverrides { subcommand, overrides },
    check: {
      assert_eq!(subcommand, cmd::SUMMARY);
      assert_eq!(overrides, map!{"bar": "baz"});
    },
  }
}
