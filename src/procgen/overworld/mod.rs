mod name;
use bracket_random::prelude::RandomNumberGenerator;
pub use name::*;

use crate::{
    data::{Element, PlanetType},
    overworld::{Overworld, OverworldTile, PlanetInfo, SectorInfo},
    resource::Resources,
};

pub trait OverworldGenerator {
    fn generate(&mut self, planet_info: PlanetInfo, resources: &mut Resources) -> Overworld;
}

pub struct StaticPlanet {}
impl OverworldGenerator for StaticPlanet {
    fn generate(&mut self, info: PlanetInfo, resources: &mut Resources) -> Overworld {
        let planet = Overworld::from_info(info);

        let planet: Overworld = match planet.info().planet_type {
            PlanetType::Barren => create_barren_planet(planet, &mut resources.rng),
            PlanetType::Lush => create_lush_planet(planet, &mut resources.rng),
            PlanetType::Aqueus => create_aqueus_planet(planet, &mut resources.rng),
            PlanetType::Mountains => create_mountain_planet(planet, &mut resources.rng),
        };

        planet
    }
}

fn create_mountain_planet(mut planet: Overworld, rng: &mut RandomNumberGenerator) -> Overworld {
    for (_point, sector_info) in planet.iter_sector_infos_mut() {
        sector_info.tile = if rng.roll_dice(1, 5) % 5 == 0 {
            OverworldTile::Mountains
        } else {
            OverworldTile::Barren
        };
    }

    planet
}

fn create_aqueus_planet(mut planet: Overworld, rng: &mut RandomNumberGenerator) -> Overworld {
    for (_point, sector_info) in planet.iter_sector_infos_mut() {
        sector_info.tile = if rng.roll_dice(1, 3) % 3 == 0 {
            OverworldTile::Water
        } else {
            OverworldTile::Jungle
        };
    }

    planet
}

fn create_lush_planet(mut planet: Overworld, rng: &mut RandomNumberGenerator) -> Overworld {
    for (_point, sector_info) in planet.iter_sector_infos_mut() {
        sector_info.tile = if rng.roll_dice(1, 5) % 5 == 0 {
            OverworldTile::Jungle
        } else {
            OverworldTile::Water
        };
    }

    planet
}

fn create_barren_planet(mut planet: Overworld, rng: &mut RandomNumberGenerator) -> Overworld {
    for (_point, sector_info) in planet.iter_sector_infos_mut() {
        sector_info.tile = if rng.roll_dice(1, 25) % 25 == 0 {
            OverworldTile::Mountains
        } else {
            OverworldTile::Barren
        };
    }

    planet
}
