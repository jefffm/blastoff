use crate::camera;
use crate::map::Map;
use crate::resource::Resources;
use bracket_lib::prelude::*;
use ggez::graphics::Canvas;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MapGenerationState {
    timer: f32,
    index: usize,
}

impl MapGenerationState {
    pub fn update(&mut self, seconds: f32) {
        tracing::warn!("timer: {:?}", self.timer);
        self.timer += seconds;

        if self.timer > 3.0 {
            self.index += 1;
            self.timer = 0.0;
        }
    }

    pub fn is_complete(&self, history: &Vec<Map>) -> bool {
        tracing::warn!("history: {:?} and index: {:?}", history.len(), self.index);
        self.index >= history.len()
    }

    pub fn draw(&self, canvas: &mut Canvas, resources: &mut Resources) {
        camera::render_debug_map(canvas, resources, true, self.index);
    }
}
