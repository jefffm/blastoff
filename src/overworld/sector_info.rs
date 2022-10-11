use crate::{
    resource::Resources,
    util::{PixelPoint, WorldSize},
};

use super::{OverworldTile, PlanetInfo};

#[derive(Debug, Clone)]
pub struct SectorInfo {
    pub planet_info: PlanetInfo,
    pub tile: OverworldTile,
    pub size: WorldSize,
}

impl SectorInfo {
    pub fn new(planet_info: PlanetInfo, tile: OverworldTile, size: WorldSize) -> Self {
        Self {
            planet_info,
            tile,
            size,
        }
    }

    pub fn render(&self, resources: &mut Resources, point: &PixelPoint) {
        self.tile.render(resources, point)
    }
}
