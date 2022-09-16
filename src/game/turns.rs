use hecs::Entity;

use crate::{component::Cardinal, util::WorldPoint};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Moves(Entity, Cardinal),
    Teleports(Entity, WorldPoint),
    Activates(Entity),
    Noop,
}

impl Action {
    pub fn cost(&self) -> i32 {
        match self {
            Action::Moves(_, _) => 80,
            Action::Teleports(_, _) => 160,
            Action::Activates(_) => 100,
            Action::Noop => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TurnState {
    Running,
    PlayerDead,
    PlayerAtExit,
}

impl Default for TurnState {
    fn default() -> Self {
        Self::Running
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TurnsHistory {
    pub steps: i32,
    pub state: TurnState,
    pub history: Vec<Vec<Action>>,
}
impl TurnsHistory {
    pub fn new() -> TurnsHistory {
        TurnsHistory {
            steps: 0,
            state: TurnState::Running,
            history: vec![],
        }
    }

    pub fn add_turn(&mut self, actions: Vec<Action>) {
        self.history.push(actions);
        self.steps += 1;
    }
}
