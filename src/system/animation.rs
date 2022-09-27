use ggez::Context;
use hecs::World;

use crate::{component::Animation, map::Map, resource::Resources};

/// Update all entities with animations with the current delta time
pub fn animation_system(
    world: &mut World,
    _resources: &mut Resources,
    _map: &mut Map,
    ctx: &Context,
) {
    for (_entity, animation) in world.query_mut::<&mut Animation>() {
        animation.update(ctx.time.delta().as_secs_f64())
    }
}
