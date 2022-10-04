mod tile;
use ggez::graphics::DrawParam;
use rgb::RGBA8;
pub use tile::*;

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    color::{RGBA8Ext, FIRE, PLANT, WATER},
    data::{Element, PlanetType},
    game::consts::{MAX_PLANET_SPRITE_SIZE, SECTOR_HEIGHT, SECTOR_WIDTH},
    procgen::{MapGenerator, SectorProcgenLoader, Spawner},
    sector,
    util::{OverworldPoint, OverworldRect, OverworldSize, Sprite, WorldSize, PLANET},
};

// TODO: World needs to be serializeable in order to implement save/load
pub type OverworldSectors = HashMap<OverworldPoint, Rc<RefCell<SectorData>>>;
pub type OverworldMap = HashMap<OverworldPoint, OverworldTile>;

pub struct SectorData {
    pub map: sector::Map,
    pub world: hecs::World,
}

// #[derive(Serialize, Deserialize)]
pub struct Overworld {
    info: PlanetInfo,
    map: OverworldMap,
    default_map_tile: OverworldTile,
    sectors: OverworldSectors,
}

impl Overworld {
    pub fn from_size(info: PlanetInfo, default_map_tile: OverworldTile) -> Self {
        // Tiles are generated upfront via default tiles (and whatever other procgen we use)
        let map = HashMap::with_capacity(info.size.area() as usize);

        // Sectors will be generated on-demand as they are visited
        let sectors = HashMap::with_capacity(info.size.area() as usize);

        Self::new(info, map, default_map_tile, sectors)
    }

    pub fn new(
        info: PlanetInfo,
        map: OverworldMap,
        default_map_tile: OverworldTile,
        sectors: OverworldSectors,
    ) -> Self {
        Self {
            info,
            map,
            default_map_tile,
            sectors,
        }
    }

    pub fn info(&self) -> &PlanetInfo {
        &self.info
    }

    pub fn clamp(&self, point: OverworldPoint) -> OverworldPoint {
        // TODO: move this logic to a RectExt trait clamp method
        let clamped_x = point
            .x
            .max(self.info.rect.min_x())
            .min(self.info.rect.max_x() - 1);
        let clamped_y = point
            .y
            .max(self.info.rect.min_y())
            .min(self.info.rect.max_y() - 1);

        OverworldPoint::new(clamped_x, clamped_y)
    }

    pub fn center(&self) -> OverworldPoint {
        self.info.center()
    }

    pub fn get_tile(&self, point: &OverworldPoint) -> Option<&OverworldTile> {
        if self.info.rect.contains(*point) {
            Some(self.map.get(point).unwrap_or(&self.default_map_tile))
        } else {
            None
        }
    }

    /// sets *JUST* the tile
    pub fn set_tile(&mut self, point: OverworldPoint, tile: OverworldTile) {
        self.map.insert(point, tile);
    }

    /// Try to get the sector at a given point. Returns None if it hasn't been created yet (and see [`Self::create_sector'])
    /// Consumers should try this, then use unwrap_or_else to call create_sector
    pub fn get_sector(&self, point: &OverworldPoint) -> Option<Rc<RefCell<SectorData>>> {
        self.sectors.get(point).cloned()
    }

    /// sets the actual sector data for a given point
    pub fn set_sector(&mut self, point: &OverworldPoint, sector: SectorData) {
        self.sectors.insert(*point, Rc::new(RefCell::new(sector)));
    }

    /// Use procgen to create a new Sector at a given overworld grid point
    pub fn create_sector<'a, T: MapGenerator + Spawner>(
        &mut self,
        point: &OverworldPoint,
        loader: &mut SectorProcgenLoader<'a, T>,
    ) -> Rc<RefCell<SectorData>> {
        // Create a new Sector and spawn to a fresh ECS world
        let mut world = hecs::World::new();
        let map = loader.load(WorldSize::new(SECTOR_WIDTH, SECTOR_HEIGHT), &mut world);

        // Set the sector to the given point
        self.set_sector(point, SectorData { map, world });

        // aaaand let's also return a reference to it in the map
        self.get_sector(point).unwrap()
    }

    pub fn color(&self) -> RGBA8 {
        self.info.color()
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = (&OverworldPoint, &OverworldTile)> {
        self.map.iter()
    }

    pub fn iter_points(&self) -> impl Iterator<Item = OverworldPoint> {
        let yrange = self.info.rect.y_range();
        let xrange = self.info.rect.x_range();

        yrange.flat_map(move |y| xrange.clone().map(move |x| OverworldPoint::new(x, y)))
    }
}

impl fmt::Display for Overworld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Overworld of {}", self.info))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetInfo {
    name: String,
    size: OverworldSize,
    rect: OverworldRect,
    planet_type: PlanetType,
    element: Element,
}

impl PlanetInfo {
    pub fn new(
        name: String,
        size: OverworldSize,
        planet_type: PlanetType,
        element: Element,
    ) -> Self {
        let rect = OverworldRect::new(OverworldPoint::new(0, 0), size);
        Self {
            name,
            size,
            rect,
            planet_type,
            element,
        }
    }

    pub fn center(&self) -> OverworldPoint {
        self.rect.center()
    }

    pub fn color(&self) -> RGBA8 {
        match self.element {
            Element::Water => WATER.five,
            Element::Fire => FIRE.five,
            Element::Plant => PLANT.five,
        }
    }

    /// Sprites are scaled and colored according to their PlanetInfo
    /// First, we figure out the scale by deriving a square size from the
    /// planet's area (planets can technically be rectangles).
    /// Then we
    pub fn sprite(&self) -> Sprite {
        // x normalized = (x – x minimum) / (x maximum – x minimum)
        let area = self.size.area() as f32;
        let x = area.sqrt();

        // TODO: remove magic numbers from planet sprite
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
}

impl fmt::Display for PlanetInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Planet {} ({:?})", self.name, self.size))
    }
}
