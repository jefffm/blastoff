use ggez::graphics::Canvas;
use hecs::World;

use crate::{
    component::{Actor, Player},
    resource::Resources,
    sector::Map,
    util::PixelPoint,
};

use super::consts::{SCREEN_WIDTH_PIXELS, TILE_SIZE};

// TODO: ui should use a framework like egui

/// UI:
pub fn draw_ui(
    ctx: &mut ggez::Context,
    _canvas: &mut Canvas,
    world: &World,
    resources: &mut Resources,
    _map: &Map,
) {
    // let _turn_history = &resources.turn_history;
    // ctx.print(1, 2, format!("Steps : {}", turn_history.steps));
    // ctx.print(1, 3, format!("Energy: {}", turn_history.energy_used));
    resources.font.push_text(
        &format!("Fps: {:.2}", ctx.time.fps()),
        &PixelPoint::new(SCREEN_WIDTH_PIXELS - 20 * TILE_SIZE.width, 0),
        None,
    );

    // Implement text printing for UI
    for (_ent, (_player, actor)) in world.query::<(&Player, &Actor)>().iter() {
        resources.font.push_text(
            &format!("Energy: {:?}", actor.energy()),
            &PixelPoint::new(50, 0),
            None,
        );
        resources.font.push_text(
            &format!("Turns: {:?}", actor.turns()),
            &PixelPoint::new(50, TILE_SIZE.height),
            None,
        );
        resources.font.push_text(
            &format!("Camera pos: {:?}", resources.viewport.game_rect().center()),
            &PixelPoint::new(70, 0),
            None,
        );
    }

    // // box around top
    // draw_batch.draw_hollow_box(
    //     Rect::with_size(0, 0, SCREEN_WIDTH, TOP_BOX_HEIGHT),
    //     ColorPair::new(RGB::from(DARKSLATEBLUE), RGB::from(BLACK)),
    // );

    // // box around viewport
    // draw_batch.draw_hollow_box(
    //     Rect::with_size(
    //         VIEWPORT_SCREEN_POINT.x,
    //         VIEWPORT_SCREEN_POINT.y,
    //         VIEWPORT_WIDTH,
    //         VIEWPORT_HEIGHT - 1,
    //     ),
    //     ColorPair::new(RGB::from(DARKGOLDENROD1), RGB::from(BLACK)),
    // );

    // // box around sidebar
    // draw_batch.draw_hollow_box(
    //     Rect::with_size(
    //         VIEWPORT_WIDTH + 1,
    //         VIEWPORT_SCREEN_POINT.y,
    //         SCREEN_WIDTH as i32 - VIEWPORT_WIDTH - 2,
    //         VIEWPORT_HEIGHT - 1,
    //     ),
    //     ColorPair::new(RGB::from(DARKGREEN), RGB::from(BLACK)),
    // );
}
