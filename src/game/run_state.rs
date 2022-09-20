use crate::scene::{GameOverSelection, MainMenuSelection, MapGenerationState, PauseMenuSelection};

#[derive(Debug, Clone, PartialEq)]
pub enum RunState {
    MainMenu(MainMenuSelection),
    PauseMenu(PauseMenuSelection),
    Initialization,
    MapGeneration(MapGenerationState),
    Game(PlayGame),
    GameOver(GameOverSelection),
    Exiting,
}

// If the RunState is a GameState, we need to do INPUT, UPDATE, and DRAW every frame
#[derive(Debug, Clone, PartialEq)]
pub enum PlayGame {
    Ticking,
    NeedPlayerInput,
}
