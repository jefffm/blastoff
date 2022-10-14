use std::time::Duration;

use crate::util::{
    PixelPoint, PixelRect, PixelSize, ScreenFloatToPixel, ScreenPoint, ScreenRect, ScreenSize,
    ScreenToPixel, TransformExt, WorldSize,
};

// 16:9
pub const SCREEN_WIDTH_PIXELS: i32 = 320;
pub const SCREEN_HEIGHT_PIXELS: i32 = 180;
pub const SCREEN_ASPECT_RATIO: f32 = SCREEN_WIDTH_PIXELS as f32 / SCREEN_HEIGHT_PIXELS as f32;
pub const PIXEL_RECT: PixelRect = PixelRect::new(
    PixelPoint::new(0, 0),
    PixelSize::new(SCREEN_WIDTH_PIXELS, SCREEN_HEIGHT_PIXELS),
);

pub const TILE_SIZE: PixelSize = PixelSize::new(16, 16);

/// euclid Rect isn't const, so this is manually calculated
pub const SCREEN_HEIGHT: i32 = 20; // 320 / 16 = 20
pub const SCREEN_WIDTH: i32 = 11; // 180 / 16 = 11.25

pub const SCREEN_SIZE: ScreenSize = ScreenSize::new(SCREEN_WIDTH, SCREEN_HEIGHT);
pub const SCREEN_RECT: ScreenRect = ScreenRect::new(ScreenPoint::new(0, 0), SCREEN_SIZE);

pub fn get_screen_to_pixel_transform() -> ScreenToPixel {
    ScreenToPixel::from_points(SCREEN_RECT.origin, PIXEL_RECT.origin)
        .then_scale(TILE_SIZE.width, TILE_SIZE.height)
}

pub fn get_screen_to_pixel_transform_float() -> ScreenFloatToPixel {
    get_screen_to_pixel_transform().as_float_transform()
}

// TODO: derive the viewport height instead
pub const VIEWPORT_WIDTH: i32 = SCREEN_WIDTH - 2;
pub const VIEWPORT_HEIGHT: i32 = SCREEN_HEIGHT - 2;

// Where on the screen is the viewport's origin?
pub const VIEWPORT_SCREEN_POINT: ScreenPoint = ScreenPoint::new(2, 2);
pub const TITLE_HEADER: &str = "BLAST!OFF!";
pub const UPDATE_FPS: u32 = 60;
pub const UPDATE_INTERVAL_SECS: f32 = 1.0 / (UPDATE_FPS as f32);

pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / UPDATE_FPS as u64);
pub const ONE_FRAME: Duration = Duration::from_nanos(1_000_000_000 / 60);

pub const TOP_BOX_HEIGHT: i32 = SCREEN_HEIGHT - VIEWPORT_HEIGHT;
pub const SIDE_BOX_WIDTH: i32 = SCREEN_WIDTH - VIEWPORT_WIDTH;

pub const RESOURCE_PATH: &str = "assets";

pub const USE_SPRITES: bool = true;

pub const MOVEMENT_ANIMATION_DURATION: f32 = 1. / 5.;

pub const SECTOR_WIDTH: i32 = 100;
pub const SECTOR_HEIGHT: i32 = 100;
pub const SECTOR_SIZE: WorldSize = WorldSize::new(SECTOR_WIDTH, SECTOR_HEIGHT);

pub const MAX_PLANET_SPRITE_SIZE: f32 = 64.;
