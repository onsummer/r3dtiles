extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;

pub struct GltfTexture {
  pub sampler: Option<u32>,
  pub source: Option<u32>,
  pub name: Option<String>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}