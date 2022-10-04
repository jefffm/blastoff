use hecs::World;

use crate::{resource::Resources, sector::Map};

pub trait Spawner {
    fn spawn(&self, map: &Map, world: &mut World, resources: &mut Resources);
}
