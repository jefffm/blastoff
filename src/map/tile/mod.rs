mod wall;
use ggez::graphics::{Canvas, DrawParam};
use rgb::RGBA8;
pub use wall::*;

mod floor;
pub use floor::*;

use bracket_lib::prelude::ColorPair;
use serde::{Deserialize, Serialize};

use crate::{
    color::{RGBA8Ext, COMMON, EMPTY},
    game::consts::get_screen_to_pixel_transform,
    resource::Resources,
    util::{BitmapFont, ScreenPoint, SpritePoint},
};
pub enum VisibilityKind {
    Torch { brightness: u32 },
    DiscoBall { value: u32 },
    Daylight,
    Dim,
    Remembered,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Floor
    }
}

impl Tile {
    pub fn handler(&self) -> Box<dyn TileHandler> {
        match self {
            Self::Floor => Box::new(Floor {}),
            Self::Wall => Box::new(Wall {}),
        }
    }

    pub fn value(&self) -> SpritePoint {
        match self {
            Self::Floor => SpritePoint::new(7, 0),
            Self::Wall => SpritePoint::new(5, 1),
        }
    }
}

/// TileHandler
pub trait TileHandler {
    fn glyph(&self) -> char;
    fn color_pair(&self) -> ColorPair {
        // Default implementation
        ColorPair::new(self.fg().to_bracket_rgba(), self.bg().to_bracket_rgba())
    }

    fn is_passable(&self) -> bool;
    fn is_opaque(&self) -> bool;
    fn fg(&self) -> RGBA8;
    fn bg(&self) -> RGBA8 {
        EMPTY
    }

    fn render(
        &self,
        canvas: &mut Canvas,
        resources: &Resources,
        point: ScreenPoint,
        visibility_kind: VisibilityKind,
    ) {
        let pixel_point = get_screen_to_pixel_transform().transform_point(point);
        // TODO: use blit for each of these
        match visibility_kind {
            VisibilityKind::Torch { brightness: _ } => {
                // TODO: use torch brightness to modify rendering brightness
                resources
                    .font
                    .draw_char_overwrite(canvas, self.glyph(), &pixel_point, None);
            }
            VisibilityKind::Remembered => {
                resources.font.draw_char_overwrite(
                    canvas,
                    self.glyph(),
                    &pixel_point,
                    Some(DrawParam::new().color(COMMON.two.to_ggez_color())),
                );
            }
            VisibilityKind::DiscoBall { value: _ } => {
                // draw_batch.set(
                //     Point::new(point.x, point.y),
                //     ColorPair::new(
                //         COMMON
                //             .one
                //             .to_bracket_rgba()
                //             .lerp(COMMON.four.to_bracket_rgba(), 1.0 / value as f32),
                //         self.bg().to_bracket_rgba(),
                //     ),
                //     to_cp437(self.glyph()),
                // );
            }
            _ => {
                todo!("not implemented yet!");
            }
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
