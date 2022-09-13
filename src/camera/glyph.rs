use bracket_lib::prelude::BTerm;
use bracket_lib::prelude::RGBA;
use serde::{Deserialize, Serialize};

use crate::util::ScreenPoint;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Glyph {
    pub glyph: u16,
    pub fg: RGBA,
    pub bg: RGBA,
}

impl Glyph {
    pub fn new(glyph: u16, fg: RGBA, bg: RGBA) -> Self {
        Self { glyph, fg, bg }
    }
    pub fn render(&self, ctx: &mut BTerm, point: &ScreenPoint) {
        ctx.set(
            point.x,
            point.y,
            self.fg,
            self.bg,
            self.glyph,
        )
    }
}
