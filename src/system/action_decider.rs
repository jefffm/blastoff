use ggez::Context;
use hecs::{Entity, World};

use crate::{
    component::{
        Actor, ActorKind, Behavior, BehaviorKind, Cardinal, DerivedBehavior, InitialBehavior,
        Position, Viewshed,
    },
    game::Action,
    map::Map,
    resource::Resources,
    util::{PointExt, WorldPoint},
};

pub struct Intention {
    entity: Entity,
    behavior: BehaviorKind,
}

impl Intention {
    pub fn into_next(
        self,
        world: &World,
        resources: &mut Resources,
        map: &Map,
        point: &WorldPoint,
        viewshed: &Viewshed,
    ) -> (BehaviorKind, Action) {
        match self.behavior {
            BehaviorKind::Initial(b) => match b {
                InitialBehavior::Wander => (
                    BehaviorKind::Initial(InitialBehavior::Wander),
                    wander(self.entity, resources),
                ),
                // Default behavior for unhandled initial behaviors
                initial => (
                    BehaviorKind::Initial(initial),
                    wander(self.entity, resources),
                ),
            },
            BehaviorKind::Derived(b) => match b {
                DerivedBehavior::FollowOrWander(target) => {
                    let target_point = target_point(world, target);
                    if viewshed.contains(&target_point) {
                        if let Some(path_next) = path_next(map, point, &target_point) {
                            return (
                                BehaviorKind::Derived(DerivedBehavior::FollowOrWander(target)),
                                Action::MovesBy(self.entity, point.get_vector(path_next)),
                            );
                        }
                    }
                    // If we've lost sight of the target, wander around until we find the target again
                    (
                        BehaviorKind::Derived(DerivedBehavior::FollowOrWander(target)),
                        wander(self.entity, resources),
                    )
                }
                // BehaviorKind::AttackOrPursue(Entity) => {},
                // BehaviorKind::AttackOrStandGround(Entity) => {},
                // BehaviorKind::AttackOrFlee(Entity) => {},
                // BehaviorKind::FollowOmniscient(Entity) => {},
                ref unimplemented => {
                    todo!("Action Decider not implemented for {:?}", &unimplemented)
                }
            },
        }
    }
}

/// Pick a random direction and walk there
pub fn wander(entity: Entity, resources: &mut Resources) -> Action {
    let possible_actions = vec![
        Action::Moves(entity, Cardinal::N),
        Action::Moves(entity, Cardinal::S),
        Action::Moves(entity, Cardinal::E),
        Action::Moves(entity, Cardinal::W),
    ];
    let idx = resources.rng.roll_dice(1, possible_actions.len() as i32) - 1;
    possible_actions[idx as usize]
}

pub fn path_next(map: &Map, start: &WorldPoint, end: &WorldPoint) -> Option<WorldPoint> {
    let result = map.astar_path(start, end);
    if let Some(path) = result {
        if path.len() > 1 {
            Some(path[1])
        } else {
            None
        }
    } else {
        None
    }
}

pub fn target_point(world: &World, target: Entity) -> WorldPoint {
    let mut q2 = world.query_one::<&Position>(target).unwrap();
    let target = q2.get().unwrap();
    target.point()
}

/// Determine which actions to take given behavior
/// For each actor, check its behavior to determine its Intention.
///
/// Process each intention, resolving each entity's target and deciding
/// on a next action and behavior for next turn.
pub fn action_decider_system(
    world: &mut World,
    resources: &mut Resources,
    map: &mut Map,
    ctx: &Context,
) {
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
    for intention in intentions.into_iter() {
        let entity = intention.entity;

        let (behavior_kind, action) = {
            let mut q1 = world
                .query_one::<(&Position, &Viewshed)>(intention.entity)
                .unwrap();
            let (position, viewshed) = q1.get().unwrap();
            let point = position.point();
            intention.into_next(world, resources, map, &point, viewshed)
        };

        let (behavior, actor) = world
            .query_one_mut::<(&mut Behavior, &mut Actor)>(entity)
            .expect("actor");

        behavior.set_kind(behavior_kind);
        actor.set_kind(ActorKind::Computer(Some(action)));
    }
}
