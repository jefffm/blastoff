//! Implement a Map Generation debugging tool that allows replaying different map generation methods
use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use hecs::World;

use crate::camera;
use crate::game::consts::UPDATE_INTERVAL_SECS;
use crate::input::Controls;
use crate::map::Map;
use crate::resource::Resources;
use crate::util::{Scene, SceneSwitch};

const MAP_SHOW_TIME: f32 = 2.0; // seconds

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MapGenerationState {
    timer: f32,
    index: usize,
}

enum MapGenerationInput {
    Skip,
}

#[derive(Default)]
pub struct MapGeneration {
    world: World,
    history: Vec<Map>,
    state: MapGenerationState,
    input: Option<MapGenerationInput>,
}
impl MapGeneration {
    pub fn new(world: World, history: Vec<Map>) -> Self {
        Self {
            world,
            history,
            state: MapGenerationState::default(),
            input: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.state.index >= self.history.len()
    }
}

impl Scene<Resources, Controls> for MapGeneration {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        self.input = match controls.read() {
            None => None,
            Some(key) => match (key, controls.control, controls.alt, controls.shift) {
                (KeyCode::Return, _, _, false) => Some(MapGenerationInput::Skip),
                _ => None,
            },
        }
    }

    fn update(
        &mut self,
        _resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        self.state.timer += UPDATE_INTERVAL_SECS;

        let skip = matches!(self.input.take(), Some(MapGenerationInput::Skip));
        if skip || self.state.timer > MAP_SHOW_TIME {
            self.state.index += 1;
            self.state.timer = 0.0;
        }

        if self.is_complete() {
            // If we're done, return to the debug menu
            SceneSwitch::Pop
        } else {
            // If we have more frames to render for map generation, pass the
            // state onto the next tick.
            SceneSwitch::None
        }
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        ctx: &mut ggez::Context,
        canvas: &mut Canvas,
    ) -> ggez::GameResult<()> {
        // TODO: implement zooming for map debug
        camera::render_debug_map(
            canvas,
            resources,
            &self.history[self.state.index],
            true,
            self.state.index,
        );

        Ok(())
    }
}
