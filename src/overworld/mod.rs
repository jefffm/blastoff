use std::collections::HashMap;

use hecs::World;
use serde::{Deserialize, Serialize};

use crate::{map::Map, util::OverworldPoint};

// TODO: World needs to be serializeable in order to implement save/load
type OverworldMap = HashMap<OverworldPoint, (Map, World)>;

// #[derive(Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetInfo {
    name: String,
}

impl PlanetInfo {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
