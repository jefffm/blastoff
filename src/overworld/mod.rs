mod tile;
use ggez::graphics::DrawParam;
use rgb::RGBA8;
pub use tile::*;

use std::{collections::HashMap, fmt};

use hecs::World;
use serde::{Deserialize, Serialize};

use crate::{
    color::{RGBA8Ext, FIRE},
    game::consts::{MAX_PLANET_SPRITE_SIZE, SECTOR_HEIGHT, SECTOR_WIDTH},
    procgen::{MapGenerator, SectorProcgenLoader, Spawner},
    resource::Resources,
    scene::Sector,
    sector,
    util::{
        OverworldPoint, OverworldSize, PixelPoint, PixelRect, PixelSize, Sprite, WorldSize, PLANET,
    },
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
        self.map
            .get(point)
            .expect("Initialization should always set a tile for each point in the map")
    }

    fn set_tile(&mut self, point: OverworldPoint, tile: OverworldTile) {
        self.map.insert(point, tile);
    }

    /// Try to get the sector at a given point. Returns None if it hasn't been created yet (and see [`Self::create_sector'])
    fn get_sector(&self, point: &OverworldPoint) -> Option<&SectorData> {
        self.sectors.get(point)
    }

    fn set_sector(&mut self, point: &OverworldPoint, sector: SectorData) {
        self.sectors.insert(*point, sector);
    }

    /// Use procgen to create a new Sector at a given overworld grid point
    fn create_sector<'a, T: MapGenerator + Spawner>(
        &mut self,
        point: &OverworldPoint,
        loader: &mut SectorProcgenLoader<'a, T>,
    ) -> &SectorData {
        // Create a new Sector and spawn to a fresh ECS world
        let mut world = hecs::World::new();
        let map = loader.load(WorldSize::new(SECTOR_WIDTH, SECTOR_HEIGHT), &mut world);

        // Set the sector to the given point
        self.set_sector(point, (map, world));

        // aaaand let's also return a reference to it in the map
        self.get_sector(point).unwrap()
    }

    /// Sprites are scaled and colored according to their PlanetInfo
    /// First, we figure out the scale by deriving a square size from the
    /// planet's area (planets can technically be rectangles).
    /// Then we
    pub fn sprite(&self) -> Sprite {
        // x normalized = (x – x minimum) / (x maximum – x minimum)
        let area = self.info.size.area() as f32;
        let x = area.sqrt();

        let x_min: f32 = 1.;
        let x_max: f32 = 18. * 18.;
        let scale = (x - x_min) / (x_max - x_min);

        PLANET.with_params(
            DrawParam::default()
                .scale([
                    MAX_PLANET_SPRITE_SIZE * scale,
                    MAX_PLANET_SPRITE_SIZE * scale,
                ])
                .color(self.color().to_ggez_color()),
        )
    }

    pub fn color(&self) -> RGBA8 {
        // TODO: match on planet info for elemental types to determine color
        FIRE.four
    }
}

impl fmt::Display for Overworld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Planet {} ({:?})", self.info.name, self.info.size))
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
