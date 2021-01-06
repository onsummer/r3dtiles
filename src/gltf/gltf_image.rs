pub struct GltfImage {
  pub uri: Option<String>,
  pub bufferview: Option<u32>,
  pub mime_type: Option<GltfImageMimeType>,
  pub name: Option<String>,
  // pub extensions: Option<Map>,
  // pub extras: Option<Map<>>,
}

pub enum GltfImageMimeType {
  Png(String),
  Jpeg(String),
  Other(String)
}