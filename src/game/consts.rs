use std::time::Duration;

use crate::util::{
    PixelPoint, PixelRect, PixelSize, ScreenPoint, ScreenRect, ScreenSize, ScreenToPixel,
    TransformExt,
};

/// euclid Rect isn't const, so this is manually calculated
pub const SCREEN_HEIGHT: i32 = SCREEN_HEIGHT_PIXELS / TILE_SIZE.height;
pub const SCREEN_WIDTH: i32 = SCREEN_WIDTH_PIXELS / TILE_SIZE.width;

pub const SCREEN_SIZE: ScreenSize = ScreenSize::new(SCREEN_WIDTH, SCREEN_HEIGHT);
pub const SCREEN_RECT: ScreenRect = ScreenRect::new(ScreenPoint::new(0, 0), SCREEN_SIZE);

pub const TILE_SIZE: PixelSize = PixelSize::new(16, 16);
// Pixel coordinates can be derived from tile coordinates
pub const PIXEL_RECT: PixelRect = PixelRect::new(
    PixelPoint::new(0, 0),
    PixelSize::new(
        SCREEN_SIZE.width * TILE_SIZE.width,
        SCREEN_SIZE.height * TILE_SIZE.height,
    ),
);

pub fn get_screen_to_pixel_transform() -> ScreenToPixel {
    ScreenToPixel::from_points(SCREEN_RECT.origin, PIXEL_RECT.origin)
        .then_scale(TILE_SIZE.width, TILE_SIZE.height)
}

// TODO: derive the viewport height instead
pub const VIEWPORT_WIDTH: i32 = 50;
pub const VIEWPORT_HEIGHT: i32 = 50;

// Where on the screen is the viewport's origin?
pub const VIEWPORT_SCREEN_POINT: ScreenPoint = ScreenPoint::new(0, SCREEN_HEIGHT - VIEWPORT_HEIGHT);
pub const TITLE_HEADER: &str = "Roguemon";
pub const UPDATE_FPS: u32 = 60;
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / UPDATE_FPS as u64);
pub const ONE_FRAME: Duration = Duration::from_nanos(1_000_000_000 / 60);

// 16:9
pub const SCREEN_WIDTH_PIXELS: i32 = 320 * 3;
pub const SCREEN_HEIGHT_PIXELS: i32 = 180 * 3;

pub const TOP_BOX_HEIGHT: i32 = SCREEN_HEIGHT - VIEWPORT_HEIGHT;
pub const SIDE_BOX_WIDTH: i32 = SCREEN_WIDTH - VIEWPORT_WIDTH;

pub const RESOURCE_PATH: &str = "assets";

pub const SCALING_FACTOR: f32 = 2.;

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn geometry() {
        let screen_point = SCREEN_RECT.center();
        let pixel_point = get_screen_to_pixel_transform().transform_point(screen_point);

        assert_eq!(pixel_point, PixelPoint::new(480, 256));
    }
}
