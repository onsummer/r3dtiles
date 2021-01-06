extern crate serde_json;
use serde_json::Value;
use std::collections::HashMap;

pub struct GltfMaterial {
  pub alpha_cutoff: Option<f64>,
  pub alpha_mode: Option<GltfAlphaMode>,
  pub double_sided: Option<bool>,

  pub pbr_metallic_roughness: Option<GltfPbr>,

  pub emissive_factor: Option<[f64; 3]>,
  pub normal_texture: Option<GltfNormalTextureInfo>,
  pub emissive_texture: Option<GltfTextureInfo>,
  pub occlusion_texture: Option<GltfOcclusionTextureInfo>,

  pub name: Option<String>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}

pub enum GltfAlphaMode {
  Opaque,
  Mask,
  Blend
}

pub struct GltfPbr {
  pub basecolor_factor: Option<[f64; 4]>,
  pub basecolor_texture: Option<GltfTextureInfo>,
  pub metallic_factor: Option<f64>,
  pub roughness_factor: Option<f64>,
  pub metallic_roughness_rtexture: Option<GltfTextureInfo>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}

pub struct GltfNormalTextureInfo {
  pub index: u32,
  pub scale: Option<f64>,
  pub tex_coord: Option<u32>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}

pub struct GltfOcclusionTextureInfo {
  pub index: u32,
  pub strength: Option<f64>,
  pub tex_coord: Option<u32>,

  pub extensions: Option<HashMap<String, Value>>,
  pub extras: Option<HashMap<String, Value>>,
}

pub struct GltfTextureInfo {
  pub index: u32,
  pub tex_coord: Option<u32>,
}