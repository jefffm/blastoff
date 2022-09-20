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
    MoveEast,
    MoveWest,
    MoveNorth,
    MoveNorthEast,
    MoveNorthWest,
    MoveSouth,
    MoveSouthEast,
    MoveSouthWest,
    PassTurn,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Controls {
    key: Option<VirtualKeyCode>,
    control: bool,
    alt: bool,
    shift: bool,
}

pub fn read_game(
    controls: Controls,
    _world: &mut World,
    _resources: &mut Resources,
) -> PlayerInput {
    match controls.key {
        None => PlayerInput::Undefined,
        Some(key) => match (key, controls.control, controls.alt, controls.shift) {
            (VirtualKeyCode::Left, _, _, false) => PlayerInput::Game(PlayerAction::MoveWest),
            (VirtualKeyCode::Left, _, _, true) => PlayerInput::Game(PlayerAction::MoveSouthWest),
            (VirtualKeyCode::Right, _, _, false) => PlayerInput::Game(PlayerAction::MoveEast),
            (VirtualKeyCode::Right, _, _, true) => PlayerInput::Game(PlayerAction::MoveNorthEast),
            (VirtualKeyCode::Up, _, _, false) => PlayerInput::Game(PlayerAction::MoveNorth),
            (VirtualKeyCode::Up, _, _, true) => PlayerInput::Game(PlayerAction::MoveNorthWest),
            (VirtualKeyCode::Down, _, _, false) => PlayerInput::Game(PlayerAction::MoveSouth),
            (VirtualKeyCode::Down, _, _, true) => PlayerInput::Game(PlayerAction::MoveSouthEast),
            (VirtualKeyCode::Key1, _, _, _) => todo!("label NPCs"),
            (VirtualKeyCode::Key2, _, _, _) => todo!("label Hostiles"),
            (VirtualKeyCode::Key3, _, _, _) => todo!("label Items"),
            (VirtualKeyCode::Key4, _, _, _) => todo!("label something"),
            (VirtualKeyCode::Key5, _, _, _) => todo!("label something else"),
            (VirtualKeyCode::A, _, _, _) => todo!("skills"),
            (VirtualKeyCode::E, _, _, _) => todo!("equipment"),
            (VirtualKeyCode::I, _, _, _) => todo!("inventory"),
            (VirtualKeyCode::X, _, _, _) => todo!("character"),
            (VirtualKeyCode::L, _, _, _) => todo!("look"),
            (VirtualKeyCode::F, _, _, _) => todo!("ranged fire mode"),
            (VirtualKeyCode::Space, _, _, _) => PlayerInput::Game(PlayerAction::PassTurn),
            (VirtualKeyCode::Escape, _, _, _) => PlayerInput::Ui(UiAction::PauseMenu),
            other => {
                tracing::debug!("unhandled keypress: {:?}", other);
                PlayerInput::Undefined
            }
        },
    }
}

// TODO: return PlayerInput instead of RunState
pub fn read_mainmenu(controls: Controls, selection: MainMenuSelection) -> RunState {
    let can_continue = false; // TODO: implement save/continue
    let entries = selection.entries(can_continue);

    let result = match controls.key {
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

// TODO: return Input instead of RunState
pub fn read_pausemenu(controls: Controls, selection: PauseMenuSelection) -> RunState {
    let entries = selection.entries();

    let result = match controls.key {
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

// TODO: return Input instead of RunState
pub fn read_gameover(controls: Controls, selection: GameOverSelection) -> RunState {
    let entries = selection.entries();
    let result = match controls.key {
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
