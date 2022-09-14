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
}
