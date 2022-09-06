use bracket_lib::prelude::Point;
use serde::{Deserialize, Serialize};

use crate::map::Tile;
use crate::util::Vec2d;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Map {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub depth: u32,
    pub tiles: Vec2d<Tile>,
}

impl Map {
    pub fn new(name: String, width: usize, height: usize, depth: u32, tiles: Vec2d<Tile>) -> Self {
        Self {
            name,
            width,
            height,
            depth,
            tiles,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::TileKind;

    use super::*;

    #[test]
    fn test_iter() {
        // Create a 5x5 grid with the border surrounded by wall
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let tiles = vec![
            TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(),
            TileKind::Wall.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Wall.into(),
            TileKind::Wall.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Wall.into(),
            TileKind::Wall.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Wall.into(),
            TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(),
        ];

        Map::new(
            String::from("test"),
            5,
            5,
            1,
            Vec2d::new(tiles.into(), 5, 5),
        );
    }
}
