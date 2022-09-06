use bracket_lib::prelude::*;

use crate::camera::Glyph;

pub struct Renderable {
    pub position: Point,
    pub fg: RGB,
    pub bg: RGB,
    pub glyph: u16,
}

impl Renderable {
    pub fn new(position: Point, fg: (u8, u8, u8), bg: (u8, u8, u8), glyph: char) -> Self {
        Self {
            position,
            fg: RGB::from(fg),
            bg: RGB::from(bg),
            glyph: to_cp437(glyph),
        }
    }
}

impl From<Renderable> for Glyph {
    fn from(renderable: Renderable) -> Self {
        Self {
            glyph: renderable.glyph,
            fg: renderable.fg.into(),
            bg: renderable.bg.into(),
        }
    }
}
