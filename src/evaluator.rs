use super::*;

pub(crate) struct Evaluator<'src: 'run, 'run> {
  pub(crate) assignments: Option<&'run Table<'src, Assignment<'src>>>,
  pub(crate) context: ExecutionContext<'src, 'run>,
  pub(crate) is_dependency: bool,
  pub(crate) scope: Scope<'src, 'run>,
}

impl<'src, 'run> Evaluator<'src, 'run> {
  pub(crate) fn evaluate_assignments(
    config: &'run Config,
    dotenv: &'run BTreeMap<String, String>,
    module: &'run Justfile<'src>,
    overrides: &BTreeMap<String, String>,
    parent: &'run Scope<'src, 'run>,
    search: &'run Search,
  ) -> RunResult<'src, Scope<'src, 'run>>
  where
    'src: 'run,
  {
    let context = ExecutionContext {
      config,
      dotenv,
      module,
      scope: parent,
      search,
    };

    let mut scope = context.scope.child();
    let mut unknown_overrides = Vec::new();

    for (name, value) in overrides {
      if let Some(assignment) = module.assignments.get(name) {
        scope.bind(Binding {
          constant: false,
          export: assignment.export,
          file_depth: 0,
          name: assignment.name,
          private: assignment.private,
          value: value.clone(),
        });
      } else {
        unknown_overrides.push(name.clone());
      }
    }

    if !unknown_overrides.is_empty() {
      return Err(Error::UnknownOverrides {
        overrides: unknown_overrides,
      });
    }

    let mut evaluator = Self {
      context,
      assignments: Some(&module.assignments),
      scope,
      is_dependency: false,
    };

    for assignment in module.assignments.values() {
      evaluator.evaluate_assignment(assignment)?;
    }

    Ok(evaluator.scope)
  }

  fn evaluate_assignment(&mut self, assignment: &Assignment<'src>) -> RunResult<'src, &str> {
    let name = assignment.name.lexeme();

    if !self.scope.bound(name) {
      let value = self.evaluate_expression(&assignment.value)?;
      self.scope.bind(Binding {
        constant: false,
        export: assignment.export,
        file_depth: 0,
        name: assignment.name,
        private: assignment.private,
        value,
      });
    }

    Ok(self.scope.value(name).unwrap())
  }

  pub(crate) fn evaluate_expression(
    &mut self,
    expression: &Expression<'src>,
  ) -> RunResult<'src, String> {
    match expression {
      Expression::And { lhs, rhs } => {
        let lhs = self.evaluate_expression(lhs)?;
        if lhs.is_empty() {
          return Ok(String::new());
        }
        self.evaluate_expression(rhs)
      }
      Expression::Assert { condition, error } => {
        if self.evaluate_condition(condition)? {
          Ok(String::new())
        } else {
          Err(Error::Assert {
            message: self.evaluate_expression(error)?,
          })
        }
      }
      Expression::Backtick { contents, token } => {
        if self.context.config.dry_run {
          Ok(format!("`{contents}`"))
        } else {
          Ok(self.run_backtick(contents, token)?)
        }
      }
      Expression::Call { thunk } => {
        use Thunk::*;
        let result = match thunk {
          Nullary { function, .. } => function(function::Context::new(self, thunk.name())),
          Unary { function, arg, .. } => {
            let arg = self.evaluate_expression(arg)?;
            function(function::Context::new(self, thunk.name()), &arg)
          }
          UnaryOpt {
            function,
            args: (a, b),
            ..
          } => {
            let a = self.evaluate_expression(a)?;
            let b = match b.as_ref() {
              Some(b) => Some(self.evaluate_expression(b)?),
              None => None,
            };
            function(function::Context::new(self, thunk.name()), &a, b.as_deref())
          }
          UnaryPlus {
            function,
            args: (a, rest),
            ..
          } => {
            let a = self.evaluate_expression(a)?;
            let mut rest_evaluated = Vec::new();
            for arg in rest {
              rest_evaluated.push(self.evaluate_expression(arg)?);
            }
            function(
              function::Context::new(self, thunk.name()),
              &a,
              &rest_evaluated,
            )
          }
          Binary {
            function,
            args: [a, b],
            ..
          } => {
            let a = self.evaluate_expression(a)?;
            let b = self.evaluate_expression(b)?;
            function(function::Context::new(self, thunk.name()), &a, &b)
          }
          BinaryPlus {
            function,
            args: ([a, b], rest),
            ..
          } => {
            let a = self.evaluate_expression(a)?;
            let b = self.evaluate_expression(b)?;
            let mut rest_evaluated = Vec::new();
            for arg in rest {
              rest_evaluated.push(self.evaluate_expression(arg)?);
            }
            function(
              function::Context::new(self, thunk.name()),
              &a,
              &b,
              &rest_evaluated,
            )
          }
          Ternary {
            function,
            args: [a, b, c],
            ..
          } => {
            let a = self.evaluate_expression(a)?;
            let b = self.evaluate_expression(b)?;
            let c = self.evaluate_expression(c)?;
            function(function::Context::new(self, thunk.name()), &a, &b, &c)
          }
        };
        result.map_err(|message| Error::FunctionCall {
          function: thunk.name(),
          message,
        })
      }
      Expression::Concatenation { lhs, rhs } => {
        Ok(self.evaluate_expression(lhs)? + &self.evaluate_expression(rhs)?)
      }
      Expression::Conditional {
        condition,
        then,
        otherwise,
      } => {
        if self.evaluate_condition(condition)? {
          self.evaluate_expression(then)
        } else {
          self.evaluate_expression(otherwise)
        }
      }
      Expression::Group { contents } => self.evaluate_expression(contents),
      Expression::Join { lhs: None, rhs } => Ok("/".to_string() + &self.evaluate_expression(rhs)?),
      Expression::Join {
        lhs: Some(lhs),
        rhs,
      } => Ok(self.evaluate_expression(lhs)? + "/" + &self.evaluate_expression(rhs)?),
      Expression::Or { lhs, rhs } => {
        let lhs = self.evaluate_expression(lhs)?;
        if !lhs.is_empty() {
          return Ok(lhs);
        }
        self.evaluate_expression(rhs)
      }
      Expression::StringLiteral { string_literal } => Ok(string_literal.cooked.clone()),
      Expression::Variable { name, .. } => {
        let variable = name.lexeme();
        if let Some(value) = self.scope.value(variable) {
          Ok(value.to_owned())
        } else if let Some(assignment) = self
          .assignments
          .and_then(|assignments| assignments.get(variable))
        {
          Ok(self.evaluate_assignment(assignment)?.to_owned())
        } else {
          Err(Error::Internal {
            message: format!("attempted to evaluate undefined variable `{variable}`"),
          })
        }
      }
    }
  }

  fn evaluate_condition(&mut self, condition: &Condition<'src>) -> RunResult<'src, bool> {
    let lhs_value = self.evaluate_expression(&condition.lhs)?;
    let rhs_value = self.evaluate_expression(&condition.rhs)?;
    let condition = match condition.operator {
      ConditionalOperator::Equality => lhs_value == rhs_value,
      ConditionalOperator::Inequality => lhs_value != rhs_value,
      ConditionalOperator::RegexMatch => Regex::new(&rhs_value)
        .map_err(|source| Error::RegexCompile { source })?
        .is_match(&lhs_value),
      ConditionalOperator::RegexMismatch => !Regex::new(&rhs_value)
        .map_err(|source| Error::RegexCompile { source })?
        .is_match(&lhs_value),
    };
    Ok(condition)
  }

  fn run_backtick(&self, raw: &str, token: &Token<'src>) -> RunResult<'src, String> {
    self
      .run_command(raw, &[])
      .map_err(|output_error| Error::Backtick {
        token: *token,
        output_error,
      })
  }

  pub(crate) fn run_command(&self, command: &str, args: &[&str]) -> Result<String, OutputError> {
    let mut cmd = self
      .context
      .module
      .settings
      .shell_command(self.context.config);

    cmd
      .arg(command)
      .args(args)
      .current_dir(self.context.working_directory())
      .export(
        &self.context.module.settings,
        self.context.dotenv,
        &self.scope,
        &self.context.module.unexports,
      )
      .stdin(Stdio::inherit())
      .stderr(if self.context.config.verbosity.quiet() {
        Stdio::null()
      } else {
        Stdio::inherit()
      })
      .stdout(Stdio::piped());

    let (result, caught) = cmd.output_guard();

    let output = result.map_err(OutputError::Io)?;

    OutputError::result_from_exit_status(output.status)?;

    let output = str::from_utf8(&output.stdout).map_err(OutputError::Utf8)?;

    if let Some(signal) = caught {
      return Err(OutputError::Interrupted(signal));
    }

    Ok(
      output
        .strip_suffix("\r\n")
        .or_else(|| output.strip_suffix("\n"))
        .unwrap_or(output)
        .into(),
    )
  }

  pub(crate) fn evaluate_line(
    &mut self,
    line: &Line<'src>,
    continued: bool,
  ) -> RunResult<'src, String> {
    let mut evaluated = String::new();
    for (i, fragment) in line.fragments.iter().enumerate() {
      match fragment {
        Fragment::Text { token } => {
          let lexeme = token.lexeme().replace("{{{{", "{{");

          if i == 0 && continued {
            evaluated += lexeme.trim_start();
          } else {
            evaluated += &lexeme;
          }
        }
        Fragment::Interpolation { expression } => {
          evaluated += &self.evaluate_expression(expression)?;
        }
      }
    }
    Ok(evaluated)
  }

  pub(crate) fn evaluate_parameters(
    context: &ExecutionContext<'src, 'run>,
    is_dependency: bool,
    arguments: &[String],
    parameters: &[Parameter<'src>],
  ) -> RunResult<'src, (Scope<'src, 'run>, Vec<String>)> {
    let mut evaluator = Self::new(context, is_dependency, context.scope);

    let mut positional = Vec::new();

    let mut rest = arguments;
    for parameter in parameters {
      let value = if rest.is_empty() {
        if let Some(ref default) = parameter.default {
          let value = evaluator.evaluate_expression(default)?;
          positional.push(value.clone());
          value
        } else if parameter.kind == ParameterKind::Star {
          String::new()
        } else {
          return Err(Error::Internal {
            message: "missing parameter without default".to_owned(),
          });
        }
      } else if parameter.kind.is_variadic() {
        for value in rest {
          positional.push(value.clone());
        }
        let value = rest.to_vec().join(" ");
        rest = &[];
        value
      } else {
        let value = rest[0].clone();
        positional.push(value.clone());
        rest = &rest[1..];
        value
      };
      evaluator.scope.bind(Binding {
        constant: false,
        export: parameter.export,
        file_depth: 0,
        name: parameter.name,
        private: false,
        value,
      });
    }

    Ok((evaluator.scope, positional))
  }

  pub(crate) fn new(
    context: &ExecutionContext<'src, 'run>,
    is_dependency: bool,
    scope: &'run Scope<'src, 'run>,
  ) -> Self {
    Self {
      assignments: None,
      context: *context,
      is_dependency,
      scope: scope.child(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  run_error! {
    name: backtick_code,
    src: "
      a:
       echo {{`f() { return 100; }; f`}}
    ",
    args: ["a"],
    error: Error::Backtick {
      token,
      output_error: OutputError::Code(code),
    },
    check: {
      assert_eq!(code, 100);
      assert_eq!(token.lexeme(), "`f() { return 100; }; f`");
    }
  }

  run_error! {
    name: export_assignment_backtick,
    src: r#"
      export exported_variable := "A"
      b := `echo $exported_variable`

      recipe:
        echo {{b}}
    "#,
    args: ["--quiet", "recipe"],
    error: Error::Backtick {
        token,
        output_error: OutputError::Code(_),
    },
    check: {
      assert_eq!(token.lexeme(), "`echo $exported_variable`");
    }
  }
}
