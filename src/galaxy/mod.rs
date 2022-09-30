use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{overworld::Overworld, util::GalaxyPoint};

type GalaxyMap = HashMap<GalaxyPoint, Overworld>;

/// A galaxy contains coordinates pointing to each star system
// #[derive(Serialize, Deserialize)]
pub struct Galaxy {
    info: GalaxyInfo,
    map: GalaxyMap,
}

impl Galaxy {
    pub fn new(info: GalaxyInfo, map: GalaxyMap) -> Self {
        Self { info, map }
    }

    pub fn info(&self) -> &GalaxyInfo {
        &self.info
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyInfo {
    name: String,
}

impl GalaxyInfo {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use hecs::World;

    use crate::{
        overworld::PlanetInfo,
        scene::Sector,
        sector::{FloorKind, Map, Tile},
        util::{OverworldPoint, WorldSize},
    };

    use super::*;
    #[test]
    fn create() {
        let map = Map::init(
            "Testing".to_owned(),
            WorldSize::new(1, 1),
            Tile::Floor(FloorKind::FloorDefault),
        );
        let mut overworld_map = HashMap::new();
        overworld_map.insert(OverworldPoint::new(0, 0), Sector::new(map, World::new()));

        let planet = Overworld::new(PlanetInfo::new("yeaaaarg".to_owned()), overworld_map);

        let mut galaxy_map = HashMap::new();
        galaxy_map.insert(GalaxyPoint::new(0, 0), planet);
        let galaxy = Galaxy::new(GalaxyInfo::new("test system".to_owned()), galaxy_map);

        assert_eq!(galaxy.info.name, "test system");
    }
}
