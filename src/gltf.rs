pub mod gltf_asset;

pub mod gltf_buffer;
pub mod gltf_bufferview;
pub mod gltf_accessor;

pub mod gltf_scene;
pub mod gltf_node;
pub mod gltf_mesh;
pub mod gltf_primitive;

pub mod gltf_material;
pub mod gltf_sampler;
pub mod gltf_image;
pub mod gltf_texture;

pub struct Gltf {
  pub asset: gltf_asset::GltfAsset,

  pub buffers: Vec<gltf_buffer::GltfBuffer>,
  pub bufferviews: Vec<gltf_bufferview::GltfBufferView>,
  pub accessors: Vec<gltf_accessor::GltfAccessor>,

  pub scenes: Vec<gltf_scene::GltfScene>,
  pub nodes: Vec<gltf_node::GltfNode>,
  pub meshes: Vec<gltf_mesh::GltfMesh>,
  
  pub materials: Vec<gltf_material::GltfMaterial>,
  pub textures: Vec<gltf_texture::GltfTexture>,
  pub samplers: Vec<gltf_sampler::GltfSampler>,
  pub images: Vec<gltf_image::GltfImage>,

  pub extensions_used: Vec<String>,
  pub extensions_required: Vec<String>
}