mod tile;
pub use tile::*;

use std::collections::HashMap;

use hecs::World;
use serde::{Deserialize, Serialize};

use crate::{
    procgen::{MapGenerator, SectorProcgenLoader, Spawner},
    scene::Sector,
    sector,
    util::{OverworldPoint, OverworldSize, WorldSize},
};

// TODO: World needs to be serializeable in order to implement save/load
pub type SectorData = (sector::Map, World);
pub type OverworldSectors = HashMap<OverworldPoint, SectorData>;
pub type OverworldMap = HashMap<OverworldPoint, OverworldTile>;

// #[derive(Serialize, Deserialize)]
pub struct Overworld {
    info: PlanetInfo,
    map: OverworldMap,
    sectors: OverworldSectors,
}

impl Overworld {
    pub fn from_size(info: PlanetInfo, default_tile: OverworldTile) -> Self {
        let mut map = HashMap::with_capacity(info.size.area() as usize);
        let mut sectors = HashMap::with_capacity(info.size.area() as usize);

        Self::new(info, map, sectors)
    }

    pub fn new(info: PlanetInfo, map: OverworldMap, sectors: OverworldSectors) -> Self {
        Self { info, map, sectors }
    }

    pub fn info(&self) -> &PlanetInfo {
        &self.info
    }

    fn get_tile(&self, point: &OverworldPoint) -> &OverworldTile {
        self.map.get(point).unwrap()
    }

    fn set_tile(&mut self, point: OverworldPoint, tile: OverworldTile) {
        self.map.insert(point, tile);
    }

    fn get_sector(&self, point: &OverworldPoint) -> Option<&SectorData> {
        self.sectors.get(point)
    }

    fn set_sector(&mut self, point: &OverworldPoint, sector: SectorData) {
        self.sectors.insert(*point, sector);
    }

    fn create_sector<'a, T: MapGenerator + Spawner>(
        &mut self,
        point: &OverworldPoint,
        loader: &mut SectorProcgenLoader<'a, T>,
    ) -> &SectorData {
        let mut world = hecs::World::new();

        const SECTOR_WIDTH: i32 = 100;
        const SECTOR_HEIGHT: i32 = 100;
        let map = loader.load(WorldSize::new(SECTOR_WIDTH, SECTOR_HEIGHT), &mut world);

        self.set_sector(point, (map, world));
        self.get_sector(point).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetInfo {
    name: String,
    size: OverworldSize,
}

impl PlanetInfo {
    pub fn new(name: String, size: OverworldSize) -> Self {
        Self { name, size }
    }
}
