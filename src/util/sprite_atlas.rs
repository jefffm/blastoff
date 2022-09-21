use std::rc::Rc;

use image::{EncodableLayout, GenericImageView};
use rgb::RGBA8;

use crate::map::Tile;

use super::{blit, Drawable, PixelPoint, PixelSize};

pub struct SpriteAtlas {
    img: Rc<image::RgbaImage>,
}

impl SpriteAtlas {
    /// Load an already-decoded image in memory into a SpriteAtlas
    pub fn from_raw(size: PixelSize, bytes: Vec<u8>) -> Self {
        let img = image::RgbaImage::from_raw(size.width as u32, size.height as u32, bytes).unwrap();
        Self { img: Rc::new(img) }
    }

    /// Load an in-memory file as an image into a SpriteAtlas
    pub fn from_memory(bytes: &[u8]) -> Self {
        let img = image::load_from_memory(bytes).unwrap();
        Self {
            img: Rc::new(img.into_rgba8()),
        }
    }

    pub fn create_view(&self, origin: PixelPoint, size: PixelSize) -> SpriteView {
        SpriteView::new(self.img.clone(), origin, size)
    }
}

pub struct SpriteView {
    origin: PixelPoint,
    size: PixelSize,
    _img: Rc<image::RgbaImage>,
}

impl SpriteView {
    pub fn new(img: Rc<image::RgbaImage>, origin: PixelPoint, size: PixelSize) -> Self {
        Self {
            origin,
            size,
            _img: img,
        }
    }
    pub fn bytes(&self) -> image::Pixels<_> {
        // TODO: use pixels and get_pixelfrom image
        self._img
            .view(
                self.origin.x as u32,
                self.origin.y as u32,
                self.size.width as u32,
                self.size.height as u32,
            )
            .pixels()
    }

    /// Draw provided Tiles kind (e.g. `Tiles::Grass`) to the given position.
    pub fn draw(&mut self, screen: &mut [u8], dest: &PixelPoint, tile: &Tile, _color: RGBA8) {
        blit(screen, dest, self)
    }
}

impl Drawable for SpriteView {
    fn width(&self) -> usize {
        self.size.width as usize
    }

    fn height(&self) -> usize {
        self.size.height as usize
    }

    fn pixels(&self) -> &[u8] {
        self.bytes().as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, Cursor};

    use super::*;

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
        let atlas = SpriteAtlas::from_raw(PixelSize::new(4, 4), Vec::<u8>::from(*buf));

        assert_eq!(
            atlas
                .create_view(PixelPoint::new(0, 0), PixelSize::new(2, 2))
                .bytes(),
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // 2x2 upper left corner
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
            ],
            "0, 0"
        );
        assert_eq!(
            atlas
                .create_view(PixelPoint::new(1, 0), PixelSize::new(2, 2))
                .bytes(),
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // 2x2 upper right corner
                1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
            ],
            "1, 0"
        );
        assert_eq!(
            atlas
                .create_view(PixelPoint::new(0, 2), PixelSize::new(2, 2))
                .bytes(),
            &[
                2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, // 2x2 bottom left
                3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8,
            ],
            "0, 2"
        );
        assert_eq!(
            atlas
                .create_view(PixelPoint::new(2, 2), PixelSize::new(2, 2))
                .bytes(),
            &[
                2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, // 2x2 bottom right
                3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8,
            ],
            "2, 2"
        );
    }

    #[test]
    fn sprite_from_png() {
        let mut cursor = Cursor::new(include_bytes!("../../assets/tileset/test.png"));
        let buf = cursor.fill_buf().unwrap();
        let atlas = SpriteAtlas::from_memory(buf);

        // top left
        assert_eq!(
            atlas
                .create_view(PixelPoint::new(0, 0), PixelSize::new(1, 1))
                .bytes(),
            &[255u8, 255u8, 255u8, 255u8]
        );
        // top right
        assert_eq!(
            atlas
                .create_view(PixelPoint::new(1, 0), PixelSize::new(1, 1))
                .bytes(),
            &[172u8, 50u8, 50u8, 255u8]
        );
        // bottom left
        assert_eq!(
            atlas
                .create_view(PixelPoint::new(0, 1), PixelSize::new(1, 1))
                .bytes(),
            &[63u8, 63u8, 116u8, 255u8]
        );
        // bottom right
        assert_eq!(
            atlas
                .create_view(PixelPoint::new(1, 1), PixelSize::new(1, 1))
                .bytes(),
            &[106u8, 190u8, 48u8, 255u8]
        );
    }
}
