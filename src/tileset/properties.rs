use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct TilesetProperties {
  pub maximum: f64,
  pub minimum: f64,

  pub extension: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}
