use crate::util::{
    TransformExt, ViewportFloatPoint, ViewportFloatRect, ViewportPoint, ViewportRect,
    WorldFloatPoint, WorldFloatRect, WorldFloatToViewport, WorldPoint, WorldToViewport,
};

// Viewport tracks the current onscreen rect
#[derive(Debug, Default)]
pub struct Viewport {
    rect: ViewportRect,
    rect_float: ViewportFloatRect,
    transform: WorldToViewport,
    transform_float: WorldFloatToViewport,
}

impl Viewport {
    pub fn new(rect: ViewportRect, transform: WorldToViewport) -> Self {
        Self {
            rect,
            rect_float: rect.to_f32(),
            transform,
            transform_float: transform.into_float_transform(),
        }
    }

    /// The current visible world points
    pub fn world_rect(&self) -> WorldFloatRect {
        self.transform_float
            .inverse()
            .unwrap()
            .outer_transformed_rect(&self.rect_float)
    }

    pub fn to_viewport_point(&self, point: WorldPoint) -> ViewportPoint {
        self.transform.transform_point(point)
    }

    pub fn to_viewport_point_f32(&self, point: WorldFloatPoint) -> ViewportFloatPoint {
        self.transform_float.transform_point(point)
    }

    pub fn update_transform(&mut self, center: WorldFloatPoint) {
        self.transform
            .update_transform(center.to_i32(), self.rect.center());
        self.transform_float
            .update_transform(center, self.rect_float.center());
    }

    pub fn center(&self) -> WorldFloatPoint {
        self.to_world_point_float(self.rect_float.center())
    }

    pub fn to_world_point(&self, point: ViewportPoint) -> WorldPoint {
        let inverse_transform = self.transform.inverse().expect("inverse transform");
        inverse_transform.transform_point(point)
    }

    pub fn to_world_point_float(&self, point: ViewportFloatPoint) -> WorldFloatPoint {
        let inverse_transform = self.transform_float.inverse().expect("inverse transform");
        inverse_transform.transform_point(point)
    }

    pub fn points(&self) -> impl Iterator<Item = ViewportPoint> {
        let xrange = self.rect.x_range();
        let yrange = self.rect.y_range();

        xrange.flat_map(move |x| yrange.clone().map(move |y| ViewportPoint::new(x, y)))
    }
}
