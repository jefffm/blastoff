use std::collections::HashMap;

use crate::{overworld::Overworld, util::GalaxyPoint};

type GalaxyMap = HashMap<GalaxyPoint, Overworld>;

/// A galaxy contains coordinates pointing to each star system
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

#[derive(Debug, Clone)]
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
        map::{FloorKind, Map, Tile},
        overworld::PlanetInfo,
        scene::Game,
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
        let scene = Game::new(map, World::new());

        let mut overworld_map = HashMap::new();
        overworld_map.insert(OverworldPoint::new(0, 0), scene);

        let planet = Overworld::new(PlanetInfo::new("yeaaaarg".to_owned()), overworld_map);

        let mut solar_map = HashMap::new();
        solar_map.insert(GalaxyPoint::new(0, 0), planet);
        let solar_system = Galaxy::new(GalaxyInfo::new("test system".to_owned()), solar_map);

        assert_eq!(solar_system.info.name, "test system");
    }
}
