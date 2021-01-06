extern crate serde_json;
use serde_json::Value;
use std::iter::Map;

pub struct GltfImage {
  pub uri: Option<String>,
  pub bufferview: Option<u32>,
  pub mime_type: Option<GltfImageMimeType>,
  pub name: Option<String>,

  pub extensions: Option<Map<String, Value>>,
  pub extras: Option<Map<String, Value>>,
}

pub enum GltfImageMimeType {
  Png(String),
  Jpeg(String),
  Other(String)
}