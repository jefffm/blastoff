mod name;
use bracket_random::prelude::RandomNumberGenerator;
pub use name::*;

use crate::{
    data::{Element, PlanetType},
    game::consts::SECTOR_SIZE,
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

fn create_mountain_planet<'a>(
    mut planet: Overworld<'a>,
    rng: &mut RandomNumberGenerator,
) -> Overworld<'a> {
    for point in planet.iter_points().into_iter() {
        let tile = if rng.roll_dice(1, 5) % 5 == 0 {
            OverworldTile::Mountains
        } else {
            OverworldTile::Barren
        };
        let sector_info = SectorInfo::new(planet.info(), tile, SECTOR_SIZE);
        planet.set_sector_info(point, sector_info);
    }

    planet
}

fn create_aqueus_planet<'a>(
    mut planet: Overworld<'a>,
    rng: &mut RandomNumberGenerator,
) -> Overworld<'a> {
    for point in planet.iter_points().into_iter() {
        let tile = if rng.roll_dice(1, 3) % 3 == 0 {
            OverworldTile::Water
        } else {
            OverworldTile::Jungle
        };
        let sector_info = SectorInfo::new(planet.info(), tile, SECTOR_SIZE);
        planet.set_sector_info(point, sector_info);
    }

    planet
}

fn create_lush_planet<'a>(
    mut planet: Overworld<'a>,
    rng: &mut RandomNumberGenerator,
) -> Overworld<'a> {
    for point in planet.iter_points().into_iter() {
        let tile = if rng.roll_dice(1, 5) % 5 == 0 {
            OverworldTile::Jungle
        } else {
            OverworldTile::Water
        };
        let sector_info = SectorInfo::new(planet.info(), tile, SECTOR_SIZE);
        planet.set_sector_info(point, sector_info);
    }

    planet
}

fn create_barren_planet<'a>(
    mut planet: Overworld<'a>,
    rng: &mut RandomNumberGenerator,
) -> Overworld<'a> {
    for point in planet.iter_points().into_iter() {
        let tile = if rng.roll_dice(1, 25) % 25 == 0 {
            OverworldTile::Mountains
        } else {
            OverworldTile::Barren
        };
        let sector_info = SectorInfo::new(planet.info(), tile, SECTOR_SIZE);
        planet.set_sector_info(point, sector_info);
    }

    planet
}
