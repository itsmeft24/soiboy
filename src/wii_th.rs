use binrw::*;

#[derive(BinRead, BinWrite, PartialEq, Debug, Clone)]
#[brw(repr = u32)]
pub(crate) enum GCTFormat {
  Rgba8 = 0x0F,
  Cmpr = 0x29,
  Cmpr_MM = 0x2A,
  Ci8 = 0x3A,
  Ci8_MM = 0x3B,
  I8 = 0x3C,
}

fn round_up(numToRound: usize, roundTo: usize) -> usize {
  return ((numToRound + roundTo - 1) / roundTo) * roundTo;
}

fn div_round_up(numToRound: usize, roundTo: usize) -> usize {
  return (numToRound + roundTo - 1) / roundTo;
}

impl GCTFormat {
  pub fn get_block_dim(&self) -> (usize, usize) {
    match self {
      GCTFormat::Rgba8 => (4, 4),
      GCTFormat::Cmpr => (8, 8),
      GCTFormat::Cmpr_MM => (8, 8),
      GCTFormat::Ci8 => (8, 4),
      GCTFormat::Ci8_MM => (8, 4),
      GCTFormat::I8 => (8, 4),
    }
  }
  pub fn get_bits_per_pixel(&self) -> usize {
    match self {
      GCTFormat::Rgba8 => 32,
      GCTFormat::Cmpr => 4,
      GCTFormat::Cmpr_MM => 4,
      GCTFormat::Ci8 => 8,
      GCTFormat::Ci8_MM => 8,
      GCTFormat::I8 => 8,
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

impl std::fmt::Display for GCTFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        GCTFormat::Rgba8 => write!(f, "Rgba8"),
        GCTFormat::Cmpr | GCTFormat::Cmpr_MM => write!(f, "Cmpr"),
        GCTFormat::Ci8  | GCTFormat::Ci8_MM => write!(f, "Ci8"),
        GCTFormat::I8 => write!(f, "I8"),
      }
    }
}

#[derive(BinRead, BinWrite, Debug, Clone)]
pub(crate) struct GCNTextureHeader {
  pub version: u32,
  pub format: GCTFormat,
  pub palette_size: u32,

  #[br(count = palette_size * 2)]
  pub palette: Vec<u8>,
  
  pub mip_count: u32,
  pub width: u32,
  pub height: u32,
}

#[derive(BinRead, BinWrite, Debug)]
pub(crate) struct GCTSurfaceHeader {
  pub width: u32,
  pub height: u32,
  pub size: u32,
}

impl GCNTextureHeader {
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