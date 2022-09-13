use crate::util::{TransformExt, ViewportPoint, ViewportRect, WorldPoint, WorldToViewport};

// Viewport tracks the current onscreen rect
#[derive(Debug, Default)]
pub struct Viewport {
    rect: ViewportRect,
    transform: WorldToViewport,
}

impl Viewport {
    pub fn new(rect: ViewportRect, transform: WorldToViewport) -> Self {
        Self { rect, transform }
    }

    pub fn to_viewport_point(&self, point: WorldPoint) -> ViewportPoint {
        self.transform.transform_point(point)
    }

    pub fn update_transform(&mut self, center: WorldPoint) {
        self.transform.update_transform(center, self.rect.center())
    }

    pub fn to_world_point(&self, point: ViewportPoint) -> WorldPoint {
        let inverse_transform = self.transform.inverse().expect("inverse transform");
        inverse_transform.transform_point(point)
    }

    pub fn points(&self) -> impl Iterator<Item = ViewportPoint> {
        let xrange = self.rect.x_range();
        let yrange = self.rect.y_range();

        xrange.flat_map(move |x| yrange.clone().map(move |y| ViewportPoint::new(x, y)))
    }
}
