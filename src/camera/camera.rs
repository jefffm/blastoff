use crate::{
    map::Map,
    util::{ScreenPoint, ScreenRect, WorldPoint, WorldToScreen},
};
use bracket_lib::prelude::*;

use super::Glyph;

pub fn render_debug_map(map: &Map, ctx: &mut BTerm, _show_boundaries: bool) {
    for (point, tile) in map.iter_tiles() {
        let w2s = WorldToScreen::default();
        let screen_point = w2s.transform_point(point);
        // if map.revealed_tiles[idx] {
        tile.render(ctx, screen_point);
    }
}

// TODO: viewport and camera: choose which to use
