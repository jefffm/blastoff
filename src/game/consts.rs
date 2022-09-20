use std::time::Duration;

use crate::util::{
    PixelPoint, PixelRect, PixelSize, PixelToScreen, ScreenPoint, ScreenRect, ScreenSize,
    ScreenToPixel,
};

pub const SCREEN_HEIGHT: i32 = 56;
pub const SCREEN_WIDTH: i32 = 100;
pub const SCREEN_SIZE: ScreenSize = ScreenSize::new(SCREEN_WIDTH, SCREEN_HEIGHT);
pub const SCREEN_RECT: ScreenRect = ScreenRect::new(ScreenPoint::new(0, 0), SCREEN_SIZE);
pub const TILE_SIZE: i32 = 16;
// Pixel coordinates can be derived from tile coordinates
pub const PIXEL_RECT: PixelRect = PixelRect::new(
    PixelPoint::new(0, 0),
    PixelSize::new(
        SCREEN_SIZE.width * TILE_SIZE,
        SCREEN_SIZE.height * TILE_SIZE,
    ),
);

pub const VIEWPORT_WIDTH: i32 = 90;
pub const VIEWPORT_HEIGHT: i32 = 50;

// Where on the screen is the viewport's origin?
pub const VIEWPORT_SCREEN_POINT: ScreenPoint = ScreenPoint::new(0, SCREEN_HEIGHT - VIEWPORT_HEIGHT);
pub const TITLE_HEADER: &str = "Roguemon";
pub const FPS: u32 = 60;
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);
pub const ONE_FRAME: Duration = Duration::from_nanos(1_000_000_000 / 60);

// TODO: none of these can be const
pub const SCREEN_WIDTH_PIXELS: i32 = SCREEN_WIDTH * TILE_SIZE;
pub const SCREEN_HEIGHT_PIXELS: i32 = SCREEN_HEIGHT * TILE_SIZE;
pub const TOP_BOX_HEIGHT: i32 = SCREEN_HEIGHT - VIEWPORT_HEIGHT;
pub const SIDE_BOX_WIDTH: i32 = SCREEN_WIDTH - VIEWPORT_WIDTH;
