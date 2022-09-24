use ggez::graphics::Canvas;
use hecs::World;

use crate::{
    component::{Actor, Player},
    map::Map,
    resource::Resources,
};

/// UI:
pub fn draw_ui(_canvas: &mut Canvas, world: &World, _resources: &Resources, _map: &Map) {
    // let _turn_history = &resources.turn_history;
    // print(Point::new(1, 0), format!("Level : {}", map.get_level()));
    // ctx.print(1, 2, format!("Steps : {}", turn_history.steps));
    // ctx.print(1, 3, format!("Energy: {}", turn_history.energy_used));
    // print(Point::new(20, 0), format!("Fps: {:.2}", ctx.fps));

    // Implement text printing for UI
    for (_ent, (_player, _actor)) in world.query::<(&Player, &Actor)>().iter() {
        // print(Point::new(50, 0), format!("Energy: {:?}", actor.energy()));
        // print(Point::new(50, 1), format!("Turns: {:?}", actor.turns()));
    }

    // TODO: create relative coordinate systems for the two ui rects

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
