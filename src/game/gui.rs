use bracket_lib::prelude::*;

use crate::resource::Resources;

use super::consts::SCREEN_HEIGHT;

pub fn draw_ui(resources: &Resources, ctx: &BTerm, draw_batch: &mut DrawBatch) {
    let map = resources.map.as_ref().unwrap();
    let _turn_history = &resources.turn_history;
    draw_batch.print(Point::new(1, 0), format!("Level : {}", map.get_level()));
    // ctx.print(1, 2, format!("Steps : {}", turn_history.steps));
    // ctx.print(1, 3, format!("Energy: {}", turn_history.energy_used));
    draw_batch.print(
        Point::new(20, SCREEN_HEIGHT - 2),
        format!("Fps: {:.2}", ctx.fps),
    );
}
