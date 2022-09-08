use bracket_lib::prelude::Point;
use legion::world::Entity;
use legion::*;

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

#[derive(Debug, Clone, PartialEq)]
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

    #[instrument(level = "debug")]
    pub fn play_turn(&mut self, ecs: &mut World, actions: Vec<Action>) {
        for &action in actions.iter() {
            match action {
                Action::Moves(entity, current, next) => {
                    let mut entry = ecs.entry(entity).unwrap();
                    let pos = entry.get_component_mut::<Position>().unwrap();
                    pos.move_to(next);
                }
                Action::Activates(entity) => {
                    let mut entry = ecs.entry(entity).unwrap();
                    entry.add_component(Activated {});
                }
            }
        }
        self.history.push(actions);
        self.steps += 1;
    }
}
