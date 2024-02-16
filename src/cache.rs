use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JustfileCache {
  /// Only serialized for user debugging
  pub(crate) working_directory: PathBuf,
  pub(crate) recipe_caches: HashMap<String, RecipeCache>,
}

impl JustfileCache {
  pub(crate) fn new(working_directory: PathBuf) -> Self {
    Self {
      working_directory,
      recipe_caches: HashMap::new(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RecipeCache {
  pub(crate) hash: String,
}
