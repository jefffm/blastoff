use euclid::{Point2D, Rect, Transform2D};

use crate::util::{
    TransformExt, ViewportFloatPoint, ViewportFloatRect, ViewportPoint, ViewportRect, ViewportSpace,
};

// Viewport tracks the current onscreen rect
#[derive(Debug, Default)]
pub struct Viewport<U> {
    rect_float: ViewportFloatRect,
    transform: Transform2D<i32, U, ViewportSpace>,
    transform_float: Transform2D<f32, U, ViewportSpace>,
}

impl<U> Viewport<U> {
    pub fn new(rect: ViewportRect, transform: Transform2D<i32, U, ViewportSpace>) -> Self {
        Self {
            rect_float: rect.to_f32(),
            transform,
            transform_float: transform.as_float_transform(),
        }
    }

    /// The current visible world points
    pub fn game_rect(&self) -> Rect<f32, U> {
        self.transform_float
            .inverse()
            .unwrap()
            .outer_transformed_rect(&self.rect_float)
    }

    pub fn to_viewport_point(&self, point: Point2D<i32, U>) -> ViewportPoint {
        self.transform.transform_point(point)
    }

    pub fn to_viewport_point_f32(&self, point: Point2D<f32, U>) -> ViewportFloatPoint {
        self.transform_float.transform_point(point)
    }

    pub fn update_transform(&mut self, center: Point2D<f32, U>) {
        // TODO: this could also clamp to Map boundaries
        self.transform
            .update_transform(center.to_i32(), self.rect_float.center().to_i32());
        self.transform_float
            .update_transform(center, self.rect_float.center());
    }

    pub fn vp_center(&self) -> ViewportFloatPoint {
        self.rect_float.center()
    }

    pub fn center(&self) -> Point2D<f32, U> {
        self.to_game_point_float(self.rect_float.center())
    }

    pub fn to_game_point(&self, point: ViewportPoint) -> Point2D<i32, U> {
        let inverse_transform = self.transform.inverse().expect("inverse transform");
        inverse_transform.transform_point(point)
    }

    pub fn to_game_point_float(&self, point: ViewportFloatPoint) -> Point2D<f32, U> {
        let inverse_transform = self.transform_float.inverse().expect("inverse transform");
        inverse_transform.transform_point(point)
    }

    pub fn points(&self) -> impl Iterator<Item = ViewportPoint> {
        let yrange = self.rect_float.to_i32().y_range();
        let xrange = self.rect_float.to_i32().x_range();

        yrange.flat_map(move |y| xrange.clone().map(move |x| ViewportPoint::new(x, y)))
    }

    pub fn visible_points(&self) -> impl Iterator<Item = Point2D<i32, U>> + '_ {
        self.points().map(|point| self.to_game_point(point))
    }
}
