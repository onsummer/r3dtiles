extern crate serde_json;
use serde_json::Value;

pub struct GltfTexture {
  pub sampler: Option<u32>,
  pub source: Option<u32>,
  pub name: Option<String>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl GltfTexture {
  pub fn new() -> GltfTexture {
    GltfTexture {
      sampler: None,
      source: None,
      name: None,
      extensions: None,
      extras: None
    }
  }
}