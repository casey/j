use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::*;

/// The version of the justfile as it is on disk. Newer cache formats are added
/// as new variants.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "version")]
pub(crate) enum JustfileCacheSerialized {
  #[serde(rename = "unstable-1")]
  Unstable1(JustfileCacheUnstable1),
}

pub(crate) type JustfileCacheUnstable1 = JustfileCache;

/// The runtime cache format. It should be the intersection of all supported
/// serialized versions, i.e. you can convert any supported version to this.
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
  pub(crate) body_hash: String,
  #[serde(skip)]
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
    let this = if !cache_file.exists() {
      Self::new_empty(search)
    } else {
      let file_contents = fs::read_to_string(&cache_file).or_else(|io_error| {
        Err(Error::CacheFileRead {
          cache_filename: cache_file.clone(),
          io_error,
        })
      })?;
      // Ignore newer versions, incompatible old versions or corrupted cache files
      serde_json::from_str(&file_contents)
        .or(Err(()))
        .and_then(|serialized: JustfileCacheSerialized| serialized.try_into())
        .unwrap_or_else(|_| Self::new_empty(search))
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

  pub(crate) fn save<'run>(self, search: &Search) -> RunResult<'run, ()> {
    let cache: JustfileCacheSerialized = self.into();
    let cache = serde_json::to_string(&cache).or_else(|_| {
      Err(Error::Internal {
        message: format!("Failed to serialize cache: {cache:?}"),
      })
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
      .and_then(|parent| fs::create_dir_all(parent))
      .and_then(|_| fs::write(&search.cache_file, cache))
      .or_else(|io_error| {
        Err(Error::CacheFileWrite {
          cache_filename: search.cache_file.clone(),
          io_error,
        })
      })
  }
}

impl From<JustfileCache> for JustfileCacheSerialized {
  fn from(value: JustfileCache) -> Self {
    JustfileCacheSerialized::Unstable1(value)
  }
}

impl TryFrom<JustfileCacheSerialized> for JustfileCache {
  type Error = ();

  fn try_from(value: JustfileCacheSerialized) -> Result<Self, Self::Error> {
    match value {
      JustfileCacheSerialized::Unstable1(unstable1) => Ok(unstable1),
    }
  }
}
