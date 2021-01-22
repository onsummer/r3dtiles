extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GltfScene {
  pub nodes: Vec<u32>,
  pub name: Option<String>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}

impl GltfScene {
  pub fn new() -> GltfScene {
    GltfScene {
      nodes: Vec::new() as Vec<u32>,
      name: None,
      extras: None,
      extensions: None
    }
  }
}