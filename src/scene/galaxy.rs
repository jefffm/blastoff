//! The Galaxy scene allows players to select a planet to travel to, and then initialize an Overworld Scene to push to the Scene Stack.

use std::collections::HashMap;

use crate::{
    galaxy::{Galaxy, GalaxyInfo},
    input::Controls,
    overworld::{Overworld, PlanetInfo},
    resource::Resources,
    util::{GalaxyPoint, Scene},
};

pub struct GalaxyTravel<G: GalaxyState> {
    galaxy: Galaxy,
    state: G,
}

impl Default for GalaxyTravel<Complete> {
    fn default() -> Self {
        // TODO: Implemnet Overworld::init_from(PlanetInfo)
        let planet = Overworld::new(PlanetInfo::new("yeaaaarg".to_owned()), HashMap::new());

        let mut galaxy_map = HashMap::new();
        galaxy_map.insert(GalaxyPoint::new(0, 0), planet);

        // TODO: Galaxy should take a list of Star Systems/planets as a vec and randomy scatter them across the sky in 2d
        // Or, should 2d represent distance between each planet...? Render planets in a top-down grid like everything else for simplicity
        // Starting location before selecting a system is just out in space somewhere
        let galaxy = Galaxy::new(GalaxyInfo::new("test system".to_owned()), galaxy_map);

        Self {
            galaxy,
            state: Complete {},
        }
    }
}

impl<G: GalaxyState> GalaxyTravel<G> {
    pub fn new(galaxy: Galaxy, state: G) -> Self {
        // TODO: initialize GalaxyTravel with a handful of star systems + planets to choose from
        Self { galaxy, state }
    }
}

impl<G: GalaxyState> Scene<Resources, Controls> for GalaxyTravel<G> {
    fn input(&mut self, resources: &mut Resources, controls: &mut Controls, started: bool) {
        // TODO: left and right cycle through planets
        todo!()
    }

    fn update(
        &mut self,
        resources: &mut Resources,
        ctx: &mut ggez::Context,
    ) -> crate::util::SceneSwitch<Resources, Controls> {
        todo!()
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult<()> {
        todo!()
    }
}

pub struct NeedsPlanets {}
pub struct Complete {}
pub trait GalaxyState {}
impl GalaxyState for NeedsPlanets {}
impl GalaxyState for Complete {}
