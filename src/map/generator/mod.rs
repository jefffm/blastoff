mod simple;
pub use simple::*;

mod bsp;
pub use bsp::*;

use crate::map::Map;
use bracket_lib::prelude::RandomNumberGenerator;

pub trait MapGenerator {
    fn generate(
        &mut self,
        rng: &mut RandomNumberGenerator,
        mapgen_history: &mut Vec<Map>,
        level: u32,
    ) -> Map;
}
