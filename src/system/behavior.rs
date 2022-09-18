use hecs::{Entity, World};

use crate::{
    component::{
        Actor, Behavior, BehaviorKind, DerivedBehavior, InitialBehavior, Position, Viewshed,
    },
    resource::Resources,
};

/// Check Behavior for all entities and resolve any behaviors that can change
pub fn behavior_system(world: &mut World, resources: &mut Resources) {
    let map = resources.map.as_ref().unwrap();
    let mut updates: Vec<(Entity, BehaviorKind)> = vec![];
    for (entity, (_actor, behavior, viewshed)) in
        world.query::<(&Actor, &Behavior, &Viewshed)>().iter()
    {
        if let BehaviorKind::Initial(b) = behavior.kind() {
            match b {
                InitialBehavior::FollowNearest => {
                    // TODO: we need a way to search the viewshed in ascending distance order
                    // Find the first Actor entity with a Position in our viewshed and start following it
                    for point in viewshed.points() {
                        for map_entity in map.get_content(point) {
                            if map_entity == &entity {
                                continue;
                            }

                            let mut query =
                                world.query_one::<(&Position, &Actor)>(*map_entity).unwrap();

                            if query.get().is_some() {
                                updates.push((
                                    entity,
                                    BehaviorKind::Derived(DerivedBehavior::FollowOrWander(
                                        *map_entity,
                                    )),
                                ));
                            }
                        }
                    }
                }
                InitialBehavior::AttackPlayer => todo!(),
                InitialBehavior::AttackNearest => todo!(),
                InitialBehavior::FollowPlayer => todo!(),
                InitialBehavior::FollowPlayerOmniscient => todo!(),
                _ => {}
            }
        }
    }

    for (entity, behavior_kind) in updates {
        let behavior = world.query_one_mut::<&mut Behavior>(entity).unwrap();
        behavior.set_kind(behavior_kind);
    }
}
