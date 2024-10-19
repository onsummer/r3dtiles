use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TilesetImplicitSubtree {
  uri: String,

  pub extension: Option<HashMap<String, serde_json::Value>>,
  pub extras: Option<HashMap<String, serde_json::Value>>
}
