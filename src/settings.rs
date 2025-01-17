use super::*;

pub(crate) const DEFAULT_SHELL: &str = "sh";
pub(crate) const DEFAULT_SHELL_ARGS: &[&str] = &["-cu"];
pub(crate) const WINDOWS_POWERSHELL_SHELL: &str = "powershell.exe";
pub(crate) const WINDOWS_POWERSHELL_ARGS: &[&str] = &["-NoLogo", "-Command"];

#[derive(Debug, PartialEq, Serialize, Default)]
pub(crate) struct Settings<'src> {
  pub(crate) allow_duplicate_recipes: bool,
  pub(crate) allow_duplicate_variables: bool,
  pub(crate) dotenv_filename: Option<String>,
  pub(crate) dotenv_load: bool,
  pub(crate) dotenv_path: Option<PathBuf>,
  pub(crate) dotenv_required: bool,
  pub(crate) export: bool,
  pub(crate) fallback: bool,
  pub(crate) ignore_comments: bool,
  pub(crate) no_exit_message: bool,
  pub(crate) positional_arguments: bool,
  pub(crate) quiet: bool,
  #[serde(skip)]
  pub(crate) script_interpreter: Option<Interpreter<'src>>,
  pub(crate) shell: Option<Interpreter<'src>>,
  pub(crate) tempdir: Option<String>,
  pub(crate) unstable: bool,
  pub(crate) windows_powershell: bool,
  pub(crate) windows_shell: Option<Interpreter<'src>>,
  pub(crate) working_directory: Option<PathBuf>,
}

impl<'src> Settings<'src> {
  pub(crate) fn from_table(sets: Table<'src, Set<'src>>) -> Self {
    let mut settings = Self::default();

    for (_name, set) in sets {
      match set.value {
        Setting::AllowDuplicateRecipes(allow_duplicate_recipes) => {
          settings.allow_duplicate_recipes = allow_duplicate_recipes;
        }
        Setting::AllowDuplicateVariables(allow_duplicate_variables) => {
          settings.allow_duplicate_variables = allow_duplicate_variables;
        }
        Setting::DotenvFilename(filename) => {
          settings.dotenv_filename = Some(filename.cooked);
        }
        Setting::DotenvLoad(dotenv_load) => {
          settings.dotenv_load = dotenv_load;
        }
        Setting::DotenvPath(path) => {
          settings.dotenv_path = Some(PathBuf::from(path.cooked));
        }
        Setting::DotenvRequired(dotenv_required) => {
          settings.dotenv_required = dotenv_required;
        }
        Setting::Export(export) => {
          settings.export = export;
        }
        Setting::Fallback(fallback) => {
          settings.fallback = fallback;
        }
        Setting::IgnoreComments(ignore_comments) => {
          settings.ignore_comments = ignore_comments;
        }
        Setting::NoExitMessage(no_exit_message) => {
          settings.no_exit_message = no_exit_message;
        }
        Setting::PositionalArguments(positional_arguments) => {
          settings.positional_arguments = positional_arguments;
        }
        Setting::Quiet(quiet) => {
          settings.quiet = quiet;
        }
        Setting::ScriptInterpreter(script_interpreter) => {
          settings.script_interpreter = Some(script_interpreter);
        }
        Setting::Shell(shell) => {
          settings.shell = Some(shell);
        }
        Setting::Unstable(unstable) => {
          settings.unstable = unstable;
        }
        Setting::WindowsPowerShell(windows_powershell) => {
          settings.windows_powershell = windows_powershell;
        }
        Setting::WindowsShell(windows_shell) => {
          settings.windows_shell = Some(windows_shell);
        }
        Setting::Tempdir(tempdir) => {
          settings.tempdir = Some(tempdir.cooked);
        }
        Setting::WorkingDirectory(working_directory) => {
          settings.working_directory = Some(working_directory.cooked.into());
        }
      }
    }

    settings
  }

  pub(crate) fn shell_command(&self, config: &Config) -> Command {
    let (command, args) = self.shell(config);

    let mut cmd = Command::new(command);

    cmd.args(args);

    cmd
  }

  pub(crate) fn shell<'a>(&'a self, config: &'a Config) -> (&'a str, Vec<&'a str>) {
    match (&config.shell, &config.shell_args) {
      (Some(shell), Some(shell_args)) => (shell, shell_args.iter().map(String::as_ref).collect()),
      (Some(shell), None) => (shell, DEFAULT_SHELL_ARGS.to_vec()),
      (None, Some(shell_args)) => (
        DEFAULT_SHELL,
        shell_args.iter().map(String::as_ref).collect(),
      ),
      (None, None) => {
        if let (true, Some(shell)) = (cfg!(windows), &self.windows_shell) {
          (
            shell.command.cooked.as_ref(),
            shell
              .arguments
              .iter()
              .map(|argument| argument.cooked.as_ref())
              .collect(),
          )
        } else if cfg!(windows) && self.windows_powershell {
          (WINDOWS_POWERSHELL_SHELL, WINDOWS_POWERSHELL_ARGS.to_vec())
        } else if let Some(shell) = &self.shell {
          (
            shell.command.cooked.as_ref(),
            shell
              .arguments
              .iter()
              .map(|argument| argument.cooked.as_ref())
              .collect(),
          )
        } else {
          (DEFAULT_SHELL, DEFAULT_SHELL_ARGS.to_vec())
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_shell() {
    let settings = Settings::default();

    let config = Config {
      shell_command: false,
      ..testing::config(&[])
    };

    assert_eq!(settings.shell(&config), ("sh", vec!["-cu"]));
  }

  #[test]
  fn default_shell_powershell() {
    let settings = Settings {
      windows_powershell: true,
      ..Default::default()
    };

    let config = Config {
      shell_command: false,
      ..testing::config(&[])
    };

    if cfg!(windows) {
      assert_eq!(
        settings.shell(&config),
        ("powershell.exe", vec!["-NoLogo", "-Command"])
      );
    } else {
      assert_eq!(settings.shell(&config), ("sh", vec!["-cu"]));
    }
  }

  #[test]
  fn overwrite_shell() {
    let settings = Settings::default();

    let config = Config {
      shell_command: true,
      shell: Some("lol".to_string()),
      shell_args: Some(vec!["-nice".to_string()]),
      ..testing::config(&[])
    };

    assert_eq!(settings.shell(&config), ("lol", vec!["-nice"]));
  }

  #[test]
  fn overwrite_shell_powershell() {
    let settings = Settings {
      windows_powershell: true,
      ..Default::default()
    };

    let config = Config {
      shell_command: true,
      shell: Some("lol".to_string()),
      shell_args: Some(vec!["-nice".to_string()]),
      ..testing::config(&[])
    };

    assert_eq!(settings.shell(&config), ("lol", vec!["-nice"]));
  }

  #[test]
  fn shell_cooked() {
    let settings = Settings {
      shell: Some(Interpreter {
        command: StringLiteral {
          kind: StringKind::from_token_start("\"").unwrap(),
          raw: "asdf.exe",
          cooked: "asdf.exe".to_string(),
          expand: false,
        },
        arguments: vec![StringLiteral {
          kind: StringKind::from_token_start("\"").unwrap(),
          raw: "-nope",
          cooked: "-nope".to_string(),
          expand: false,
        }],
      }),
      ..Default::default()
    };

    let config = Config {
      shell_command: false,
      ..testing::config(&[])
    };

    assert_eq!(settings.shell(&config), ("asdf.exe", vec!["-nope"]));
  }

  #[test]
  fn shell_present_but_not_shell_args() {
    let settings = Settings {
      windows_powershell: true,
      ..Default::default()
    };

    let config = Config {
      shell: Some("lol".to_string()),
      ..testing::config(&[])
    };

    assert_eq!(settings.shell(&config).0, "lol");
  }

  #[test]
  fn shell_args_present_but_not_shell() {
    let settings = Settings {
      windows_powershell: true,
      ..Default::default()
    };

    let config = Config {
      shell_command: false,
      shell_args: Some(vec!["-nice".to_string()]),
      ..testing::config(&[])
    };

    assert_eq!(settings.shell(&config), ("sh", vec!["-nice"]));
  }
}
