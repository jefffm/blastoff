use legion::*;

use super::Map;

pub trait Spawner {
    fn spawn(&self, map: &Map, world: &mut World);
}
