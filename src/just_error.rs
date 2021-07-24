use crate::common::*;

pub(crate) enum JustError<'src> {
  Search(SearchError),
  Compile(CompilationError<'src>),
  Load(LoadError),
  Code(i32),
  Run(RuntimeError<'src>),
}

// TODO: remove this impl
impl<'src> From<i32> for JustError<'src> {
  fn from(code: i32) -> Self {
    Self::Code(code)
  }
}

impl<'src> From<SearchError> for JustError<'src> {
  fn from(error: SearchError) -> Self {
    Self::Search(error)
  }
}

impl<'src> From<CompilationError<'src>> for JustError<'src> {
  fn from(error: CompilationError<'src>) -> Self {
    Self::Compile(error)
  }
}

impl<'src> From<RuntimeError<'src>> for JustError<'src> {
  fn from(error: RuntimeError<'src>) -> Self {
    Self::Run(error)
  }
}

impl<'src> From<LoadError> for JustError<'src> {
  fn from(error: LoadError) -> Self {
    Self::Load(error)
  }
}
