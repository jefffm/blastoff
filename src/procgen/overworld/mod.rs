use std::collections::HashMap;

use bracket_random::prelude::RandomNumberGenerator;

use crate::{
    galaxy::GalaxyInfo,
    overworld::{Overworld, OverworldTile, PlanetInfo},
    util::OverworldSize,
};

pub trait OverworldGenerator {
    fn generate(&mut self, planet_info: PlanetInfo, rng: &mut RandomNumberGenerator) -> Overworld;
}

pub struct StaticPlanet {}
impl OverworldGenerator for StaticPlanet {
    fn generate(&mut self, info: PlanetInfo, rng: &mut RandomNumberGenerator) -> Overworld {
        // TODO: derive default tile from PlanetInfo
        let overworld = Overworld::from_size(info, OverworldTile::Barren);

        overworld
    }
}
impl StaticPlanet {
    pub fn generate_overworld_info(
        info: &GalaxyInfo,
        rng: &mut RandomNumberGenerator,
    ) -> PlanetInfo {
        let width = rng.roll_dice(3, 6);
        let height = rng.roll_dice(3, 6);
        PlanetInfo::new(
            format!("Procgen Planet Name #{}", rng.roll_dice(2, 20)).to_owned(),
            OverworldSize::new(width, height),
        )
    }
}
