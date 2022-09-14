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
}
