use hecs::{Entity, World};
use tracing::instrument;

use crate::{
    component::{Activated, Position},
    util::WorldPoint,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Moves(Entity, WorldPoint, WorldPoint),
    Activates(Entity),
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

    #[instrument(skip_all, level = "debug")]
    pub fn play_turn(&mut self, ecs: &mut World, actions: Vec<Action>) {
        for &action in actions.iter() {
            match action {
                Action::Moves(entity, current, next) => {
                    let mut pos = ecs.get::<&mut Position>(entity).unwrap();
                    pos.move_to(next);
                }
                Action::Activates(entity) => {
                    ecs.insert_one(entity, Activated {});
                }
            }
        }
        self.history.push(actions);
        self.steps += 1;
    }
}
