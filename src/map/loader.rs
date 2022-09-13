use bracket_lib::prelude::RandomNumberGenerator;
use hecs::World;

use super::Map;
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

    /// Generates the map and returns the Spawner
    pub fn load(&mut self, level: u32, world: &mut World) -> Map {
        let map = self.inner.generate(self.rng, level);
        self.inner.spawn(&map, world);

        map
    }
}
