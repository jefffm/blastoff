use bracket_lib::prelude::RandomNumberGenerator;
use legion::*;

use super::MapGenerator;
use super::Spawner;

pub struct Loader<'a, T>
where
    T: MapGenerator + Spawner,
{
    inner: T,
    rng: &'a mut RandomNumberGenerator,
}

impl<'a, T> Loader<'a, T>
where
    T: MapGenerator + Spawner,
{
    pub fn new(inner: T, rng: &'a mut RandomNumberGenerator) -> Self {
        Self { inner, rng }
    }

    pub fn load(&mut self, level: u32, world: &mut World, resources: &mut Resources) {
        let map = self.inner.generate(self.rng, level);
        self.inner.spawn(&map, world);
        resources.insert(map);
    }
}
