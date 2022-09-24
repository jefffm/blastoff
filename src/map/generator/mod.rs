mod bsp;
pub use bsp::*;

mod wfcgen;
pub use wfcgen::*;

use bracket_random::prelude::RandomNumberGenerator;

use crate::map::Map;

pub trait MapGenerator {
    fn generate(
        &mut self,
        rng: &mut RandomNumberGenerator,
        mapgen_history: &mut Vec<Map>,
        level: u32,
    ) -> Map;
}
