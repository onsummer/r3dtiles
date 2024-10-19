use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{bounding_volume::TilesetBoundingVolume, metadata::TilesetMetadata};

#[derive(Serialize, Deserialize)]
pub struct TilesetTileContent {
  pub uri: String,
  pub group: Option<u32>,
  #[serde(rename = "boundingVolume")]
  pub bounding_volume: Option<TilesetBoundingVolume>,
  pub metadata: Option<TilesetMetadata>,

  pub extension: Option<HashMap<String, serde_json::Value>>,
  pub extras: Option<HashMap<String, serde_json::Value>>,
}
