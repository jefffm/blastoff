use bracket_lib::prelude::RandomNumberGenerator;

mod viewport;
pub use viewport::*;

use crate::{
    game::{RunState, TurnsHistory},
    map::Map,
    util::{BitmapFont, SpriteSheet},
};

pub struct Resources {
    pub rng: RandomNumberGenerator,
    pub map: Option<Map>,
    pub mapgen_history: Vec<Map>,
    pub run_state: Option<RunState>,
    pub turn_history: TurnsHistory,
    pub viewport: Viewport,
    // TODO: use newtype here to prevent issues
    pub turn_number: u32,
    pub font: BitmapFont,
    pub spritesheet: SpriteSheet,
}

impl Resources {
    pub fn take_state(&mut self) -> RunState {
        self.run_state.take().expect("run state")
    }
    pub fn replace_state(&mut self, state: RunState) {
        self.run_state = Some(state)
    }
}
