use bracket_random::prelude::RandomNumberGenerator;

use crate::{overworld::Overworld, util::OverworldSize};

pub trait OverworldGenerator {
    fn generate(
        &mut self,
        size: OverworldSize,
        rng: &mut RandomNumberGenerator,
        overworldgen_history: &mut Vec<Overworld>,
    ) -> Overworld;
}
