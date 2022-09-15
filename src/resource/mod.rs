use bracket_lib::prelude::RandomNumberGenerator;

mod viewport;
pub use viewport::*;

use crate::{
    game::{RunState, TurnsHistory},
    input::PlayerAction,
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
    pub player_action: Option<PlayerAction>,
    // TODO: use newtype here to prevent issues
    pub turn_number: u32,
}

impl Resources {
    pub fn take_player_action_unchecked(&mut self) -> PlayerAction {
        let action = self.player_action.expect("Player action set");
        self.player_action = None;
        action
    }
}
