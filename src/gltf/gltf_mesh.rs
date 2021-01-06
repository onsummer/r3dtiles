extern crate serde_json;
use serde_json::Value;

use super::gltf_primitive::GltfPrimitive;

pub struct GltfMesh {
  pub primitives: Vec<GltfPrimitive>,
  pub weights: Option<Vec<u32>>,
  pub name: Option<String>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl GltfMesh {
  pub fn new() -> GltfMesh {
    GltfMesh {
      primitives: Vec::new() as Vec<GltfPrimitive>,
      weights: None,
      name: None,

      extensions: None,
      extras: None
    }
  }
}