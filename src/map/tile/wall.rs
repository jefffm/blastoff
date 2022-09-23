use serde::{Deserialize, Serialize};

use crate::{color::COMMON, component::Cardinal, map::Map, util::WorldPoint};

use super::Tile;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum WallKind {
    WallPillar,   // Pillar because we can't see neighbors
    WallN,        // Wall only to the north
    WallS,        // Wall only to the south
    WallNS,       // Wall to the north and south
    WallW,        // Wall only to the west
    WallNW,       // Wall to the north and west
    WallSW,       // Wall to the south and west
    WallNSW,      // Wall to the north, south and west
    WallE,        // Wall only to the east
    WallNE,       // Wall to the north and east
    WallSE,       // Wall to the south and east
    WallNSE,      // Wall to the north, south and east
    WallEW,       // Wall to the east and west
    WallEWS,      // Wall to the east, west, and south
    WallEWN,      // Wall to the east, west, and north
    WallAllSides, // ╬ Wall on all sides
    WallDefault,  // We missed one?
}

impl WallKind {
    pub fn from_bitmask(mask: u8) -> Self {
        match mask {
            0 => Self::WallPillar,    // Pillar because we can't see neighbors
            1 => Self::WallN,         // Wall only to the north
            2 => Self::WallS,         // Wall only to the south
            3 => Self::WallNS,        // Wall to the north and south
            4 => Self::WallW,         // Wall only to the west
            5 => Self::WallNW,        // Wall to the north and west
            6 => Self::WallSW,        // Wall to the south and west
            7 => Self::WallNSW,       // Wall to the north, south and west
            8 => Self::WallE,         // Wall only to the east
            9 => Self::WallNE,        // Wall to the north and east
            10 => Self::WallSE,       // Wall to the south and east
            11 => Self::WallNSE,      // Wall to the north, south and east
            12 => Self::WallEW,       // Wall to the east and west
            13 => Self::WallEWS,      // Wall to the east, west, and south
            14 => Self::WallEWN,      // Wall to the east, west, and north
            15 => Self::WallAllSides, // ╬ Wall on all sides
            _ => Self::WallDefault,   // We missed one?
        }
    }

    pub fn from_map_position(map: &Map, point: WorldPoint) -> Self {
        if !map.contains(point) {
            return Self::default();
        }

        let mut mask: u8 = 0;

        if let Some(Tile::Wall(_)) = map.get(point - *Cardinal::S.to_vector()) {
            mask += 1;
        }
        if let Some(Tile::Wall(_)) = map.get(point - *Cardinal::N.to_vector()) {
            mask += 2;
        }
        if let Some(Tile::Wall(_)) = map.get(point - *Cardinal::W.to_vector()) {
            mask += 4;
        }
        if let Some(Tile::Wall(_)) = map.get(point - *Cardinal::E.to_vector()) {
            mask += 8;
        }

        Self::from_bitmask(mask)
    }
    pub fn glyph(&self) -> char {
        match self {
            Self::WallPillar => '○',
            Self::WallN | Self::WallS | Self::WallNS => '║',
            Self::WallW => '═',
            Self::WallNW => '╝',
            Self::WallSW => '╗',
            Self::WallNSW => '╣',
            Self::WallE => '═',
            Self::WallNE => '╚',
            Self::WallSE => '╔',
            Self::WallNSE => '╠',
            Self::WallEW => '═',
            Self::WallEWS => '╩',
            Self::WallEWN => '╦',
            Self::WallAllSides => '╬',
            Self::WallDefault => '#',
        }
    }

    pub fn is_passable(&self) -> bool {
        false
    }

    pub fn is_opaque(&self) -> bool {
        true
    }

    pub fn fg(&self) -> rgb::RGBA8 {
        COMMON.four
    }
}

impl Default for WallKind {
    fn default() -> Self {
        Self::WallDefault
    }
}
