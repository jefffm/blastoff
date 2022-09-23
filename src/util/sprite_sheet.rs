use std::ops::Index;

use ggez::{context::Has, graphics};

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

#[derive(Debug)]
pub struct SpriteSheet {
    batch: graphics::InstanceArray,
    sprite_grid: SpriteGrid,
    sprite_size: PixelSize,
}

impl SpriteSheet {
    pub fn from_grid(
        gfx: &impl Has<graphics::GraphicsContext>,
        image: graphics::Image,
        sprite_sheet_size: SpriteSize,
    ) -> Self {
        let sprite_grid = SpriteGrid::from_grid(sprite_sheet_size);
        let batch = graphics::InstanceArray::new(gfx, image, 100, true);
        Self::new(batch, sprite_grid)
    }

    pub fn new(batch: graphics::InstanceArray, sprite_grid: SpriteGrid) -> Self {
        let sheet_size = sprite_grid.sheet_size();

        let rect_width = 1.0 / (sheet_size.width as f32);
        let rect_height = 1.0 / (sheet_size.height as f32);

        let sprite_size = PixelSize::new(
            (batch.image().width() as f32 * rect_width) as i32,
            (batch.image().height() as f32 * rect_height) as i32,
        );

        tracing::info!(
            "Loaded spritesheet of size {:?} with tiles {:?}",
            sheet_size,
            sprite_size
        );

        Self {
            batch,
            sprite_grid,
            sprite_size,
        }
    }

    fn sprite(&self, sprite: SpritePoint, point: PixelPoint) -> graphics::DrawParam {
        assert!(self.sprite_grid.contains(sprite));

        let sprite_rect = self.sprite_grid[&sprite];
        let dest_rect = graphics::Rect::new_i32(
            point.x,
            point.y,
            self.sprite_size.width,
            self.sprite_size.height,
        );
        graphics::DrawParam::new()
            .src(sprite_rect)
            .dest_rect(dest_rect)
            .image_scale(false)
    }

    pub fn push_sprite(
        &mut self,
        sprite: SpritePoint,
        point: PixelPoint,
    ) -> &impl graphics::Drawable {
        self.batch.set(vec![self.sprite(sprite, point)]);

        &self.batch
    }
}
