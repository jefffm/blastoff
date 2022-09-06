use bracket_lib::prelude::{to_cp437, BTerm, BLACK, WHITE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Tile {
    kind: TileKind,
    revealed: bool,
    visible: bool,
}

impl Tile {
    pub fn render(&self, ctx: &mut BTerm, x: i32, y: i32) {
        self.kind.render(ctx, x, y)
    }
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

const FLOOR_GLYPH: char = '.';
const WALL_GLYPH: char = '#';

impl TileKind {
    pub fn render(&self, ctx: &mut BTerm, x: i32, y: i32) {
        let glyph = match self {
            Self::Floor => FLOOR_GLYPH,
            Self::Wall => WALL_GLYPH,
        };
        ctx.set(x, y, WHITE, BLACK, to_cp437(glyph));
    }
}

impl From<TileKind> for Tile {
    fn from(kind: TileKind) -> Self {
        Self {
            kind: kind,
            revealed: false,
            visible: false,
        }
    }
}
