use crate::{
    map::Map,
    util::{ViewportToScreen, WorldToViewport},
};
use bracket_lib::prelude::*;

// Create default transforms and render the map as absolute coordinates
pub fn render_debug_map(map: &Map, ctx: &mut BTerm, _show_boundaries: bool) {
    for (point, tile) in map.iter_tiles() {
        let t1 = WorldToViewport::default();
        let t2 = ViewportToScreen::default();
        let viewport_point = t1.transform_point(point);
        let screen_point = t2.transform_point(viewport_point);
        tile.render(ctx, screen_point);
    }
}
