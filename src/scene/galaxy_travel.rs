//! The Galaxy scene allows players to select a planet to travel to, and then initialize an Overworld Scene to push to the Scene Stack.

use crate::{
    galaxy::{Galaxy, GalaxyInfo},
    game::consts::{MAX_PLANET_SPRITE_SIZE, TILE_SIZE},
    input::Controls,
    procgen::{GalaxyGenerator, StaticGalaxy},
    resource::Resources,
    util::{GalaxySize, PixelPoint, Scene, SceneSwitch},
};

/// The GalaxyTravel Scene allows players to select a planet for landing
pub struct GalaxyTravel {
    galaxy: Galaxy,
}

impl GalaxyTravel {
    pub fn new(galaxy: Galaxy) -> Self {
        Self { galaxy }
    }

    /// Use Resources to procgen a Galaxy of planets
    pub fn create(resources: &mut Resources) -> GalaxyTravel {
        let mut loader = StaticGalaxy {};
        let galaxy = loader.generate(&mut resources.rng);

        Self::new(galaxy)
    }
}

impl Scene<Resources, Controls> for GalaxyTravel {
    fn input(&mut self, resources: &mut Resources, controls: &mut Controls, started: bool) {
        // TODO: left and right cycle through planets
    }

    fn update(
        &mut self,
        resources: &mut Resources,
        ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        // TODO: read input and determine where to put the selection rectangle

        SceneSwitch::None
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult<()> {
        for (i, (point, planet)) in self.galaxy.iter_content().enumerate() {
            let y = i as i32 * MAX_PLANET_SPRITE_SIZE as i32;

            resources.font.push_text(
                &format!("{} at {:?}", planet, point),
                &PixelPoint::new(2 * MAX_PLANET_SPRITE_SIZE as i32, y),
                None,
            );

            resources
                .spritesheet
                .push_sprite(planet.sprite(), PixelPoint::new(1 * TILE_SIZE.width, y));
        }

        Ok(())
    }
}
