use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TilesetAsset {
  pub version: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TileRefine {
  Add,
  Replace,
}

impl Default for TileRefine {
  fn default() -> Self {
    TileRefine::Add
  }
}

#[derive(Serialize, Deserialize)]
pub struct TilesetBoundingVolume {
  pub region: Option<[f32; 6]>,
}

// impl<'de> Deserialize<'de> for TilesetBoundingVolume {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//       D: serde::Deserializer<'de> {
//           // TODO
//   }
// }

#[derive(Serialize, Deserialize)]
pub struct Tile {
  #[serde(default)]
  pub refine: TileRefine,
  #[serde(rename = "boundingVolume")]
  pub bounding_volume: TilesetBoundingVolume,
  #[serde(rename = "geometricError")]
  pub geometric_error: u32,
  pub content: TilesetTileContent,
}

#[derive(Serialize, Deserialize)]
pub struct TilesetTileContent {
  uri: String
}

#[derive(Serialize, Deserialize)]
pub struct Tileset {
  pub asset: TilesetAsset,
  pub root: Tile
}
