use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JustfileCache {
  pub(crate) recipe_caches: HashMap<String, RecipeCache>,
}

impl JustfileCache {
  pub(crate) fn new() -> Self {
    Self {
      recipe_caches: HashMap::new(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RecipeCache {
  pub(crate) hash: String,
}
