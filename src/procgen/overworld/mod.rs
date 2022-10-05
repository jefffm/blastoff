mod name;
pub use name::*;

use bracket_random::prelude::RandomNumberGenerator;

use crate::overworld::{Overworld, OverworldTile, PlanetInfo};

pub trait OverworldGenerator {
    fn generate(&mut self, planet_info: PlanetInfo, rng: &mut RandomNumberGenerator) -> Overworld;
}

pub struct StaticPlanet {}
impl OverworldGenerator for StaticPlanet {
    fn generate(&mut self, info: PlanetInfo, _rng: &mut RandomNumberGenerator) -> Overworld {
        // TODO: derive default tile from PlanetInfo

        Overworld::from_size(info, OverworldTile::Barren)
    }
}
