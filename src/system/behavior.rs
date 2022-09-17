use hecs::{Entity, World};

use crate::{
    component::{Actor, Behavior, BehaviorKind, Position, Viewshed},
    resource::Resources,
};

/// Check Behavior for all entities and resolve any behaviors that can change
pub fn behavior_system(world: &mut World, resources: &mut Resources) {
    let map = resources.map.as_ref().unwrap();
    let mut updates: Vec<(Entity, BehaviorKind)> = vec![];
    for (entity, (_actor, behavior, viewshed)) in
        world.query::<(&Actor, &Behavior, &Viewshed)>().iter()
    {
        match behavior.kind() {
            BehaviorKind::FollowNearest => {
                // Find the first Actor entity with a Position in our viewshed and start following it
                for point in viewshed.points() {
                    for map_entity in map.get_content(point) {
                        let mut query =
                            world.query_one::<(&Position, &Actor)>(*map_entity).unwrap();
                        if query.get().is_some() {
                            updates.push((entity, BehaviorKind::FollowOrWander(*map_entity)));
                        }
                    }
                }
            }
            BehaviorKind::AttackPlayer => todo!(),
            BehaviorKind::AttackNearest => todo!(),
            BehaviorKind::FollowPlayer => todo!(),
            BehaviorKind::FollowPlayerOmniscient => todo!(),
            _ => {}
        }
    }

    for (entity, behavior_kind) in updates {
        let behavior = world.query_one_mut::<&mut Behavior>(entity).unwrap();
        behavior.set_kind(behavior_kind);
    }
}
