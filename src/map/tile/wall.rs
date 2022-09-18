use crate::color::COMMON;

use super::TileHandler;

pub struct Wall {}
impl TileHandler for Wall {
    fn glyph(&self) -> char {
        '#'
    }

    fn is_passable(&self) -> bool {
        false
    }

    fn is_opaque(&self) -> bool {
        true
    }

    fn fg(&self) -> rgb::RGBA8 {
        COMMON.four
    }
}
