use hecs::{Entity, World};

use crate::{
    component::{Actor, ActorKind, Behavior, BehaviorKind, Cardinal, Position, Viewshed},
    game::Action,
    resource::Resources,
};

pub struct BehaviorAction {
    behavior: BehaviorKind,
    action: Action,
}

pub struct Intention {
    entity: Entity,
    behavior: BehaviorKind,
}

impl Intention {
    pub fn next(&self, world: &mut World, resources: &mut Resources) -> BehaviorAction {
        match self.behavior {
            BehaviorKind::Wander => BehaviorAction {
                behavior: BehaviorKind::Wander,
                action: self.wander(resources),
            },
            BehaviorKind::Follow(target) => BehaviorAction {
                behavior: BehaviorKind::Follow(target),
                action: self.follow(world, target),
            },
            _ => todo!(),
        }
    }

    /// Pick a random direction and walk there
    pub fn wander(&self, resources: &mut Resources) -> Action {
        let possible_actions = vec![
            Action::Moves(self.entity, Cardinal::N),
            Action::Moves(self.entity, Cardinal::S),
            Action::Moves(self.entity, Cardinal::E),
            Action::Moves(self.entity, Cardinal::W),
        ];
        let idx = resources.rng.roll_dice(1, possible_actions.len() as i32) - 1;
        possible_actions[idx as usize]
    }

    pub fn follow(&self, world: &World, target: Entity) -> Action {
        // TODO: pass entity viewshed from above
        let mut q1 = world.query_one::<&Viewshed>(self.entity).unwrap();
        let viewshed = q1.get().unwrap();

        let mut q2 = world.query_one::<&Position>(target).unwrap();
        let target = q2.get().unwrap();

        if viewshed.contains(&target.point()) {
            // TODO use a-star path to move to next direction
            todo!()
        }
        Action::Noop
    }
}

/// Determine which actions to take given behavior
/// For each actor, check its behavior to determine its Intention.
///
/// Process each intention, resolving each entity's target and deciding
/// on a next action and behavior for next turn.
pub fn action_decider_system(world: &mut World, resources: &mut Resources) {
    let mut intentions: Vec<Intention> = vec![];

    // Find all entities without an action set
    for (entity, (actor, behavior)) in world.query::<(&Actor, &mut Behavior)>().iter() {
        if let ActorKind::Computer(None) = actor.kind() {
            let intention = Intention {
                entity,
                behavior: behavior.kind().clone(),
            };
            intentions.push(intention);
        }
    }

    // Process each intention to resolve next action and behavior
    for intention in intentions {
        let next = intention.next(world, resources);
        let (actor, behavior) = world
            .query_one_mut::<(&mut Actor, &mut Behavior)>(intention.entity)
            .expect("actor");

        actor.set_kind(ActorKind::Computer(Some(next.action)));
        behavior.set_kind(next.behavior)
    }
}
