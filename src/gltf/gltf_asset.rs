extern crate serde_json;
use serde_json::Value;

pub struct GltfAsset {
  pub version: u8,
  pub copyright: Option<String>,
  pub generator: Option<String>,
  pub min_version: Option<String>,
  
  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl GltfAsset {
  pub fn new() -> GltfAsset {
    GltfAsset {
      version: 2,
      copyright: None,
      generator: Some(String::from("r3dtiles v0.1")),
      min_version: None,
      extras: None,
      extensions: None
    }
  }
}