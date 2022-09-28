use ggez::Context;
use hecs::World;

use crate::{component::Renderable, map::Map, resource::Resources};

/// Update all entities with animations with the current delta time
pub fn animation_system(
    world: &mut World,
    _resources: &mut Resources,
    _map: &mut Map,
    ctx: &Context,
) {
    for (_entity, renderable) in world.query_mut::<&mut Renderable>() {
        renderable.update_time(ctx.time.delta().as_secs_f64())
    }
}
