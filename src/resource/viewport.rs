use crate::util::{TransformExt, ViewportPoint, ViewportRect, WorldPoint, WorldToViewport};

// Viewport tracks the current onscreen rect
#[derive(Debug, Default)]
pub struct Viewport {
    viewport: ViewportRect,
    transform: WorldToViewport,
}

impl Viewport {
    pub fn new(viewport: ViewportRect, transform: WorldToViewport) -> Self {
        Self {
            viewport,
            transform,
        }
    }

    pub fn to_viewport_point(&self, point: WorldPoint) -> ViewportPoint {
        self.transform.transform_point(point)
    }

    pub fn update_transform(&mut self, center: WorldPoint) {
        self.transform
            .update_transform(center, self.viewport.center())
    }
}
