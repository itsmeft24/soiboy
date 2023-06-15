use binrw::{BinRead, BinWrite};

use crate::{Bone, MeshName};

#[derive(BinRead, BinWrite, PartialEq, Debug, Clone)]
#[brw(repr = u8)]
pub enum GXCompType {
  GX_U8 = 0,
  GX_S8 = 1,
  GX_U16 = 2,
  GX_S16 = 3,
  GX_F32 = 4,
  GX_U32 = 5,
}

#[derive(BinRead, BinWrite, PartialEq, Debug, Clone)]
#[brw(repr = u8)]
pub enum GXAttrType {
  GX_NONE = 0,
  GX_DIRECT = 1,
  GX_INDEX8 = 2,
  GX_INDEX16 = 3,
}

#[derive(BinRead, BinWrite, Debug)]
#[brw(big)]
pub struct StreamingGCGMesh {
  pub surface_index: u32,
  pub vertex_type: u8,

  pub vertex_attr_type: GXAttrType,
  pub vertex_data_type: GXCompType,
  pub xyz_frac_bits: u8,

  #[br(if ((vertex_type & 0x1) == 0x1 && (vertex_type & 0x8) == 0x8))]
  pub normal_attr_type: Option<GXAttrType>,
  #[br(if ((vertex_type & 0x1) == 0x1 && (vertex_type & 0x8) == 0x8))]
  pub normal_data_type: Option<GXCompType>,

  #[br(if ((vertex_type & 0x1) == 0x0))]
  pub color_attr_type: Option<GXCompType>,
  #[br(if ((vertex_type & 0x1) == 0x0))]
  pub color_data_type: Option<GXCompType>,

  pub uv_attr_type: GXAttrType,
  pub uv_data_type: GXCompType,
  pub tex_frac_bits: u8,

  pub vertex_count: u16,
  pub byte_stride: u8,

  #[br(if ((vertex_type & 0x1) == 0x1 && (vertex_type & 0x8) == 0x8))]
  pub normal_count: u16,
  #[br(if ((vertex_type & 0x1) == 0x1 && (vertex_type & 0x8) == 0x8))]
  pub normal_stride: u8,

  #[br(if ((vertex_type & 0x1) == 0x0))]
  pub color_count: u16,
  #[br(if ((vertex_type & 0x1) == 0x0))]
  pub color_stride: u8,

  pub uv_count: u16,
  pub uv_stride: u8,
  pub face_chunk_size: i32,
}

#[derive(BinRead, BinWrite, Debug)]
#[brw(big)]
pub struct GCGLod {
  pub auto_lod_value: f32,
  pub num_meshes: u32,

  #[br(count = num_meshes)]
  pub meshes: Vec<StreamingGCGMesh>,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct GCGWeight {
  pub bone_id: u16,
  pub weight: f32,
}

#[derive(BinRead, Debug)]
#[br(big)]
#[br(magic = b"ggs\0")]
pub struct GCGHeader {
  version: i32,
  num_bones: u32,

  #[br(count = num_bones)]
  bones: Vec<Bone>,

  num_mesh_names: i32,

  #[br(count = num_mesh_names)]
  pub mesh_names: Vec<MeshName>,

  pub num_lod: u8,
  skin_animates_flag: u8,
  has_weight: u8,
  unused: u8,

  #[br(if(has_weight != 0))]
  weight_count: u16,

  #[br(if(has_weight != 0), count = weight_count)]
  pub weights: Option<Vec<GCGWeight>>,

  #[br(count = num_lod)]
  pub lods: Vec<GCGLod>,
}
