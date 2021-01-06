extern crate r3dtiles;
use std::collections::HashMap;
use r3dtiles::gltf::gltf_primitive::{ GltfPrimitive, GltfPrimitiveMode };

fn main() {
  let mut contacts = HashMap::new();
  contacts.insert(String::from("POSITION"), 0);

  let mut prmt = GltfPrimitive {
    attributes: contacts,
    mode: Some(GltfPrimitiveMode::Triangles(4u8)),
    name: Some(String::from("test primitives")),
    indices: Some(1),
    material: Some(0),
    extensions: None,
    extras: None,
    targets: None
  };
  prmt.name = Some(String::from("first primitive"));
  println!("{}", prmt.name.unwrap());
}
