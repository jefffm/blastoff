use bracket_random::prelude::RandomNumberGenerator;
use hecs::World;

use crate::sector::Map;
use crate::util::WorldSize;

use super::MapGenerator;
use super::Spawner;

pub struct SectorProcgenLoader<'a, T>
where
    T: MapGenerator + Spawner,
{
    inner: T,
    rng: &'a mut RandomNumberGenerator,
    mapgen_history: &'a mut Vec<Map>,
}

impl<'a, T> SectorProcgenLoader<'a, T>
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
    pub fn load(&mut self, size: WorldSize, world: &mut World) -> Map {
        let map = self.inner.generate(size, self.rng, self.mapgen_history);

        self.inner.spawn(&map, world, self.rng);

        map
    }
}
