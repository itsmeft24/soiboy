use std::{collections::HashMap, fs::File, io::Read, path::Path};

use binrw::{BinRead, BinReaderExt, BinResult};
use flate2::read::ZlibDecoder;

use crate::clean_path;

#[derive(BinRead)]
pub struct OffsetEntry {
  pub name_len: u32,

  #[br(count = name_len)]
  pub name: Vec<u8>,

  pub start_offset: u32,
  pub size: u32,
}

#[derive(BinRead)]
pub struct Header {
  pub version: u32,
  pub header_size: u32,

  #[br(if(version == 3))]
  pub resource_file_type_flags: Option<u32>,

  pub user_data_type: u32,

  /// Size in bytes
  pub sector_list_size: u32,

  #[br(count = sector_list_size / 2)]
  pub sector_list: Vec<i16>,

  #[br(count = header_size - (8 + sector_list_size + (resource_file_type_flags.is_some() as u32 * 4)))]
  pub user_data: Vec<u8>,

  /// Actual length
  pub offset_table_len: u32,
  /// Size in bytes
  pub offset_table_size: u32,

  #[br(count = offset_table_len)]
  pub offset_table: Vec<OffsetEntry>,
}

pub struct Res {
  res_file_header: Header,
  files: HashMap<String, Vec<u8>>,
}

impl Res {
  pub fn read(path: &Path) -> BinResult<Self> {
    let mut file = File::open(path)?;
    Self::read_file(&mut file)
  }

  pub fn read_file(file: &mut File) -> BinResult<Self> {
    let res_file_header: Header = file.read_le()?;
    let mut files = HashMap::new();
    let mut compressed_data = Vec::new();
    file.read_to_end(&mut compressed_data).unwrap();

    let mut current_offset = 0;
    while compressed_data[current_offset] == 0 {
      current_offset += 1;
    }

    for offset_entry in res_file_header.offset_table.iter() {
      let path = clean_path(&offset_entry.name);

      let mut decompressed_data = Vec::new();
      let mut decoder = ZlibDecoder::new(&compressed_data[current_offset..]);
      decoder.read_to_end(&mut decompressed_data)?;

      // advance the offset in the compressed data buffer by the size that the decoder read while decompressing this specific file.
      current_offset += decoder.total_in() as usize;

      // a dummy 0xFF byte is sometimes placed so the game knows when a file has been read, so we check for it and advance the offset accordingly.
      if compressed_data[current_offset] == 0xFF {
        current_offset += 1;
      }

      files.insert(path, decompressed_data);
    }
    Ok(Self {
      res_file_header,
      files,
    })
  }

  pub fn get_file(&self, path: String) -> Option<&[u8]> {
    if self.files.contains_key(&path) {
      Some(&self.files[&path])
    } else {
      None
    }
  }
}
