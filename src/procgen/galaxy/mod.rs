use bracket_random::prelude::RandomNumberGenerator;

use crate::{
    galaxy::{Galaxy, GalaxyInfo},
    util::{GalaxyPoint, GalaxySize},
};

use super::{OverworldGenerator, StaticPlanet};

pub trait GalaxyGenerator {
    fn generate(&mut self, rng: &mut RandomNumberGenerator) -> Galaxy;
}

pub struct StaticGalaxy {}
impl StaticGalaxy {
    fn generate_galaxy_info(rng: &mut RandomNumberGenerator) -> GalaxyInfo {
        let width = rng.roll_dice(3, 6);
        let height = rng.roll_dice(3, 6);
        GalaxyInfo::new(
            "Procgen Galaxy Name".to_owned(),
            GalaxySize::new(width, height),
        )
    }
}
impl GalaxyGenerator for StaticGalaxy {
    fn generate(&mut self, rng: &mut RandomNumberGenerator) -> Galaxy {
        let info = Self::generate_galaxy_info(rng);
        let num_planets = rng.roll_dice(3, 6);

        // TODO: GalaxyGenerator should decide how to generate a templated number of different types of Planets
        let mut overworld_generator = StaticPlanet {};
        let planets: Vec<_> = (0..num_planets)
            // First, create all the OverworldInfos
            .map(|_| StaticPlanet::generate_overworld_info(&info, rng))
            .collect::<Vec<_>>()
            .into_iter()
            // Then, generate a Planet from each of them
            .map(|planet_info| overworld_generator.generate(planet_info, rng))
            .collect::<Vec<_>>()
            .into_iter()
            // Finally, figure out which Galaxy coordinates to use for this planet
            .map(|planet| {
                let x = rng.roll_dice(1, info.width() - 1);
                let y = rng.roll_dice(1, info.height() - 1);

                // TODO: make it so that galaxy generation doesn't accidentally overwrite collisions
                (GalaxyPoint::new(x, y), planet)
            })
            .collect();

        // TODO: Distribute planets across the 2d space
        Galaxy::from_size(info).with_planets(planets)
    }
}
