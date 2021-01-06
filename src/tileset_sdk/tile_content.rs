extern crate serde_json;
use serde_json::Value;
use super::bounding_volume::BoundingVolume;

pub struct TileContent {
  pub bounding_volume: Option<BoundingVolume>,
  pub uri: String,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl TileContent {
  pub fn new() -> TileContent {
    TileContent {
      uri: String::from(""),
      bounding_volume: None,
      extras: None,
      extensions: None
    }
  }
}