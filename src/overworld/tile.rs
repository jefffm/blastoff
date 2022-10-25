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
    Mountains,
}

impl OverworldTile {
    pub fn glyph(&self) -> char {
        match self {
            OverworldTile::City => '⌂',
            OverworldTile::Barren => '≡',
            OverworldTile::Water => '~',
            OverworldTile::Lava => '~',
            OverworldTile::Jungle => '♣',
            OverworldTile::Mountains => '▲',
        }
    }

    pub fn sprite(&self) -> u32 {
        match self {
            OverworldTile::City => 1032,
            OverworldTile::Barren => 6,
            OverworldTile::Water => 253,
            OverworldTile::Lava => 253,
            OverworldTile::Jungle => 101,
            OverworldTile::Mountains => 264,
        }
    }

    pub fn fg(&self) -> Color {
        match self {
            OverworldTile::City => COMMON.three,
            OverworldTile::Barren => COMMON.two,
            OverworldTile::Water => WATER.three,
            OverworldTile::Lava => FIRE.three,
            OverworldTile::Jungle => PLANT.three,
            OverworldTile::Mountains => COMMON.four,
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
