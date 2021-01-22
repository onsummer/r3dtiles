extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GltfImage {
  pub uri: Option<String>,
  pub bufferview: Option<u32>,
  pub mime_type: Option<GltfImageMimeType>,
  pub name: Option<String>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GltfImageMimeType {
  Png(String),
  Jpeg(String),
  Other(String)
}

impl GltfImage {
  pub fn new() -> GltfImage {
    GltfImage {
      uri: None,
      bufferview: None,
      mime_type: None,
      name: None,

      extensions: None,
      extras: None
    }
  }
}