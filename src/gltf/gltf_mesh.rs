use super::gltf_primitive::GltfPrimitive;

pub struct GltfMesh {
  pub primitives: Vec<GltfPrimitive>,
  pub weights: Option<Vec<u32>>,
  pub name: Option<String>,
  // pub extensions: Option<Map>,
  // pub extras: Option<Map<>>,
}