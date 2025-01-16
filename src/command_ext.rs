use super::*;

pub(crate) trait CommandExt {
  fn export(
    &mut self,
    settings: &Settings,
    dotenv: &BTreeMap<String, String>,
    scope: &Scope,
    unexports: &HashSet<String>,
  ) -> &mut Command;

  fn export_scope(&mut self, settings: &Settings, scope: &Scope, unexports: &HashSet<String>);

  fn status_guard(self) -> (io::Result<ExitStatus>, Option<Signal>);

  fn output_guard(self) -> (io::Result<process::Output>, Option<Signal>);
}

impl CommandExt for Command {
  fn export(
    &mut self,
    settings: &Settings,
    dotenv: &BTreeMap<String, String>,
    scope: &Scope,
    unexports: &HashSet<String>,
  ) -> &mut Command {
    for (name, value) in dotenv {
      self.env(name, value);
    }

    if let Some(parent) = scope.parent() {
      self.export_scope(settings, parent, unexports);
    }

    self
  }

  fn export_scope(&mut self, settings: &Settings, scope: &Scope, unexports: &HashSet<String>) {
    if let Some(parent) = scope.parent() {
      self.export_scope(settings, parent, unexports);
    }

    for unexport in unexports {
      self.env_remove(unexport);
    }

    for binding in scope.bindings() {
      if binding.export || (settings.export && !binding.constant) {
        self.env(binding.name.lexeme(), &binding.value);
      }
    }
  }

  fn status_guard(self) -> (io::Result<ExitStatus>, Option<Signal>) {
    SignalHandler::spawn(self, |mut child| child.wait())
  }

  fn output_guard(self) -> (io::Result<process::Output>, Option<Signal>) {
    SignalHandler::spawn(self, process::Child::wait_with_output)
  }
}
