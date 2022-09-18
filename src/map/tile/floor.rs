use crate::color::COMMON;

use super::TileHandler;

pub struct Floor {}
impl TileHandler for Floor {
    fn glyph(&self) -> char {
        '.'
    }

    fn is_passable(&self) -> bool {
        true
    }

    fn is_opaque(&self) -> bool {
        false
    }

    fn fg(&self) -> rgb::RGBA8 {
        COMMON.three
    }
}
