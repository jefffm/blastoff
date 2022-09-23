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
    util::ScreenPoint,
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
    Floor(FloorKind),
    Wall(WallKind),
}

impl Default for Tile {
    fn default() -> Self {
        Self::Floor(FloorKind::default())
    }
}

impl Tile {
    pub fn glyph(&self) -> char {
        match self {
            Self::Floor(floor) => floor.glyph(),
            Self::Wall(wall) => wall.glyph(),
        }
    }

    fn color_pair(&self) -> ColorPair {
        ColorPair::new(self.fg().to_bracket_rgba(), self.bg().to_bracket_rgba())
    }

    pub fn is_passable(&self) -> bool {
        match self {
            Self::Floor(floor) => floor.is_passable(),
            Self::Wall(wall) => wall.is_passable(),
        }
    }

    pub fn is_opaque(&self) -> bool {
        match self {
            Self::Floor(floor) => floor.is_opaque(),
            Self::Wall(wall) => wall.is_opaque(),
        }
    }

    pub fn fg(&self) -> RGBA8 {
        match self {
            Self::Floor(floor) => floor.fg(),
            Self::Wall(wall) => wall.fg(),
        }
    }

    pub fn bg(&self) -> RGBA8 {
        EMPTY
    }

    pub fn render(
        &self,
        canvas: &mut Canvas,
        resources: &Resources,
        point: ScreenPoint,
        visibility_kind: VisibilityKind,
    ) {
        let pixel_point = get_screen_to_pixel_transform().transform_point(point);
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
