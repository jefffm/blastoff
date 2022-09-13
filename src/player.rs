use bracket_lib::prelude::*;
use hecs::World;

use tracing::debug;

use crate::{
    component::{Cardinal, Player, Position},
    game::{Action, RunState},
    map::Map,
    resource::Resources,
    scene::PauseMenuSelection,
    util::WorldPoint,
};

/// Move the player by a vector2d of 1 in a single cardinal direction
pub fn try_move_player(direction: Cardinal, world: &mut World, map: &Map) -> Vec<Action> {
    let mut actions = vec![];

    for (id, (pos, _player)) in world.query::<(&Position, &Player)>().iter() {
        // Get the destination position after move
        let mut dest = *pos;
        dest.move_by(direction.to_vector());
        dest.clamp(&map.get_rect().to_box2d());
        if !map.is_blocked(&dest.p) {
            // If the move is not blocked, push it to the stack
            actions.push(Action::Moves(id, pos.p, dest.p));
        }
    }

    actions
}

/// Move the player instantly to a point
pub fn try_teleport_player(dest: WorldPoint, world: &mut World, map: &Map) -> Vec<Action> {
    let mut actions = vec![];
    for (id, (pos, _player)) in world.query::<(&Position, &Player)>().iter() {
        if pos.p != dest && !map.is_blocked(&dest) {
            actions.push(Action::Moves(id, pos.p, dest));
        }
    }

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
    let map = resources.map.as_ref().unwrap();
    let actions;
    match ctx.key {
        None => {
            return RunState::GameAwaitingInput;
        }
        Some(key) => match key {
            VirtualKeyCode::Left => actions = try_move_player(Cardinal::W, ecs, map),
            VirtualKeyCode::Right => actions = try_move_player(Cardinal::E, ecs, map),
            VirtualKeyCode::Up => actions = try_move_player(Cardinal::N, ecs, map),
            VirtualKeyCode::Down => actions = try_move_player(Cardinal::S, ecs, map),
            // VirtualKeyCode::Space => actions = try_activate(ecs, resources),
            VirtualKeyCode::Escape => return RunState::PauseMenu(PauseMenuSelection::Continue),
            other => {
                debug!("unhandled keypress: {:?}", other);
                return RunState::GameAwaitingInput;
            }
        },
    }

    if !actions.is_empty() {
        resources.turn_history.play_turn(ecs, actions);
    }
    RunState::GameTurn
}
