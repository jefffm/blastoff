use ggez::graphics::Canvas;

use crate::{
    game::consts::SCREEN_RECT,
    map::{Map, VisibilityKind},
    resource::{Resources, Viewport},
    util::{
        ScreenPoint, TransformExt, ViewportPoint, ViewportRect, ViewportSize, ViewportToScreen,
        WorldToViewport,
    },
};

// Create default transforms and render the map as absolute coordinates
pub fn render_debug_map(
    canvas: &mut Canvas,
    resources: &mut Resources,
    map: &Map,
    _show_boundaries: bool,
    index: usize,
) {
    let t1 = WorldToViewport::default();
    let t2 = ViewportToScreen::from_points(ViewportPoint::new(0, 0), ScreenPoint::new(0, 1));
    let viewport = Viewport::new(
        ViewportRect::new(
            ViewportPoint::new(0, 0),
            ViewportSize::new(SCREEN_RECT.height() - 2, SCREEN_RECT.width() - 2),
        ),
        t1,
    );

    for viewport_point in viewport.points() {
        let world_point = viewport.to_world_point(viewport_point);
        if let Some(tile) = map.get(world_point) {
            let screen_point = t2.transform_point(viewport_point);
            tile.render(
                canvas,
                resources,
                screen_point,
                VisibilityKind::Torch { brightness: 50 },
            );
        }
    }

    // print(Point::new(0, 0), format!("Index: {}", index));
    // print(Point::new(10, 0), format!("Fps: {:.2}", ctx.fps));
}
