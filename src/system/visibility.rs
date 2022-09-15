use bracket_lib::prelude::{field_of_view, Point};
use hecs::{Entity, World};
use std::collections::HashSet;

use crate::{
    component::{Player, Position, Viewshed},
    resource::Resources,
    util::{WorldPoint, WorldPointExt},
};

// Update the viewport to be centered on the Camera position
pub fn visibility_system(world: &mut World, resources: &mut Resources) {
    let map = resources.map.as_mut().unwrap();

    let mut updated_ents = HashSet::<Entity>::new();

    for (entity, (pos, viewshed)) in world.query_mut::<(&Position, &mut Viewshed)>() {
        if viewshed.dirty() {
            viewshed.init();

            // All FOV points should be mapped into WorldPoints
            let points = field_of_view(Point::new(pos.p.x, pos.p.y), viewshed.range(), &*map)
                .iter()
                .map(move |point| WorldPoint::from_bracket_point(*point))
                .collect();

            viewshed.set(points);

            updated_ents.insert(entity);
        }
    }

    // Update the player viewshed only if it has changed
    for (player_ent, (viewshed, _player)) in world.query_mut::<(&Viewshed, &Player)>() {
        if updated_ents.contains(&player_ent) {
            map.reset_visible();
            for point in viewshed.points() {
                map.set_visible(point);
                map.set_revealed(point);
            }
        }
    }
}
