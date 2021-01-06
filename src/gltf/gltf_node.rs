extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;

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

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}