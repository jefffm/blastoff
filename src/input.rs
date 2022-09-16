use bracket_lib::prelude::{BTerm, VirtualKeyCode};
use hecs::World;

use crate::{
    game::{PlayGame, RunState},
    resource::Resources,
    scene::{
        GameOverResult, GameOverSelection, MainMenuResult, MainMenuSelection, PauseMenuResult,
        PauseMenuSelection,
    },
};

#[derive(Debug, Clone)]
pub enum PlayerInput {
    Ui(UiAction),
    Game(PlayerAction),
    Undefined,
}

#[derive(Debug, Clone)]
pub enum UiAction {
    MainMenu,
    PauseMenu,
    GameOverMenu,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerAction {
    MoveWest,
    MoveEast,
    MoveNorth,
    MoveSouth,
    PassTurn,
}

pub fn read_game(_world: &mut World, _resources: &mut Resources, ctx: &mut BTerm) -> PlayerInput {
    match ctx.key {
        None => PlayerInput::Undefined,
        Some(key) => match key {
            VirtualKeyCode::Left => PlayerInput::Game(PlayerAction::MoveWest),
            VirtualKeyCode::Right => PlayerInput::Game(PlayerAction::MoveEast),
            VirtualKeyCode::Up => PlayerInput::Game(PlayerAction::MoveNorth),
            VirtualKeyCode::Down => PlayerInput::Game(PlayerAction::MoveSouth),
            VirtualKeyCode::Space => PlayerInput::Game(PlayerAction::PassTurn),
            VirtualKeyCode::Escape => PlayerInput::Ui(UiAction::PauseMenu),
            other => {
                tracing::debug!("unhandled keypress: {:?}", other);
                PlayerInput::Undefined
            }
        },
    }
}

pub fn read_mainmenu(selection: MainMenuSelection, ctx: &BTerm) -> RunState {
    let can_continue = false; // TODO: implement save/continue
    let entries = selection.entries(can_continue);

    let result = match ctx.key {
        None => MainMenuResult::NoSelection {
            selected: selection,
        },
        Some(key) => match key {
            VirtualKeyCode::Escape => MainMenuResult::NoSelection {
                selected: MainMenuSelection::Quit,
            },
            VirtualKeyCode::Up => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                MainMenuResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            VirtualKeyCode::Down => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                MainMenuResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            VirtualKeyCode::Return => MainMenuResult::Selected {
                selected: selection,
            },
            _ => MainMenuResult::NoSelection {
                selected: selection,
            },
        },
    };

    match result {
        MainMenuResult::NoSelection { selected } => RunState::MainMenu(selected),
        MainMenuResult::Selected { selected } => match selected {
            MainMenuSelection::NewGame => RunState::Initialization,
            MainMenuSelection::Continue => RunState::Game(PlayGame::Ticking),
            MainMenuSelection::Quit => {
                ::std::process::exit(0);
            }
        },
    }
}

pub fn read_pausemenu(selection: PauseMenuSelection, ctx: &BTerm) -> RunState {
    let entries = selection.entries();

    let result = match ctx.key {
        None => PauseMenuResult::NoSelection {
            selected: selection,
        },
        Some(key) => match key {
            VirtualKeyCode::Escape => PauseMenuResult::NoSelection {
                selected: PauseMenuSelection::Continue,
            },
            VirtualKeyCode::Up => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                PauseMenuResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            VirtualKeyCode::Down => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                PauseMenuResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            VirtualKeyCode::Return => PauseMenuResult::Selected {
                selected: selection,
            },
            _ => PauseMenuResult::NoSelection {
                selected: selection,
            },
        },
    };
    match result {
        PauseMenuResult::NoSelection { selected } => RunState::PauseMenu(selected),
        PauseMenuResult::Selected { selected } => match selected {
            PauseMenuSelection::Continue => RunState::Game(PlayGame::Ticking),
            PauseMenuSelection::ExitToMainMenu => RunState::MainMenu(MainMenuSelection::NewGame),
        },
    }
}

pub fn read_gameover(selection: GameOverSelection, ctx: &BTerm) -> RunState {
    let entries = selection.entries();
    let result = match ctx.key {
        None => GameOverResult::NoSelection {
            selected: selection,
        },
        Some(key) => match key {
            VirtualKeyCode::Escape => GameOverResult::NoSelection {
                selected: GameOverSelection::Quit,
            },
            VirtualKeyCode::Up => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                GameOverResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            VirtualKeyCode::Down => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                GameOverResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            VirtualKeyCode::Return => GameOverResult::Selected {
                selected: selection,
            },
            _ => GameOverResult::NoSelection {
                selected: selection,
            },
        },
    };
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
