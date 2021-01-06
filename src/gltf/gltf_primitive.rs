use std::iter::Map;

pub struct GltfPrimitive {
  pub attributes: Map<String, u32>,
  pub indices: Option<u32>,
  pub material: Option<u32>,
  pub mode: Option<GltfPrimitiveMode>,
  pub targets: Option<Vec<Map<String, u32>>>,
  pub name: Option<String>,
  // pub extensions: Option<Map>,
  // pub extras: Option<Map<>>,
}

pub enum GltfPrimitiveMode {
  Points(u8),
  Lines(u8),
  LineLoop(u8),
  LineStrip(u8),
  Triangles(u8),
  TrianglesStrip(u8),
  TrianglesFan(u8),
}