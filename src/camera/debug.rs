use crate::{
    game::consts::{SCREEN_HEIGHT, SCREEN_WIDTH},
    map::{Map, VisibilityKind},
    resource::Viewport,
    util::{
        ScreenPoint, TransformExt, ViewportPoint, ViewportRect, ViewportSize, ViewportToScreen,
        WorldToViewport,
    },
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
    let viewport = Viewport::new(
        ViewportRect::new(
            ViewportPoint::new(0, 0),
            ViewportSize::new(SCREEN_HEIGHT as i32 - 2, SCREEN_WIDTH as i32 - 2),
        ),
        t1,
    );

    for viewport_point in viewport.points() {
        let world_point = viewport.to_world_point(viewport_point);
        if let Some(tile) = map.get(world_point) {
            let screen_point = t2.transform_point(viewport_point);
            tile.handler().render(
                draw_batch,
                screen_point,
                VisibilityKind::Torch { brightness: 50 },
            );
        }
    }

    draw_batch.print(Point::new(0, 0), format!("Index: {}", index));
    draw_batch.print(Point::new(10, 0), format!("Fps: {:.2}", ctx.fps));

    draw_batch.submit(0).expect("DrawBatch submit");
}
