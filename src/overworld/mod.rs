use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{scene::Sector, util::OverworldPoint};

// TODO: World needs to be serializeable in order to implement save/load
type OverworldMap = HashMap<OverworldPoint, Sector>;

// #[derive(Serialize, Deserialize)]
pub struct Overworld {
    info: PlanetInfo,
    map: OverworldMap,
}

impl Overworld {
    // TODO: procgen a world map using PlanetInfo and anything else we might need (rng, spawn tables, other resource data)
    pub fn init_from(info: PlanetInfo) -> Self {
        let map = OverworldMap::new();
        Self { info, map }
    }

    pub fn new(info: PlanetInfo, map: OverworldMap) -> Self {
        Self { info, map }
    }

    pub fn info(&self) -> &PlanetInfo {
        &self.info
    }

    fn get_scene(&self, point: &OverworldPoint) -> Option<&Sector> {
        self.map.get(point)
    }

    fn set_scene(&mut self, point: &OverworldPoint, sector: Sector) {
        self.map.insert(*point, sector);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetInfo {
    name: String,
}

impl PlanetInfo {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
