use std::collections::HashMap;

use ggez::graphics::{Canvas, DrawParam};
use hecs::{Entity, World};

use crate::color::RGBA8Ext;
use crate::component::{Position, Renderable};
use crate::game::consts::{get_screen_to_pixel_transform, USE_SPRITES};
use crate::game::draw_ui;
use crate::map::{Map, Tile, VisibilityKind};
use crate::resource::Resources;
use crate::util::{
    PixelPoint, ScreenPoint, TransformExt, ViewportFloatToScreen, ViewportPoint, ViewportToScreen,
    WorldFloatPoint, PLAYER,
};

enum RenderCell {
    Tile(Tile, VisibilityKind),
    Entity(Entity),
}

// Viewport tracks the current onscreen rect
pub struct Screen {
    transform: ViewportToScreen,
    transform_float: ViewportFloatToScreen,
}

impl Screen {
    pub fn new(transform: ViewportToScreen) -> Self {
        // TODO: move this to TransformExt in geometry.rs
        let params = transform.to_array();
        let transform_float = ViewportFloatToScreen::new(
            params[0] as f32,
            params[1] as f32,
            params[2] as f32,
            params[3] as f32,
            params[4] as f32,
            params[5] as f32,
        );
        Self {
            transform,
            transform_float,
        }
    }

    fn draw(
        &self,
        canvas: &mut Canvas,
        world: &World,
        resources: &mut Resources,
        entity: Entity,
        point: ScreenPoint,
    ) {
        let mut query = world.query_one::<&Renderable>(entity).unwrap();

        let renderable = query.get().unwrap();
        let pixel_point = get_screen_to_pixel_transform().transform_point(point);

        if let Some(animation) = renderable.sequence {
            let pos = renderable.current_pos();
        }

        if USE_SPRITES {
            let sprite = PLAYER;
            resources.spritesheet.push_sprite(sprite, pixel_point);
        } else {
            let glyph = renderable.glyph;
            resources.font.push_char(
                glyph.glyph,
                &pixel_point,
                Some(DrawParam::new().color(glyph.fg.to_ggez_color())),
            );
        }
    }

    pub fn to_screen_point(&self, point: ViewportPoint) -> ScreenPoint {
        self.transform.transform_point(point)
    }

    pub fn worldfloat_to_pixel(&self, resources: &Resources, point: WorldFloatPoint) -> PixelPoint {
        let vp = resources.viewport.to_viewport_point_f32(point);
        let sp = self.transform_float.transform_point(vp);
        let pp = get_screen_to_pixel_transform().transform_float_point(sp);

        pp
    }

    pub fn draw_game(
        &self,
        ctx: &mut ggez::Context,
        canvas: &mut Canvas,
        world: &World,
        resources: &mut Resources,
        map: &Map,
    ) {
        let viewport_points: Vec<_> = resources.viewport.points().collect();

        let mut cells: HashMap<ScreenPoint, RenderCell> = HashMap::new();

        // Use the viewport to find and render all visible Map tiles
        for viewport_point in viewport_points {
            let world_point = resources.viewport.to_world_point(viewport_point);
            // It's important to make sure the point is actually in the map
            // before trying to make an index for it
            if !map.contains(world_point) {
                continue;
            }

            if map.is_visible(&world_point) {
                if let Some(tile) = map.get(world_point) {
                    let screen_point = self.to_screen_point(viewport_point);
                    let vis = VisibilityKind::Torch {
                        brightness: resources.rng.roll_dice(1, 40) as u32,
                    };
                    cells.insert(screen_point, RenderCell::Tile(*tile, vis));
                }

                let mut data = map
                    .get_content(&world_point)
                    .iter()
                    .map(|entity| {
                        let mut query = world
                            .query_one::<(&Position, &Renderable)>(*entity)
                            .unwrap();

                        query.get().map(|(pos, render)| {
                            let viewport_point = resources.viewport.to_viewport_point(pos.p);
                            let screen_point = self.to_screen_point(viewport_point);

                            (*entity, screen_point, render.render_order)
                        })
                    })
                    .filter(|x| x.is_some())
                    .collect::<Option<Vec<(Entity, ScreenPoint, u32)>>>()
                    .unwrap();

                data.sort_by(|(_, _, r1), (_, _, r2)| r1.cmp(&r2));

                for (entity, screen_point, _render_order) in data.into_iter() {
                    cells.insert(screen_point, RenderCell::Entity(entity));
                }
            } else if map.is_revealed(&world_point) {
                if let Some(tile) = map.get(world_point) {
                    let screen_point = self.to_screen_point(viewport_point);
                    cells.insert(
                        screen_point,
                        RenderCell::Tile(*tile, VisibilityKind::Remembered),
                    );
                }
            }
        }

        for (screen_point, cell) in cells.into_iter() {
            match cell {
                RenderCell::Tile(tile, vis) => tile.render(canvas, resources, screen_point, vis),
                RenderCell::Entity(entity) => {
                    self.draw(canvas, world, resources, entity, screen_point)
                }
            }
        }

        // Draw the UI overlay last
        draw_ui(ctx, canvas, world, resources, map);
    }
}
