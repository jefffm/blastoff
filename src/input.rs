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
