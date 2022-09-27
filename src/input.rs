use ggez::input::keyboard::{KeyCode, KeyInput};

#[derive(Debug, Clone)]
pub enum PlayerInput {
    Ui(UiAction),
    Game(PlayerAction),
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
    _key: Option<KeyCode>,
    pub control: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Controls {
    // look into using this(?) https://github.com/ggez/ggez-goodies/blob/master/src/input.rs
    pub fn key_down(&mut self, input: KeyInput) {
        self._key = input.keycode
    }
    pub fn key_up(&mut self, _input: KeyInput) {
        // Do nothing
    }

    pub fn read(&mut self) -> Option<KeyCode> {
        self._key.take()
    }
}
