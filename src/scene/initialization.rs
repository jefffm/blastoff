use hecs::World;

use crate::{
    game::consts::PIXEL_RECT,
    input::Controls,
    map::{seed, Combo, FloorKind, Loader, Map, MapTemplate, SubMap, Tile, WfcGen},
    resource::Resources,
    scene::Sector,
    util::{PixelPoint, Scene, SceneSwitch, WorldPoint, WorldSize},
};

pub type CreateSceneFn = fn(World, Map, Vec<Map>) -> SceneSwitch<Resources, Controls>;

// TODO: Initialization should take different parameters for different types of maps and worlds to generate
// Game and MapGeneration/debug menu should both use SceneSwitch to pop themselves and pass new parameters to Initialization
pub struct Initialization {
    create_scene: CreateSceneFn,
}

impl Initialization {
    pub fn new(create_scene: CreateSceneFn) -> Self {
        Self { create_scene }
    }
}

impl Default for Initialization {
    /// Default implementation for Initialization is called to implement New Game
    fn default() -> Self {
        Self {
            create_scene: |world: World, map: Map, _history: Vec<Map>| {
                SceneSwitch::Replace(Box::new(Sector::new(map, world)))
            },
        }
    }
}

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

        // TODO: Generate a Planet's Overworld first. Then, use the Overworld point to determine which Sector template to use for initialization.
        // TODO: Abstract this into a Sector Template
        let map_size = WorldSize::new(100, 100);
        let mapgen = Combo::new(MapTemplate::new(
            map_size,
            Tile::Floor(FloorKind::FloorScenery('~')),
            vec![
                // First create an entire map of craters
                SubMap::new(
                    Box::new(WfcGen::new(seed::CRATERS)),
                    map_size,
                    WorldPoint::new(0, 0),
                ),
                // Then, create a city in the middle
                SubMap::new(
                    Box::new(WfcGen::new(seed::CITY)),
                    WorldSize::new(50, 50),
                    WorldPoint::new(25, 25),
                ),
            ],
        ));

        // Create the loader
        let mut loader = Loader::new(mapgen, &mut resources.rng, &mut mapgen_history);

        // Load and spawn the map
        let mut world = World::default();
        let map = loader.load(map_size, &mut world);

        (self.create_scene)(world, map, mapgen_history)
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
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

    fn input(&mut self, _resources: &mut Resources, _event: &mut Controls, _started: bool) {}
}
