

use crate::game::RunState;
use crate::game::{self, PlayGame};
use crate::map::Map;
use crate::scene::*;

#[derive(Debug, PartialEq, Default)]
pub struct Controller {}
impl Controller {
    pub fn map_generation(
        &self,
        screen: &mut [u8],
        state: MapGenerationState,
        history: &Vec<Map>,
    ) -> RunState {
        if game::env().show_map_generation {
            if state.is_complete(history) {
                // TODO: make it so that arrow keys pan around and enter allows us to continue
                // If we're done, move on to the next state
                RunState::Game(PlayGame::Ticking)
            } else {
                // If we have more frames to render for map generation, pass the
                // state onto the next tick.
                state.draw(screen, history);
                // TODO: fix update for mapgen debugger
                // state.update(ctx);
                RunState::MapGeneration(state)
            }
        } else {
            RunState::Game(PlayGame::Ticking)
        }
    }
}
