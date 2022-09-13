use crate::{
    map::Map,
    util::{ScreenPoint, TransformExt, ViewportPoint, ViewportToScreen, WorldToViewport},
};
use bracket_lib::prelude::*;

// Create default transforms and render the map as absolute coordinates
pub fn render_debug_map(map: &Map, ctx: &mut BTerm, _show_boundaries: bool, index: usize) {
    for (point, tile) in map.iter_tiles() {
        let t1 = WorldToViewport::default();
        let t2 = ViewportToScreen::from_points(ViewportPoint::new(0, 0), ScreenPoint::new(0, 1));
        let viewport_point = t1.transform_point(point);
        let screen_point = t2.transform_point(viewport_point);
        tile.render(ctx, screen_point);

        ctx.print(0, 0, format!("Index: {}", index))
    }
}
