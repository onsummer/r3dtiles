use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct TilesetMetadata {
  pub class: String,
  pub properties: Option<HashMap<String, Value>>,

  pub extension: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}
