mod bsp;
pub use bsp::*;

mod wfcgen;
pub use wfcgen::*;

mod combo;
pub use combo::*;

use crate::{overworld::SectorInfo, resource::Resources, sector::Map};

pub trait MapGenerator {
    fn generate(
        &mut self,
        sector_info: &SectorInfo,
        resources: &mut Resources,
        mapgen_history: &mut Vec<Map>,
    ) -> Map;
}
