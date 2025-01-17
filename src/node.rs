use super::*;

/// Methods common to all AST nodes. Currently only used in parser unit tests.
pub(crate) trait Node<'src> {
  /// Construct an untyped tree of atoms representing this Node. This function,
  /// and `Tree` type, are only used in parser unit tests.
  fn tree(&self) -> Tree<'src>;
}

impl<'src> Node<'src> for Ast<'src> {
  fn tree(&self) -> Tree<'src> {
    Tree::atom("justfile")
      .extend(self.items.iter().map(Node::tree))
      .extend(self.warnings.iter().map(Node::tree))
  }
}

impl<'src> Node<'src> for Item<'src> {
  fn tree(&self) -> Tree<'src> {
    match self {
      Self::Alias(alias) => alias.tree(),
      Self::Assignment(assignment) => assignment.tree(),
      Self::Comment(comment) => comment.tree(),
      Self::Import {
        relative, optional, ..
      } => {
        let mut tree = Tree::atom("import");

        if *optional {
          tree = tree.push("?");
        }

        tree.push(format!("{relative}"))
      }
      Self::Module {
        name,
        optional,
        relative,
        ..
      } => {
        let mut tree = Tree::atom("mod");

        if *optional {
          tree = tree.push("?");
        }

        tree = tree.push(name.lexeme());

        if let Some(relative) = relative {
          tree = tree.push(format!("{relative}"));
        }

        tree
      }
      Self::Recipe(recipe) => recipe.tree(),
      Self::Set(set) => set.tree(),
      Self::Unexport { name } => {
        let mut unexport = Tree::atom(Keyword::Unexport.lexeme());
        unexport.push_mut(name.lexeme().replace('-', "_"));
        unexport
      }
    }
  }
}

impl<'src> Node<'src> for Alias<'src, Name<'src>> {
  fn tree(&self) -> Tree<'src> {
    Tree::atom(Keyword::Alias.lexeme())
      .push(self.name.lexeme())
      .push(self.target.lexeme())
  }
}

impl<'src> Node<'src> for Assignment<'src> {
  fn tree(&self) -> Tree<'src> {
    if self.export {
      Tree::atom("assignment")
        .push("#")
        .push(Keyword::Export.lexeme())
    } else {
      Tree::atom("assignment")
    }
    .push(self.name.lexeme())
    .push(self.value.tree())
  }
}

impl<'src> Node<'src> for Expression<'src> {
  fn tree(&self) -> Tree<'src> {
    match self {
      Self::And { lhs, rhs } => Tree::atom("&&").push(lhs.tree()).push(rhs.tree()),
      Self::Assert {
        condition: Condition { lhs, rhs, operator },
        error,
      } => Tree::atom(Keyword::Assert.lexeme())
        .push(lhs.tree())
        .push(operator.to_string())
        .push(rhs.tree())
        .push(error.tree()),
      Self::Backtick { contents, .. } => Tree::atom("backtick").push(Tree::string(contents)),
      Self::Call { thunk } => {
        use Thunk::*;
        let mut tree = Tree::atom("call");
        match thunk {
          Nullary { name, .. } => tree.push_mut(name.lexeme()),
          Unary { name, arg, .. } => {
            tree.push_mut(name.lexeme());
            tree.push_mut(arg.tree());
          }
          UnaryOpt {
            name, args: (a, b), ..
          } => {
            tree.push_mut(name.lexeme());
            tree.push_mut(a.tree());
            if let Some(b) = b.as_ref() {
              tree.push_mut(b.tree());
            }
          }
          UnaryPlus {
            name,
            args: (a, rest),
            ..
          } => {
            tree.push_mut(name.lexeme());
            tree.push_mut(a.tree());
            for arg in rest {
              tree.push_mut(arg.tree());
            }
          }
          Binary {
            name, args: [a, b], ..
          } => {
            tree.push_mut(name.lexeme());
            tree.push_mut(a.tree());
            tree.push_mut(b.tree());
          }
          BinaryPlus {
            name,
            args: ([a, b], rest),
            ..
          } => {
            tree.push_mut(name.lexeme());
            tree.push_mut(a.tree());
            tree.push_mut(b.tree());
            for arg in rest {
              tree.push_mut(arg.tree());
            }
          }
          Ternary {
            name,
            args: [a, b, c],
            ..
          } => {
            tree.push_mut(name.lexeme());
            tree.push_mut(a.tree());
            tree.push_mut(b.tree());
            tree.push_mut(c.tree());
          }
        }
        tree
      }
      Self::Concatenation { lhs, rhs } => Tree::atom("+").push(lhs.tree()).push(rhs.tree()),
      Self::Conditional {
        condition: Condition { lhs, rhs, operator },
        then,
        otherwise,
      } => {
        let mut tree = Tree::atom(Keyword::If.lexeme());
        tree.push_mut(lhs.tree());
        tree.push_mut(operator.to_string());
        tree.push_mut(rhs.tree());
        tree.push_mut(then.tree());
        tree.push_mut(otherwise.tree());
        tree
      }
      Self::Group { contents } => Tree::List(vec![contents.tree()]),
      Self::Join { lhs: None, rhs } => Tree::atom("/").push(rhs.tree()),
      Self::Join {
        lhs: Some(lhs),
        rhs,
      } => Tree::atom("/").push(lhs.tree()).push(rhs.tree()),
      Self::Or { lhs, rhs } => Tree::atom("||").push(lhs.tree()).push(rhs.tree()),
      Self::StringLiteral {
        string_literal: StringLiteral { cooked, .. },
      } => Tree::string(cooked),
      Self::Variable { name } => Tree::atom(name.lexeme()),
    }
  }
}

impl<'src> Node<'src> for UnresolvedRecipe<'src> {
  fn tree(&self) -> Tree<'src> {
    let mut t = Tree::atom("recipe");

    if self.quiet {
      t.push_mut("#");
      t.push_mut("quiet");
    }

    if let Some(doc) = &self.doc {
      t.push_mut(Tree::string(doc));
    }

    t.push_mut(self.name.lexeme());

    if !self.parameters.is_empty() {
      let mut params = Tree::atom("params");

      for parameter in &self.parameters {
        if let Some(prefix) = parameter.kind.prefix() {
          params.push_mut(prefix);
        }

        params.push_mut(parameter.tree());
      }

      t.push_mut(params);
    }

    if !self.dependencies.is_empty() {
      let mut dependencies = Tree::atom("deps");
      let mut subsequents = Tree::atom("sups");

      for (i, dependency) in self.dependencies.iter().enumerate() {
        let mut d = Tree::atom(dependency.recipe.lexeme());

        for argument in &dependency.arguments {
          d.push_mut(argument.tree());
        }

        if i < self.priors {
          dependencies.push_mut(d);
        } else {
          subsequents.push_mut(d);
        }
      }

      if let Tree::List(_) = dependencies {
        t.push_mut(dependencies);
      }

      if let Tree::List(_) = subsequents {
        t.push_mut(subsequents);
      }
    }

    if !self.body.is_empty() {
      t.push_mut(Tree::atom("body").extend(self.body.iter().map(Node::tree)));
    }

    t
  }
}

impl<'src> Node<'src> for Parameter<'src> {
  fn tree(&self) -> Tree<'src> {
    let mut children = vec![Tree::atom(self.name.lexeme())];

    if let Some(default) = &self.default {
      children.push(default.tree());
    }

    Tree::List(children)
  }
}

impl<'src> Node<'src> for Line<'src> {
  fn tree(&self) -> Tree<'src> {
    Tree::list(self.fragments.iter().map(Node::tree))
  }
}

impl<'src> Node<'src> for Fragment<'src> {
  fn tree(&self) -> Tree<'src> {
    match self {
      Self::Text { token } => Tree::string(token.lexeme()),
      Self::Interpolation { expression } => Tree::List(vec![expression.tree()]),
    }
  }
}

impl<'src> Node<'src> for Set<'src> {
  fn tree(&self) -> Tree<'src> {
    let mut set = Tree::atom(Keyword::Set.lexeme());
    set.push_mut(self.name.lexeme().replace('-', "_"));

    match &self.value {
      Setting::AllowDuplicateRecipes(value)
      | Setting::AllowDuplicateVariables(value)
      | Setting::DotenvLoad(value)
      | Setting::DotenvRequired(value)
      | Setting::Export(value)
      | Setting::Fallback(value)
      | Setting::NoExitMessage(value)
      | Setting::PositionalArguments(value)
      | Setting::Quiet(value)
      | Setting::Unstable(value)
      | Setting::WindowsPowerShell(value)
      | Setting::IgnoreComments(value) => {
        set.push_mut(value.to_string());
      }
      Setting::ScriptInterpreter(Interpreter { command, arguments })
      | Setting::Shell(Interpreter { command, arguments })
      | Setting::WindowsShell(Interpreter { command, arguments }) => {
        set.push_mut(Tree::string(&command.cooked));
        for argument in arguments {
          set.push_mut(Tree::string(&argument.cooked));
        }
      }
      Setting::DotenvFilename(value)
      | Setting::DotenvPath(value)
      | Setting::Tempdir(value)
      | Setting::WorkingDirectory(value) => {
        set.push_mut(Tree::string(&value.cooked));
      }
    }

    set
  }
}

impl<'src> Node<'src> for Warning {
  fn tree(&self) -> Tree<'src> {
    unreachable!()
  }
}

impl<'src> Node<'src> for str {
  fn tree(&self) -> Tree<'src> {
    Tree::atom("comment").push(["\"", self, "\""].concat())
  }
}
