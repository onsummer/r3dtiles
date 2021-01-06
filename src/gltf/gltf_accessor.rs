pub struct GltfAccessor {
  pub component_type: GltfComponentType,
  pub attribute_type: GltfAttributeType,
  pub count: u32,

  pub max: Option<GltfMinMax>,
  pub min: Option<GltfMinMax>,
  pub bufferview: Option<u32>,
  pub byte_offset: Option<u32>,

  pub normalized: Option<bool>,
  pub sparse: Option<GltfAccessorSparse>,
}

impl GltfAccessor {
  pub fn new(
    component_type: GltfComponentType,
    attribute_type: GltfAttributeType,
    count: u32
  ) -> GltfAccessor {
    GltfAccessor {
      component_type: component_type,
      attribute_type: attribute_type,
      count: count,
      max: None,
      min: None,
      bufferview: None,
      byte_offset: Some(0),
      normalized: None,
      sparse: None
    }
  }
}

pub struct GltfAccessorSparse {
  // TODO
}

pub enum GltfComponentType {
  Byte(u16),
  UnsignedByte(u16),
  Short(u16),
  UnsignedShort(u16),
  UnsignedInt(u16),
  Float(u16)
}

pub enum GltfAttributeType {
  Scalar,
  Vec2,
  Vec3,
  Vec4,
  Mat2,
  Mat3,
  Mat4,
}

pub enum GltfMinMax {
  Arr1([f64; 1]),
  Arr2([f64; 2]),
  Arr3([f64; 3]),
  Arr4([f64; 4]),
  Arr9([f64; 9]),
  Arr16([f64; 16]),
}