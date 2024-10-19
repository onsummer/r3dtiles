use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
  enums::TilesetImplicitSubdivisionScheme,
  implicit_subtree::TilesetImplicitSubtree,
};

#[derive(Serialize, Deserialize)]
pub struct TilesetImplicitTiling {
  #[serde(rename = "subdivisionScheme")]
  pub subdivision_scheme: TilesetImplicitSubdivisionScheme,
  #[serde(rename = "subtreeLevels")]
  pub subtree_levels: u32,
  #[serde(rename = "availableLevels")]
  pub available_levels: u32,
  pub subtrees: TilesetImplicitSubtree,

  pub extension: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>
}