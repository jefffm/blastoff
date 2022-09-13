//! This module defines three coordinate systems: ScreenSpace -> ViewportSpace
//! -> WorldSpace.
//!
//! - ScreenSpace is the literal tiles rendered to the screen
//! - ViewportSpace is the viewport rendering the game world. The viewport is
//! not 100% of the screen, and so these coordinates are mapped back into
//! ScreenSpace.
//! - WorldSpace is the game world's coordinates. The center of the map is the center of a WorldRect.

use std::{convert::TryInto, fmt::Debug};

use euclid::{Point2D, Rect, Size2D, Transform2D, UnknownUnit, Vector2D};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenSpace;

pub type ScreenPoint = Point2D<i32, ScreenSpace>;
pub type ScreenSize = Size2D<i32, ScreenSpace>;
pub type ScreenRect = Rect<i32, ScreenSpace>;

pub type ViewportToScreen = Transform2D<i32, ViewportSpace, ScreenSpace>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewportSpace;

pub type ViewportPoint = Point2D<i32, ViewportSpace>;
pub type ViewportSize = Size2D<i32, ViewportSpace>;
pub type ViewportRect = Rect<i32, ViewportSpace>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldSpace;

pub type WorldPoint = Point2D<i32, WorldSpace>;
pub type WorldVector = Vector2D<i32, WorldSpace>;
pub type WorldSize = Size2D<i32, WorldSpace>;
pub type WorldRect = Rect<i32, WorldSpace>;

pub type WorldToViewport = Transform2D<i32, WorldSpace, ViewportSpace>;

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

impl<Src, Dest> TransformExt<i32, Src, Dest> for Transform2D<i32, Src, Dest> {
    fn from_points(src_point: Point2D<i32, Src>, dest_point: Point2D<i32, Dest>) -> Self {
        let translation = Self::create_translation(src_point, dest_point);
        Self::new(1, 0, 0, 1, translation.x, translation.y)
    }

    fn update_transform(&mut self, src_point: Point2D<i32, Src>, dest_point: Point2D<i32, Dest>) {
        let translation = Self::create_translation(src_point, dest_point);
        self.m31 = translation.x;
        self.m32 = translation.y;
    }
}

pub trait PointExt<T, U>
where
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    /// Helper to get the Vec index for any given WorldPoint (assuming the
    /// vector is height * width for this instance of Map).
    fn to_index(&self, width: T) -> usize;
}

impl<T, U> PointExt<T, U> for Point2D<T, U>
where
    T: TryInto<usize> + Copy,
    <T as TryInto<usize>>::Error: Debug,
{
    fn to_index(&self, width: T) -> usize {
        let x: usize = self.x.try_into().unwrap();
        let y: usize = self.y.try_into().unwrap();
        let w: usize = width.try_into().unwrap();
        (y * w) + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_translation_tests(viewport: ViewportRect, world: WorldRect) {
        // centered
        let mut w2s = WorldToViewport::from_points(world.center(), viewport.center());
        assert_eq!(
            w2s.transform_point(world.center()),
            viewport.center(),
            "Viewport should be centered over the map center ({:?})",
            world.center()
        );

        // viewport over 0, 0
        w2s.update_transform(world.min(), viewport.center());
        assert_eq!(
            w2s.transform_point(world.min()),
            viewport.center(),
            "Viewport should be centered over {:?}",
            world.min()
        );

        // viewport over bottom right corner
        w2s.update_transform(world.max(), viewport.center());
        assert_eq!(
            w2s.transform_point(world.max()),
            viewport.center(),
            "Screen should be centered over {:?}",
            world.max()
        );
    }

    fn create_viewport(width: i32, height: i32) -> ViewportRect {
        ViewportRect::new(ViewportPoint::new(0, 0), ViewportSize::new(width, height))
    }

    fn create_world(width: i32, height: i32) -> WorldRect {
        WorldRect::new(WorldPoint::new(0, 0), WorldSize::new(width, height))
    }

    #[test]
    fn translation_same_size() {
        run_translation_tests(create_viewport(10, 10), create_world(10, 10))
    }

    #[test]
    fn translation_bigmap() {
        run_translation_tests(create_viewport(10, 10), create_world(100, 100))
    }

    #[test]
    fn translation_smallmap() {
        run_translation_tests(create_viewport(50, 50), create_world(5, 5))
    }

    #[test]
    fn translation_longmap() {
        run_translation_tests(create_viewport(50, 50), create_world(100, 5))
    }

    #[test]
    fn translation_tallmap() {
        run_translation_tests(create_viewport(50, 50), create_world(6, 75))
    }

    #[test]
    fn index_test() {
        let width = 13;
        let height = 11;
        let world = WorldRect::new(WorldPoint::new(0, 0), WorldSize::new(width, height));

        let map = &mut vec![0; (width * height) as usize];

        // Create every point from the world rect and make sure we can set it in the vec
        let _result = world
            .x_range()
            .flat_map(move |x| {
                world
                    .y_range()
                    .clone()
                    .map(move |y| WorldPoint::new(x, y).to_index(width))
            })
            .map(|idx| map[idx] = 1)
            .collect::<Vec<_>>();
    }
}
