use ggez::Context;


use crate::{
    component::{Camera, Player, Position},
    overworld::SectorData,
    resource::Resources,
    util::WorldPoint,
};

// Update the viewport to be centered on the Camera position
pub fn viewport_system(resources: &mut Resources, sector: &mut SectorData, _ctx: &Context) {
    let mut player_point: Option<WorldPoint> = None;
    for (_, (pos, _player)) in sector.world.query::<(&Position, &Player)>().iter() {
        player_point = Some(pos.grid_point());
    }

    let viewport = &mut resources.viewport;

    for (_, (pos, _cam)) in sector.world.query_mut::<(&mut Position, &Camera)>() {
        let grid_point = pos.grid_point();
        let render_point = pos.render_point();

        // Update the viewport's transform using the camera's position
        viewport.update_transform(render_point);

        if let Some(player_point) = player_point {
            if grid_point != player_point {
                // Queue up moves for the camera
                // We want the camera to lag slightly behind the player
                pos.move_to_smooth(player_point, 1.);
            }
        }
    }
}
