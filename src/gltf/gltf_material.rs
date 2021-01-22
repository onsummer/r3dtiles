extern crate serde_json;
use serde_json::Value;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl GltfMaterial {
  pub fn new() -> GltfMaterial {
    GltfMaterial {
      alpha_cutoff: Some(0.0),
      alpha_mode: Some(GltfAlphaMode::Opaque),
      double_sided: Some(false),

      pbr_metallic_roughness: None,

      emissive_factor: Some([0.0, 0.0, 0.0]),
      normal_texture: None,
      emissive_texture: None,
      occlusion_texture: None,

      name: None,

      extensions: None,
      extras: None
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GltfAlphaMode {
  Opaque,
  Mask,
  Blend
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GltfPbr {
  pub basecolor_factor: Option<[f64; 4]>,
  pub basecolor_texture: Option<GltfTextureInfo>,
  pub metallic_factor: Option<f64>,
  pub roughness_factor: Option<f64>,
  pub metallic_roughness_rtexture: Option<GltfTextureInfo>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl GltfPbr {
  pub fn new() -> GltfPbr {
    GltfPbr {
      basecolor_factor: None,
      basecolor_texture: None,
      metallic_factor: None,
      metallic_roughness_rtexture: None,
      roughness_factor: None,

      extensions: None,
      extras: None
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GltfNormalTextureInfo {
  pub index: u32,
  pub scale: Option<f64>,
  pub tex_coord: Option<u32>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl GltfNormalTextureInfo {
  pub fn new(
    texture_index: u32
  ) -> GltfNormalTextureInfo {
    GltfNormalTextureInfo {
      index: texture_index,
      scale: None,
      tex_coord: None,
      extensions: None,
      extras: None
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GltfOcclusionTextureInfo {
  pub index: u32,
  pub strength: Option<f64>,
  pub tex_coord: Option<u32>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl GltfOcclusionTextureInfo {
  pub fn new(
    texture_index: u32
  ) -> GltfOcclusionTextureInfo {
    GltfOcclusionTextureInfo {
      index: texture_index,
      strength: None,
      tex_coord: None,
      extensions: None,
      extras: None
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GltfTextureInfo {
  pub index: u32,
  pub tex_coord: Option<u32>,
}

impl GltfTextureInfo {
  pub fn new(
    texture_index: u32
  ) -> GltfTextureInfo {
    GltfTextureInfo {
      index: texture_index,
      tex_coord: None,
    }
  }
}