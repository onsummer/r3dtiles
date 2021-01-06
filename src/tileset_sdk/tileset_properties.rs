extern crate serde_json;
use serde_json::Value;

pub struct TilesetProperties {
  pub maximum: f64,
  pub minimum: f64,
  
  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl TilesetProperties {
  pub fn new() -> TilesetProperties {
    TilesetProperties {
      maximum: 0.0,
      minimum: 0.0,
      extras: None,
      extensions: None
    }
  }
}