use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JustfileCache {
  /// Only serialized for user debugging
  pub(crate) justfile_path: PathBuf,
  pub(crate) working_directory: PathBuf,
  pub(crate) recipes: HashMap<String, RecipeCache>,
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RecipeCache {
  pub(crate) body_hash: String,
}
