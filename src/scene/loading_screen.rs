use hecs::World;

use crate::{
    game::consts::PIXEL_RECT,
    input::Controls,
    procgen::{seed, Combo, MapTemplate, SectorProcgenLoader, SubMap, WfcGen},
    resource::Resources,
    scene::Sector,
    sector::{FloorKind, Map, Tile},
    util::{PixelPoint, Scene, SceneSwitch, WorldPoint, WorldSize},
};

// TODO: Callback needs to somehow return to the caller the Scene (eg a Sector should be owned by an overworld)
pub type CallbackSceneFn = fn(World, Map, Vec<Map>) -> SceneSwitch<Resources, Controls>;

// TODO: Initialization should take different parameters for different types of maps and worlds to generate
// Game and MapGeneration/debug menu should both use SceneSwitch to pop themselves and pass new parameters to Initialization
pub struct LoadingScreen<S: LoaderState> {
    /// Called after generating the map for a sector
    callback_scene: CallbackSceneFn,
    state: S,
}

impl<S> LoadingScreen<S>
where
    S: LoaderState,
{
    pub fn new(callback_scene: CallbackSceneFn, state: S) -> Self {
        Self {
            callback_scene,
            state,
        }
    }
}

impl Default for LoadingScreen<NeedsSector> {
    /// Default implementation for Initialization is called to implement New Game
    fn default() -> Self {
        Self {
            // TODO: Let Planet Overworld scene use this callback to receive the generated Sector
            // TODO: since Overworld owns the Sector, let Overworld push it to the SceneStack
            callback_scene: |world: World, map: Map, _history: Vec<Map>| {
                SceneSwitch::Replace(Box::new(Sector::new(map, world)))
            },
            state: NeedsSector {},
        }
    }
}

impl Scene<Resources, Controls> for LoadingScreen<NeedsSector> {
    // TODO: somehow this needs to be non-blocking (threading + check thread status?)
    fn update(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
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
        let mut loader = SectorProcgenLoader::new(mapgen, &mut resources.rng, &mut mapgen_history);

        // Load and spawn the map
        let mut world = World::default();
        let map = loader.load(map_size, &mut world);

        (self.callback_scene)(world, map, mapgen_history)
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

pub struct Initial {}
pub struct NeedsGalaxy {}
pub struct NeedsPlanet {}
pub struct NeedsSector {}
pub struct Completed {}

pub trait LoaderState {}
impl LoaderState for Initial {}
impl LoaderState for NeedsGalaxy {}
impl LoaderState for NeedsPlanet {}
impl LoaderState for NeedsSector {}
impl LoaderState for Completed {}
