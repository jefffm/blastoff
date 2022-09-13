mod simple;
pub use simple::*;

use crate::map::Map;
use bracket_lib::prelude::RandomNumberGenerator;

pub trait MapGenerator {
    fn generate(
        &self,
        rng: &mut RandomNumberGenerator,
        mapgen_history: &mut Vec<Map>,
        level: u32,
    ) -> Map;
}
