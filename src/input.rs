use ggez::input::keyboard::{KeyCode, KeyInput};
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

// TODO: Use a HashSet to store the state for each button
#[derive(Debug, Clone, Copy, Default)]
pub struct Controls {
    key: Option<KeyCode>,
    control: bool,
    alt: bool,
    shift: bool,
}

impl Controls {
    // TODO: input handling should support multiple keys at once
    // maybe don't reinvent the wheel, though: https://github.com/joetsoi/OpenMoonstone/blob/master/rust/src/main.rs#L85
    pub fn key_down(&mut self, input: KeyInput) {
        self.key = input.keycode
    }
    pub fn key_up(&mut self, input: KeyInput) {
        self.key = None
    }
}

pub fn read_game(
    controls: &Controls,
    _world: &mut World,
    _resources: &mut Resources,
) -> PlayerInput {
    match controls.key {
        None => PlayerInput::Undefined,
        Some(key) => match (key, controls.control, controls.alt, controls.shift) {
            (KeyCode::Left, _, _, false) => PlayerInput::Game(PlayerAction::MoveWest),
            (KeyCode::Left, _, _, true) => PlayerInput::Game(PlayerAction::MoveSouthWest),
            (KeyCode::Right, _, _, false) => PlayerInput::Game(PlayerAction::MoveEast),
            (KeyCode::Right, _, _, true) => PlayerInput::Game(PlayerAction::MoveNorthEast),
            (KeyCode::Up, _, _, false) => PlayerInput::Game(PlayerAction::MoveNorth),
            (KeyCode::Up, _, _, true) => PlayerInput::Game(PlayerAction::MoveNorthWest),
            (KeyCode::Down, _, _, false) => PlayerInput::Game(PlayerAction::MoveSouth),
            (KeyCode::Down, _, _, true) => PlayerInput::Game(PlayerAction::MoveSouthEast),
            (KeyCode::Key1, _, _, _) => todo!("label NPCs"),
            (KeyCode::Key2, _, _, _) => todo!("label Hostiles"),
            (KeyCode::Key3, _, _, _) => todo!("label Items"),
            (KeyCode::Key4, _, _, _) => todo!("label something"),
            (KeyCode::Key5, _, _, _) => todo!("label something else"),
            (KeyCode::A, _, _, _) => todo!("skills"),
            (KeyCode::E, _, _, _) => todo!("equipment"),
            (KeyCode::I, _, _, _) => todo!("inventory"),
            (KeyCode::X, _, _, _) => todo!("character"),
            (KeyCode::L, _, _, _) => todo!("look"),
            (KeyCode::F, _, _, _) => todo!("ranged fire mode"),
            (KeyCode::Space, _, _, _) => PlayerInput::Game(PlayerAction::PassTurn),
            (KeyCode::Escape, _, _, _) => PlayerInput::Ui(UiAction::PauseMenu),
            other => {
                tracing::debug!("unhandled keypress: {:?}", other);
                PlayerInput::Undefined
            }
        },
    }
}

// TODO: return PlayerInput instead of RunState
pub fn read_mainmenu(controls: &Controls, selection: MainMenuSelection) -> RunState {
    let can_continue = false; // TODO: implement save/continue
    let entries = selection.entries(can_continue);

    let result = match controls.key {
        None => MainMenuResult::NoSelection {
            selected: selection,
        },
        Some(key) => match key {
            KeyCode::Escape => MainMenuResult::NoSelection {
                selected: MainMenuSelection::Quit,
            },
            KeyCode::Up => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                MainMenuResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            KeyCode::Down => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                MainMenuResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            KeyCode::Return => MainMenuResult::Selected {
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
pub fn read_pausemenu(controls: &Controls, selection: PauseMenuSelection) -> RunState {
    let entries = selection.entries();

    let result = match controls.key {
        None => PauseMenuResult::NoSelection {
            selected: selection,
        },
        Some(key) => match key {
            KeyCode::Escape => PauseMenuResult::NoSelection {
                selected: PauseMenuSelection::Continue,
            },
            KeyCode::Up => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                PauseMenuResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            KeyCode::Down => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                PauseMenuResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            KeyCode::Return => PauseMenuResult::Selected {
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
pub fn read_gameover(controls: &Controls, selection: GameOverSelection) -> RunState {
    let entries = selection.entries();
    let result = match controls.key {
        None => GameOverResult::NoSelection {
            selected: selection,
        },
        Some(key) => match key {
            KeyCode::Escape => GameOverResult::NoSelection {
                selected: GameOverSelection::Quit,
            },
            KeyCode::Up => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                GameOverResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            KeyCode::Down => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                GameOverResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            KeyCode::Return => GameOverResult::Selected {
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
