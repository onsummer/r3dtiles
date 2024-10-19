use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct TilesetAsset {
  pub version: String,
  #[serde(rename = "tilesetVersion")]
  pub tileset_version: Option<String>,

  pub extension: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>
}
