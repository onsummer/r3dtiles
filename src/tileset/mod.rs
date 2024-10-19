pub mod enums;
pub mod asset;
pub mod tile;
pub mod tile_content;
pub mod bounding_volume;

pub mod properties;

pub mod implicit_subtree;
pub mod implicit_tiling;
pub mod metadata;

use asset::TilesetAsset;
use tile::Tile;
use properties::TilesetProperties;

use metadata::TilesetMetadata;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Tileset {
  pub asset: TilesetAsset,
  #[serde(rename = "geometricError")]
  pub geometric_error: u32,
  pub root: Tile,

  #[serde(rename = "extensionUsed")]
  pub extension_used: Option<Vec<String>>,
  #[serde(rename = "extensionRequired")]
  pub extension_required: Option<Vec<String>>,

  pub groups: Option<u32>,
  pub metadata: Option<TilesetMetadata>,

  pub properties: Option<TilesetProperties>,

  pub extension: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>
}
