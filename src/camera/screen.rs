use std::collections::HashMap;

use ggez::graphics::{Canvas, DrawParam};
use hecs::{Entity, World};

use crate::color::RGBA8Ext;
use crate::component::{Position, Renderable};
use crate::game::consts::{get_screen_to_pixel_transform_float, USE_SPRITES};
use crate::game::draw_ui;
use crate::resource::Resources;
use crate::sector::{Map, Tile, VisibilityKind};
use crate::util::{
    PixelPoint, TransformExt, ViewportFloatToScreen, ViewportToScreen, WorldFloatPoint, WorldPoint,
    PLAYER,
};

enum RenderCell {
    Tile(Tile, VisibilityKind),
    Entity(Entity),
}

// Viewport tracks the current onscreen rect
pub struct Screen {
    transform_float: ViewportFloatToScreen,
}

impl Screen {
    pub fn new(transform: ViewportToScreen) -> Self {
        Self {
            transform_float: transform.as_float_transform(),
        }
    }

    fn draw(&self, _canvas: &mut Canvas, world: &World, resources: &mut Resources, entity: Entity) {
        let mut query = world.query_one::<(&Position, &Renderable)>(entity).unwrap();

        let (position, renderable) = query.get().unwrap();

        let worldfloat_point = position.render_point();
        let pixel_point = self.worldfloat_to_pixel(resources, worldfloat_point);

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

    /// Transform a floating point World Point allllllll the way through into an integer PixelPoint
    pub fn worldfloat_to_pixel(&self, resources: &Resources, point: WorldFloatPoint) -> PixelPoint {
        // i32 grid WorldPoint translates to a floating point viewport point
        let vp = resources.viewport.to_viewport_point_f32(point);

        // floating point viewport point translates to some division of the screen grid
        let sp = self.transform_float.transform_point(vp);

        // Screen grid floating point translates to some absolute pixel coordinates
        get_screen_to_pixel_transform_float()
            .transform_point(sp)
            .to_i32()
    }

    pub fn draw_game(
        &self,
        ctx: &mut ggez::Context,
        canvas: &mut Canvas,
        world: &World,
        resources: &mut Resources,
        map: &Map,
    ) {
        // TODO: Floating Point Viewport Translation
        // Rather than translating points in Viewport, instead we should translate the viewport rect
        // using the floating point transform in Viewport.
        //
        // Then, we determine all Grid world points visible in the rectangle.
        // For each world point with an origin in the floating point viewport rectangle, draw the rect

        let mut cells: HashMap<WorldPoint, RenderCell> = HashMap::new();

        // Use the viewport to find and render all visible Map tiles
        let viewport_points: Vec<_> = resources.viewport.points().collect();
        for viewport_point in viewport_points {
            // Use the integer world point to locate which tile to render from the map grid
            let world_point = resources.viewport.to_game_point(viewport_point);

            // It's important to make sure the point is actually in the map
            // before trying to make an index for it
            if !map.contains(world_point) {
                continue;
            }

            if map.is_visible(&world_point) {
                if let Some(tile) = map.get(world_point) {
                    // Translate the world point into floating point screen coordinates
                    let vis = VisibilityKind::Torch {
                        brightness: resources.rng.roll_dice(1, 40) as u32,
                    };
                    cells.insert(world_point, RenderCell::Tile(*tile, vis));
                }

                let mut data = map
                    .get_content(&world_point)
                    .iter()
                    .map(|entity| {
                        let mut query = world
                            .query_one::<(&Position, &Renderable)>(*entity)
                            .unwrap();

                        query
                            .get()
                            .map(|(pos, render)| (*entity, pos.grid_point(), render.render_order))
                    })
                    .filter(|x| x.is_some())
                    .collect::<Option<Vec<(Entity, WorldPoint, u32)>>>()
                    .unwrap();

                data.sort_by(|(_, _, r1), (_, _, r2)| r1.cmp(r2));

                for (entity, screen_point, _render_order) in data.into_iter() {
                    cells.insert(screen_point, RenderCell::Entity(entity));
                }
            } else if map.is_revealed(&world_point) {
                if let Some(tile) = map.get(world_point) {
                    cells.insert(
                        world_point,
                        RenderCell::Tile(*tile, VisibilityKind::Remembered),
                    );
                }
            }
        }

        for (world_point, cell) in cells.into_iter() {
            // Translate each world point on the grid through floating point transforms and into an absolute pixel point
            let pixel_point = self.worldfloat_to_pixel(resources, world_point.to_f32());
            match cell {
                RenderCell::Tile(tile, vis) => tile.render(canvas, resources, pixel_point, vis),
                RenderCell::Entity(entity) => self.draw(canvas, world, resources, entity),
            }
        }

        // Draw the UI overlay last
        draw_ui(ctx, canvas, world, resources, map);
    }
}
