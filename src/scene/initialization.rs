use hecs::World;

use crate::{
    game::{self, consts::PIXEL_RECT},
    input::Controls,
    map::{Loader, WfcGen},
    resource::Resources,
    scene::{Game, MapGeneration},
    util::{PixelPoint, Scene, SceneSwitch},
};

// TODO: Initialization should take different parameters for different types of maps and worlds to generate
// Game and MapGeneration/debug menu should both use SceneSwitch to pop themselves and pass new parameters to Initialization
#[derive(Default)]
pub struct Initialization {}
impl Scene<Resources, Controls> for Initialization {
    fn update(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        // If we're in debug mode, show the debug menu
        // Otherwise, start the game (intro scene?)

        tracing::info!("Initializing level");
        tracing::info!("Map generation");
        // Initialize mapgen history

        let mut mapgen_history = Vec::new();
        // Create the loader
        let mut loader = Loader::new(
            WfcGen {},
            // Bsp::new(WorldSize::new(50, 50)),
            &mut resources.rng,
            &mut mapgen_history,
        );

        // Load and spawn the map
        let mut world = World::default();
        let map = loader.load(1, &mut world);

        if game::env().show_map_generation {
            // View Map generation (if enabled)
            SceneSwitch::Replace(Box::new(MapGeneration::new(world, mapgen_history)))
        } else {
            SceneSwitch::Replace(Box::new(Game::new(map, world)))
        }
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult<()> {
        resources.font.draw_each_char(
            canvas,
            "Loading...",
            &PixelPoint::new(PIXEL_RECT.center().x, 0),
            None,
        );

        Ok(())
    }

    fn input(&mut self, _resources: &mut Resources, _event: Controls, _started: bool) {}
}
