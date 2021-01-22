extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;

extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GltfPrimitive {
  pub attributes: HashMap<String, u32>,
  pub indices: Option<u32>,

  pub material: Option<u32>,

  pub mode: Option<GltfPrimitiveMode>,

  pub targets: Option<Vec<HashMap<String, u32>>>,

  pub name: Option<String>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GltfPrimitiveMode {
  Points(u8),
  Lines(u8),
  LineLoop(u8),
  LineStrip(u8),
  Triangles(u8),
  TrianglesStrip(u8),
  TrianglesFan(u8),
}

impl GltfPrimitive {
  pub fn new() -> GltfPrimitive {
    GltfPrimitive {
      attributes: HashMap::new() as HashMap<String, u32>,
      material: None,
      indices: None,
      targets: None,
      name: None,
      mode: Some(GltfPrimitiveMode::Triangles(4)),

      extras: None,
      extensions: None
    }
  }
}