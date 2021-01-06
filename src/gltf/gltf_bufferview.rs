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