use bracket_lib::prelude::*;
use legion::*;

use crate::map::Map;

use crate::game::TurnsHistory;

pub fn draw_ui(resources: &Resources, ctx: &mut BTerm) {
    let map = resources.get::<Map>().unwrap();
    // let turn_history = resources.get::<TurnsHistory>().expect("TurnsHistory");
    ctx.print(1, 1, format!("Level : {}", map.get_level()));
    // ctx.print(1, 2, format!("Steps : {}", turn_history.steps));
    // ctx.print(1, 3, format!("Energy: {}", turn_history.energy_used));
    ctx.print(20, 29, format!("Fps: {:.2}", ctx.fps));
}
