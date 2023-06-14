use std::io::Write;
use std::path::{Path, PathBuf};

use binrw::BinWrite;
use x_flipper_360::*;

use crate::utils::*;
use crate::wii_th::{GCNTextureHeader, GCTSurfaceHeader};
use crate::ComponentKind::{self, *};
use crate::{CollisionModelArgs, ComponentData, SoiSoup, Str, XNGHeaderArgs};

#[test]
fn extract() {
  let toc_path = Path::new("./data/CR_03.gcn.toc");
  let soi_path = Path::new("./data/CR_03.gcn.soi");
  let str_path = Path::new("./data/CR_03.gcn.str");

  let soup = SoiSoup::<GCNTextureHeader>::cook(toc_path, soi_path).unwrap();
  let mut str = Str::read(str_path).unwrap();

  for (id, section) in soup.find_sections().iter().enumerate() {
    let section_data = str.read_section_data(section).unwrap();
  
    for component in section_data.uncached {
      process_component_wii(&soup, id as u32, component);
    }
  
    for component in section_data.cached {
      process_component_wii(&soup, id as u32, component);
    }
  }

  for static_texture in soup.static_textures().iter() {
    let path = PathBuf::from(format!(
      ".\\data\\CR_03\\{}.gct",
      clean_path(&static_texture.model_info.name)
    ));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut out = std::fs::File::create(path).unwrap();
    out.write_all(&static_texture.header_file).unwrap();
  }
}

#[test]
fn dump_scn() {
  let toc_path = Path::new("./data/CR_03.gcn.toc");
  let soi_path = Path::new("./data/CR_03.gcn.soi");
  let str_path = Path::new("./data/CR_03.gcn.str");
  let soup = SoiSoup::<GCNTextureHeader>::cook(toc_path, soi_path).unwrap();
  let mut str = Str::read(str_path).unwrap();
  let mut num_anim_models = 1;
  let mut num_static_models = 1;
  let mut num_objects = 1;
  for (id, section) in soup.find_sections().iter().enumerate() {
    let section_data = str.read_section_data(section).unwrap();

    for component in section_data.uncached {
      print_component(
        &soup,
        id as u32,
        component,
        &mut num_anim_models,
        &mut num_static_models,
        &mut num_objects,
      );
    }

    for component in section_data.cached {
      print_component(
        &soup,
        id as u32,
        component,
        &mut num_anim_models,
        &mut num_static_models,
        &mut num_objects,
      );
    }
  }
}

fn print_component<TH: binrw::BinRead<Args<'static> = ()> + 'static>(
  soup: &SoiSoup<TH>,
  section_id: u32,
  component: ComponentData,
  num_anim_models: &mut i32,
  num_static_models: &mut i32,
  num_objects: &mut i32,
) {
  match component.kind {
    // RenderableModel => {
    //   let header = soup
    //     .find_model(section_id, component.id, component.instance_id)
    //     .unwrap();
    //   if header.model_info.is_animated == 1 {
    //     println!("[AnimatedModel{}]\n{}", num_anim_models, header);
    //     *num_anim_models = *num_anim_models + 1;
    //   } else {
    //     println!("[Model{}]\n{}", num_static_models, header);
    //     *num_static_models = *num_static_models + 1;
    //   }
    // }
    Texture => {
      // println!("found Texture component kind; skipping...");
    }
    // CollisionModel => {
    //   let header = soup
    //     .find_collision_model(section_id, component.id, component.instance_id)
    //     .unwrap();
    //   println!("[Object{}]\n{}", num_objects, header);
    //   *num_objects = *num_objects + 1;
    // }
    UserData => {
      // println!("found UserData component kind; skipping...");
    }
    MotionPack => {
      // println!("found MotionPack component kind; skipping...");
    }
    CollisionGrid => {
      // println!("found CollisionGrid component kind; skipping...");
    }
    RenderableModel => todo!(),
    CollisionModel => todo!(),
  }
}


fn process_component_wii(soup: &SoiSoup<GCNTextureHeader>, section_id: u32, component: ComponentData) {
  if component.kind == ComponentKind::MotionPack {
    let header = soup
      .find_motion_pack(section_id, component.id, component.instance_id)
      .unwrap();

    let path = PathBuf::from(format!(".\\data\\CR_03\\{}.got", component.path));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut out = std::fs::File::create(path).unwrap();
    header.header.write(&mut out).unwrap();
    out.write_all(&component.data).unwrap();
  }

  // if component.kind == ComponentKind::RenderableModel {
  //   let header = soup
  //     .find_model(section_id, component.id, component.instance_id)
  //     .unwrap();
  //
  //   let path = PathBuf::from(format!(
  //     ".\\data\\CR_03\\{}.xng",
  //     component.path
  //   ));
  //   std::fs::create_dir_all(path.parent().unwrap()).unwrap();
  //   let mut out = std::fs::File::create(path).unwrap();
  //   header
  //     .streaming_model_header
  //     .write_options(
  //       &mut out,
  //       binrw::Endian::Big,
  //       &XNGHeaderArgs {
  //         streaming_data: component.data.clone(),
  //       },
  //     )
  //     .unwrap();
  // }
  // if component.kind == ComponentKind::CollisionModel {
  //   let header = soup
  //     .find_collision_model(section_id, component.id, component.instance_id)
  //     .unwrap();
  //
  //   let path = PathBuf::from(format!(
  //     ".\\data\\CR_03\\{}.gol",
  //     component.path
  //   ));
  //   std::fs::create_dir_all(path.parent().unwrap()).unwrap();
  //   let mut out = std::fs::File::create(path).unwrap();
  //   header
  //     .collision_model
  //     .write_options(
  //       &mut out,
  //       binrw::Endian::Big,
  //       &CollisionModelArgs {
  //         ror: false,
  //         streaming_data: component.data.clone(),
  //       },
  //     )
  //     .unwrap();
  // }
  if component.kind == ComponentKind::Texture {
    match soup.find_streaming_texture(section_id, component.id, component.instance_id) {
      Some(streaming_texture) => {

        let path = PathBuf::from(format!(".\\data\\CR_03\\{}.gct", component.path));
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut out = std::fs::File::create(path).unwrap();

        // Clone the header so we can set the version back to 2 (streamed GCTs have a flag OR'd on the version)
        let mut gct_file_header = streaming_texture.header.clone();
        gct_file_header.version = 2;
        gct_file_header.write_be(&mut out).unwrap();

        // Keeps track of our position in the streaming data. 
        let mut offset = 0;
        let mip_count = streaming_texture.header.mip_count;

        // Holds the offsets and sizes (in a Range<usize>) for each mip so we can iterate backwards over this later
        let mut mips = Vec::with_capacity(mip_count as usize);

        // Since mips are stored in forwards order (biggest to smallest) in the streaming data we need to first collect them in the vec,
        for i in 0..mip_count {
          let mip_width = 1.max(streaming_texture.header.width as usize / (2 as usize).pow(i));
          let mip_height = 1.max(streaming_texture.header.height as usize / (2 as usize).pow(i));

          let mip_size = streaming_texture.header.format.calculate_mip_size(mip_width, mip_height);

          mips.push(offset..offset + mip_size);

          offset += mip_size as usize;
        }

        // (At this point, we should have read through the entire streaming data)
        assert_eq!(offset, streaming_texture.header.calculate_image_size());
        assert_eq!(offset, component.data.len());

        // and we can then terate over the mips in backwards order and write them to the GCT.
        for i in (0..mip_count).rev() {
          let mip_width = 1.max(streaming_texture.header.width as usize / (2 as usize).pow(i));
          let mip_height = 1.max(streaming_texture.header.height as usize / (2 as usize).pow(i));

          let surface_header = GCTSurfaceHeader {
            width: mip_width as u32,
            height: mip_height as u32,
            size: streaming_texture.header.format.calculate_mip_size(mip_width, mip_height) as u32
          };
          surface_header.write_be(&mut out).unwrap();

          component.data[mips.get(i as usize).unwrap().clone()].write(&mut out).unwrap();
        }
      }
      None => {
        panic!("Failed to find texture header.");
      },
    }
  }
}


fn process_component(soup: &SoiSoup<TextureHeader>, section_id: u32, component: ComponentData) {
  if component.kind == ComponentKind::MotionPack {
    let header = soup
      .find_motion_pack(section_id, component.id, component.instance_id)
      .unwrap();

    let path = PathBuf::from(format!(".\\data\\CR_03\\{}.got", component.path));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut out = std::fs::File::create(path).unwrap();
    header.header.write(&mut out).unwrap();
    out.write_all(&component.data).unwrap();
  }

  // if component.kind == ComponentKind::RenderableModel {
  //   let header = soup
  //     .find_model(section_id, component.id, component.instance_id)
  //     .unwrap();
  //
  //   let path = PathBuf::from(format!(
  //     ".\\data\\CR_03\\{}.xng",
  //     component.path
  //   ));
  //   std::fs::create_dir_all(path.parent().unwrap()).unwrap();
  //   let mut out = std::fs::File::create(path).unwrap();
  //   header
  //     .streaming_model_header
  //     .write_options(
  //       &mut out,
  //       binrw::Endian::Big,
  //       &XNGHeaderArgs {
  //         streaming_data: component.data.clone(),
  //       },
  //     )
  //     .unwrap();
  // }
  // if component.kind == ComponentKind::CollisionModel {
  //   let header = soup
  //     .find_collision_model(section_id, component.id, component.instance_id)
  //     .unwrap();
  //
  //   let path = PathBuf::from(format!(
  //     ".\\data\\CR_03\\{}.gol",
  //     component.path
  //   ));
  //   std::fs::create_dir_all(path.parent().unwrap()).unwrap();
  //   let mut out = std::fs::File::create(path).unwrap();
  //   header
  //     .collision_model
  //     .write_options(
  //       &mut out,
  //       binrw::Endian::Big,
  //       &CollisionModelArgs {
  //         ror: false,
  //         streaming_data: component.data.clone(),
  //       },
  //     )
  //     .unwrap();
  // }
  if component.kind == ComponentKind::Texture {
    match soup.find_streaming_texture(section_id, component.id, component.instance_id) {
      Some(header) => {
        let metadata = header.header.metadata();

        match metadata.format() {
          TextureFormat::Dxt1 => {
            let texture_size: TextureSize2D =
              TextureSize2D::from_bytes(metadata.texture_size().to_le_bytes());

            let config = Config {
              width: texture_size.width() as u32 + 1,
              height: texture_size.height() as u32 + 1,
              depth: None,
              pitch: metadata.pitch() as u32,
              tiled: metadata.tiled(),
              packed_mips: metadata.packed_mips(),
              format: Format::Dxt1,
              mipmap_levels: Some(1.max(metadata.max_mip_level() - metadata.min_mip_level()) as u32),
              base_address: metadata.base_address(),
              mip_address: metadata.mip_address(),
            };

            let path = PathBuf::from(format!(".\\data\\CR_03\\{}.dds", component.path));
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut out = std::fs::File::create(path).unwrap();
            x_flipper_360::convert_to_dds(&config, &component.data, &mut out).unwrap();
          }
          TextureFormat::Dxt4_5 => {
            let texture_size: TextureSize2D =
              TextureSize2D::from_bytes(metadata.texture_size().to_le_bytes());

            let config = Config {
              width: texture_size.width() as u32 + 1,
              height: texture_size.height() as u32 + 1,
              depth: None,
              pitch: metadata.pitch() as u32,
              tiled: metadata.tiled(),
              packed_mips: metadata.packed_mips(),
              format: Format::Dxt5,
              mipmap_levels: Some(1.max(metadata.max_mip_level() - metadata.min_mip_level()) as u32),
              base_address: metadata.base_address(),
              mip_address: metadata.mip_address(),
            };

            let path = PathBuf::from(format!(".\\data\\CR_03\\{}.dds", component.path));
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut out = std::fs::File::create(path).unwrap();
            x_flipper_360::convert_to_dds(&config, &component.data, &mut out).unwrap();
          }
          TextureFormat::Dxt2_3 => {
            let texture_size: TextureSize2D =
              TextureSize2D::from_bytes(metadata.texture_size().to_le_bytes());

            let config = Config {
              width: texture_size.width() as u32 + 1,
              height: texture_size.height() as u32 + 1,
              depth: None,
              pitch: metadata.pitch() as u32,
              tiled: metadata.tiled(),
              packed_mips: metadata.packed_mips(),
              format: Format::Dxt3,
              mipmap_levels: Some(1.max(metadata.max_mip_level() - metadata.min_mip_level()) as u32),
              base_address: metadata.base_address(),
              mip_address: metadata.mip_address(),
            };
            
            let path = PathBuf::from(format!(".\\data\\CR_03\\{}.dds", component.path));
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut out = std::fs::File::create(path).unwrap();
            x_flipper_360::convert_to_dds(&config, &component.data, &mut out).unwrap();
          }
          _ => {
            let texture_size: TextureSize2D =
              TextureSize2D::from_bytes(metadata.texture_size().to_le_bytes());

            let config = Config {
              width: texture_size.width() as u32 + 1,
              height: texture_size.height() as u32 + 1,
              depth: None,
              pitch: metadata.pitch() as u32,
              tiled: metadata.tiled(),
              packed_mips: metadata.packed_mips(),
              format: Format::RGBA8,
              mipmap_levels: Some(1.max(metadata.max_mip_level() - metadata.min_mip_level()) as u32),
              base_address: metadata.base_address(),
              mip_address: metadata.mip_address(),
            };

            let path = PathBuf::from(format!(".\\data\\CR_03\\{}.dds", component.path));
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut out = std::fs::File::create(path).unwrap();
            x_flipper_360::convert_to_dds(&config, &component.data, &mut out).unwrap();
          }
        }
      }
      None => match soup.find_static_texture(section_id, component.id, component.instance_id) {
        Some(static_texture) => {
          let path = PathBuf::from(format!(".\\data\\CR_03\\{}.dds", component.path));
          std::fs::create_dir_all(path.parent().unwrap()).unwrap();
          let mut out = std::fs::File::create(path).unwrap();
          out.write_all(&static_texture.header_file).unwrap();
        }
        None => panic!("Failed to find texture header."),
      },
    }
  }
}
