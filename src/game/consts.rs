use crate::util::ScreenPoint;

pub const TILE_SIZE: i32 = 16;
pub const SCREEN_WIDTH: i32 = 100;
pub const SCREEN_HEIGHT: i32 = 56;
pub const VIEWPORT_WIDTH: i32 = 90;
pub const VIEWPORT_HEIGHT: i32 = 50;
pub const TOP_BOX_HEIGHT: i32 = SCREEN_HEIGHT - VIEWPORT_HEIGHT;
pub const SIDE_BOX_WIDTH: i32 = SCREEN_WIDTH - VIEWPORT_WIDTH;
pub const VIEWPORT_SCREEN_POINT: ScreenPoint = ScreenPoint::new(0, SCREEN_HEIGHT - VIEWPORT_HEIGHT);
pub const TITLE_HEADER: &str = "Roguemon";
