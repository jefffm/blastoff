use macroquad::prelude::*;

use crate::util::{
    to_cp437, PixelPoint, PixelSize, PointExt, SpritePoint, SpriteSize, SpriteToPixel,
};
#[derive(Debug)]
pub struct BitmapFont {
    texture: Texture2D,
    pub char_size: PixelSize,
    pub sheet_size: SpriteSize,
    sprite_to_pixel: SpriteToPixel,
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
            sprite_to_pixel: SpriteToPixel::scale(char_size.width, char_size.height),
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

        let sprite_point = SpritePoint::from_index(byte as usize, self.sheet_size.width);
        let sprite_pixel_point = self.sprite_to_pixel.transform_point(sprite_point).to_f32();

        Rect {
            x: sprite_pixel_point.x,
            y: sprite_pixel_point.y,
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
            let dest =
                PixelPoint::new(point.x + (i as i32 * self.char_size.width), point.y).to_f32();
            draw_texture_ex(self.texture, dest.x, dest.y, WHITE, param);
        }
    }
}
