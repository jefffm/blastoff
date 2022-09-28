use ggez::Context;
use hecs::World;

use crate::{
    component::{Position, Renderable},
    map::Map,
    resource::Resources,
};

/// Update all entities with animations with the current delta time
pub fn animation_system(
    world: &mut World,
    _resources: &mut Resources,
    _map: &mut Map,
    ctx: &Context,
) {
    for (_entity, (position, renderable)) in world.query_mut::<(&mut Position, &mut Renderable)>() {
        position.update_time(ctx.time.delta().as_secs_f64());
        renderable.update_time(ctx.time.delta().as_secs_f64());
    }
}
