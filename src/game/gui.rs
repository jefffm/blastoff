use bracket_lib::prelude::*;

use crate::{resource::Resources};

pub fn draw_ui(resources: &Resources, ctx: &mut BTerm) {
    let map = resources.map.as_ref().unwrap();
    let _turn_history = &resources.turn_history;
    ctx.print(1, 1, format!("Level : {}", map.get_level()));
    // ctx.print(1, 2, format!("Steps : {}", turn_history.steps));
    // ctx.print(1, 3, format!("Energy: {}", turn_history.energy_used));
    ctx.print(20, 29, format!("Fps: {:.2}", ctx.fps));
}
