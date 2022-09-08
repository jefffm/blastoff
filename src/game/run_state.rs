use crate::scene::{GameOverSelection, MainMenuSelection, MapGenerationState, PauseMenuSelection};

#[derive(Debug, Clone, PartialEq)]
pub enum RunState {
    MainMenu(MainMenuSelection),
    PauseMenu(PauseMenuSelection),
    Initialization,
    MapGeneration(MapGenerationState),
    GameAwaitingInput,
    GameTurn,
    GameDraw,
    GameOver(GameOverSelection),
}
