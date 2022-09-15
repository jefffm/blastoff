use bracket_lib::prelude::RandomNumberGenerator;

mod viewport;
pub use viewport::*;

use crate::{
    game::{RunState, TurnsHistory},
    map::Map,
    scene::Controller,
};

pub struct Resources {
    pub rng: RandomNumberGenerator,
    pub controller: Controller,
    pub map: Option<Map>,
    pub mapgen_history: Vec<Map>,
    pub run_state: RunState,
    pub turn_history: TurnsHistory,
    pub viewport: Viewport,
    // TODO: use newtype here to prevent issues
    pub turn_number: u32,
}
