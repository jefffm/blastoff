use bracket_lib::prelude::*;

use crate::game;
use crate::game::RunState;
use crate::map::Map;
use crate::scene::*;

#[derive(Debug, PartialEq, Default)]
pub struct Controller {}
impl Controller {
    pub fn main_menu(
        &self,
        ctx: &mut BTerm,
        draw_batch: &mut DrawBatch,
        menu_selection: MainMenuSelection,
        can_continue: bool,
    ) -> RunState {
        let result = main_menu(ctx, draw_batch, menu_selection, can_continue);
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

    pub fn pause_menu(
        &self,
        ctx: &mut BTerm,
        draw_batch: &mut DrawBatch,
        menu_selection: PauseMenuSelection,
    ) -> RunState {
        let result = pause_menu(ctx, draw_batch, menu_selection);
        match result {
            PauseMenuResult::NoSelection { selected } => RunState::PauseMenu(selected),
            PauseMenuResult::Selected { selected } => match selected {
                PauseMenuSelection::Continue => RunState::GameDraw,
                PauseMenuSelection::ExitToMainMenu => {
                    RunState::MainMenu(MainMenuSelection::NewGame)
                }
            },
        }
    }

    pub fn map_generation(
        &self,
        ctx: &mut BTerm,
        draw_batch: &mut DrawBatch,
        mut state: MapGenerationState,
        history: &Vec<Map>,
    ) -> RunState {
        if game::env().show_map_generation {
            if state.is_complete(history) {
                // If we're done, move on to the next state
                RunState::GameTurn
            } else {
                // If we have more frames to render for map generation, pass the
                // state onto the next tick.
                state.render(ctx, draw_batch, history);
                state.tick(ctx);
                RunState::MapGeneration(state)
            }
        } else {
            RunState::GameTurn
        }
    }

    pub fn game_over(
        &self,
        ctx: &mut BTerm,
        draw_batch: &mut DrawBatch,
        menu_selection: GameOverSelection,
    ) -> RunState {
        let result = game_over(ctx, draw_batch, menu_selection);
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
