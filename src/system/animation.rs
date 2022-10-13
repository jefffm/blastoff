use crate::{
    component::{Position, Renderable},
    overworld::SectorData,
    resource::Resources,
};

/// Update all entities with animations with the current delta time
pub fn animation_system(_resources: &mut Resources, sector: &mut SectorData) {
    for (_entity, position) in sector.world.query_mut::<&mut Position>() {

        // TODO: update frame delta time for systems
        // position.update_time(ctx.time.delta().as_secs_f64());
    }

    for (_entity, renderable) in sector.world.query_mut::<&mut Renderable>() {
        // renderable.update_time(ctx.time.delta().as_secs_f64());
    }
}
