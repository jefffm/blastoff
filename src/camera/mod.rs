use crate::map::Map;
use bracket_lib::prelude::*;
use tracing::debug;

mod glyph;
pub use glyph::*;

pub fn render_debug_map(map: &Map, ctx: &mut BTerm, show_boundaries: bool) {
    for point in map.tiles.values() {
        // if map.revealed_tiles[idx] {
        let tile = &map.tiles[point];
        tile.render(ctx, point.x, point.y);
    }
}
