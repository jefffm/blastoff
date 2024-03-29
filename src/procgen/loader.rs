use hecs::World;

use crate::overworld::Overworld;
use crate::overworld::PlanetInfo;
use crate::overworld::SectorInfo;
use crate::resource::Resources;
use crate::sector;

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
    pub fn load(&mut self, sector_info: &SectorInfo, world: &mut World) -> sector::Map {
        let map = self
            .inner
            .generate(sector_info, self.resources, self.mapgen_history);

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
}

impl<'a, T> OverworldProcgenLoader<'a, T>
where
    T: OverworldGenerator,
{
    pub fn new(inner: T, resources: &'a mut Resources) -> Self {
        Self { inner, resources }
    }
    pub fn load(&mut self, info: PlanetInfo) -> Overworld {
        self.inner.generate(info, self.resources)
    }
}

pub struct GalaxyProcgenLoader<'a, T>
where
    T: GalaxyGenerator,
{
    inner: T,
    resources: &'a mut Resources,
    overworldgen_history: &'a mut Vec<Overworld>,
}
