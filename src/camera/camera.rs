use crate::{
    map::Map,
    util::{ScreenPoint, TransformExt, ViewportPoint, ViewportToScreen, WorldToViewport},
};
use bracket_lib::prelude::*;

// Create default transforms and render the map as absolute coordinates
pub fn render_debug_map(
    ctx: &BTerm,
    draw_batch: &mut DrawBatch,
    map: &Map,
    _show_boundaries: bool,
    index: usize,
) {
    let t1 = WorldToViewport::default();
    let t2 = ViewportToScreen::from_points(ViewportPoint::new(0, 0), ScreenPoint::new(0, 1));

    for (point, tile) in map.iter_tiles() {
        let viewport_point = t1.transform_point(point);
        let screen_point = t2.transform_point(viewport_point);
        tile.render(draw_batch, screen_point);
    }

    draw_batch.print(Point::new(0, 0), format!("Index: {}", index));
    draw_batch.print(Point::new(10, 0), format!("Fps: {:.2}", ctx.fps));

    draw_batch.submit(0).expect("DrawBatch submit");
}
