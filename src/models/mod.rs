mod gcg;
mod xng;
use binrw::BinRead;
use binrw::BinWrite;

use crate::clean_string;

pub use self::gcg::*;
pub use self::xng::*;

#[derive(BinRead, Debug)]
pub struct StreamingRenderableModel<MH: BinRead<Args<'static> = ()> + 'static> {
  pub model_info: crate::ModelInfo,

  #[br(count = model_info.parameter_count)]
  pub parameters: Vec<crate::StreamingParameter>,

  pub streaming_model_header: MH,
}

impl<MH: BinRead<Args<'static> = ()>> std::fmt::Display for StreamingRenderableModel<MH> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.model_info.zone != -1 {
      write!(
        f,
        "SLT={}\nPosition={}\nLookVector={}\nUpVector={}\nZone={}\n",
        clean_string(&self.model_info.name),
        self.model_info.position,
        self.model_info.look_vector,
        self.model_info.up_vector,
        self.model_info.zone
      );
    } else {
      write!(
        f,
        "SLT={}\nPosition={}\nLookVector={}\nUpVector={}\n",
        clean_string(&self.model_info.name),
        self.model_info.position,
        self.model_info.look_vector,
        self.model_info.up_vector,
      );
    }
    Ok(for param in self.parameters.iter() {
      write!(f, "{}\n", param);
    })
  }
}

#[derive(BinRead, BinWrite, Debug)]
#[brw(big)]
pub struct MeshName {
  #[br(count = 64)]
  name: Vec<u8>,
}

#[derive(BinRead, BinWrite, Debug)]
#[brw(big)]
pub struct Bone {
  name: [u8; 128],
  matrix: [f32; 16],
  bounding_box_center: [f32; 3],
  bounding_box_half: [f32; 3],
  bounding_box_radius: f32,
  parent_index: u32,
}
