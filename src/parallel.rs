use crate::RunResult;
use crossbeam::thread;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

type ScopeResult<'src> = RunResult<'src, ()>;

pub(crate) struct TaskScope<'env, 'src, 'inner_scope> {
  inner: &'inner_scope thread::Scope<'env>,
  join_handles: Vec<thread::ScopedJoinHandle<'inner_scope, ScopeResult<'src>>>,
  parallel: bool,
}

impl<'env, 'src, 'inner_scope> TaskScope<'env, 'src, 'inner_scope> {
  /// run the given task, either directly synchronously or spawned in a background thread.
  pub(crate) fn run<'scope, F>(&'scope mut self, f: F) -> ScopeResult<'src>
  where
    'src: 'env,
    F: FnOnce() -> ScopeResult<'src>,
    F: Send + 'env,
  {
    if self.parallel {
      self.join_handles.push(self.inner.spawn(|_scope| f()));
      Ok(())
    } else {
      f()
    }
  }
}

/// task runner scope, based on `crossbeam::thread::scope`.
///
/// The `scope` object can be used to `.run` new tasks to be
/// executed. Depending on the `parallel` parameter, these are
/// directly run, or spawned in a background thread.
///
/// The first error will be returned as result of this `task_scope`.
///
/// Only works for tasks with an `RunResult<'src, ()>` result type.
pub(crate) fn task_scope<'env, 'src, F>(parallel: bool, f: F) -> ScopeResult<'src>
where
  F: for<'inner_scope> FnOnce(&mut TaskScope<'env, 'src, 'inner_scope>) -> ScopeResult<'src>,
{
  thread::scope(|scope| {
    let mut task_scope = TaskScope {
      parallel,
      inner: scope,
      join_handles: Vec::new(),
    };

    f(&mut task_scope)?;

    for handle in task_scope.join_handles {
      handle.join().expect("could not join thread")?;
    }
    Ok(())
  })
  .expect("could not join thread")
}

/// track which tasks were already run, across all running threads.
#[derive(Clone)]
pub(crate) struct Ran(Arc<Mutex<HashSet<Vec<String>>>>);

impl Ran {
  pub(crate) fn new() -> Self {
    Self(Arc::new(Mutex::new(HashSet::new())))
  }

  pub(crate) fn insert(&self, args: Vec<String>) {
    let mut ran = self.0.lock().unwrap();
    ran.insert(args);
  }

  pub(crate) fn contains(&self, args: &Vec<String>) -> bool {
    let ran = self.0.lock().unwrap();
    ran.contains(args)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ran_empty() {
    let r = Ran::new();
    assert!(!r.contains(&vec![]));
  }

  #[test]
  fn test_ran_insert_contains() {
    let r = Ran::new();
    r.insert(vec!["1".into(), "2".into(), "3".into()]);
    assert!(r.contains(&vec!["1".into(), "2".into(), "3".into()]));
  }
}
