use std::fs::File;
use std::path::Path;

use binrw::{BinRead, BinReaderExt, BinResult};

use crate::texture_header::TextureHeader;

#[derive(BinRead, PartialEq, Debug)]
#[br(repr = i32)]
enum StreamingMode {
  Unknown = -1,
  _1D,
  _2D,
  Manual,
}

#[derive(BinRead, Debug)]
struct Header {
  version: i32,

  flags: i32,
  sections: i32,
  collision_models: i32,
  renderable_models: i32,
  motion_packs: i32,
  streaming_textures: i32,
  static_textures: i32,
  uncached_pages: i32,
  cached_pages: i32,

  motion_packs_offset: i32,
  renderable_models_offset: i32,
  collision_models_offset: i32,
  textures_offset: i32,
  collision_grids_offset: i32,

  streaming_mode: StreamingMode,
  reserved: [u8; 16],
}

#[derive(BinRead, Debug)]
struct ModelInfo {
  flags: i32,
  position: [f32; 4],
  look_vector: [f32; 4],
  up_vector: [f32; 4],
  is_animated: i32,
  section_id: i32,
  component_id: i32,

  name: [char; 260],

  zone: i32,
  parameter_count: i32,
}

#[derive(BinRead, Debug)]
struct StreamingTexture {
  model_info: ModelInfo,
  // might be something, currently only padding
  padding: u32,
  header: TextureHeader,
}

#[derive(BinRead, Debug)]
pub struct Soi {
  header: Header,

  #[br(count = header.uncached_pages)]
  uncached_page_sizes: Vec<i32>,

  #[br(count = header.cached_pages)]
  cached_page_sizes: Vec<i32>,

  #[br(count = header.streaming_textures)]
  streaming_textures: Vec<StreamingTexture>,
  // #[br(count = header.static_textures)]
  // static_textures: Vec<StaticTexture>,

  // #[br(count = header.motion_packs)]
  // motion_packs: Vec<MotionPack>,
}

impl Soi {
  pub fn read(path: &Path) -> BinResult<Self> {
    let mut file = File::open(path)?;
    Self::read_file(&mut file)
  }

  pub fn read_file(file: &mut File) -> BinResult<Self> {
    file.read_be()
  }

  pub fn find_texture_header(&self, section_id: u32, component_id: u32) -> Option<&TextureHeader> {
    for texture in &self.streaming_textures {
      let model_info = &texture.model_info;
      if model_info.section_id == section_id as i32
        && model_info.component_id == component_id as i32
      {
        return Some(&texture.header);
      }
    }

    None
  }
}
