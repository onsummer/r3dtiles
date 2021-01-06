extern crate serde_json;
use serde_json::Value;

pub struct GltfSampler {
  pub mag_filter: Option<GltfMagnificationFilter>,
  pub min_filter: Option<GltfMinificationFilter>,
  pub wrap_s: Option<GltfWrapType>,
  pub wrap_t: Option<GltfWrapType>,
  pub name: Option<String>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl GltfSampler {
  pub fn new() -> GltfSampler {
    GltfSampler {
      mag_filter: None,
      min_filter: None,
      wrap_s: None,
      wrap_t: None,
      name: None,

      extensions: None,
      extras: None
    }
  }
}

pub enum GltfMagnificationFilter {
  Nearest(u16),
  Linear(u16),
  Other(u16)
}
pub enum GltfMinificationFilter {
  Nearest(u16),
  Linear(u16),
  NearestMipmapNearest(u16),
  LinearMipmapNearest(u16),
  NearestMipmapLinear(u16),
  LinearMipmapLinear(u16),
  Other(u16)
}

pub enum GltfWrapType {
  ClampToEdge(u16),
  MirroredRepeat(u16),
  Repeat(u16),
  Other(u16)
}