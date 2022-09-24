use ggez::graphics::{Canvas, DrawParam};
use hecs::World;

use crate::camera::Glyph;
use crate::color::RGBA8Ext;
use crate::component::{Position, Renderable};
use crate::game::consts::get_screen_to_pixel_transform;
use crate::game::draw_ui;
use crate::map::VisibilityKind;
use crate::resource::Resources;
use crate::util::{ScreenPoint, ViewportPoint, ViewportToScreen};

// Viewport tracks the current onscreen rect
pub struct Screen {
    transform: ViewportToScreen,
}

impl Screen {
    pub fn new(transform: ViewportToScreen) -> Self {
        Self { transform }
    }

    fn draw(&self, canvas: &mut Canvas, resources: &Resources, glyph: &Glyph, point: ScreenPoint) {
        let pixel_point = get_screen_to_pixel_transform().transform_point(point);

        resources.font.draw_char_overwrite(
            canvas,
            glyph.glyph,
            &pixel_point,
            Some(DrawParam::new().color(glyph.fg.to_ggez_color())),
        );
    }

    pub fn to_screen_point(&self, point: ViewportPoint) -> ScreenPoint {
        self.transform.transform_point(point)
    }

    pub fn draw_game(&self, canvas: &mut Canvas, world: &World, resources: &mut Resources) {
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
                        let vis = VisibilityKind::Torch {
                            brightness: resources.rng.roll_dice(1, 40) as u32,
                        };
                        tile.render(canvas, resources, screen_point, vis);
                    }

                    let mut data = map
                        .get_content(&world_point)
                        .iter()
                        .map(|entity| {
                            let mut query = world
                                .query_one::<(&Position, &Renderable)>(*entity)
                                .unwrap();

                            query.get().map(|(pos, render)| (*pos, *render))
                        })
                        .filter(|x| x.is_some())
                        .collect::<Option<Vec<(Position, Renderable)>>>()
                        .unwrap();

                    data.sort_by(|(_, r1), (_, r2)| r1.render_order.cmp(&r2.render_order));
                    for (pos, render) in data.iter() {
                        let viewport_point = viewport.to_viewport_point(pos.p);
                        let screen_point = self.to_screen_point(viewport_point);
                        self.draw(canvas, resources, &render.glyph, screen_point);
                    }
                } else if map.is_revealed(&world_point) {
                    if let Some(tile) = map.get(world_point) {
                        let screen_point = self.to_screen_point(viewport_point);
                        tile.render(canvas, resources, screen_point, VisibilityKind::Remembered);
                    }
                }
            }
        }

        // Draw the UI overlay last
        draw_ui(canvas, world, resources);
    }
}
