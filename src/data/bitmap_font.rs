use std::collections::HashMap;

use euclid::Size2D;
use macroquad::prelude::*;

use crate::{
    color::EMPTY,
    util::{to_cp437, PixelPoint, PixelSize, PixelSpace, SpriteSize},
};
#[derive(Debug)]
pub struct BitmapFont {
    texture: Texture2D,
    pub char_size: PixelSize,
    pub sheet_size: SpriteSize,
}

// TODO: implement bitmap font drawing. Some inspo: https://github.com/gamma-delta/haxagon/blob/main/src/utils/text/mod.rs
// also see draw_text_ex in macroquad
impl BitmapFont {
    /// Load a texture and a Sprite-index-size into a BitmapFont
    ///
    /// SpriteSize is measured in units of uniform-sized sprites, eg a
    /// spritesheet containing 30x25 sprites. The sprite size in pixels is
    /// derived from the total pixel size of the sheet.
    pub fn new(texture: Texture2D, char_size: PixelSize, sheet_size: SpriteSize) -> Self {
        Self {
            texture,
            char_size,
            sheet_size,
        }
    }

    /// Transform a string of chars to a Vec of Rects mapping into the font Texture2d
    fn string_to_draw_params(
        &self,
        text: &str,
        draw_param: Option<DrawTextureParams>,
    ) -> Vec<DrawTextureParams> {
        let base_param = draw_param.unwrap_or_default();
        text.chars()
            .map(|c| self.get_for_char(c))
            .enumerate()
            .map(|(i, rect)| {
                let mut param = base_param.clone();

                param.source = Some(rect);
                param.dest_size = Some(vec2(
                    self.char_size.width as f32,
                    self.char_size.height as f32,
                ));

                param
            })
            .collect()
    }

    /// Get the Rect corresponding to a given character
    fn get_for_char(&self, c: char) -> Rect {
        let byte = to_cp437(c);
        let col = byte as f32 % self.sheet_size.width as f32;
        let row = self.sheet_size.width as f32 / byte as f32;

        Rect {
            x: col,
            y: row,
            w: self.char_size.width as f32,
            h: self.char_size.height as f32,
        }
    }

    /// Draw a string of chars starting at a top-left corner point
    pub fn draw(&self, text: &str, point: PixelPoint) {
        for (i, param) in self
            .string_to_draw_params(text, None)
            .into_iter()
            .enumerate()
        {
            draw_texture_ex(
                self.texture,
                point.x as f32 * (i as f32 * self.char_size.width as f32),
                point.y as f32,
                WHITE,
                param,
            );
        }
    }
}
