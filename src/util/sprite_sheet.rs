use std::ops::Index;

use ggez::{
    context::Has,
    graphics::{self, Canvas, DrawParam, Drawable},
};

use crate::color::{RGBA8Ext, EMPTY};

use super::{PixelPoint, PixelSize, PointExt, SpritePoint, SpriteRect, SpriteSize};

/// Describes the layout of characters in your
/// bitmap font.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpriteGrid {
    sprites: Vec<graphics::Rect>,
    sheet_rect: SpriteRect,
}

impl SpriteGrid {
    fn from_grid(sheet_size: SpriteSize) -> Self {
        let sheet_rect = SpriteRect::new(SpritePoint::new(0, 0), sheet_size);

        let rect_width = 1.0 / (sheet_size.width as f32);
        let rect_height = 1.0 / (sheet_size.height as f32);
        let mut sprites = Vec::with_capacity(sheet_size.area() as usize);

        let mut current_x = 0;
        let mut current_y = 0;

        for _ in 0..sheet_size.area() {
            let x_offset = current_x as f32 * rect_width;
            let y_offset = current_y as f32 * rect_height;
            let sprite_rect = graphics::Rect {
                x: x_offset,
                y: y_offset,
                w: rect_width,
                h: rect_height,
            };

            sprites.push(sprite_rect);
            current_x = (current_x + 1) % sheet_size.width;
            if current_x == 0 {
                current_y += 1;
            }
        }

        Self {
            sprites,
            sheet_rect,
        }
    }

    pub fn sheet_size(&self) -> SpriteSize {
        self.sheet_rect.size
    }

    pub fn contains(&self, point: SpritePoint) -> bool {
        self.sheet_rect.contains(point)
    }
}

impl Index<&SpritePoint> for SpriteGrid {
    type Output = graphics::Rect;

    fn index(&self, point: &SpritePoint) -> &Self::Output {
        let idx = point.to_index(self.sheet_size().width);
        &self.sprites[idx]
    }
}

pub struct SpriteSheet {
    batch: graphics::InstanceArray,
    sprite_grid: SpriteGrid,
    sprite_size: PixelSize,
    clear_rect: graphics::Mesh,
}

impl SpriteSheet {
    pub fn from_grid(
        gfx: &impl Has<graphics::GraphicsContext>,
        image: graphics::Image,
        sprite_sheet_size: SpriteSize,
    ) -> Self {
        let sprite_grid = SpriteGrid::from_grid(sprite_sheet_size);
        let batch = graphics::InstanceArray::new(gfx, image, 100);

        let rect_width = 1.0 / (sprite_sheet_size.width as f32);
        let rect_height = 1.0 / (sprite_sheet_size.height as f32);

        let sprite_size = PixelSize::new(
            (batch.image().width() as f32 * rect_width) as i32,
            (batch.image().height() as f32 * rect_height) as i32,
        );

        tracing::info!(
            "Loaded spritesheet of size {:?} with tiles {:?}",
            sprite_sheet_size,
            sprite_size
        );

        let clear_rect = graphics::Mesh::new_rectangle(
            gfx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(0, 0, sprite_size.width, sprite_size.height),
            EMPTY.to_ggez_color(),
        )
        .expect("create clear rect");

        Self::new(batch, sprite_grid, sprite_size, clear_rect)
    }

    pub fn new(
        batch: graphics::InstanceArray,
        sprite_grid: SpriteGrid,
        sprite_size: PixelSize,
        clear_rect: graphics::Mesh,
    ) -> Self {
        Self {
            batch,
            sprite_grid,
            sprite_size,
            clear_rect,
        }
    }

    fn sprite(
        &self,
        sprite: SpritePoint,
        point: PixelPoint,
        param: Option<graphics::DrawParam>,
    ) -> graphics::DrawParam {
        assert!(self.sprite_grid.contains(sprite));

        let sprite_rect = self.sprite_grid[&sprite];

        param
            .unwrap_or_else(graphics::DrawParam::new)
            .src(sprite_rect)
            .dest([point.x as f32, point.y as f32])
    }

    pub fn draw_sprite(&self, canvas: &mut Canvas, sprite: Sprite, point: PixelPoint) {
        let draw_param = self.sprite(sprite.idx, point, sprite.param);
        canvas.draw(&self.batch.image(), draw_param);
    }

    pub fn draw_sprite_overwrite(&self, canvas: &mut Canvas, sprite: Sprite, point: PixelPoint) {
        let draw_param = self.sprite(sprite.idx, point, sprite.param);
        canvas.draw(&self.clear_rect, draw_param);
        canvas.draw(&self.batch.image(), draw_param);
    }

    pub fn push_sprite(&mut self, sprite: Sprite, point: PixelPoint) -> &impl graphics::Drawable {
        self.batch
            .push(self.sprite(sprite.idx, point, sprite.param));

        &self.batch
    }

    pub fn clear(&mut self) {
        self.batch.clear()
    }
}

impl Drawable for SpriteSheet {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        canvas.draw(&self.batch, param)
    }

    fn dimensions(&self, gfx: &impl Has<graphics::GraphicsContext>) -> Option<graphics::Rect> {
        self.batch.dimensions(gfx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
    idx: SpritePoint,
    param: Option<DrawParam>,
}

impl Sprite {
    pub const fn new(idx: SpritePoint, param: Option<DrawParam>) -> Self {
        Self { idx, param }
    }

    pub fn with_params(mut self, param: DrawParam) -> Self {
        self.param = Some(param);
        self
    }
}

pub const PLAYER: Sprite = Sprite::new(SpritePoint::new(28, 9), None);
pub const ANIMAL1: Sprite = Sprite::new(SpritePoint::new(29, 7), None);
pub const ANIMAL2: Sprite = Sprite::new(SpritePoint::new(30, 7), None);
pub const ANIMAL3: Sprite = Sprite::new(SpritePoint::new(31, 7), None);
pub const ANIMAL4: Sprite = Sprite::new(SpritePoint::new(32, 7), None);
pub const PLANET: Sprite = Sprite::new(SpritePoint::new(20, 5), None);
