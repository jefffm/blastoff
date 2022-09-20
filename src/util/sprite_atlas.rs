use std::{collections::HashMap, io::Read};

use rgb::RGBA8;

use crate::map::Tile;

use super::{blit, Drawable, PixelPoint, PixelSize, Sprite, SpritePoint, SpriteRect, SpriteSize};

/// Is used to split one `Texture2D` into different tiles.
#[derive(Debug)]
pub struct SpriteAtlas {
    pixels: Vec<u8>,
    sprite_size: SpriteSize,
    sheet_rect: SpriteRect,
    loaded: HashMap<SpritePoint, Sprite>,
}

impl SpriteAtlas {
    pub fn from_png<R: Read>(r: R, sprite_size: SpriteSize) -> Self {
        let decoder = png::Decoder::new(r);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];

        let sheet_size = SpriteSize::new(info.width as i32, info.height as i32);

        Self::new(Vec::from(bytes), sprite_size, sheet_size)
    }

    /// Initialize the atlas from the texture and tile size.
    pub fn new(pixels: Vec<u8>, sprite_size: SpriteSize, sheet_size: SpriteSize) -> Self {
        Self {
            pixels,
            sprite_size,
            sheet_rect: SpriteRect::new(SpritePoint::new(0, 0), sheet_size),
            loaded: HashMap::new(),
        }
    }

    fn sprite_rect(&self, origin: SpritePoint) -> SpriteRect {
        SpriteRect::new(origin, self.sprite_size)
    }

    fn sprite(&mut self, origin: SpritePoint) -> &Sprite {
        let already_loaded = self.loaded.contains_key(&origin);
        match already_loaded {
            true => &self.loaded[&origin],
            false => {
                let sprite_rect = self.sprite_rect(origin);

                // Create a destination buffer with length to contain each four-byte RGBA pixel
                let mut buf = vec![0; sprite_rect.area() as usize * 4];

                let width = sprite_rect.width() as usize * 4;

                let mut s = 0;
                for y in 0..sprite_rect.height() as usize {
                    let i = origin.x as usize * 4
                        + origin.y as usize * self.sheet_rect.width() as usize * 4
                        + y * self.sheet_rect.width() as usize * 4;

                    let zipped = buf[s..s + width].iter_mut().zip(&self.pixels[i..i + width]);
                    for (left, &right) in zipped {
                        *left = right;
                    }

                    s += width;
                }

                self.loaded.insert(
                    origin,
                    Sprite::new(
                        PixelSize::new(self.sprite_size.height, self.sprite_size.width),
                        buf,
                    ),
                );

                &self.loaded[&origin]
            }
        }
    }

    /// Draw provided Tiles kind (e.g. `Tiles::Grass`) to the given position.
    pub fn draw(&mut self, screen: &mut [u8], dest: &PixelPoint, tile: &Tile, _color: RGBA8) {
        let sprite = self.sprite(tile.value());
        blit(screen, dest, sprite)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use crate::util::Drawable;

    #[test]
    fn sprite_references() {
        // 2x2 RGBA pixels
        let buf = &[
            // first row, 4 pixels
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            // second row, 4 pixels
            1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
            // third row, 4 pixels
            2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8,
            // fourth row, 4 pixels
            3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8,
        ];
        // A 2x2 spritesheet with 2px x 2px sprites
        let mut atlas = SpriteAtlas::new(
            Vec::<u8>::from(*buf),
            SpriteSize::new(2, 2),
            SpriteSize::new(4, 4),
        );

        assert_eq!(
            atlas.sprite(SpritePoint::new(0, 0)).pixels(),
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // 2x2 upper left corner
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
            ],
            "0, 0"
        );
        assert_eq!(
            atlas.sprite(SpritePoint::new(1, 0)).pixels(),
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // 2x2 upper right corner
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
            ],
            "1, 0"
        );
        assert_eq!(
            atlas.sprite(SpritePoint::new(0, 2)).pixels(),
            &[
                2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, // 2x2 bottom left
                3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8,
            ],
            "0, 2"
        );
        assert_eq!(
            atlas.sprite(SpritePoint::new(2, 2)).pixels(),
            &[
                2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, // 2x2 bottom right
                3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8,
            ],
            "2, 2"
        );
    }

    #[test]
    fn sprite_from_png() {
        let mut atlas = SpriteAtlas::from_png(
            Cursor::new(include_bytes!("../../assets/tileset/test.png")),
            SpriteSize::new(1, 1),
        );

        // top left
        assert_eq!(
            atlas.sprite(SpritePoint::new(0, 0)).pixels(),
            &[255u8, 255u8, 255u8, 255u8]
        );
        // top right
        assert_eq!(
            atlas.sprite(SpritePoint::new(1, 0)).pixels(),
            &[172u8, 50u8, 50u8, 255u8]
        );
        // bottom left
        assert_eq!(
            atlas.sprite(SpritePoint::new(0, 1)).pixels(),
            &[63u8, 63u8, 116u8, 255u8]
        );
        // bottom right
        assert_eq!(
            atlas.sprite(SpritePoint::new(1, 1)).pixels(),
            &[106u8, 190u8, 48u8, 255u8]
        );
    }
}
