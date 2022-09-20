use crate::camera;
use crate::map::Map;
use bracket_lib::prelude::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MapGenerationState {
    timer: f32,
    index: usize,
}

impl MapGenerationState {
    pub fn update(&mut self, ctx: &BTerm) {
        self.timer += ctx.frame_time_ms;

        if self.timer > 50.0 {
            self.index += 1;
            self.timer = 0.0;
        }
    }

    pub fn is_complete(&self, history: &Vec<Map>) -> bool {
        self.index >= history.len()
    }

    pub fn draw(&self, screen: &mut [u8], history: &[Map]) {
        camera::render_debug_map(screen, &history[self.index], true, self.index);
    }
}
