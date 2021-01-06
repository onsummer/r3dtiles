extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;

pub struct GltfImage {
  pub uri: Option<String>,
  pub bufferview: Option<u32>,
  pub mime_type: Option<GltfImageMimeType>,
  pub name: Option<String>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}

pub enum GltfImageMimeType {
  Png(String),
  Jpeg(String),
  Other(String)
}