use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::{
  enums,
  tile_content::TilesetTileContent,
  bounding_volume::TilesetBoundingVolume,
  implicit_tiling::TilesetImplicitTiling,
  metadata::TilesetMetadata,
};

#[derive(Serialize, Deserialize)]
pub struct Tile {
  #[serde(default)]
  pub refine: enums::TileRefine,
  #[serde(rename = "boundingVolume")]
  pub bounding_volume: TilesetBoundingVolume,
  #[serde(rename = "viewerRequestVolume")]
  pub viewer_request_volume: Option<TilesetBoundingVolume>,
  #[serde(rename = "geometricError")]
  pub geometric_error: u32,
  pub content: Option<TilesetTileContent>,
  pub contents: Option<Vec<TilesetTileContent>>,
  pub transform: Option<Vec<f32>>,
  pub children: Option<Vec<Tile>>,

  #[serde(rename = "implicitTiling")]
  pub implicit_tiling: Option<TilesetImplicitTiling>,
  pub metadata: Option<TilesetMetadata>,

  pub extension: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}
