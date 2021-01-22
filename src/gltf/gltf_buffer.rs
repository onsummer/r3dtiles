extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GltfBuffer {
  pub byte_length: u32,
  pub uri: Option<String>,
  pub name: Option<String>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}

impl GltfBuffer {
  pub fn new(
    byte_length: u32
  ) -> GltfBuffer {
    GltfBuffer {
      byte_length: byte_length,
      uri: None,
      name: None,
      extensions: None,
      extras: None
    }
  }
}