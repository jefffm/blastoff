use bracket_random::prelude::RandomNumberGenerator;

use crate::{
    galaxy::{Galaxy, GalaxyInfo, GalaxyProbability},
    overworld::PlanetInfo,
    util::{GalaxyPoint, GalaxySize, OverworldSize},
};

use super::StaticPlanet;

pub trait GalaxyGenerator {
    fn generate(&mut self, rng: &mut RandomNumberGenerator) -> Galaxy;
}

pub struct StaticGalaxy {}
impl GalaxyGenerator for StaticGalaxy {
    fn generate(&mut self, rng: &mut RandomNumberGenerator) -> Galaxy {
        let info = generate_galaxy_info(rng);
        let num_planets = rng.roll_dice(3, 6);

        // TODO: GalaxyGenerator should decide how to generate a templated number of different types of Planets
        let mut overworld_generator = StaticPlanet {};
        let mut planets: Vec<_> = (0..num_planets)
            // First, create all the OverworldInfos
            .map(|_| generate_planet_info(&info, rng))
            .collect::<Vec<_>>()
            .into_iter()
            // Then figure out which Galaxy coordinates to use for this planet
            .map(|planet_info| {
                let x = rng.roll_dice(1, info.width() - 1);
                let y = rng.roll_dice(1, info.height() - 1);

                // TODO: make it so that galaxy generation doesn't accidentally overwrite collisions
                (GalaxyPoint::new(x, y), planet_info)
            })
            .collect();

        planets.dedup_by_key(|(point, _)| *point);

        Galaxy::from_size(info).with_planet_infos(planets)
    }
}

/// Galaxy Info
fn generate_galaxy_info(rng: &mut RandomNumberGenerator) -> GalaxyInfo {
    // TODO: GalaxyInfo should have probability definitions for which types of planets to create
    let width = rng.roll_dice(3, 6);
    let height = rng.roll_dice(3, 6);

    GalaxyInfo::new(
        "Procgen Galaxy Name".to_owned(),
        GalaxySize::new(width, height),
        // TODO: load GalaxyProbability from yaml
        GalaxyProbability::default(),
    )
}

fn generate_planet_info(_info: &GalaxyInfo, rng: &mut RandomNumberGenerator) -> PlanetInfo {
    let width = rng.roll_dice(3, 6);
    let height = rng.roll_dice(3, 6);

    // TODO: use GalaxyInfo to derive a World Type (to determine the majority of terrain types and elements)
    PlanetInfo::new(
        format!("Procgen Planet Name #{}", rng.roll_dice(2, 20)),
        OverworldSize::new(width, height),
    )
}
