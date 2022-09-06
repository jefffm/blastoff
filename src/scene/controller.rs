use bracket_lib::prelude::*;
use tracing::debug;

use crate::game;
use crate::game::RunState;
use crate::map::Map;
use crate::scene::*;

#[derive(Debug, PartialEq, Default)]
pub struct Controller {}
impl Controller {
    pub fn main_menu(&self, ctx: &mut BTerm, menu_selection: MainMenuSelection) -> RunState {
        let can_continue = false; // TODO: need to implement save/load before continue can work
        ctx.cls();
        let result = main_menu(ctx, menu_selection, can_continue);
        match result {
            MainMenuResult::NoSelection { selected } => RunState::MainMenu(selected),
            MainMenuResult::Selected { selected } => match selected {
                MainMenuSelection::NewGame => RunState::Initialization,
                MainMenuSelection::Continue => RunState::GameDraw,
                MainMenuSelection::Quit => {
                    ::std::process::exit(0);
                }
            },
        }
    }

    pub fn map_generation(
        &self,
        ctx: &mut BTerm,
        mut state: MapGenerationState,
        history: &Vec<Map>,
    ) -> RunState {
        if game::env().show_map_generation {
            debug!("Rendering debug map generation");
            if state.is_complete(history) {
                // If we're done, move on to the next state
                debug!("Map generation complete!");
                RunState::GameAwaitingInput
            } else {
                // If we have more frames to render for map generation, pass the
                // state onto the next tick.
                debug!("Rendering next tick");
                ctx.cls();
                state.render(ctx, history);
                state.tick(ctx);
                RunState::MapGeneration(state)
            }
        } else {
            RunState::GameAwaitingInput
        }
    }

    pub fn game_over(&self, ctx: &mut BTerm, menu_selection: GameOverSelection) -> RunState {
        let result = game_over(ctx, menu_selection);
        match result {
            GameOverResult::NoSelection { selected } => RunState::GameOver(selected),
            GameOverResult::Selected { selected } => match selected {
                GameOverSelection::MainMenu => RunState::MainMenu(MainMenuSelection::NewGame),
                GameOverSelection::Quit => {
                    ::std::process::exit(0);
                }
            },
        }
    }
}
