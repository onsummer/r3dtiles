extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;

pub struct GltfBufferView {
  pub buffer: u32,
  pub byte_length: u32,
  pub byte_offset: Option<u32>,
  pub target: Option<GltfBufferViewTarget>,
  pub byte_stride: Option<u32>,
  pub name: Option<String>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}

pub enum GltfBufferViewTarget {
  ArrayBuffer(u16),
  ElementArrayBuffer(u16),
  Other(u16)
}

impl GltfBufferView {
  pub fn new(
    buffer_index: u32,
    byte_length: u32
  ) -> GltfBufferView {
    GltfBufferView {
      buffer: buffer_index,
      byte_length: byte_length,
      byte_offset: Some(0),
      // target: Some(GltfBufferViewTarget::ArrayBuffer(5123)),
      target: None,
      byte_stride: Some(0),
      name: None,
      extensions: None,
      extras: None
    }
  }
}