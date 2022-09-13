// use glamour::prelude::*;

// struct ViewSpace;
// impl Unit for ViewSpace {
//     type Scalar = f32;
// }

// struct WorldSpace;
// impl Unit for WorldSpace {
//     type Scalar = f32;
// }

// pub type ViewPoint = Point2<ViewSpace>;
// pub type ViewVector = Vector2<ViewSpace>;

// pub type WorldPoint = Point2<WorldSpace>;
// pub type WorldVector = Vector2<WorldSpace>;

// pub type ViewToWorld = Transform2<ViewSpace, WorldSpace>;
// pub type WorldToView = Transform2<WorldSpace, ViewSpace>;

use std::{convert::TryInto, fmt::Debug};

use euclid::{num::Round, Point2D, Rect, Size2D, Transform2D, UnknownUnit, Vector2D};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenSpace;

pub type ScreenPoint = Point2D<f32, ScreenSpace>;
pub type ScreenSize = Size2D<f32, ScreenSpace>;
pub type ScreenRect = Rect<f32, ScreenSpace>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldSpace;

pub type WorldPoint = Point2D<f32, WorldSpace>;
pub type WorldVector = Vector2D<f32, WorldSpace>;
pub type WorldSize = Size2D<f32, WorldSpace>;
pub type WorldRect = Rect<f32, WorldSpace>;

pub type WorldToScreen = Transform2D<f32, WorldSpace, ScreenSpace>;
pub type ScreenToWorld = Transform2D<f32, ScreenSpace, WorldSpace>;

pub trait TransformExt<T, Src, Dest>
where
    T: Copy + std::ops::Sub + std::ops::Sub<Output = T>,
{
    fn from_points(src_point: Point2D<T, Src>, dest_point: Point2D<T, Dest>) -> Self;
    fn create_translation(
        src_point: Point2D<T, Src>,
        dest_point: Point2D<T, Dest>,
    ) -> Vector2D<T, UnknownUnit> {
        dest_point.to_untyped() - src_point.to_untyped()
    }

    fn update_transform(&mut self, src_point: Point2D<T, Src>, dest_point: Point2D<T, Dest>);
}

impl<Src, Dest> TransformExt<f32, Src, Dest> for Transform2D<f32, Src, Dest> {
    fn from_points(src_point: Point2D<f32, Src>, dest_point: Point2D<f32, Dest>) -> Self {
        let translation = Self::create_translation(src_point, dest_point);
        Self::new(1.0, 0.0, 0.0, 1.0, translation.x, translation.y)
    }

    fn update_transform(&mut self, src_point: Point2D<f32, Src>, dest_point: Point2D<f32, Dest>) {
        let translation = Self::create_translation(src_point, dest_point);
        self.m31 = translation.x;
        self.m32 = translation.y;
    }
}

pub trait PointExt<U> {
    /// Helper to get the Vec index for any given WorldPoint (assuming the
    /// vector is height * width for this instance of Map).
    fn to_index(&self, width: f32) -> usize;
}

impl<U> PointExt<U> for Point2D<f32, U> {
    fn to_index(&self, width: f32) -> usize {
        let x: usize = self.x.round() as usize;
        let y: usize = self.y.round() as usize;
        let w: usize = width.round() as usize;
        (y * w) + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_translation_tests(screen: ScreenRect, world: WorldRect) {
        // centered
        let mut w2s = WorldToScreen::from_points(world.center(), screen.center());
        assert_eq!(
            w2s.transform_point(world.center()),
            screen.center(),
            "Screen should be centered over the map center ({:?})",
            world.center()
        );

        // viewport over 0, 0
        w2s.update_transform(world.min(), screen.center());
        assert_eq!(
            w2s.transform_point(world.min()),
            screen.center(),
            "Screen should be centered over {:?}",
            world.min()
        );

        // viewport over bottom right corner
        w2s.update_transform(world.max(), screen.center());
        assert_eq!(
            w2s.transform_point(world.max()),
            screen.center(),
            "Screen should be centered over {:?}",
            world.max()
        );
    }

    #[test]
    fn translation_same_size() {
        let screen = ScreenRect::new(ScreenPoint::new(0.0, 0.0), ScreenSize::new(10.0, 10.0));
        let world = WorldRect::new(WorldPoint::new(0.0, 0.0), WorldSize::new(10.0, 10.0));

        run_translation_tests(screen, world)
    }

    #[test]
    fn translation_bigmap() {
        let screen = ScreenRect::new(ScreenPoint::new(0.0, 0.0), ScreenSize::new(10.0, 10.0));
        let world = WorldRect::new(WorldPoint::new(0.0, 0.0), WorldSize::new(100.0, 100.0));

        run_translation_tests(screen, world)
    }

    #[test]
    fn translation_smallmap() {
        let screen = ScreenRect::new(ScreenPoint::new(0.0, 0.0), ScreenSize::new(50.0, 50.0));
        let world = WorldRect::new(WorldPoint::new(0.0, 0.0), WorldSize::new(5.0, 5.0));

        run_translation_tests(screen, world)
    }

    #[test]
    fn translation_longmap() {
        let screen = ScreenRect::new(ScreenPoint::new(0.0, 0.0), ScreenSize::new(50.0, 50.0));
        let world = WorldRect::new(WorldPoint::new(0.0, 0.0), WorldSize::new(100.0, 5.0));

        run_translation_tests(screen, world)
    }

    #[test]
    fn translation_tallmap() {
        let screen = ScreenRect::new(ScreenPoint::new(0.0, 0.0), ScreenSize::new(50.0, 50.0));
        let world = WorldRect::new(WorldPoint::new(0.0, 0.0), WorldSize::new(6.0, 75.0));

        run_translation_tests(screen, world)
    }

    #[test]
    fn index_test() {
        let width = 13.0;
        let height = 11.0;
        let world = WorldRect::new(WorldPoint::new(0.0, 0.0), WorldSize::new(width, height));

        let mut map = vec![0; (width * height) as usize];

        // TODO... no way to range over floats...?
        // for (x, y) in world.x_range().zip(world.y_range()) {
        //     let idx = WorldPoint::new(x, y).to_index(width);
        //     map[idx] = 1;
        // }
    }
}
