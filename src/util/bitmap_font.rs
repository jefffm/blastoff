use std::collections::HashMap;

use bracket_lib::terminal::to_char;
use ggez::{context::Has, graphics};

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
    char_size: PixelSize,
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
            char_size: PixelSize::new(rect_width as i32, rect_height as i32),
        }
    }

    pub fn char_width(&self) -> i32 {
        self.char_size.width
    }

    pub fn char_height(&self) -> i32 {
        self.char_size.height
    }
}

#[derive(Debug)]
pub struct BitmapFont {
    batch: graphics::InstanceArray,
    text_map: TextMap,
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
        Self { batch, text_map }
    }

    /// Create and return a Drawable InstanceArray of a single string
    pub fn text(&mut self, text: &str, point: &PixelPoint) -> &impl graphics::Drawable {
        let draw_params: Vec<_> = text
            .chars()
            // TODO: how to handle whitespace??
            .map(|c| self.get_for_char(c))
            .enumerate()
            .map(|(i, rect)| {
                let dest_rect = graphics::Rect::new_i32(
                    point.x + (i as i32 * self.text_map.char_width()),
                    point.y,
                    self.text_map.char_width(),
                    self.text_map.char_height(),
                );
                graphics::DrawParam::new()
                    .src(*rect)
                    .dest_rect(dest_rect)
                    .image_scale(false)
            })
            .collect();

        assert!(self.batch.capacity() > draw_params.len());
        assert!(text.len() == draw_params.len());

        self.batch.set(draw_params);

        &self.batch
    }

    fn get_for_char(&self, c: char) -> &graphics::Rect {
        self.text_map.map.get(&c).unwrap()
    }
}
