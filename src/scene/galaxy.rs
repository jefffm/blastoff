//! The Galaxy scene allows players to select a planet to travel to, and then initialize an Overworld Scene to push to the Scene Stack.

use std::collections::HashMap;

use crate::{
    galaxy::{Galaxy, GalaxyInfo},
    input::Controls,
    overworld::{Overworld, PlanetInfo},
    resource::Resources,
    util::{GalaxyPoint, Scene},
};

pub struct GalaxyTravel {
    galaxy: Galaxy,
}

impl Default for GalaxyTravel {
    fn default() -> Self {
        // TODO: Implemnet Overworld::init_from(PlanetInfo)
        let planet = Overworld::new(PlanetInfo::new("yeaaaarg".to_owned()), HashMap::new());

        let mut galaxy_map = HashMap::new();
        galaxy_map.insert(GalaxyPoint::new(0, 0), planet);

        // TODO: Galaxy should take a list of Star Systems/planets as a vec and randomy scatter them across the sky in 2d
        // Or, should 2d represent distance between each planet...? Render planets in a top-down grid like everything else for simplicity
        // Starting location before selecting a system is just out in space somewhere
        let galaxy = Galaxy::new(GalaxyInfo::new("test system".to_owned()), galaxy_map);

        Self { galaxy }
    }
}

impl GalaxyTravel {
    pub fn new(galaxy: Galaxy) -> Self {
        // TODO: initialize GalaxyTravel with a handful of star systems + planets to choose from
        Self { galaxy }
    }
}

impl Scene<Resources, Controls> for GalaxyTravel {
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
