use hecs::World;

use crate::overworld::Overworld;
use crate::resource::Resources;
use crate::sector;
use crate::util::WorldSize;

use super::GalaxyGenerator;
use super::MapGenerator;
use super::OverworldGenerator;
use super::Spawner;

pub struct SectorProcgenLoader<'a, T>
where
    T: MapGenerator + Spawner,
{
    inner: T,
    resources: &'a mut Resources,
    mapgen_history: &'a mut Vec<sector::Map>,
}

impl<'a, T> SectorProcgenLoader<'a, T>
where
    T: MapGenerator + Spawner,
{
    pub fn new(
        inner: T,
        resources: &'a mut Resources,
        mapgen_history: &'a mut Vec<sector::Map>,
    ) -> Self {
        Self {
            inner,
            resources,
            mapgen_history,
        }
    }

    /// Generates the map and returns the Spawner
    pub fn load(&mut self, size: WorldSize, world: &mut World) -> sector::Map {
        let map = self
            .inner
            .generate(size, self.resources, self.mapgen_history);

        self.inner.spawn(&map, world, self.resources);

        map
    }
}

pub struct OverworldProcgenLoader<'a, T>
where
    T: OverworldGenerator,
{
    inner: T,
    resources: &'a mut Resources,
    overworldgen_history: &'a mut Vec<Overworld>,
}

pub struct GalaxyProcgenLoader<'a, T>
where
    T: GalaxyGenerator,
{
    inner: T,
    resources: &'a mut Resources,
    overworldgen_history: &'a mut Vec<Overworld>,
}
