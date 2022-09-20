use euclid::{Rect, Size2D, UnknownUnit};
use rgb::RGBA8;

use crate::map::Tile;

use super::{
    blit, Drawable, PixelPoint, PixelSize, SpritePoint, SpriteRect, SpriteRef, SpriteSize,
};

/// Is used to split one `Texture2D` into different tiles.
#[derive(Clone, Debug)]
pub struct SpriteAtlas {
    pixels: Vec<u8>,
    size: SpriteSize,
}

impl SpriteAtlas {
    /// Initialize the atlas from the texture and tile size.
    pub const fn new(pixels: Vec<u8>, size: SpriteSize) -> Self {
        Self { pixels, size }
    }

    fn rect(&self, point: SpritePoint) -> SpriteRect {
        SpriteRect::new(point, self.size)
    }

    fn sprite(&self, point: SpritePoint) -> SpriteRef {
        // for i in 0..bytes.len() {
        //     self.pixels[i * 4] = bytes[i].0[0];
        //     self.pixels[i * 4 + 1] = bytes[i].0[1];
        //     self.pixels[i * 4 + 2] = bytes[i].0[2];
        //     self.pixels[i * 4 + 3] = bytes[i].0[3];
        // }

        SpriteRef::new(
            PixelSize::new(self.size.height, self.size.width),
            &self.pixels, // TODO: load the slice containing our sprite
        )
    }

    /// Draw provided Tiles kind (e.g. `Tiles::Grass`) to the given position.
    pub fn draw(&self, screen: &mut [u8], dest: &PixelPoint, tile: &Tile, color: RGBA8) {
        blit(screen, dest, &self.sprite(tile.value()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Drawable;

    #[test]
    fn sprite_references() {
        let buf = &[
            // first
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            // second
            1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
            // second
            2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8,
            // fourth
            3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8,
        ];
        let atlas = SpriteAtlas::new(Vec::<u8>::from(*buf), SpriteSize::new(2, 2));

        assert_eq!(
            atlas.sprite(SpritePoint::new(0, 0)).pixels(),
            &[0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,]
        );
        assert_eq!(
            atlas.sprite(SpritePoint::new(1, 0)).pixels(),
            &[1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,]
        );
        assert_eq!(
            atlas.sprite(SpritePoint::new(2, 0)).pixels(),
            &[2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8,]
        );
        assert_eq!(
            atlas.sprite(SpritePoint::new(3, 0)).pixels(),
            &[3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8,]
        );
    }
}
