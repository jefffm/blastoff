use ggez::graphics::Canvas;

use crate::{
    game::consts::get_screen_to_pixel_transform_float,
    resource::{Resources, Viewport},
    sector::{Map, VisibilityKind},
    util::{PixelPoint, ViewportFloatToScreen, WorldFloatPoint, WorldSpace},
};

// Create default transforms and render the map as absolute coordinates
pub fn render_debug_map(
    _ctx: &mut ggez::Context,
    canvas: &mut Canvas,
    viewport: &mut Viewport<WorldSpace>,
    screen_transform: &ViewportFloatToScreen,
    resources: &mut Resources,
    map: &Map,
) {
    for viewport_point in viewport.points() {
        let world_point = viewport.to_game_point(viewport_point);
        if let Some(tile) = map.get(world_point) {
            let pixel_point =
                worldfloat_to_pixel(resources, world_point.to_f32(), screen_transform);
            tile.render(
                canvas,
                resources,
                pixel_point,
                VisibilityKind::Torch { brightness: 50 },
            );
        }
    }

    // print(Point::new(0, 0), format!("Index: {}", index));
    // print(Point::new(10, 0), format!("Fps: {:.2}", ctx.fps));
}

/// Transform a floating point World Point allllllll the way through into an integer PixelPoint
pub fn worldfloat_to_pixel(
    resources: &Resources,
    point: WorldFloatPoint,
    screen_transform: &ViewportFloatToScreen,
) -> PixelPoint {
    let vp = resources.viewport.to_viewport_point_f32(point);
    let sp = screen_transform.transform_point(vp);

    get_screen_to_pixel_transform_float()
        .transform_point(sp)
        .to_i32()
}
