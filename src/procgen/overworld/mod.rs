use std::collections::HashMap;

use bracket_random::prelude::RandomNumberGenerator;

use crate::{
    overworld::{Overworld, OverworldTile, PlanetInfo},
    util::OverworldSize,
};

pub trait OverworldGenerator {
    fn generate(
        &mut self,
        planet_info: PlanetInfo,
        size: OverworldSize,
        rng: &mut RandomNumberGenerator,
        overworldgen_history: &mut Vec<Overworld>,
    ) -> Overworld;
}

struct StaticPlanet {}
impl OverworldGenerator for StaticPlanet {
    fn generate(
        &mut self,
        planet_info: PlanetInfo,
        size: OverworldSize,
        rng: &mut RandomNumberGenerator,
        overworldgen_history: &mut Vec<Overworld>,
    ) -> Overworld {
        // TODO: Overworld needs to take a SectorGenerator and use it to build maps (on demand) for each sector
        let overworld = Overworld::from_size(
            PlanetInfo::new("Test Planet".to_owned(), OverworldSize::new(10, 10)),
            OverworldTile::Barren,
        );

        // overworldgen_history.push(overworld.clone());
        overworld
    }
}
