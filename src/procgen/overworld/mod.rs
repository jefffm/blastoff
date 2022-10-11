mod name;
pub use name::*;

use crate::{
    data::{Element, PlanetType},
    overworld::{Overworld, PlanetInfo},
    resource::Resources,
};

pub trait OverworldGenerator {
    fn generate(&mut self, planet_info: PlanetInfo, resources: &mut Resources) -> Overworld;
}

pub struct StaticPlanet {}
impl OverworldGenerator for StaticPlanet {
    fn generate(&mut self, info: PlanetInfo, _resources: &mut Resources) -> Overworld {
        let mut planet = Overworld::from_info(info);

        let planet: Overworld = match planet.info().planet_type {
            PlanetType::Barren => create_barren_planet(planet),
            PlanetType::Lush => create_lush_planet(planet),
            PlanetType::Aqueus => create_aqueus_planet(planet),
            PlanetType::Mountains => create_mountain_planet(planet),
        };

        planet
    }
}

fn create_mountain_planet(mut planet: Overworld) -> Overworld {
    // TODO: create mountain planet
    let sector_infos: Vec<_> = planet.iter_sector_infos().collect();
    for (point, sector_info) in sector_infos {}

    planet
}

fn create_aqueus_planet(mut planet: Overworld) -> Overworld {
    // TODO: create aqueus planet
    planet
}

fn create_lush_planet(mut planet: Overworld) -> Overworld {
    // TODO: create lush planet
    planet
}

fn create_barren_planet(mut planet: Overworld) -> Overworld {
    // TODO: create barren planet
    planet
}
