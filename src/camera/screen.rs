use bracket_lib::prelude::*;
use hecs::World;

use crate::camera::Glyph;
use crate::component::{Position, Renderable};
use crate::game::draw_ui;
use crate::map::VisibilityKind;
use crate::resource::Resources;
use crate::util::{ScreenPoint, ScreenRect, ViewportPoint, ViewportToScreen};

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

    pub fn draw_game(
        &self,
        world: &World,
        resources: &mut Resources,
        ctx: &BTerm,
        draw_batch: &mut DrawBatch,
    ) {
        draw_batch.cls();

        let viewport = &resources.viewport;
        let map = resources.map.as_ref().unwrap();

        // Use the viewport to find and render all visible Map tiles
        for viewport_point in viewport.points() {
            let world_point = viewport.to_world_point(viewport_point);
            // It's important to make sure the point is actually in the map
            // before trying to make an index for it
            if map.contains(world_point) {
                if map.is_visible(&world_point) {
                    if let Some(tile) = map.get(world_point) {
                        let screen_point = self.to_screen_point(viewport_point);
                        tile.handler().render(
                            draw_batch,
                            screen_point,
                            VisibilityKind::Torch {
                                brightness: resources.rng.roll_dice(1, 40) as u32,
                            },
                        );
                    }
                } else if map.is_revealed(&world_point) {
                    if let Some(tile) = map.get(world_point) {
                        let screen_point = self.to_screen_point(viewport_point);
                        tile.handler()
                            .render(draw_batch, screen_point, VisibilityKind::Remembered);
                    }
                }
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
        draw_ui(resources, ctx, draw_batch);

        draw_batch.submit(0).expect("DrawBatch submit");
    }
}
