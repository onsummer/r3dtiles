use serde::{Deserialize, Serialize};

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
#[serde(rename_all = "UPPERCASE")]
pub enum TilesetImplicitSubdivisionScheme {
  Quadtree,
  Octree,
}
