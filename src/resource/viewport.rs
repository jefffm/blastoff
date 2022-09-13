use bracket_lib::prelude::*;

use crate::camera::Glyph;
use crate::util::{ScreenPoint, ScreenRect, WorldPoint, WorldToScreen};

// Viewport tracks the current onscreen rect
pub struct Viewport {
    screen: ScreenRect,
    w2s: WorldToScreen,
}

impl Viewport {
    pub fn new(screen: ScreenRect, w2s: WorldToScreen) -> Self {
        Self { screen, w2s }
    }

    pub fn from_points(screen_point: ScreenPoint, world_point: WorldPoint) -> WorldToScreen {
        let translation = screen_point.to_untyped() - world_point.to_untyped();
        WorldToScreen::new(1, 0, 0, 1, translation.x, translation.y)
    }

    pub fn draw_from_world(&mut self, ctx: &mut BTerm, glyph: &Glyph, point: WorldPoint) {
        let screen_point = self.w2s.transform_point(point);
        self.draw(ctx, glyph, &screen_point)
    }

    pub fn draw(&mut self, ctx: &mut BTerm, glyph: &Glyph, point: &ScreenPoint) {
        ctx.set(point.x, point.y, glyph.fg, glyph.bg, glyph.glyph)
    }
}
