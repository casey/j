use super::*;

pub(crate) trait CommandExt {
  fn export(
    &mut self,
    settings: &Settings,
    dotenv: &BTreeMap<String, String>,
    scope: &Scope,
    unexports: &HashSet<String>,
  );

  fn export_scope(&mut self, settings: &Settings, scope: &Scope, unexports: &HashSet<String>);
}

impl CommandExt for Command {
  fn export(
    &mut self,
    settings: &Settings,
    dotenv: &BTreeMap<String, String>,
    scope: &Scope,
    unexports: &HashSet<String>,
  ) {
    for (name, value) in dotenv {
      self.env(name, value);
    }

    if let Some(parent) = scope.parent() {
      self.export_scope(settings, parent, unexports);
    }
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
        self.env(binding.name.lexeme(), binding.value.to_joined());
      }
    }
  }
}
