use std::collections::HashMap;

use bracket_lib::terminal::to_char;
use ggez::{
    context::Has,
    graphics::{self, Canvas, DrawParam, Drawable},
};

use crate::game::consts::SCALING_FACTOR;

use super::{PixelPoint, PixelSize, SpriteSize};

fn create_cp437_string() -> String {
    (0..255u8).map(to_char).collect()
}

/// Describes the layout of characters in your
/// bitmap font.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextMap {
    pub map: HashMap<char, graphics::Rect>,
    sheet_size: SpriteSize,
}

impl TextMap {
    /// Creates a new `TextMap` from a uniform grid of
    /// sprites.  Takes the number of sprites wide and
    /// tall that the bitmap should be, and a string
    /// describing the characters in the map... in order,
    /// left to right, top to bottom.
    ///
    /// The characters do not necessarily need to fill
    /// the entire image.  ie, if your image is 16x16 glyphs
    /// for 256 total, and you only use the first 150 of them,
    /// that's fine.
    ///
    /// The floating point math involved should always be
    /// exact for `Image`'s and sprites with a resolution
    /// that is a power of two, I think.
    fn from_grid(mapping: &str, width: usize, height: usize) -> Self {
        // Assert the given width and height can fit the listed characters.
        let num_chars = mapping.chars().count();
        assert!(
            num_chars <= width * height,
            "expected {:?} characters for this spritesheet (got {:?})",
            width * height,
            num_chars
        );
        let rect_width = 1.0 / (width as f32);
        let rect_height = 1.0 / (height as f32);
        let mut map = HashMap::with_capacity(num_chars);
        let mut current_x = 0;
        let mut current_y = 0;
        for c in mapping.chars() {
            let x_offset = current_x as f32 * rect_width;
            let y_offset = current_y as f32 * rect_height;
            let char_rect = graphics::Rect {
                x: x_offset,
                y: y_offset,
                w: rect_width,
                h: rect_height,
            };
            map.insert(c, char_rect);
            current_x = (current_x + 1) % width;
            if current_x == 0 {
                current_y += 1;
            }
        }

        Self {
            map,
            // TODO: this is currently a float relative to 100% of the spritesheet size. it needs to be mapped back to absolute pixels
            sheet_size: SpriteSize::new(width as i32, height as i32),
        }
    }

    pub fn sheet_size(&self) -> SpriteSize {
        self.sheet_size
    }
}

#[derive(Debug)]
pub struct BitmapFont {
    batch: graphics::InstanceArray,
    text_map: TextMap,
    pub char_size: PixelSize,
}

impl BitmapFont {
    pub fn from_grid(
        gfx: &impl Has<graphics::GraphicsContext>,
        image: graphics::Image,
        sprite_sheet_size: &SpriteSize,
    ) -> Self {
        let mapping = create_cp437_string();
        let text_map = TextMap::from_grid(
            &mapping,
            sprite_sheet_size.width as usize,
            sprite_sheet_size.height as usize,
        );
        let batch = graphics::InstanceArray::new(gfx, image, 100, true);
        Self::new(batch, text_map)
    }

    pub fn new(batch: graphics::InstanceArray, text_map: TextMap) -> Self {
        let sheet_size = text_map.sheet_size();

        let rect_width = 1.0 / (sheet_size.width as f32);
        let rect_height = 1.0 / (sheet_size.height as f32);

        let char_size = PixelSize::new(
            (batch.image().width() as f32 * rect_width) as i32,
            (batch.image().width() as f32 * rect_height) as i32,
        );

        Self {
            batch,
            text_map,
            char_size,
        }
    }
    pub fn draw_char(
        &self,
        canvas: &mut Canvas,
        c: char,
        point: &PixelPoint,
        draw_param: Option<DrawParam>,
    ) {
        let base_param = draw_param.unwrap_or_else(DrawParam::new);
        let rect = self.get_for_char(c);
        let dest_rect = graphics::Rect::new_i32(
            point.x,
            point.y,
            self.char_size.width,
            self.char_size.height,
        );
        let draw_param = base_param
            .src(*rect)
            .dest_rect(dest_rect)
            .image_scale(false);

        canvas.draw(&self.batch.image(), draw_param);
    }

    pub fn draw_each_char(
        &self,
        canvas: &mut Canvas,
        text: &str,
        point: &PixelPoint,
        draw_param: Option<DrawParam>,
    ) {
        let draw_params = self.string_to_draw_params(text, point, draw_param);
        for draw_param in draw_params {
            canvas.draw(&self.batch.image(), draw_param);
        }
    }

    pub fn push_text(&mut self, text: &str, point: &PixelPoint, draw_param: Option<DrawParam>) {
        let draw_params = self.string_to_draw_params(text, point, draw_param);
        assert!(self.batch.capacity() > draw_params.len());
        assert!(text.len() == draw_params.len());

        for draw_param in draw_params {
            self.batch.push(draw_param);
        }
    }

    fn string_to_draw_params(
        &self,
        text: &str,
        point: &PixelPoint,
        draw_param: Option<DrawParam>,
    ) -> Vec<DrawParam> {
        let base_param = draw_param.unwrap_or_else(DrawParam::new);
        text.chars()
            // TODO: how to handle whitespace??
            .map(|c| self.get_for_char(c))
            .enumerate()
            .map(|(i, rect)| {
                let dest_rect = graphics::Rect::new_i32(
                    point.x + (i as i32 * self.char_size.width),
                    point.y,
                    self.char_size.width,
                    self.char_size.height,
                );
                base_param
                    .src(*rect)
                    .dest_rect(dest_rect)
                    .image_scale(false)
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.batch.clear()
    }

    fn get_for_char(&self, c: char) -> &graphics::Rect {
        self.text_map.map.get(&c).unwrap()
    }
}

impl Drawable for BitmapFont {
    fn draw(&self, canvas: &mut Canvas, param: DrawParam) {
        canvas.draw(&self.batch, param)
    }

    fn dimensions(
        &self,
        gfx: &mut impl ggez::context::HasMut<graphics::GraphicsContext>,
    ) -> Option<graphics::Rect> {
        self.batch.dimensions(gfx)
    }
}
