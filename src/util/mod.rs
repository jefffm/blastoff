mod geometry;
pub use geometry::*;

mod tile_atlas;
pub use tile_atlas::*;

mod sprite;
pub use sprite::*;

mod dither;
pub use dither::*;

/// Clear the screen
pub fn clear(screen: &mut [u8]) {
    for (i, byte) in screen.iter_mut().enumerate() {
        *byte = if i % 4 == 3 { 255 } else { 0 };
    }
}
