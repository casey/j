use super::*;

pub(crate) struct ExpressionWalker<'expression, 'src> {
  stack: Vec<&'expression Expression<'src>>,
}

impl<'expression, 'src> ExpressionWalker<'expression, 'src> {
  pub(crate) fn new(root: &'expression Expression<'src>) -> ExpressionWalker<'expression, 'src> {
    ExpressionWalker { stack: vec![root] }
  }
}

impl<'expression, 'src> Iterator for ExpressionWalker<'expression, 'src> {
  type Item = &'expression Expression<'src>;

  fn next(&mut self) -> Option<Self::Item> {
    let top = self.stack.pop()?;

    match top {
      Expression::StringLiteral { .. }
      | Expression::Variable { .. }
      | Expression::Backtick { .. } => {}
      Expression::Call { thunk } => match thunk {
        Thunk::Nullary { .. } => {}
        Thunk::Unary { arg, .. } => self.stack.push(arg),
        Thunk::UnaryOpt {
          args: (a, opt_b), ..
        } => {
          self.stack.push(a);
          if let Some(b) = opt_b.as_ref() {
            self.stack.push(b);
          }
        }
        Thunk::Binary { args, .. } => {
          for arg in args.iter().rev() {
            self.stack.push(arg);
          }
        }
        Thunk::BinaryPlus {
          args: ([a, b], rest),
          ..
        } => {
          let first: &[&Expression] = &[a, b];
          for arg in first.iter().copied().chain(rest).rev() {
            self.stack.push(arg);
          }
        }
        Thunk::Ternary { args, .. } => {
          for arg in args.iter().rev() {
            self.stack.push(arg);
          }
        }
      },
      Expression::Conditional {
        lhs,
        rhs,
        then,
        otherwise,
        ..
      } => {
        self.stack.push(otherwise);
        self.stack.push(then);
        self.stack.push(rhs);
        self.stack.push(lhs);
      }
      Expression::Concatenation { lhs, rhs } => {
        self.stack.push(rhs);
        self.stack.push(lhs);
      }
      Expression::Join { lhs, rhs } => {
        self.stack.push(rhs);
        if let Some(lhs) = lhs {
          self.stack.push(lhs);
        }
      }
      Expression::Group { contents } => {
        self.stack.push(contents);
      }
    }

    Some(top)
  }
}
