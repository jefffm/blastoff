mod sector_info;
pub use sector_info::*;
mod tile;

use rgb::RGBA8;
pub use tile::*;

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    color::{FIRE, PLANT, WATER},
    data::{Element, PlanetType, SectorProbability},
    game::consts::MAX_PLANET_SPRITE_SIZE,
    procgen::{MapGenerator, SectorProcgenLoader, Spawner},
    sector,
    util::{OverworldPoint, OverworldRect, OverworldSize},
};

// TODO: World needs to be serializeable in order to implement save/load
pub type OverworldSectors = HashMap<OverworldPoint, Rc<RefCell<SectorData>>>;
pub type OverworldMap = HashMap<OverworldPoint, SectorInfo>;

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
    pub fn from_info(info: PlanetInfo) -> Self {
        // Tiles are generated upfront via default tiles (and whatever other procgen we use)
        let map = HashMap::with_capacity(info.size.area() as usize);

        // Sectors will be generated on-demand as they are visited
        let sectors = HashMap::with_capacity(info.size.area() as usize);
        let default_map_tile = info.default_tile();

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

    /// Return an Overworld point clamped to the Overworld's rect
    pub fn clamp(&self, point: OverworldPoint) -> OverworldPoint {
        // TODO: move clamping logic to a RectExt trait clamp method
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

    pub fn get_sector_info(&self, point: &OverworldPoint) -> Option<&SectorInfo> {
        if self.info.rect.contains(*point) {
            self.map.get(point)
        } else {
            None
        }
    }

    /// sets *JUST* the tile
    pub fn set_sector_info(&mut self, point: OverworldPoint, sector_info: SectorInfo) {
        self.map.insert(point, sector_info);
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
        // TODO: it's confusing how Overworld and procgen/overworld/ interact. Consolidate?

        let sector_info = self
            .get_sector_info(point)
            .expect("SectorInfo should exist for point");

        // Create a new Sector and spawn to a fresh ECS world
        let mut world = hecs::World::new();

        let map = loader.load(sector_info, &mut world);

        // Set the sector to the given point
        self.set_sector(point, SectorData { map, world });

        // aaaand let's also return a reference to it in the map
        self.get_sector(point).unwrap()
    }

    pub fn color(&self) -> RGBA8 {
        self.info.color()
    }

    pub fn iter_sector_infos(&self) -> impl Iterator<Item = (&OverworldPoint, &SectorInfo)> {
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
    pub name: String,
    pub size: OverworldSize,
    pub rect: OverworldRect,
    pub planet_type: PlanetType,
    pub element: Element,
    pub sector_probability: SectorProbability,
}

impl PlanetInfo {
    pub fn new(
        name: String,
        size: OverworldSize,
        planet_type: PlanetType,
        element: Element,
        sector_probability: SectorProbability,
    ) -> Self {
        let rect = OverworldRect::new(OverworldPoint::new(0, 0), size);
        Self {
            name,
            size,
            rect,
            planet_type,
            element,
            sector_probability,
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

    pub fn default_tile(&self) -> OverworldTile {
        match self.element {
            Element::Water => OverworldTile::Water,
            Element::Fire => OverworldTile::Lava,
            Element::Plant => OverworldTile::Jungle,
        }
    }

    pub fn sprite(&self) -> u32 {
        // TODO: planet returns a sprite object of some sort
        180 // return the round planet looking thing

        // // Determine the sprite scale by deriving a square size from the
        // // planet's area (planets can technically be rectangles).
        // // x normalized = (x – x minimum) / (x maximum – x minimum)
        // let area = self.size.area() as f32;
        // let x = area.sqrt();

        // // TODO: remove magic numbers from planet sprite
        // let x_min: f32 = 1.;
        // let x_max: f32 = 18. * 18.;
        // let scale = (x - x_min) / (x_max - x_min);

        // PLANET.with_params(
        //     DrawParam::default()
        //         .scale([
        //             MAX_PLANET_SPRITE_SIZE * scale,
        //             MAX_PLANET_SPRITE_SIZE * scale,
        //         ])
        //         .color(self.color().to_ggez_color()),
        // )
    }
}

impl fmt::Display for PlanetInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} ({:?})", self.name, self.size))
    }
}
