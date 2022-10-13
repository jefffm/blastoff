use rgb::RGBA8;

use crate::{
    color::{COMMON, FIRE, PLANT, WATER},
    resource::Resources,
    util::PixelPoint,
};

#[derive(Debug, Clone, Copy)]
pub enum OverworldTile {
    City,
    Barren,
    Water,
    Lava,
    Jungle,
}

impl OverworldTile {
    pub fn glyph(&self) -> char {
        match self {
            OverworldTile::City => '⌂',
            OverworldTile::Barren => '≡',
            OverworldTile::Water => '~',
            OverworldTile::Lava => '~',
            OverworldTile::Jungle => '♣',
        }
    }

    pub fn fg(&self) -> RGBA8 {
        match self {
            OverworldTile::City => COMMON.three,
            OverworldTile::Barren => COMMON.two,
            OverworldTile::Water => WATER.three,
            OverworldTile::Lava => FIRE.three,
            OverworldTile::Jungle => PLANT.three,
        }
    }

    pub fn render(&self, resources: &mut Resources, point: &PixelPoint) {
        // TODO: render overworld tiles
    }
}
