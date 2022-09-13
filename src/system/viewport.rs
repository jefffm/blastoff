use hecs::World;

use crate::component::{Camera, Player, Position};
use crate::resource::{Resources, Viewport};
use crate::util::WorldPoint;

/// Keep the camera focused on the player's location
pub fn viewport_system(world: &mut World, _resources: &mut Resources) {
    let mut player_position: Option<WorldPoint> = None;
    for (_, (pos, _player)) in world.query::<(&Position, &Player)>().iter() {
        player_position = Some(pos.p);
    }

    for (_, (pos, _camera)) in world.query::<(&mut Position, &Camera)>().iter() {
        if let Some(p) = player_position {
            pos.move_to(p);
        };
    }
}
