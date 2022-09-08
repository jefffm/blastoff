mod simple;
pub use simple::*;

use crate::map::Map;
use bracket_lib::prelude::RandomNumberGenerator;

pub trait MapGenerator {
    fn generate(&self, rng: &mut RandomNumberGenerator, level: u32) -> Map;
}
