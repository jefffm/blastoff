//! Planet implements an Overworld-syle scene for the player to travel between sectors

use crate::{input::Controls, overworld::Overworld, resource::Resources, util::Scene};

pub struct PlanetOverworld {
    overworld: Overworld,
}

impl PlanetOverworld {
    pub fn new(overworld: Overworld) -> Self {
        Self { overworld }
    }
}
impl Scene<Resources, Controls> for PlanetOverworld {
    fn input(&mut self, resources: &mut Resources, controls: &mut Controls, started: bool) {
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