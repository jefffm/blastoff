mod bsp;
pub use bsp::*;

mod wfcgen;
pub use wfcgen::*;

mod combo;
pub use combo::*;

use bracket_random::prelude::RandomNumberGenerator;

use crate::{sector::Map, util::WorldSize};

pub trait MapGenerator {
    fn generate(
        &mut self,
        size: WorldSize,
        rng: &mut RandomNumberGenerator,
        mapgen_history: &mut Vec<Map>,
    ) -> Map;
}
