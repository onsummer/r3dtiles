extern crate serde_json;
use serde_json::Value;
use std::iter::Map;

pub struct GltfBuffer {
  pub byte_length: u32,
  pub uri: Option<String>,
  pub name: Option<String>,

  pub extensions: Option<Map<String, Value>>,
  pub extras: Option<Map<String, Value>>,
}