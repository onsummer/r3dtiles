extern crate serde_json;
use serde_json::Value;
use std::iter::Map;

pub struct GltfTexture {
  pub sampler: Option<u32>,
  pub source: Option<u32>,
  pub name: Option<String>,

  pub extensions: Option<Map<String, Value>>,
  pub extras: Option<Map<String, Value>>,
}