use std::collections::HashMap;

use crate::{scene::Sector, util::OverworldPoint};

type OverworldMap = HashMap<OverworldPoint, Sector>;

pub struct Overworld {
    info: PlanetInfo,
    map: OverworldMap,
}

impl Overworld {
    pub fn new(info: PlanetInfo, map: OverworldMap) -> Self {
        Self { info, map }
    }

    pub fn info(&self) -> &PlanetInfo {
        &self.info
    }
}

#[derive(Debug, Clone)]
pub struct PlanetInfo {
    name: String,
}

impl PlanetInfo {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
