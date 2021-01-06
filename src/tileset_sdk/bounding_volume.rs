extern crate serde_json;
use serde_json::Value;

pub struct BoundingVolume {
  pub r#box: Option<[f32; 12]>,
  pub region: Option<[f32; 6]>,
  pub sphere: Option<[f32; 4]>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl BoundingVolume {
  pub fn new() -> BoundingVolume {
    BoundingVolume {
      r#box: None,
      region: None,
      sphere: None,
      extensions: None,
      extras: None
    }
  }
}