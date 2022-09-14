use bracket_lib::prelude::*;
use hecs::World;

use crate::camera::Glyph;
use crate::component::{Position, Renderable};

use crate::util::{ScreenPoint, ScreenRect, ViewportPoint, ViewportToScreen};

use super::Resources;

// Viewport tracks the current onscreen rect
pub struct Screen {
    rect: ScreenRect,
    transform: ViewportToScreen,
}

impl Screen {
    pub fn new(rect: ScreenRect, transform: ViewportToScreen) -> Self {
        Self { rect, transform }
    }

    fn draw(&self, draw_batch: &mut DrawBatch, glyph: &Glyph, point: &ScreenPoint) {
        draw_batch.set(
            Point::new(point.x, point.y),
            ColorPair::new(glyph.fg, glyph.bg),
            glyph.glyph,
        );
    }

    pub fn to_screen_point(&self, point: ViewportPoint) -> ScreenPoint {
        self.transform.transform_point(point)
    }

    pub fn draw_game(&self, world: &World, resources: &Resources, draw_batch: &mut DrawBatch) {
        draw_batch.cls();

        let viewport = &resources.viewport;
        let map = resources.map.as_ref().unwrap();

        // Use the viewport to find and render all visible Map tiles
        for viewport_point in viewport.points() {
            let world_point = viewport.to_world_point(viewport_point);
            if let Some(tile) = map.get(world_point) {
                let screen_point = self.to_screen_point(viewport_point);
                tile.render(draw_batch, screen_point);
            }
        }

        // Use the ECS to find and draw every renderable component
        let mut data = world
            .query::<(&Position, &Renderable)>()
            .iter()
            .map(|(_ent, (&pos, render))| (pos, render.clone()))
            .collect::<Vec<_>>();

        // Implement render ordering
        data.sort_by(|(_, r1), (_, r2)| r1.render_order.cmp(&r2.render_order));
        for (pos, render) in data.iter() {
            let viewport_point = viewport.to_viewport_point(pos.p);
            let screen_point = self.to_screen_point(viewport_point);
            self.draw(draw_batch, &render.glyph, &screen_point);
        }

        // Draw the UI overlay last
        // draw_ui(&self.resources, ctx);

        draw_batch.submit(0).expect("DrawBatch submit");
    }
}
