use binrw::*;

use crate::div_round_up;

#[derive(BinRead, BinWrite, PartialEq, Debug, Clone)]
#[brw(repr = u32)]
pub enum DXTFormat {
  Dxt1 = 37,
  Dxt1Mm = 38,
  Dxt2 = 47,
  Dxt2Mm = 48,
  Dxt3 = 49,
  Dxt3Mm = 50,
  Dxt4 = 51,
  Dxt4Mm = 52,
  Dxt5 = 53,
  Dxt5Mm = 54,
  Pal8 = 55,
  Pal8Mm = 56,
  Lum8 = 57,
}

impl DXTFormat {
  pub fn get_block_dim(&self) -> (usize, usize) {
    match self {
      DXTFormat::Pal8 => (1, 1),
      DXTFormat::Pal8Mm => (1, 1),
      DXTFormat::Lum8 => (1, 1),
      _ => (4, 4),
    }
  }
  pub fn get_bits_per_pixel(&self) -> usize {
    match self {
      DXTFormat::Dxt1 => 4,
      DXTFormat::Dxt1Mm => 4,
      DXTFormat::Dxt2 => 8,
      DXTFormat::Dxt2Mm => 8,
      DXTFormat::Dxt3 => 8,
      DXTFormat::Dxt3Mm => 8,
      DXTFormat::Dxt4 => 4,
      DXTFormat::Dxt4Mm => 4,
      DXTFormat::Dxt5 => 8,
      DXTFormat::Dxt5Mm => 8,
      DXTFormat::Pal8 => 8,
      DXTFormat::Pal8Mm => 8,
      DXTFormat::Lum8 => 8,
    }
  }
  pub fn calculate_mip_size(&self, width: usize, height: usize) -> usize {
    let (blk_width_pixels, blk_height_pixels) = self.get_block_dim();
    let bits_per_pixel = self.get_bits_per_pixel();
    let blk_size_bytes = ((blk_width_pixels * blk_height_pixels) * bits_per_pixel) / 8;
    let size_bytes = div_round_up(width, blk_width_pixels)
      * div_round_up(height, blk_height_pixels)
      * blk_size_bytes;

    size_bytes
  }
}

impl std::fmt::Display for DXTFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DXTFormat::Dxt1 | DXTFormat::Dxt1Mm => write!(f, "Dxt1"),
      DXTFormat::Dxt2 | DXTFormat::Dxt2Mm => write!(f, "Dxt2"),
      DXTFormat::Dxt3 | DXTFormat::Dxt3Mm => write!(f, "Dxt3"),
      DXTFormat::Dxt4 | DXTFormat::Dxt4Mm => write!(f, "Dxt4"),
      DXTFormat::Dxt5 | DXTFormat::Dxt5Mm => write!(f, "Dxt5"),
      DXTFormat::Pal8 | DXTFormat::Pal8Mm => write!(f, "Pal8"),
      DXTFormat::Lum8 => write!(f, "Lum8"),
    }
  }
}

#[derive(BinRead, BinWrite, Debug, Clone)]
pub struct DXTTextureHeader {
  pub format: DXTFormat,
  pub palette_size: u32,

  #[br(count = palette_size * 2)]
  pub palette: Vec<u8>,

  pub mip_count: u32,
  pub width: u32,
  pub height: u32,
}

#[derive(BinRead, BinWrite, Debug)]
pub struct DXTSurfaceHeader {
  pub width: u32,
  pub height: u32,
  pub size: u32,
}

impl DXTTextureHeader {
  pub fn calculate_image_size(&self) -> usize {
    let (blk_width_pixels, blk_height_pixels) = self.format.get_block_dim();
    let bits_per_pixel = self.format.get_bits_per_pixel();
    let blk_size_bytes = ((blk_width_pixels * blk_height_pixels) * bits_per_pixel) / 8;
    let mut size_bytes = div_round_up(self.width as usize, blk_width_pixels)
      * div_round_up(self.height as usize, blk_height_pixels)
      * blk_size_bytes;

    for i in 1..self.mip_count {
      let mip_width = self.width as usize / (2 as usize).pow(i);
      let mip_height = self.height as usize / (2 as usize).pow(i);

      size_bytes += div_round_up(mip_width as usize, blk_width_pixels)
        * div_round_up(mip_height as usize, blk_height_pixels)
        * blk_size_bytes;
    }

    size_bytes
  }
}

#[derive(BinRead, BinWrite)]
pub struct DXTSurface {
  pub header: DXTSurfaceHeader,

  #[br(count = header.size)]
  pub data: Vec<u8>,
}

#[derive(BinRead, BinWrite)]
pub struct DXTStaticTextureHeader {
  pub version: u32,
  pub format: DXTFormat,
  pub palette_size: u32,

  #[br(count = palette_size * 2)]
  pub palette: Vec<u8>,

  pub mip_count: u32,
  pub width: u32,
  pub height: u32,

  #[br(count = mip_count)]
  pub mips: Vec<DXTSurface>,
}
