use crate::scene::{GameOverSelection, MainMenuSelection, MapGenerationState};

#[derive(Clone, PartialEq)]
pub enum RunState {
    MainMenu(MainMenuSelection),
    Initialization,
    MapGeneration(MapGenerationState),
    GameAwaitingInput,
    GameTurn,
    GameDraw,
    GameOver(GameOverSelection),
}
