use bracket_lib::prelude::*;
use legion::*;

use tracing::debug;

use crate::{
    component::{Activated, Cardinal, Player, Position},
    game::{Action, RunState, TurnsHistory},
    map::Map,
    scene::PauseMenuSelection,
    util::WorldPoint,
};

/// Move the player by a vector2d of 1 in a single cardinal direction
pub fn try_move_player(
    direction: Cardinal,
    ecs: &mut World,
    resources: &mut Resources,
) -> Vec<Action> {
    let mut query = <(Entity, Read<Position>, Read<Player>)>::query();
    let map = resources.get::<Map>().unwrap();
    let mut actions = vec![];

    query.for_each(ecs, |(entity, pos, _player)| {
        // Get the destination position after move
        let mut dest = pos.clone();
        dest.move_by(direction.to_vector());
        dest.clamp(&map.get_rect().to_box2d());
        if !map.is_blocked(&dest.p) {
            // If the move is not blocked, push it to the stack
            actions.push(Action::Moves(*entity, pos.p, dest.p));
        }
    });
    actions
}

/// Move the player instantly to a point
pub fn try_teleport_player(
    dest: WorldPoint,
    ecs: &mut World,
    resources: &mut Resources,
) -> Vec<Action> {
    let mut query = <(Entity, Read<Position>, Read<Player>)>::query();
    let map = resources.get::<Map>().unwrap();
    let mut actions = vec![];
    query.for_each(ecs, |(entity, pos, _player)| {
        if pos.p != dest && !map.is_blocked(&dest) {
            actions.push(Action::Moves(*entity, pos.p, dest));
        }
    });

    actions
}
// pub fn try_activate(ecs: &mut World, resources: &mut Resources) -> Vec<Action> {
//     let query = <(Read<Entity>, Read<Position>)>::query().filter(component::<Player>());
//     let map = resources.get::<Map>().unwrap();
//     let mut actions = vec![];
//     query.for_each(ecs, |(entity, pos)| {
//         for (delta_x, delta_y) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
//             let surround_rect = map.iter_surrounding(pos.p);
//             for &entity in map.iter_content(pos.x + delta_x, pos.y + delta_y) {
//                 if ecs.get_component::<Actuator>(entity).is_some() {
//                     actions.push(Action::Actuates(entity));
//                     actions.push(Action::UseEnergy(1));
//                 }
//             }
//         }
//     });

//     actions
// }

pub fn game_turn_input(ecs: &mut World, resources: &mut Resources, ctx: &mut BTerm) -> RunState {
    let actions;
    match ctx.key {
        None => {
            return RunState::GameAwaitingInput;
        }
        Some(key) => match key {
            VirtualKeyCode::Left => actions = try_move_player(Cardinal::W, ecs, resources),
            VirtualKeyCode::Right => actions = try_move_player(Cardinal::E, ecs, resources),
            VirtualKeyCode::Up => actions = try_move_player(Cardinal::N, ecs, resources),
            VirtualKeyCode::Down => actions = try_move_player(Cardinal::S, ecs, resources),
            // VirtualKeyCode::Space => actions = try_activate(ecs, resources),
            VirtualKeyCode::Escape => return RunState::PauseMenu(PauseMenuSelection::Continue),
            other => {
                debug!("unhandled keypress: {:?}", other);
                return RunState::GameAwaitingInput;
            }
        },
    }

    if actions.len() > 0 {
        let mut turn_history = resources.get_mut::<TurnsHistory>().expect("TurnsHistory");
        turn_history.play_turn(ecs, actions);
    }
    RunState::GameTurn
}
