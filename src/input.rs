use bracket_lib::prelude::{BTerm, VirtualKeyCode};
use hecs::World;

use crate::resource::Resources;

#[derive(Debug, Clone)]
pub enum PlayerInput {
    Ui(UiAction),
    Game(PlayerAction),
    Undefined,
}

#[derive(Debug, Clone)]
pub enum UiAction {
    PauseMenu,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerAction {
    MoveWest,
    MoveEast,
    MoveNorth,
    MoveSouth,
    PassTurn,
}

pub fn read(_world: &mut World, _resources: &mut Resources, ctx: &mut BTerm) -> PlayerInput {
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
