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
    mapgen_history: &'a mut Vec<Map>,
}

impl<'a, T> Loader<'a, T>
where
    T: MapGenerator + Spawner,
{
    pub fn new(
        inner: T,
        rng: &'a mut RandomNumberGenerator,
        mapgen_history: &'a mut Vec<Map>,
    ) -> Self {
        Self {
            inner,
            rng,
            mapgen_history,
        }
    }

    /// Generates the map and returns the Spawner
    pub fn load(&mut self, level: u32, world: &mut World) -> Map {
        let map = self.inner.generate(self.rng, self.mapgen_history, level);
        self.inner.spawn(&map, world);

        map
    }
}
