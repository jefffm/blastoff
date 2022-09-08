use legion::systems::ParallelRunnable;
use legion::SystemBuilder;
use legion::*;

use crate::component::{Camera, Player, Position};
use crate::resource::Viewport;
use crate::util::WorldPoint;

/// Keep the camera focused on the player's location
pub fn viewport_system() -> impl ParallelRunnable {
    SystemBuilder::new("viewport_system")
        .write_resource::<Viewport>()
        .with_query(<(Entity, Read<Player>, Read<Position>)>::query())
        .with_query(<(Entity, Read<Camera>, Write<Position>)>::query())
        .build(|_, ecs, viewport, (player_query, camera_query)| {
            let mut player_position: Option<WorldPoint> = None;
            player_query.for_each(ecs, |(entity, camera, pos)| player_position = Some(pos.p));
            camera_query.for_each_mut(ecs, |(entity, camera, pos)| {
                if let Some(p) = player_position {
                    pos.move_to(p);
                };
            });
        })
}
