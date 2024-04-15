use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JustfileCache {
  /// Only serialized for user debugging
  pub(crate) justfile_path: PathBuf,
  /// Only serialized for user debugging
  pub(crate) working_directory: PathBuf,

  pub(crate) recipes: HashMap<String, RecipeCache>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RecipeCache {
  /// The hash of the recipe body after evaluation
  pub(crate) body_hash: String,
  #[serde(skip)]
  /// Has the hash changed this run. Needed for nested cached dependencies.
  pub(crate) hash_changed: bool,
}

impl JustfileCache {
  fn new_empty(search: &Search) -> Self {
    Self {
      justfile_path: search.justfile.clone(),
      working_directory: search.working_directory.clone(),
      recipes: HashMap::new(),
    }
  }

  pub(crate) fn new<'run>(search: &Search) -> RunResult<'run, Self> {
    let cache_file = &search.cache_file;
    let this = if cache_file.exists() {
      let file_contents =
        fs::read_to_string(cache_file).map_err(|io_error| Error::CacheFileRead {
          cache_filename: cache_file.clone(),
          io_error,
        })?;
      // Ignore newer versions, incompatible old versions or corrupted cache files
      serde_json::from_str(&file_contents)
        .or(Err(()))
        .unwrap_or_else(|()| Self::new_empty(search))
    } else {
      Self::new_empty(search)
    };
    Ok(this)
  }

  pub(crate) fn insert_recipe(&mut self, name: String, body_hash: String) {
    self.recipes.insert(
      name,
      RecipeCache {
        body_hash,
        hash_changed: true,
      },
    );
  }

  pub(crate) fn save<'run>(&self, search: &Search) -> RunResult<'run, ()> {
    let cache = serde_json::to_string(self).map_err(|_| Error::Internal {
      message: format!("Failed to serialize cache: {self:?}"),
    })?;

    search
      .cache_file
      .parent()
      .ok_or_else(|| {
        io::Error::new(
          io::ErrorKind::Unsupported,
          format!(
            "Cannot create parent directory of {}",
            search.cache_file.display()
          ),
        )
      })
      .and_then(fs::create_dir_all)
      .and_then(|()| fs::write(&search.cache_file, cache))
      .map_err(|io_error| Error::CacheFileWrite {
        cache_filename: search.cache_file.clone(),
        io_error,
      })
  }
}
