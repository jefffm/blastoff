use line_drawing::Bresenham;
use std::cmp::min;

use crate::game::consts::{PIXEL_RECT, SCREEN_HEIGHT_PIXELS, SCREEN_WIDTH_PIXELS};

use super::{PixelPoint, PixelSize};

#[derive(Debug)]
pub struct SpriteRef<'a> {
    size: PixelSize,
    pixels: &'a [u8],
}

impl<'a> SpriteRef<'a> {
    pub fn new(size: PixelSize, pixels: &'a [u8]) -> Self {
        Self { size, pixels }
    }
}

/// Drawables can be blitted to the pixel buffer and animated.
pub trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

impl<'a> Drawable for SpriteRef<'a> {
    fn width(&self) -> usize {
        self.size.width as usize
    }

    fn height(&self) -> usize {
        self.size.height as usize
    }

    fn pixels(&self) -> &'a [u8] {
        &self.pixels
    }
}

/// Blit a drawable to the pixel buffer.
pub fn blit<S>(screen: &mut [u8], dest: &PixelPoint, sprite: &S)
where
    S: Drawable,
{
    assert!(PIXEL_RECT.contains(*dest));

    let pixels = sprite.pixels();
    let width = sprite.width() * 4;

    let mut s = 0;
    for y in 0..sprite.height() {
        let i = dest.x as usize * 4
            + dest.y as usize * SCREEN_WIDTH_PIXELS as usize * 4
            + y * SCREEN_WIDTH_PIXELS as usize * 4;

        // Merge pixels from sprite into screen
        let zipped = screen[i..i + width].iter_mut().zip(&pixels[s..s + width]);
        // TODO: implement dither transparency (instead of alpha blending). See https://github.com/PhalanxHead/dithering/blob/main/src/dither_tools/bayer_dithering.rs
        for (left, &right) in zipped {
            if right > 0 {
                *left = right;
            }
        }

        s += width;
    }
}

/// Draw a line to the pixel buffer using Bresenham's algorithm.
pub fn line(screen: &mut [u8], p1: &PixelPoint, p2: &PixelPoint, color: [u8; 4]) {
    let p1 = (p1.x as i64, p1.y as i64);
    let p2 = (p2.x as i64, p2.y as i64);

    for (x, y) in Bresenham::new(p1, p2) {
        // TODO: what's going on in here...?
        let x = min(x as usize, SCREEN_WIDTH_PIXELS as usize - 1);
        let y = min(y as usize, SCREEN_HEIGHT_PIXELS as usize - 1);
        let i = x * 4 + y * (SCREEN_WIDTH_PIXELS as usize) * 4;

        screen[i..i + 4].copy_from_slice(&color);
    }
}

/// Draw a rectangle to the pixel buffer using two points in opposite corners.
pub fn rect(screen: &mut [u8], p1: &PixelPoint, p2: &PixelPoint, color: [u8; 4]) {
    let p2 = PixelPoint::new(p2.x - 1, p2.y - 1);
    let p3 = PixelPoint::new(p1.x, p2.y);
    let p4 = PixelPoint::new(p2.x, p1.y);

    line(screen, p1, &p3, color);
    line(screen, &p3, &p2, color);
    line(screen, &p2, &p4, color);
    line(screen, &p4, p1, color);
}
