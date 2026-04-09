use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::io::{Seek, Write};

use crate::{utils::*, Bone, MeshName};

// https://github.com/leeao/carsraceorama/blob/master/carsraceorama/CarsTypes.h#L43
const D3DVSDT_FLOAT1: u8 = 0x12; // 1D float expanded to (value, 0., 0., 1.)
const D3DVSDT_FLOAT2: u8 = 0x22; // 2D float expanded to (value, value, 0., 1.)
const D3DVSDT_FLOAT3: u8 = 0x32; // 3D float expanded to (value, value, value, 1.)
const D3DVSDT_FLOAT4: u8 = 0x42; // 4D float
const D3DVSDT_D3DCOLOR: u8 = 0x40; // 4D packed unsigned bytes mapped to 0. to 1. range
                                   // Input is in D3DCOLOR format (ARGB) expanded to (R, G, B, A)
const D3DVSDT_SHORT2: u8 = 0x25; // 2D signed short expanded to (value, value, 0., 1.)
const D3DVSDT_SHORT4: u8 = 0x45; // 4D signed short

// The following are Xbox extensions
const D3DVSDT_NORMSHORT1: u8 = 0x11; // 1D signed, normalized short expanded to (value, 0, 0., 1.)

// (signed, normalized short maps from -1.0 to 1.0)
const D3DVSDT_NORMSHORT2: u8 = 0x21; // 2D signed, normalized short expanded to (value, value, 0., 1.)
const D3DVSDT_NORMSHORT3: u8 = 0x31; // 3D signed, normalized short expanded to (value, value, value, 1.)
const D3DVSDT_NORMSHORT4: u8 = 0x41; // 4D signed, normalized short expanded to (value, value, value, value)
const D3DVSDT_NORMPACKED3: u8 = 0x16; // 3 signed, normalized components packed in 32-bits.  (11,11,10).

// Each component ranges from -1.0 to 1.0.
// Expanded to (value, value, value, 1.)
const D3DVSDT_SHORT1: u8 = 0x15; // 1D signed short expanded to (value, 0., 0., 1.)

// Signed shorts map to the range [-32768, 32767]
const D3DVSDT_SHORT3: u8 = 0x35; // 3D signed short expanded to (value, value, value, 1.)
const D3DVSDT_PBYTE1: u8 = 0x14; // 1D packed byte expanded to (value, 0., 0., 1.)

// Packed bytes map to the range [0, 1]
const D3DVSDT_PBYTE2: u8 = 0x24; // 2D packed byte expanded to (value, value, 0., 1.)
const D3DVSDT_PBYTE3: u8 = 0x34; // 3D packed byte expanded to (value, value, value, 1.)
const D3DVSDT_PBYTE4: u8 = 0x44; // 4D packed byte expanded to (value, value, value, value)
const D3DVSDT_FLOAT2H: u8 = 0x72; // 2D homogeneous float expanded to (value, value,0., value.)

// Useful for projective texture coordinates.
const D3DVSDT_NONE: u8 = 0x02; // No stream data

const DEFAULT_VERTEX_FORMATS: [u8; 16] = [
  D3DVSDT_FLOAT3,
  D3DVSDT_FLOAT3,
  D3DVSDT_D3DCOLOR,
  D3DVSDT_FLOAT2,
  D3DVSDT_FLOAT1,
  D3DVSDT_NONE,
  D3DVSDT_NONE,
  D3DVSDT_NONE,
  D3DVSDT_FLOAT4,
  D3DVSDT_FLOAT4,
  D3DVSDT_FLOAT2,
  D3DVSDT_NONE,
  D3DVSDT_NONE,
  D3DVSDT_NONE,
  D3DVSDT_NONE,
  D3DVSDT_NONE,
];

#[derive(Default, BinRead, BinWrite, Debug)]
#[brw(little)]
pub struct DXGDeltaBlock {
  num_channels: u32,

  #[br(count = 64)]
  controller_name: Vec<u8>,

  num_vertices: u32,
  xyz_bits: u32,
  force_unique: u8,
  unk: u32,
  unk2: u32,

  delta_count: u32,

  #[br(count = delta_count)]
  delta_positions: Vec<Vector4>,

  #[br(count = delta_count)]
  delta_normals: Vec<Vector4>,

  #[br(count = delta_count)]
  delta_indices: Vec<i32>,

  #[br(count = num_vertices)]
  positions: Vec<Vector4>,

  #[br(count = num_vertices)]
  normals: Vec<Vector4>,
}

#[derive(BinRead, BinWrite, Debug)]
#[brw(little)]
pub struct StreamingDXGMesh {
  pub surface_index: u32,
  pub vertex_type: u32,

  #[br(if ((vertex_type & 0x2000) == 0x2000))]
  pub compression_stuff: Option<[f32; 8]>,

  #[br(if ((vertex_type & 0x2000) == 0x2000))]
  pub vertex_formats: Option<[u8; 16]>,

  pub num_texture_coordinate_sets: u8,
  pub compressed: u8,
  pub streaming: u8,
  pub unk: u8,
  pub unk2: u8,
  pub unk3: u8,

  #[br(count = num_texture_coordinate_sets)]
  pub texture_coordinate_sets: Vec<f32>,

  pub num_vertices: u16,
  pub num_face_indices: u16,

  #[br(if ((vertex_type & 0x100) == 0x100))]
  pub delta_block: Option<DXGDeltaBlock>,
}

#[derive(BinRead, BinWrite, Debug)]
#[brw(little)]
pub struct DXGLod {
  pub auto_lod_value: f32,
  pub num_meshes: u32,

  #[br(count = num_meshes)]
  pub meshes: Vec<StreamingDXGMesh>,
}

#[derive(BinRead, Debug)]
#[brw(little)]
#[br(magic = b"dgs\0")]
pub struct DXGHeader {
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

  #[br(count = num_lod)]
  pub lods: Vec<DXGLod>,
}

// BinrwNamedArgs
#[derive(Clone, Debug)]
pub struct DXGHeaderArgs {
  pub streaming_data: Vec<u8>,
}

// This BinWrite implementation actually restructures the streaming component data plus the header data in the SOI to form a proper DXG file.
// As such, the streaming data must be passed to write_options.
impl BinWrite for DXGHeader {
  type Args<'a> = &'a DXGHeaderArgs;

  fn write_options<W: Write + Seek>(
    &self,
    writer: &mut W,
    endian: Endian,
    args: Self::Args<'_>,
  ) -> BinResult<()> {
    let magic = b"dxg\0".to_vec();
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

    let mut offset_in_data: usize = 0;

    for lod in &self.lods {
      f32::write_options(&lod.auto_lod_value, writer, endian, ())?;
      u32::write_options(&lod.num_meshes, writer, endian, ())?;
      for mesh in &lod.meshes {
        u32::write_options(&mesh.surface_index, writer, endian, ())?;
        u32::write_options(&mesh.vertex_type, writer, endian, ())?;
        if let Some(compression_stuff) = mesh.compression_stuff {
          compression_stuff.write_options(writer, endian, ())?;
        }

        let vertex_formats = if let Some(vertex_formats) = mesh.vertex_formats {
          vertex_formats
        } else {
          DEFAULT_VERTEX_FORMATS.clone()
        };
        if mesh.vertex_formats.is_some() {
          vertex_formats.write_options(writer, endian, ())?;
        }

        u8::write_options(&mesh.num_texture_coordinate_sets, writer, endian, ())?;
        u8::write_options(&mesh.compressed, writer, endian, ())?;
        u8::write_options(&0, writer, endian, ())?;
        u8::write_options(&mesh.unk, writer, endian, ())?;
        u8::write_options(&mesh.unk2, writer, endian, ())?;
        u8::write_options(&mesh.unk3, writer, endian, ())?;
        Vec::<f32>::write_options(&mesh.texture_coordinate_sets, writer, endian, ())?;

        u16::write_options(&mesh.num_vertices, writer, endian, ())?;
        u16::write_options(&mesh.num_face_indices, writer, endian, ())?;

        assert!(mesh.streaming == 1);

        let ty = mesh.vertex_type;
        let mut offset: usize = mesh.num_face_indices as usize * 2;

        if (ty & 0x01) == 0x01 {
          let attribute_size = if vertex_formats[0] == D3DVSDT_FLOAT3 {
            mesh.num_vertices as usize * 12
          } else if vertex_formats[0] == D3DVSDT_SHORT3 {
            mesh.num_vertices as usize * 6
          } else if vertex_formats[0] == D3DVSDT_PBYTE3 {
            mesh.num_vertices as usize * 3
          } else {
            0
          };
          offset += attribute_size;
        }
        if (ty & 0x02) == 0x02 {
          let attribute_size = if vertex_formats[1] == D3DVSDT_FLOAT3 {
            mesh.num_vertices as usize * 12
          } else if vertex_formats[1] == D3DVSDT_SHORT3 {
            mesh.num_vertices as usize * 6
          } else if vertex_formats[1] == D3DVSDT_PBYTE3 {
            mesh.num_vertices as usize * 3
          } else {
            0
          };
          offset += attribute_size;
        }
        if (ty & 0x08) == 0x08 {
          offset += mesh.num_vertices as usize * 4;
        }
        if (ty & 0x04) == 0x04 {
          let attribute_size = if vertex_formats[3] == D3DVSDT_FLOAT2 {
            mesh.num_vertices as usize * 8
          } else if vertex_formats[3] == D3DVSDT_SHORT2 {
            mesh.num_vertices as usize * 4
          } else if vertex_formats[3] == D3DVSDT_PBYTE2 {
            mesh.num_vertices as usize * 2
          } else {
            0
          };
          offset += attribute_size;
        }
        if (ty & 0x10) == 0x10 {
          offset += mesh.num_vertices as usize * 8;
        }
        if (ty & 0x40) == 0x40 {
          offset += mesh.num_vertices as usize * 4;
        }
        if (ty & 0x1000) == 0x1000 {
          offset += mesh.num_vertices as usize * 32;
        }

        let data = (&args.streaming_data[offset_in_data..offset_in_data + offset]).to_vec();
        Vec::<u8>::write_options(&data, writer, endian, ())?;

        offset_in_data += offset;

        if let Some(delta_block) = &mesh.delta_block {
          delta_block.write_options(writer, endian, ())?;
        }
      }
    }
    Ok(())
  }
}
