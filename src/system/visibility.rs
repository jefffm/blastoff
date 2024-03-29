use hecs::Entity;
use std::collections::HashSet;
use symmetric_shadowcasting::compute_fov;

use crate::{
    component::{Player, Position, Viewshed},
    overworld::SectorData,
    resource::Resources,
    util::{WorldFloatPoint, WorldPoint},
};

// Update the viewport to be centered on the Camera position
pub fn visibility_system(_resources: &mut Resources, sector: &mut SectorData) {
    let mut updated_ents = HashSet::<Entity>::new();

    for (entity, (pos, viewshed)) in sector.world.query_mut::<(&Position, &mut Viewshed)>() {
        if viewshed.dirty() {
            viewshed.init();

            let point = pos.grid_point();
            let origin = (point.x as isize, point.y as isize);
            let range = viewshed.range();

            let in_range = |other: &WorldPoint| {
                let p1 = WorldFloatPoint::new(point.x as f32, point.y as f32);
                let p2 = WorldFloatPoint::new(other.x as f32, other.y as f32);
                let distance = p1.distance_to(p2);
                distance <= range as f32
            };

            // TODO: add something to PointExt to convert into this isize tuple
            let mut is_blocking = |(x, y)| {
                let point = WorldPoint::new(x as i32, y as i32);
                if !sector.map.contains(point) {
                    return true;
                }
                sector.map.is_opaque_point(&point)
            };

            let mut mark_visible = |(x, y)| {
                let point = WorldPoint::new(x as i32, y as i32);
                if in_range(&point) && sector.map.contains(point) {
                    viewshed.insert(point)
                }
            };
            compute_fov(origin, &mut is_blocking, &mut mark_visible);

            updated_ents.insert(entity);
        }
    }

    // Update the player viewshed only if it has changed
    let mut query = sector.world.query::<(&Viewshed, &Player)>();
    let players: Vec<_> = query.iter().collect();
    let update_player_viewsheds = players.iter().any(|(ent, _)| updated_ents.contains(ent));

    if update_player_viewsheds {
        sector.map.reset_visible();
        for (_, (viewshed, _)) in players {
            for point in viewshed.points() {
                sector.map.set_visible(point);
                sector.map.set_revealed(point);
            }
        }
    }
}
