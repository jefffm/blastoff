use bracket_lib::prelude::BTerm;
use bracket_lib::prelude::Point;
use serde::{Deserialize, Serialize};

use crate::{component::Renderable, util::RGB};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Glyph {
    pub glyph: u16,
    pub fg: RGB,
    pub bg: RGB,
}

impl Glyph {
    pub fn new(glyph: u16, fg: RGB, bg: RGB) -> Self {
        Self { glyph, fg, bg }
    }
    pub fn render(&self, ctx: &mut BTerm, x: i32, y: i32) {
        ctx.set(x, y, self.fg.clone(), self.bg.clone(), self.glyph)
    }

    pub fn into_renderable(self, x: i32, y: i32) -> Renderable {
        Renderable {
            position: Point::new(x, y),
            fg: self.fg.into(),
            bg: self.bg.into(),
            glyph: self.glyph,
        }
    }
}
