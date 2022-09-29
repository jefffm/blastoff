use std::collections::HashMap;

use crate::{overworld::Overworld, util::SolarPoint};

type SolarMap = HashMap<SolarPoint, Overworld>;

pub struct SolarSystem {
    info: SolarSystemInfo,
    map: SolarMap,
}

impl SolarSystem {
    pub fn new(info: SolarSystemInfo, map: SolarMap) -> Self {
        Self { info, map }
    }

    pub fn info(&self) -> &SolarSystemInfo {
        &self.info
    }
}

#[derive(Debug, Clone)]
pub struct SolarSystemInfo {
    name: String,
}

impl SolarSystemInfo {
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
        solar_map.insert(SolarPoint::new(0, 0), planet);
        let solar_system =
            SolarSystem::new(SolarSystemInfo::new("test system".to_owned()), solar_map);

        assert_eq!(solar_system.info.name, "test system");
    }
}
