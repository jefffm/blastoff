use crate::{
    data::GalaxyProbability,
    galaxy::{Galaxy, GalaxyInfo},
    resource::Resources,
    util::{GalaxyPoint, GalaxySize},
};

use super::generate_planet_name;

pub trait GalaxyGenerator {
    fn generate(&mut self, resources: &mut Resources) -> Galaxy;
}

pub struct StaticGalaxy {}
impl GalaxyGenerator for StaticGalaxy {
    fn generate(&mut self, resources: &mut Resources) -> Galaxy {
        let info = generate_galaxy_info(resources);
        let num_planets = resources.rng.roll_dice(3, 6);

        let mut planets: Vec<_> = (0..num_planets)
            // First, create all the OverworldInfos
            .map(|_| {
                let name = generate_planet_name(resources);
                tracing::warn!("Generated planet name: {}", &name);
                let rng = &mut resources.rng;
                info.probability.roll_planet(name.to_owned(), rng)
            })
            .collect::<Vec<_>>()
            .into_iter()
            // Then figure out which Galaxy coordinates to use for this planet
            .map(|planet_info| {
                let x = resources.rng.roll_dice(1, info.width() - 1);
                let y = resources.rng.roll_dice(1, info.height() - 1);

                // TODO: make it so that galaxy generation doesn't accidentally overwrite collisions
                (GalaxyPoint::new(x, y), planet_info)
            })
            .collect();

        planets.dedup_by_key(|(point, _)| *point);

        Galaxy::from_size(info).with_planet_infos(planets)
    }
}

/// Galaxy Info
fn generate_galaxy_info(resources: &mut Resources) -> GalaxyInfo {
    // TODO: GalaxyInfo should have probability definitions for which types of planets to create
    let width = resources.rng.roll_dice(3, 6);
    let height = resources.rng.roll_dice(3, 6);

    // Load GalaxyProbability from assets data files
    let galaxy_probability = resources.load_asset::<GalaxyProbability>("data.galaxy_probability");

    GalaxyInfo::new(
        "Procgen Galaxy Name".to_owned(),
        GalaxySize::new(width, height),
        galaxy_probability.read().clone(),
    )
}
