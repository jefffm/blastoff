use hecs::World;

use crate::{
    component::{Camera, Position},
    resource::Resources,
};

// Update the viewport to be centered on the Camera position
pub fn viewport_system(world: &mut World, resources: &mut Resources) {
    let viewport = &mut resources.viewport;
    for (_, (pos, _cam)) in world.query::<(&Position, &Camera)>().iter() {
        viewport.update_transform(pos.p)
    }
}
