mod wall;
pub use wall::*;

mod floor;
pub use floor::*;

use macroquad::prelude::Color;
use serde::{Deserialize, Serialize};

use crate::{color::EMPTY, resource::Resources, util::PixelPoint};
pub enum VisibilityKind {
    Torch { brightness: u32 },
    DiscoBall { value: u32 },
    Daylight,
    Dim,
    Remembered,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Tile {
    Floor(FloorKind),
    Wall(WallKind),
}

impl Default for Tile {
    fn default() -> Self {
        Self::Floor(FloorKind::default())
    }
}

impl From<FloorKind> for Tile {
    fn from(fk: FloorKind) -> Self {
        Self::Floor(fk)
    }
}

impl From<WallKind> for Tile {
    fn from(wk: WallKind) -> Self {
        Self::Wall(wk)
    }
}

impl Tile {
    /// This is a really primitive way to map from char to tile
    /// In the future, we'll need to use more clever metadata in one of the yaml
    /// files to define tiles.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '\\' | '\n' => None,
            '.' => Some(Self::Floor(FloorKind::FloorDefault)),
            '_' => Some(Self::Floor(FloorKind::FloorInterior)),
            '○' => Some(Self::Wall(WallKind::WallPillar)),
            '║' => Some(Self::Wall(WallKind::WallNS)),
            '╝' => Some(Self::Wall(WallKind::WallNW)),
            '╗' => Some(Self::Wall(WallKind::WallSW)),
            '╣' => Some(Self::Wall(WallKind::WallNSW)),
            '╚' => Some(Self::Wall(WallKind::WallNE)),
            '╔' => Some(Self::Wall(WallKind::WallSE)),
            '╠' => Some(Self::Wall(WallKind::WallNSE)),
            '═' => Some(Self::Wall(WallKind::WallEW)),
            '╦' => Some(Self::Wall(WallKind::WallEWS)),
            '╩' => Some(Self::Wall(WallKind::WallEWN)),
            '╬' => Some(Self::Wall(WallKind::WallAllSides)),
            '#' => Some(Self::Wall(WallKind::WallDefault)),
            c => Some(Self::Floor(FloorKind::FloorScenery(c))),
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            Self::Floor(floor) => floor.glyph(),
            Self::Wall(wall) => wall.glyph(),
        }
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

    pub fn fg(&self) -> Color {
        match self {
            Self::Floor(floor) => floor.fg(),
            Self::Wall(wall) => wall.fg(),
        }
    }

    pub fn bg(&self) -> Color {
        EMPTY
    }

    pub fn render(
        &self,
        resources: &mut Resources,
        pixel_point: PixelPoint,
        visibility_kind: VisibilityKind,
    ) {
        match visibility_kind {
            VisibilityKind::Torch { brightness: _ } => {
                // TODO: use torch brightness to modify rendering brightness
                // resources.font.push_char(self.glyph(), &pixel_point, None);
            }
            VisibilityKind::Remembered => {
                // resources.font.push_char(
                //     self.glyph(),
                //     &pixel_point,
                //     Some(DrawParam::new().color(COMMON.two.to_ggez_color())),
                // );
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
