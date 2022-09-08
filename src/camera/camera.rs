use crate::{
    component::Position,
    map::Map,
    util::{ScreenPoint, WorldPoint, WorldToScreen},
};
use bracket_lib::prelude::*;

use super::Glyph;

pub fn render_debug_map(map: &Map, ctx: &mut BTerm, show_boundaries: bool) {
    for (point, tile) in map.iter_tiles() {
        let w2s = WorldToScreen::default();
        let screen_point = w2s.transform_point(point);
        // if map.revealed_tiles[idx] {
        tile.render(ctx, screen_point);
    }
}

pub struct Camera {
    w2s: WorldToScreen,
}

impl Camera {
    pub fn new(w2s: WorldToScreen) -> Self {
        Self { w2s }
    }

    pub fn update_point(&mut self, point: &WorldPoint) {
        self.w2s.m31 = point.x;
        self.w2s.m32 = point.y;
        self.w2s = WorldToScreen::new(1, 0, 0, 1, 5, 5)
    }

    pub fn draw_from_world(&mut self, ctx: &mut BTerm, glyph: &Glyph, point: WorldPoint) {
        let screen_point = self.w2s.transform_point(point);
        self.draw(ctx, glyph, &screen_point)
    }

    pub fn draw(&mut self, ctx: &mut BTerm, glyph: &Glyph, point: &ScreenPoint) {
        ctx.set(point.x, point.y, glyph.fg, glyph.bg, glyph.glyph)
    }
}
