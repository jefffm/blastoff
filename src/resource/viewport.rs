use crate::util::{
    PointExt, TransformExt, ViewportFloatPoint, ViewportPoint, ViewportRect, WorldFloatPoint,
    WorldFloatToViewport, WorldPoint, WorldToViewport,
};

// Viewport tracks the current onscreen rect
#[derive(Debug, Default)]
pub struct Viewport {
    rect: ViewportRect,
    transform: WorldToViewport,
    transform_float: WorldFloatToViewport,
}

impl Viewport {
    pub fn new(rect: ViewportRect, transform: WorldToViewport) -> Self {
        let params = transform.to_array();
        let transform_float = WorldFloatToViewport::new(
            params[0] as f32,
            params[1] as f32,
            params[2] as f32,
            params[3] as f32,
            params[4] as f32,
            params[5] as f32,
        );
        Self {
            rect,
            transform,
            transform_float,
        }
    }

    pub fn to_viewport_point(&self, point: WorldPoint) -> ViewportPoint {
        self.transform.transform_point(point)
    }

    pub fn to_viewport_point_f32(&self, point: WorldFloatPoint) -> ViewportFloatPoint {
        self.transform_float.transform_point(point)
    }

    pub fn update_transform(&mut self, center: WorldPoint) {
        self.transform.update_transform(center, self.rect.center());
        self.transform_float
            .update_transform(center.as_float(), self.rect.center().as_float());
    }

    pub fn center(&self) -> WorldPoint {
        self.to_world_point(self.rect.center())
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
