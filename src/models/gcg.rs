use core::num;
use std::io::{Seek, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian};

use crate::{Bone, MeshName};

#[derive(BinRead, BinWrite, PartialEq, Debug, Clone, Copy)]
#[brw(repr = u8)]
pub enum GXCompType {
  GX_U8 = 0,
  GX_S8 = 1,
  GX_U16 = 2,
  GX_S16 = 3,
  GX_F32 = 4,
  GX_U32 = 5,
}

#[derive(BinRead, BinWrite, PartialEq, Debug, Clone, Copy)]
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
  pub color_attr_type: Option<GXAttrType>,
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
  pub face_chunk_size: u32,
}

#[derive(BinRead, BinWrite, Debug)]
#[brw(big)]
pub struct GCGLod {
  pub auto_lod_value: f32,
  pub num_meshes: u32,

  #[br(count = num_meshes)]
  pub meshes: Vec<StreamingGCGMesh>,
}

#[derive(BinRead, BinWrite, Debug, Clone, Copy)]
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

// BinrwNamedArgs
#[derive(Clone, Debug)]
pub struct GCGHeaderArgs {
  pub streaming_data: Vec<u8>,
}

// This BinWrite implementation actually restructures the streaming component data plus the header data in the SOI to form a proper XNG file.
// As such, the streaming data must be passed to write_options.
impl BinWrite for GCGHeader {
  type Args<'a> = &'a GCGHeaderArgs;

  fn write_options<W: Write + Seek>(
    &self,
    writer: &mut W,
    endian: Endian,
    args: Self::Args<'_>,
  ) -> BinResult<()> {
    let magic = b"gcg\0".to_vec();
    Vec::<u8>::write_options(&magic, writer, endian, ())?;
    i32::write_options(&self.version, writer, endian, ())?;
    u32::write_options(&self.num_bones, writer, endian, ())?;
    Vec::<Bone>::write_options(&self.bones, writer, endian, ())?;
    i32::write_options(&self.num_mesh_names, writer, endian, ())?;
    Vec::<MeshName>::write_options(&self.mesh_names, writer, endian, ())?;
    u8::write_options(&self.num_lod, writer, endian, ())?;
    u8::write_options(&self.skin_animates_flag, writer, endian, ())?;
    u8::write_options(&self.has_weight, writer, endian, ())?;
    u8::write_options(&self.unused, writer, endian, ())?;
    if self.has_weight != 0 {
      u16::write_options(&self.weight_count, writer, endian, ())?;
      Vec::<GCGWeight>::write_options(&self.weights.clone().unwrap(), writer, endian, ())?;
    }

    let mut offset_in_data: usize = 0;

    for lod in &self.lods {
      f32::write_options(&lod.auto_lod_value, writer, endian, ())?;
      u32::write_options(&lod.num_meshes, writer, endian, ())?;
      for mesh in &lod.meshes {
        u32::write_options(&mesh.surface_index, writer, endian, ())?;
        u8::write_options(&(mesh.vertex_type - 0x80), writer, endian, ())?;
        GXAttrType::write_options(&mesh.vertex_attr_type, writer, endian, ())?;
        GXCompType::write_options(&mesh.vertex_data_type, writer, endian, ())?;
        u8::write_options(&mesh.xyz_frac_bits, writer, endian, ())?;
        
        if (mesh.vertex_type & 0x1) == 0x1 && (mesh.vertex_type & 0x8) == 0x8 {
          GXAttrType::write_options(&mesh.normal_attr_type.unwrap(), writer, endian, ())?;
          GXCompType::write_options(&mesh.normal_data_type.unwrap(), writer, endian, ())?;
        }

        if (mesh.vertex_type & 0x1) == 0x0 {
          GXAttrType::write_options(&mesh.color_attr_type.unwrap(), writer, endian, ())?;
          GXCompType::write_options(&mesh.color_data_type.unwrap(), writer, endian, ())?;
        }

        GXAttrType::write_options(&mesh.uv_attr_type, writer, endian, ())?;
        GXCompType::write_options(&mesh.uv_data_type, writer, endian, ())?;
        u8::write_options(&mesh.tex_frac_bits, writer, endian, ())?;

        u16::write_options(&mesh.vertex_count, writer, endian, ())?;
        u8::write_options(&mesh.byte_stride, writer, endian, ())?;

        if (mesh.vertex_type & 0x1) == 0x1 && (mesh.vertex_type & 0x8) == 0x8 {
          u16::write_options(&mesh.normal_count, writer, endian, ())?;
          u8::write_options(&mesh.normal_stride, writer, endian, ())?;
        }

        if (mesh.vertex_type & 0x1) == 0x0 {
          u16::write_options(&mesh.color_count, writer, endian, ())?;
          u8::write_options(&mesh.color_stride, writer, endian, ())?;
        }

        u16::write_options(&mesh.uv_count, writer, endian, ())?;
        u8::write_options(&mesh.uv_stride, writer, endian, ())?;

        u32::write_options(&mesh.face_chunk_size, writer, endian, ())?;
        let mut vertex_block_size = 0;

        vertex_block_size += match mesh.vertex_data_type {
          GXCompType::GX_U8 => mesh.vertex_count as usize * 3,
          GXCompType::GX_S8 => mesh.vertex_count as usize * 3,
          GXCompType::GX_U16 => mesh.vertex_count as usize * 6,
          GXCompType::GX_S16 => mesh.vertex_count as usize * 6,
          GXCompType::GX_F32 => mesh.vertex_count as usize * 12,
          GXCompType::GX_U32 => mesh.vertex_count as usize * 12,
        };

        if (mesh.vertex_type & 0x1) == 0x1 && (mesh.vertex_type & 0x8) == 0x8 {
          vertex_block_size += match mesh.normal_data_type {
            Some(GXCompType::GX_U8) => mesh.normal_count as usize * 3,
            Some(GXCompType::GX_S8) => mesh.normal_count as usize * 3,
            Some(GXCompType::GX_U16) => mesh.normal_count as usize * 6,
            Some(GXCompType::GX_S16) => mesh.normal_count as usize * 6,
            Some(GXCompType::GX_F32) => mesh.normal_count as usize * 12,
            Some(GXCompType::GX_U32) => mesh.normal_count as usize * 12,
            None => 0,
          };
        }

        if (mesh.vertex_type & 0x1) == 0x0 {
          vertex_block_size += 4 * mesh.color_count as usize;
        }

        if (mesh.vertex_type & 0x2) == 0x2 {
          vertex_block_size += match mesh.uv_data_type {
            GXCompType::GX_U8 => mesh.uv_count as usize * 2,
            GXCompType::GX_S8 => mesh.uv_count as usize * 2,
            GXCompType::GX_U16 => mesh.uv_count as usize * 4,
            GXCompType::GX_S16 => mesh.uv_count as usize * 4,
            GXCompType::GX_F32 => mesh.uv_count as usize * 8,
            GXCompType::GX_U32 => mesh.uv_count as usize * 8,
          };
        }

        // write the vertex block (positions, normals or vertex colors, uvs)
        let vertex_block = (&args.streaming_data[offset_in_data..offset_in_data + vertex_block_size]).to_vec();
        Vec::<u8>::write_options(&vertex_block, writer, endian, ())?;
        offset_in_data = crate::round_up(offset_in_data + vertex_block_size, 32);

        let face_chunk: Vec<u8> = (&args.streaming_data
          [offset_in_data..offset_in_data + mesh.face_chunk_size as usize])
          .to_vec();
        Vec::<u8>::write_options(&face_chunk, writer, endian, ())?;
        offset_in_data += mesh.face_chunk_size as usize;
      }
      println!("{}, {}", offset_in_data, args.streaming_data.len());
    }
    Ok(())
  }
}
