use rgb::RGBA8;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Glyph {
    pub glyph: char,
    pub fg: RGBA8,
    pub bg: RGBA8,
}

impl Glyph {
    pub fn new(glyph: char, fg: RGBA8, bg: RGBA8) -> Self {
        Self { glyph, fg, bg }
    }
}
