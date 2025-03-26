use binrw::BinRead;

#[derive(BinRead, Debug)]
pub struct X360StaticTextureHeader {
  pub dds_size: u32,
  #[br(count = dds_size)]
  pub header_file: Vec<u8>,
}
