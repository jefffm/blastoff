mod bsp;
pub use bsp::*;

mod wfcgen;
pub use wfcgen::*;

mod combo;
pub use combo::*;

use bracket_random::prelude::RandomNumberGenerator;

use crate::{resource::Resources, sector::Map, util::WorldSize};

pub trait MapGenerator {
    fn generate(
        &mut self,
        size: WorldSize, // TODO: take a WorldInfo (or SectorInfo) instead of just a Size
        resources: &mut Resources,
        mapgen_history: &mut Vec<Map>,
    ) -> Map;
}
