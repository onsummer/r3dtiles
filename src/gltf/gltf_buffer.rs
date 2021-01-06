extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;

pub struct GltfBuffer {
  pub byte_length: u32,
  pub uri: Option<String>,
  pub name: Option<String>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}