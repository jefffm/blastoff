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
    fn generate(&mut self, info: PlanetInfo, _rng: &mut RandomNumberGenerator) -> Overworld {
        // TODO: derive default tile from PlanetInfo
        

        Overworld::from_size(info, OverworldTile::Barren)
    }
}
impl StaticPlanet {
    pub fn generate_overworld_info(
        _info: &GalaxyInfo,
        rng: &mut RandomNumberGenerator,
    ) -> PlanetInfo {
        let width = rng.roll_dice(3, 6);
        let height = rng.roll_dice(3, 6);
        PlanetInfo::new(
            format!("Procgen Planet Name #{}", rng.roll_dice(2, 20)),
            OverworldSize::new(width, height),
        )
    }
}
