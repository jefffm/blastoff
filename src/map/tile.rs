use bracket_lib::prelude::{
    to_cp437, ColorPair, DrawBatch, Point, BLACK, GRAY1, GRAY22, LIGHT_YELLOW, WHITE,
};
use serde::{Deserialize, Serialize};

use crate::util::ScreenPoint;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Tile {
    kind: TileKind,
    revealed: bool,
    visible: bool,
}

impl Tile {
    pub fn render(
        &self,
        draw_batch: &mut DrawBatch,
        point: ScreenPoint,
        visibility_kind: VisibilityKind,
    ) {
        self.kind.render(draw_batch, point, visibility_kind)
    }

    pub fn is_passable(&self) -> bool {
        self.kind.is_passable()
    }

    pub fn get_kind(&self) -> &TileKind {
        &self.kind
    }
}

pub enum VisibilityKind {
    Torch,
    Daylight,
    Dim,
    Remembered,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TileKind {
    Floor,
    Wall,
}

impl Default for TileKind {
    fn default() -> Self {
        Self::Floor
    }
}

impl TileKind {
    fn glyph(&self) -> char {
        match self {
            Self::Floor => '.',
            Self::Wall => '#',
        }
    }

    fn is_passable(&self) -> bool {
        match self {
            Self::Floor => true,
            Self::Wall => false,
        }
    }

    pub fn render(
        &self,
        draw_batch: &mut DrawBatch,
        point: ScreenPoint,
        visibility_kind: VisibilityKind,
    ) {
        match visibility_kind {
            VisibilityKind::Torch => {
                draw_batch.set(
                    Point::new(point.x, point.y),
                    ColorPair::new(LIGHT_YELLOW, BLACK),
                    to_cp437(self.glyph()),
                );
            }
            VisibilityKind::Remembered => {
                draw_batch.set(
                    Point::new(point.x, point.y),
                    ColorPair::new(GRAY22, BLACK),
                    to_cp437(self.glyph()),
                );
            }
            _ => {
                todo!("not implemented yet!");
            }
        }
    }
}

impl From<TileKind> for Tile {
    fn from(kind: TileKind) -> Self {
        Self {
            kind,
            revealed: false,
            visible: false,
        }
    }
}

// fn wall_glyph(map: &Map, x: i32, y: i32) -> FontCharType {
//     if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2 as i32 {
//         return 35;
//     }
//     let mut mask: u8 = 0;

//     if is_revealed_and_wall(map, x, y - 1) {
//         mask += 1;
//     }
//     if is_revealed_and_wall(map, x, y + 1) {
//         mask += 2;
//     }
//     if is_revealed_and_wall(map, x - 1, y) {
//         mask += 4;
//     }
//     if is_revealed_and_wall(map, x + 1, y) {
//         mask += 8;
//     }

//     match mask {
//         0 => 9,    // Pillar because we can't see neighbors
//         1 => 186,  // Wall only to the north
//         2 => 186,  // Wall only to the south
//         3 => 186,  // Wall to the north and south
//         4 => 205,  // Wall only to the west
//         5 => 188,  // Wall to the north and west
//         6 => 187,  // Wall to the south and west
//         7 => 185,  // Wall to the north, south and west
//         8 => 205,  // Wall only to the east
//         9 => 200,  // Wall to the north and east
//         10 => 201, // Wall to the south and east
//         11 => 204, // Wall to the north, south and east
//         12 => 205, // Wall to the east and west
//         13 => 202, // Wall to the east, west, and south
//         14 => 203, // Wall to the east, west, and north
//         15 => 206, // ╬ Wall on all sides
//         _ => 35,   // We missed one?
//     }
// }
