use crate::util::{ScreenPoint, WorldPoint, WorldToScreen};

pub struct Viewport {
    w2s: WorldToScreen,
}

impl Viewport {
    pub fn new(w2s: WorldToScreen) -> Self {
        Self { w2s }
    }

    pub fn from_points(screen_point: ScreenPoint, world_point: WorldPoint) -> WorldToScreen {
        let translation = screen_point.to_untyped() - world_point.to_untyped();
        WorldToScreen::new(1, 0, 0, 1, translation.x, translation.y)
    }
}
