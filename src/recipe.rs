use super::*;

/// Return a `Error::Signal` if the process was terminated by a signal,
/// otherwise return an `Error::UnknownFailure`
fn error_from_signal(recipe: &str, line_number: Option<usize>, exit_status: ExitStatus) -> Error {
  match Platform::signal_from_exit_status(exit_status) {
    Some(signal) => Error::Signal {
      recipe,
      line_number,
      signal,
    },
    None => Error::Unknown {
      recipe,
      line_number,
    },
  }
}

/// A recipe, e.g. `foo: bar baz`
#[derive(PartialEq, Debug, Clone, Serialize)]
pub(crate) struct Recipe<'src, D = Dependency<'src>> {
  pub(crate) attributes: AttributeSet<'src>,
  pub(crate) body: Vec<Line<'src>>,
  pub(crate) dependencies: Vec<D>,
  pub(crate) doc: Option<String>,
  #[serde(skip)]
  pub(crate) file_depth: u32,
  #[serde(skip)]
  pub(crate) import_offsets: Vec<usize>,
  pub(crate) name: Name<'src>,
  pub(crate) namepath: Namepath<'src>,
  pub(crate) parameters: Vec<Parameter<'src>>,
  pub(crate) priors: usize,
  pub(crate) private: bool,
  pub(crate) quiet: bool,
  pub(crate) shebang: bool,
}

impl<'src, D> Recipe<'src, D> {
  pub(crate) fn argument_range(&self) -> RangeInclusive<usize> {
    self.min_arguments()..=self.max_arguments()
  }

  pub(crate) fn min_arguments(&self) -> usize {
    self
      .parameters
      .iter()
      .filter(|p| p.default.is_none() && p.kind != ParameterKind::Star)
      .count()
  }

  pub(crate) fn max_arguments(&self) -> usize {
    if self.parameters.iter().any(|p| p.kind.is_variadic()) {
      usize::MAX - 1
    } else {
      self.parameters.len()
    }
  }

  pub(crate) fn name(&self) -> &'src str {
    self.name.lexeme()
  }

  pub(crate) fn line_number(&self) -> usize {
    self.name.line
  }

  pub(crate) fn confirm(&self) -> RunResult<'src, bool> {
    if let Some(Attribute::Confirm(ref prompt)) =
      self.attributes.get(AttributeDiscriminant::Confirm)
    {
      if let Some(prompt) = prompt {
        eprint!("{} ", prompt.cooked);
      } else {
        eprint!("Run recipe `{}`? ", self.name);
      }
      let mut line = String::new();
      std::io::stdin()
        .read_line(&mut line)
        .map_err(|io_error| Error::GetConfirmation { io_error })?;
      let line = line.trim().to_lowercase();
      return Ok(line == "y" || line == "yes");
    }
    Ok(true)
  }

  pub(crate) fn check_can_be_default_recipe(&self) -> RunResult<'src, ()> {
    let min_arguments = self.min_arguments();
    if min_arguments > 0 {
      return Err(Error::DefaultRecipeRequiresArguments {
        recipe: self.name.lexeme(),
        min_arguments,
      });
    }

    Ok(())
  }

  pub(crate) fn is_public(&self) -> bool {
    !self.private && !self.attributes.contains(AttributeDiscriminant::Private)
  }

  pub(crate) fn is_script(&self) -> bool {
    self.shebang
  }

  pub(crate) fn takes_positional_arguments(&self, settings: &Settings) -> bool {
    settings.positional_arguments
      || self
        .attributes
        .contains(AttributeDiscriminant::PositionalArguments)
  }

  pub(crate) fn change_directory(&self) -> bool {
    !self.attributes.contains(AttributeDiscriminant::NoCd)
  }

  pub(crate) fn enabled(&self) -> bool {
    let linux = self.attributes.contains(AttributeDiscriminant::Linux);
    let macos = self.attributes.contains(AttributeDiscriminant::Macos);
    let openbsd = self.attributes.contains(AttributeDiscriminant::Openbsd);
    let unix = self.attributes.contains(AttributeDiscriminant::Unix);
    let windows = self.attributes.contains(AttributeDiscriminant::Windows);

    (!windows && !linux && !macos && !openbsd && !unix)
      || (cfg!(target_os = "linux") && (linux || unix))
      || (cfg!(target_os = "macos") && (macos || unix))
      || (cfg!(target_os = "openbsd") && (openbsd || unix))
      || (cfg!(target_os = "windows") && windows)
      || (cfg!(unix) && unix)
      || (cfg!(windows) && windows)
  }

  fn print_exit_message(&self, settings: &Settings) -> bool {
    if self.attributes.contains(AttributeDiscriminant::ExitMessage) {
      true
    } else if settings.no_exit_message {
      false
    } else {
      !self
        .attributes
        .contains(AttributeDiscriminant::NoExitMessage)
    }
  }

  fn working_directory<'a>(&'a self, context: &'a ExecutionContext) -> Option<PathBuf> {
    if !self.change_directory() {
      return None;
    }

    let working_directory = context.working_directory();

    for attribute in &self.attributes {
      if let Attribute::WorkingDirectory(dir) = attribute {
        return Some(working_directory.join(&dir.cooked));
      }
    }

    Some(working_directory)
  }

  fn no_quiet(&self) -> bool {
    self.attributes.contains(AttributeDiscriminant::NoQuiet)
  }

  pub(crate) fn run<'run>(
    &self,
    context: &ExecutionContext<'src, 'run>,
    scope: &Scope<'src, 'run>,
    positional: &[String],
    is_dependency: bool,
  ) -> RunResult<'src, ()> {
    let color = context.config.color.stderr().banner();
    let prefix = color.prefix();
    let suffix = color.suffix();

    if context.config.verbosity.loquacious() {
      eprintln!("{prefix}===> Running recipe `{}`...{suffix}", self.name);
    }

    if context.config.explain {
      if let Some(doc) = self.doc() {
        eprintln!("{prefix}#### {doc}{suffix}");
      }
    }

    let evaluator = Evaluator::new(context, is_dependency, scope);

    if self.is_script() {
      self.run_script(context, scope, positional, evaluator)
    } else {
      self.run_linewise(context, scope, positional, evaluator)
    }
  }

  fn run_linewise<'run>(
    &self,
    context: &ExecutionContext<'src, 'run>,
    scope: &Scope<'src, 'run>,
    positional: &[String],
    mut evaluator: Evaluator<'src, 'run>,
  ) -> RunResult<'src, ()> {
    let config = &context.config;

    let mut lines = self.body.iter().peekable();
    let mut line_number = self.line_number() + 1;
    loop {
      if lines.peek().is_none() {
        return Ok(());
      }
      let mut evaluated = String::new();
      let mut continued = false;
      let quiet_line = lines.peek().is_some_and(|line| line.is_quiet());
      let infallible_line = lines.peek().is_some_and(|line| line.is_infallible());

      let comment_line = context.module.settings.ignore_comments
        && lines.peek().is_some_and(|line| line.is_comment());

      loop {
        if lines.peek().is_none() {
          break;
        }
        let line = lines.next().unwrap();
        line_number += 1;
        if !comment_line {
          evaluated += &evaluator.evaluate_line(line, continued)?;
        }
        if line.is_continuation() && !comment_line {
          continued = true;
          evaluated.pop();
        } else {
          break;
        }
      }

      if comment_line {
        continue;
      }

      let mut command = evaluated.as_str();

      let sigils = usize::from(infallible_line) + usize::from(quiet_line);

      command = &command[sigils..];

      if command.is_empty() {
        continue;
      }

      if config.dry_run
        || config.verbosity.loquacious()
        || !((quiet_line ^ self.quiet)
          || (context.module.settings.quiet && !self.no_quiet())
          || config.verbosity.quiet())
      {
        let color = config
          .highlight
          .then(|| config.color.command(config.command_color))
          .unwrap_or(config.color)
          .stderr();

        if config.timestamp {
          eprint!(
            "[{}] ",
            color.paint(
              &chrono::Local::now()
                .format(&config.timestamp_format)
                .to_string()
            ),
          );
        }

        eprintln!("{}", color.paint(command));
      }

      if config.dry_run {
        continue;
      }

      let mut cmd = context.module.settings.shell_command(config);

      if let Some(working_directory) = self.working_directory(context) {
        cmd.current_dir(working_directory);
      }

      cmd.arg(command);

      if self.takes_positional_arguments(&context.module.settings) {
        cmd.arg(self.name.lexeme());
        cmd.args(positional);
      }

      if config.verbosity.quiet() {
        cmd.stderr(Stdio::null());
        cmd.stdout(Stdio::null());
      }

      cmd.export(
        &context.module.settings,
        context.dotenv,
        scope,
        &context.module.unexports,
      );

      match InterruptHandler::guard(|| cmd.status()) {
        Ok(exit_status) => {
          if let Some(code) = exit_status.code() {
            if code != 0 && !infallible_line {
              return Err(Error::Code {
                recipe: self.name(),
                line_number: Some(line_number),
                code,
                print_message: self.print_exit_message(&context.module.settings),
              });
            }
          } else {
            return Err(error_from_signal(
              self.name(),
              Some(line_number),
              exit_status,
            ));
          }
        }
        Err(io_error) => {
          return Err(Error::Io {
            recipe: self.name(),
            io_error,
          });
        }
      };
    }
  }

  pub(crate) fn run_script<'run>(
    &self,
    context: &ExecutionContext<'src, 'run>,
    scope: &Scope<'src, 'run>,
    positional: &[String],
    mut evaluator: Evaluator<'src, 'run>,
  ) -> RunResult<'src, ()> {
    let config = &context.config;

    let mut evaluated_lines = Vec::new();
    for line in &self.body {
      evaluated_lines.push(evaluator.evaluate_line(line, false)?);
    }

    if config.verbosity.loud() && (config.dry_run || self.quiet) {
      for line in &evaluated_lines {
        eprintln!(
          "{}",
          config
            .color
            .command(config.command_color)
            .stderr()
            .paint(line)
        );
      }
    }

    if config.dry_run {
      return Ok(());
    }

    let executor = if let Some(Attribute::Script(interpreter)) =
      self.attributes.get(AttributeDiscriminant::Script)
    {
      Executor::Command(
        interpreter
          .as_ref()
          .or(context.module.settings.script_interpreter.as_ref())
          .unwrap_or_else(|| Interpreter::default_script_interpreter()),
      )
    } else {
      let line = evaluated_lines
        .first()
        .ok_or_else(|| Error::internal("evaluated_lines was empty"))?;

      let shebang =
        Shebang::new(line).ok_or_else(|| Error::internal(format!("bad shebang line: {line}")))?;

      Executor::Shebang(shebang)
    };

    let mut tempdir_builder = tempfile::Builder::new();
    tempdir_builder.prefix("just-");
    let tempdir = match &context.module.settings.tempdir {
      Some(tempdir) => tempdir_builder.tempdir_in(context.search.working_directory.join(tempdir)),
      None => {
        if let Some(runtime_dir) = dirs::runtime_dir() {
          let path = runtime_dir.join("just");
          fs::create_dir_all(&path).map_err(|io_error| Error::RuntimeDirIo {
            io_error,
            path: path.clone(),
          })?;
          tempdir_builder.tempdir_in(path)
        } else {
          tempdir_builder.tempdir()
        }
      }
    }
    .map_err(|error| Error::TempdirIo {
      recipe: self.name(),
      io_error: error,
    })?;
    let mut path = tempdir.path().to_path_buf();

    let extension = self.attributes.iter().find_map(|attribute| {
      if let Attribute::Extension(extension) = attribute {
        Some(extension.cooked.as_str())
      } else {
        None
      }
    });

    path.push(executor.script_filename(self.name(), extension));

    let script = executor.script(self, &evaluated_lines);

    if config.verbosity.grandiloquent() {
      eprintln!("{}", config.color.doc().stderr().paint(&script));
    }

    fs::write(&path, script).map_err(|error| Error::TempdirIo {
      recipe: self.name(),
      io_error: error,
    })?;

    let mut command = executor.command(
      &path,
      self.name(),
      self.working_directory(context).as_deref(),
    )?;

    if self.takes_positional_arguments(&context.module.settings) {
      command.args(positional);
    }

    command.export(
      &context.module.settings,
      context.dotenv,
      scope,
      &context.module.unexports,
    );

    // run it!
    match InterruptHandler::guard(|| command.status()) {
      Ok(exit_status) => exit_status.code().map_or_else(
        || Err(error_from_signal(self.name(), None, exit_status)),
        |code| {
          if code == 0 {
            Ok(())
          } else {
            Err(Error::Code {
              recipe: self.name(),
              line_number: None,
              code,
              print_message: self.print_exit_message(&context.module.settings),
            })
          }
        },
      ),
      Err(io_error) => Err(executor.error(io_error, self.name())),
    }
  }

  pub(crate) fn groups(&self) -> BTreeSet<String> {
    self
      .attributes
      .iter()
      .filter_map(|attribute| {
        if let Attribute::Group(group) = attribute {
          Some(group.cooked.clone())
        } else {
          None
        }
      })
      .collect()
  }

  pub(crate) fn doc(&self) -> Option<&str> {
    for attribute in &self.attributes {
      if let Attribute::Doc(doc) = attribute {
        return doc.as_ref().map(|s| s.cooked.as_ref());
      }
    }

    self.doc.as_deref()
  }

  pub(crate) fn subsequents(&self) -> impl Iterator<Item = &D> {
    self.dependencies.iter().skip(self.priors)
  }
}

impl<D: Display> ColorDisplay for Recipe<'_, D> {
  fn fmt(&self, f: &mut Formatter, color: Color) -> fmt::Result {
    if !self
      .attributes
      .iter()
      .any(|attribute| matches!(attribute, Attribute::Doc(_)))
    {
      if let Some(doc) = &self.doc {
        writeln!(f, "# {doc}")?;
      }
    }

    for attribute in &self.attributes {
      writeln!(f, "[{attribute}]")?;
    }

    if self.quiet {
      write!(f, "@{}", self.name)?;
    } else {
      write!(f, "{}", self.name)?;
    }

    for parameter in &self.parameters {
      write!(f, " {}", parameter.color_display(color))?;
    }
    write!(f, ":")?;

    for (i, dependency) in self.dependencies.iter().enumerate() {
      if i == self.priors {
        write!(f, " &&")?;
      }

      write!(f, " {dependency}")?;
    }

    for (i, line) in self.body.iter().enumerate() {
      if i == 0 {
        writeln!(f)?;
      }
      for (j, fragment) in line.fragments.iter().enumerate() {
        if j == 0 {
          write!(f, "    ")?;
        }
        match fragment {
          Fragment::Text { token } => write!(f, "{}", token.lexeme())?,
          Fragment::Interpolation { expression, .. } => write!(f, "{{{{ {expression} }}}}")?,
        }
      }
      if i + 1 < self.body.len() {
        writeln!(f)?;
      }
    }
    Ok(())
  }
}

impl<'src, D> Keyed<'src> for Recipe<'src, D> {
  fn key(&self) -> &'src str {
    self.name.lexeme()
  }
}
