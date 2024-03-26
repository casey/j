use std::cell::RefCell;

use super::*;

pub(crate) struct RecipeContext<'src: 'run, 'run> {
  pub(crate) cache: &'run RefCell<JustfileCache>,
  pub(crate) config: &'run Config,
  pub(crate) scope: &'run Scope<'src, 'run>,
  pub(crate) search: &'run Search,
  pub(crate) settings: &'run Settings<'src>,
}
