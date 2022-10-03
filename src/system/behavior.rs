use ggez::Context;
use hecs::{Entity, World};

use crate::{
    component::{
        Actor, Behavior, BehaviorKind, DerivedBehavior, InitialBehavior, Player, Position, Viewshed,
    },
    overworld::SectorData,
    resource::Resources,
};

/// Check Behavior for all entities and resolve any behaviors that can change
pub fn behavior_system(_resources: &mut Resources, sector: &mut SectorData, _ctx: &Context) {
    let mut updates: Vec<(Entity, BehaviorKind)> = vec![];
    for (entity, (_actor, behavior, viewshed)) in sector
        .world
        .query::<(&Actor, &Behavior, &Viewshed)>()
        .iter()
    {
        if let BehaviorKind::Initial(b) = behavior.kind() {
            match b {
                InitialBehavior::FollowNearest => {
                    // TODO: we need a way to search the viewshed in ascending distance order
                    // Find the first Actor entity with a Position in our viewshed and start following it
                    for point in viewshed.points() {
                        for map_entity in sector.map.get_content(point) {
                            if map_entity == &entity {
                                continue;
                            }

                            let mut query = sector
                                .world
                                .query_one::<(&Position, &Actor)>(*map_entity)
                                .unwrap();

                            let is_actor = query.get().is_some();

                            if is_actor && we_want_to_follow(&sector.world, map_entity) {
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
        let behavior = sector.world.query_one_mut::<&mut Behavior>(entity).unwrap();
        behavior.set_kind(behavior_kind);
    }
}

/// Currently, entities will only really want to follow the player
pub fn we_want_to_follow(world: &World, entity: &Entity) -> bool {
    let mut query = world.query_one::<&Player>(*entity).unwrap();
    let is_player = query.get().is_some();

    is_player
}
