use std::time::Duration;

use crate::util::{
    PixelPoint, PixelRect, PixelSize, ScreenFloatToPixel, ScreenPoint, ScreenRect, ScreenSize,
    ScreenToPixel, TransformExt,
};

// 16:9
pub const SCREEN_WIDTH_PIXELS: i32 = 320 * 4;
pub const SCREEN_HEIGHT_PIXELS: i32 = 180 * 4;
pub const TILE_SIZE: PixelSize = PixelSize::new(16, 16);

/// euclid Rect isn't const, so this is manually calculated
pub const SCREEN_HEIGHT: i32 = SCREEN_HEIGHT_PIXELS / TILE_SIZE.height; // (320 * 3) / 16 = 60
pub const SCREEN_WIDTH: i32 = SCREEN_WIDTH_PIXELS / TILE_SIZE.width; // (180 * 3) / 16 = 33.75 = 33 + 0.75 extra

pub const SCREEN_SIZE: ScreenSize = ScreenSize::new(SCREEN_WIDTH, SCREEN_HEIGHT);
pub const SCREEN_RECT: ScreenRect = ScreenRect::new(ScreenPoint::new(0, 0), SCREEN_SIZE);

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

pub fn get_screen_to_pixel_transform_float() -> ScreenFloatToPixel {
    get_screen_to_pixel_transform().as_float_transform()
}

// TODO: derive the viewport height instead
pub const VIEWPORT_WIDTH: i32 = 60;
pub const VIEWPORT_HEIGHT: i32 = 50;

// Where on the screen is the viewport's origin?
pub const VIEWPORT_SCREEN_POINT: ScreenPoint = ScreenPoint::new(2, 2);
pub const TITLE_HEADER: &str = "Roguemon";
pub const UPDATE_FPS: u32 = 60;
pub const UPDATE_INTERVAL_SECS: f32 = 1.0 / (UPDATE_FPS as f32);

pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / UPDATE_FPS as u64);
pub const ONE_FRAME: Duration = Duration::from_nanos(1_000_000_000 / 60);

pub const TOP_BOX_HEIGHT: i32 = SCREEN_HEIGHT - VIEWPORT_HEIGHT;
pub const SIDE_BOX_WIDTH: i32 = SCREEN_WIDTH - VIEWPORT_WIDTH;

pub const RESOURCE_PATH: &str = "assets";

pub const SCALING_FACTOR: f32 = 1.;

pub const USE_SPRITES: bool = true;

pub const MOVEMENT_ANIMATION_DURATION: f32 = 1. / 5.;

pub const SECTOR_WIDTH: i32 = 100;
pub const SECTOR_HEIGHT: i32 = 100;

pub const MAX_PLANET_SPRITE_SIZE: f32 = 64.;

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
