mod gltf_asset;

mod gltf_buffer;
mod gltf_bufferview;
mod gltf_accessor;

mod gltf_scene;
mod gltf_node;
mod gltf_mesh;
mod gltf_primitive;

mod gltf_material;
mod gltf_sampler;
mod gltf_image;
mod gltf_texture;

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