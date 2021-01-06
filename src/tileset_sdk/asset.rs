extern crate serde_json;
use serde_json::Value;

pub struct Asset {
  pub version: String,
  pub tileset_version: Option<String>,
  
  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl Asset {
  pub fn new() -> Asset {
    Asset {
      version: String::from("1.0"),
      tileset_version: None,
      extensions: None,
      extras: None
    }
  }
}