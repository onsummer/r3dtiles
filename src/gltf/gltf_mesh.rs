extern crate serde_json;
use serde_json::Value;
use std::iter::Map;

use super::gltf_primitive::GltfPrimitive;

pub struct GltfMesh {
  pub primitives: Vec<GltfPrimitive>,
  pub weights: Option<Vec<u32>>,
  pub name: Option<String>,

  pub extensions: Option<Map<String, Value>>,
  pub extras: Option<Map<String, Value>>,
}