extern crate serde_json;
use serde_json::Value;
use super::bounding_volume::BoundingVolume;
use super::tile_content::TileContent;
use super::tile_refine::TileRefine;

pub struct Tile {
  pub geometric_error: f32,
  pub bounding_volume: BoundingVolume,

  pub content: Option<TileContent>,
  pub transform: Option<[f32; 16]>,
  pub refine: Option<TileRefine>,
  pub viewer_request_volume: Option<BoundingVolume>,
  pub children: Option<Vec<Tile>>,

  pub extensions: Option<Value>,
  pub extras: Option<Value>,
}

impl Tile {
  pub fn new() -> Tile {
    Tile {
      geometric_error: 0.0,
      bounding_volume: BoundingVolume::new(),
      content: None,
      transform: None,
      refine: None,
      viewer_request_volume: None,
      children: None,
      extensions: None,
      extras: None
    }
  }
}

/*
设计树结构时，节点这么写是错误的
pub struct Node {
  pub value: i32,
  pub left: Node,
  pub right: Node
}
要用指向堆的智能指针 Box<T>
pub struct Node {
  pub value: i32,
  pub left: Option<Box<Node>>,
  pub right: Option<Box<Node>>
}
*/