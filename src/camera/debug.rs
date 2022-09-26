use ggez::graphics::Canvas;

use crate::{
    map::{Map, VisibilityKind},
    resource::{Resources, Viewport},
    util::ViewportToScreen,
};

// Create default transforms and render the map as absolute coordinates
pub fn render_debug_map(
    _ctx: &mut ggez::Context,
    canvas: &mut Canvas,
    viewport: &mut Viewport,
    screen_transform: &ViewportToScreen,
    resources: &mut Resources,
    map: &Map,
) {
    for viewport_point in viewport.points() {
        let world_point = viewport.to_world_point(viewport_point);
        if let Some(tile) = map.get(world_point) {
            let screen_point = screen_transform.transform_point(viewport_point);
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
