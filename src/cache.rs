use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "version")]
pub(crate) enum JustfileCacheSerialized {
  #[serde(rename = "unstable-1")]
  Unstable1(JustfileCacheUnstable1),
}

pub(crate) type JustfileCacheUnstable1 = JustfileCache;

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
}

impl JustfileCache {
  pub(crate) fn new(search: &Search) -> Self {
    Self {
      justfile_path: search.justfile.clone(),
      working_directory: search.working_directory.clone(),
      recipes: HashMap::new(),
    }
  }
}

impl From<JustfileCache> for JustfileCacheSerialized {
  fn from(value: JustfileCache) -> Self {
    JustfileCacheSerialized::Unstable1(value)
  }
}

impl TryFrom<JustfileCacheSerialized> for JustfileCache {
  type Error = Error<'static>;

  fn try_from(value: JustfileCacheSerialized) -> Result<Self, Self::Error> {
    match value {
      JustfileCacheSerialized::Unstable1(unstable1) => Ok(unstable1),
    }
  }
}
