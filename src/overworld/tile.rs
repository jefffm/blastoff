use macroquad::prelude::Color;

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

    pub fn sprite(&self) -> u32 {
        match self {
            OverworldTile::City => 1032,
            OverworldTile::Barren => 6,
            OverworldTile::Water => 253,
            OverworldTile::Lava => 253,
            OverworldTile::Jungle => 101,
        }
    }

    pub fn fg(&self) -> Color {
        match self {
            OverworldTile::City => COMMON.three,
            OverworldTile::Barren => COMMON.two,
            OverworldTile::Water => WATER.three,
            OverworldTile::Lava => FIRE.three,
            OverworldTile::Jungle => PLANT.three,
        }
    }

    pub fn render(&self, resources: &mut Resources, point: &PixelPoint) {
        // TODO: render ASCII too
        resources
            .assets
            .tileset
            .draw(self.sprite(), point, Some(self.fg()), None, false)
    }
}

/*

- egg rolls
- shrimp and pork rice dish
- pho chicken

860 284 1196
*/
