use crate::common::*;

pub(crate) struct Loader {
  src: Arena<String>,
}

impl Loader {
  pub(crate) fn new() -> Self {
    Loader { src: Arena::new() }
  }

  pub(crate) fn load<'src>(&'src self, path: &Path) -> LoadResult<&'src str> {
    let src = fs::read_to_string(path).map_err(|io_error| LoadError {
      path: path.to_owned(),
      io_error,
    })?;
    Ok(self.src.alloc(src))
  }
}
