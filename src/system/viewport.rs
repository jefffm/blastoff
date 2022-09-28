use ggez::Context;
use hecs::World;

use crate::{
    component::{Camera, Player, Position},
    map::Map,
    resource::Resources,
    util::WorldFloatPoint,
};

// Update the viewport to be centered on the Camera position
pub fn viewport_system(
    world: &mut World,
    resources: &mut Resources,
    _map: &mut Map,
    ctx: &Context,
) {
    let mut player_point: Option<WorldFloatPoint> = None;
    for (_, (pos, _player)) in world.query::<(&Position, &Player)>().iter() {
        player_point = Some(pos.render_point());
    }

    let viewport = &mut resources.viewport;
    for (_, (pos, _cam)) in world.query_mut::<(&mut Position, &Camera)>() {
        if let Some(player_point) = player_point {
            pos.move_to(player_point, 3.3);
            viewport.update_transform(player_point)
        }
    }
}
