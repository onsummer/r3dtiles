extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;

pub struct GltfAsset {
  pub version: u8,
  pub copyright: Option<String>,
  pub generator: Option<String>,
  pub min_version: Option<String>,
  
  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}