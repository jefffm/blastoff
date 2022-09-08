use legion::systems::ParallelRunnable;
use legion::SystemBuilder;
use legion::*;
use tracing::debug;

use crate::component::{BlocksTile, Door, Position};
use crate::map::Map;

pub fn map_indexing_system() -> impl ParallelRunnable {
    SystemBuilder::new("map_indexing_system")
        .write_resource::<Map>()
        .with_query(<(Entity, Read<Position>)>::query())
        .with_query(<(Entity, Read<Position>, Read<Door>)>::query())
        .build(|_, ecs, map, (query1, query2)| {
            map.reset_blocked();
            map.reset_content();
            query1.for_each(ecs, |(entity, pos)| {
                map.add_content(&pos.p, entity);
                if ecs
                    .entry_ref(*entity)
                    .unwrap()
                    .get_component::<BlocksTile>()
                    .is_ok()
                {
                    map.set_blocked(&pos.p);
                }
            });
            query2.for_each(ecs, |(entity, pos, door)| {
                if !door.opened {
                    map.set_blocked(&pos.p);
                }
            });
        })
}
