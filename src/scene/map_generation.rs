//! Implement a Map Generation debugging tool that allows replaying different map generation methods

use crate::game::consts::UPDATE_INTERVAL_SECS;
use crate::input::Controls;
use crate::map::Map;
use crate::resource::Resources;
use crate::util::Scene;
use crate::{camera, util::SceneSwitch};
use ggez::graphics::Canvas;
use hecs::World;

use super::MainMenu;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MapGenerationState {
    timer: f32,
    index: usize,
}

impl MapGenerationState {
    pub fn update(&mut self, seconds: f32) {
        self.timer += seconds;

        if self.timer > 3.0 {
            self.index += 1;
            self.timer = 0.0;
        }
    }

    pub fn is_complete(&self, history: &Vec<Map>) -> bool {
        self.index >= history.len()
    }
}

#[derive(Default)]
pub struct MapGeneration {
    world: World,
    history: Vec<Map>,
    state: MapGenerationState,
}
impl MapGeneration {
    pub fn new(world: World, history: Vec<Map>) -> Self {
        let state = MapGenerationState::default();
        Self {
            world,
            history,
            state,
        }
    }
}

impl Scene<Resources, Controls> for MapGeneration {
    fn input(&mut self, _resources: &mut Resources, _event: &mut Controls, _started: bool) {
        // TODO: make it so that arrow keys pan around and enter allows us to continue
    }

    fn update(
        &mut self,
        _resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        self.state.update(UPDATE_INTERVAL_SECS);

        if self.state.is_complete(&self.history) {
            // If we're done, return to the debug menu
            SceneSwitch::Pop
        } else {
            // If we have more frames to render for map generation, pass the
            // state onto the next tick.
            SceneSwitch::None
        }
    }

    fn draw(&mut self, resources: &mut Resources, canvas: &mut Canvas) -> ggez::GameResult<()> {
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
