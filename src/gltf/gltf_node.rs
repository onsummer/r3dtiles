extern crate serde_json;
use serde_json::Value;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GltfNode {
  pub camera: Option<u32>,
  pub children: Option<Vec<u32>>,
  pub translation: Option<[f64; 3]>,
  pub scale: Option<[f64; 3]>,
  pub rotation: Option<[f64; 4]>,
  pub weights: Option<Vec<u32>>,
  pub mesh: Option<u32>,
  pub matrix: Option<[f64; 16]>,
  pub skin: Option<u32>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl GltfNode {
  pub fn new() -> GltfNode {
    GltfNode {
      camera: None,
      children: None,
      translation: None,
      scale: None,
      rotation: None,
      weights: None,
      mesh: None,
      matrix: None,
      skin: None,

      extensions: None,
      extras: None
    }
  }
}