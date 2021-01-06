pub mod tile;
pub mod asset;
pub mod tileset_properties;
pub mod tile_content;
pub mod bounding_volume;
pub mod tile_refine;

extern crate serde_json;
use serde_json::Value;

pub struct Tileset {
  pub asset: asset::Asset,
  pub root: tile::Tile,
  pub geometric_error: f32,
  pub extensions_used: Option<Vec<String>>,
  pub extensions_required: Option<Vec<String>>,
  pub extensions: Option<Value>,
  pub extras: Option<Value>,
  pub properties: Option<tileset_properties::TilesetProperties>
}

impl Tileset {
  pub fn new() -> Tileset {
    Tileset {
      asset: asset::Asset::new(),
      root: tile::Tile::new(),
      geometric_error: 0.0,
      extensions: None,
      extras: None,
      extensions_used: Some(Vec::new() as Vec<String>),
      extensions_required: Some(Vec::new() as Vec<String>),
      properties: None
    }
  }
}