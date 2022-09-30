use bracket_random::prelude::RandomNumberGenerator;

use crate::{galaxy::Galaxy, util::GalaxySize};

pub trait GalaxyGenerator {
    fn generate(
        &mut self,
        size: GalaxySize,
        rng: &mut RandomNumberGenerator,
        overworldgen_history: &mut Vec<Galaxy>,
    ) -> Galaxy;
}
