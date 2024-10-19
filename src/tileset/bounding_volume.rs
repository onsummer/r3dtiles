use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct TilesetBoundingVolume {
  #[serde(rename = "box")]
  pub bounding_box: Option<[f32; 12]>,
  pub region: Option<[f32; 6]>,
  pub sphere: Option<[f32; 4]>,

  pub extension: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>
}

// TODO

// impl<'de> Deserialize<'de> for TilesetBoundingVolume {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//       D: serde::Deserializer<'de> {
//           // TODO
//   }
// }
