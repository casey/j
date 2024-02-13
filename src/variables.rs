use super::*;

pub(crate) struct Variables<'expression, 'src> {
  walker: ExpressionWalker<'expression, 'src>,
}

impl<'expression, 'src> Variables<'expression, 'src> {
  pub(crate) fn new(root: &'expression Expression<'src>) -> Variables<'expression, 'src> {
    Variables {
      walker: root.walk(),
    }
  }
}

impl<'expression, 'src> Iterator for Variables<'expression, 'src> {
  type Item = Token<'src>;

  fn next(&mut self) -> Option<Token<'src>> {
    loop {
      match self.walker.next()? {
        Expression::Variable { name } => return Some(name.token),
        _ => continue,
      }
    }
  }
}
