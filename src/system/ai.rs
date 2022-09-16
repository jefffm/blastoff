use hecs::World;

use crate::{
    component::{Actor, ActorKind, Cardinal},
    game::Action,
    resource::Resources,
};

pub fn ai_system(world: &mut World, resources: &mut Resources) {
    for (ent, actor) in world.query_mut::<&mut Actor>() {
        if let ActorKind::Computer(None) = actor.kind() {
            // TODO: use another component to track behavior types

            let possible_actions = vec![
                Action::Moves(ent, Cardinal::N),
                Action::Moves(ent, Cardinal::S),
                Action::Moves(ent, Cardinal::E),
                Action::Moves(ent, Cardinal::W),
            ];

            let idx = resources.rng.roll_dice(1, possible_actions.len() as i32) - 1;

            actor.set_kind(ActorKind::Computer(Some(possible_actions[idx as usize])));
        }
    }
}
