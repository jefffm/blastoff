mod name;
pub use name::*;

use crate::{
    overworld::{Overworld, PlanetInfo},
    resource::Resources,
};

pub trait OverworldGenerator {
    fn generate(&mut self, planet_info: PlanetInfo, resources: &mut Resources) -> Overworld;
}

pub struct StaticPlanet {}
impl OverworldGenerator for StaticPlanet {
    fn generate(&mut self, info: PlanetInfo, _resources: &mut Resources) -> Overworld {
        

        // TODO: generate terrain using overworld info and rng
        Overworld::from_info(info)
    }
}
