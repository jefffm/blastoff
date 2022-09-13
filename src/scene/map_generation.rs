use crate::camera;
use crate::map::Map;
use bracket_lib::prelude::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MapGenerationState {
    timer: f32,
    index: usize,
}

impl MapGenerationState {
    pub fn tick(&mut self, ctx: &mut BTerm) {
        self.timer += ctx.frame_time_ms;

        if self.timer > 500.0 {
            self.index += 1;
        }
    }

    pub fn is_complete(&self, history: &Vec<Map>) -> bool {
        self.index >= history.len()
    }

    pub fn render(&self, ctx: &mut BTerm, history: &[Map]) {
        camera::render_debug_map(&history[self.index], ctx, true);
    }
}
