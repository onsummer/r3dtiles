extern crate serde_json;
use serde_json::Value;
use std::iter::Map;

pub struct GltfAsset {
  pub version: i32,
  pub copyright: Option<String>,
  pub generator: Option<String>,
  pub min_version: Option<String>,
  
  pub extensions: Option<Map<String, Value>>,
  pub extras: Option<Map<String, Value>>,
}