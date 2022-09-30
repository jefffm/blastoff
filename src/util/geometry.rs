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
use ggez::mint::Point2;
use serde::{Deserialize, Serialize};

/// SpriteSpace is coordinates to refer to different sprites in a spritesheet of equal tile sizes
/// eg. 0, 0 is the top left sprite
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSpace;

pub type SpritePoint = Point2D<i32, SpriteSpace>;
pub type SpriteSize = Size2D<i32, SpriteSpace>;
pub type SpriteRect = Rect<i32, SpriteSpace>;

pub type SpriteToPixel = Transform2D<i32, SpriteSpace, PixelSpace>;

/// Pixel space is a subdivision of screen space
#[derive(Debug, Serialize, Deserialize)]
pub struct PixelSpace;

pub type PixelPoint = Point2D<i32, PixelSpace>;
pub type PixelSize = Size2D<i32, PixelSpace>;
pub type PixelRect = Rect<i32, PixelSpace>;

pub type PixelToScreen = Transform2D<i32, PixelSpace, ScreenSpace>;
pub type ScreenToPixel = Transform2D<i32, ScreenSpace, PixelSpace>;
pub type ScreenFloatToPixel = Transform2D<f32, ScreenSpace, PixelSpace>;

/// Screen space translates Viewport space into the right position for the GUI
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenSpace;

pub type ScreenPoint = Point2D<i32, ScreenSpace>;
pub type ScreenFloatPoint = Point2D<f32, ScreenSpace>;
pub type ScreenSize = Size2D<i32, ScreenSpace>;
pub type ScreenRect = Rect<i32, ScreenSpace>;

pub type ViewportToScreen = Transform2D<i32, ViewportSpace, ScreenSpace>;
pub type ViewportFloatToScreen = Transform2D<f32, ViewportSpace, ScreenSpace>;
/// Viewport space translates the current visible chunk of map for rendering
#[derive(Debug, Serialize, Deserialize)]
pub struct ViewportSpace;

pub type ViewportPoint = Point2D<i32, ViewportSpace>;
pub type ViewportFloatPoint = Point2D<f32, ViewportSpace>;
pub type ViewportSize = Size2D<i32, ViewportSpace>;
pub type ViewportFloatSize = Size2D<f32, ViewportSpace>;
pub type ViewportRect = Rect<i32, ViewportSpace>;
pub type ViewportFloatRect = Rect<f32, ViewportSpace>;

/// World space is relative to game world map coordinates
/// Our world is integer-based because we have a map tile system
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldSpace;

pub type WorldPoint = Point2D<i32, WorldSpace>;
pub type WorldVector = Vector2D<i32, WorldSpace>;
pub type WorldSize = Size2D<i32, WorldSpace>;
pub type WorldRect = Rect<i32, WorldSpace>;

pub type WorldFloatPoint = Point2D<f32, WorldSpace>;
pub type WorldFloatVector = Vector2D<f32, WorldSpace>;
pub type WorldFloatSize = Size2D<f32, WorldSpace>;
pub type WorldFloatRect = Rect<f32, WorldSpace>;

pub type WorldToViewport = Transform2D<i32, WorldSpace, ViewportSpace>;
pub type WorldFloatToViewport = Transform2D<f32, WorldSpace, ViewportSpace>;

/// Overworld space maps to independent map and ecs world instances
#[derive(Debug, Serialize, Deserialize)]
pub struct OverworldSpace;

pub type OverworldPoint = Point2D<i32, OverworldSpace>;
pub type OverworldVector = Vector2D<i32, OverworldSpace>;
pub type OverworldSize = Size2D<i32, OverworldSpace>;
pub type OverworldRect = Rect<i32, OverworldSpace>;

pub type OverworldFloatPoint = Point2D<f32, OverworldSpace>;
pub type OverworldFloatVector = Vector2D<f32, OverworldSpace>;
pub type OverworldFloatSize = Size2D<f32, OverworldSpace>;
pub type OverworldFloatRect = Rect<f32, OverworldSpace>;

/// Overworldwmaps to viewport space in order to render the overworld view
pub type OverworldToViewport = Transform2D<i32, OverworldSpace, ViewportSpace>;
pub type OverworldFloatToViewport = Transform2D<f32, OverworldSpace, ViewportSpace>;

/// Overworld space maps to independent map and ecs world instances
#[derive(Debug, Serialize, Deserialize)]
pub struct GalaxySpace;

pub type GalaxyPoint = Point2D<i32, GalaxySpace>;
pub type GalaxyVector = Vector2D<i32, GalaxySpace>;
pub type GalaxySize = Size2D<i32, GalaxySpace>;
pub type GalaxyRect = Rect<i32, GalaxySpace>;

pub type GalaxyFloatPoint = Point2D<f32, GalaxySpace>;
pub type GalaxyFloatVector = Vector2D<f32, GalaxySpace>;
pub type GalaxyFloatSize = Size2D<f32, GalaxySpace>;
pub type GalaxyFloatRect = Rect<f32, GalaxySpace>;

/// Overworldwmaps to viewport space in order to render the overworld view
pub type GalaxyToViewport = Transform2D<i32, OverworldSpace, GalaxySpace>;
pub type GalaxyFloatToViewport = Transform2D<f32, OverworldSpace, GalaxySpace>;

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
    fn transform_float_point(&self, src_point: Point2D<f32, Src>) -> Point2D<T, Dest>;
    fn as_float_transform(&self) -> Transform2D<f32, Src, Dest>;
}

impl<Src, Dest> TransformExt<i32, Src, Dest> for Transform2D<i32, Src, Dest> {
    fn from_points(src_point: Point2D<i32, Src>, dest_point: Point2D<i32, Dest>) -> Self {
        let translation = Self::create_translation(src_point, dest_point);
        Self::translation(translation.x, translation.y)
    }

    fn update_transform(&mut self, src_point: Point2D<i32, Src>, dest_point: Point2D<i32, Dest>) {
        let translation = Self::create_translation(src_point, dest_point);
        self.m31 = translation.x;
        self.m32 = translation.y;
    }

    fn transform_float_point(&self, src_point: Point2D<f32, Src>) -> Point2D<i32, Dest> {
        self.transform_point(Point2D::<i32, Src>::new(
            src_point.x.round() as i32,
            src_point.y.round() as i32,
        ))
    }
    fn as_float_transform(&self) -> Transform2D<f32, Src, Dest> {
        let params = self.to_array();
        Transform2D::<f32, Src, Dest>::new(
            params[0] as f32,
            params[1] as f32,
            params[2] as f32,
            params[3] as f32,
            params[4] as f32,
            params[5] as f32,
        )
    }
}

impl<Src, Dest> TransformExt<f32, Src, Dest> for Transform2D<f32, Src, Dest> {
    fn from_points(src_point: Point2D<f32, Src>, dest_point: Point2D<f32, Dest>) -> Self {
        let translation = Self::create_translation(src_point, dest_point);
        Self::translation(translation.x, translation.y)
    }

    fn update_transform(&mut self, src_point: Point2D<f32, Src>, dest_point: Point2D<f32, Dest>) {
        let translation = Self::create_translation(src_point, dest_point);
        self.m31 = translation.x;
        self.m32 = translation.y;
    }

    fn transform_float_point(&self, src_point: Point2D<f32, Src>) -> Point2D<f32, Dest> {
        self.transform_point(src_point)
    }

    fn as_float_transform(&self) -> Transform2D<f32, Src, Dest> {
        *self
    }
}

pub trait PointExt<T, U> {
    /// Helper to get the Vec index for any given WorldPoint (assuming the
    /// vector is height * width for this instance of Map).
    fn to_index(&self, width: T) -> usize;
    fn from_index(idx: usize, width: T) -> Point2D<T, U>;
    fn get_vector(self, other: Self) -> Vector2D<T, U>;
    fn into_mint_f32(&self) -> Point2<f32>;
}

impl<U> PointExt<i32, U> for Point2D<i32, U> {
    fn to_index(&self, width: i32) -> usize {
        let x: usize = self.x.try_into().expect("unwrap x");
        let y: usize = self.y.try_into().expect("unwrap y");
        let w: usize = width.try_into().expect("unwrap width");
        (y * w) + x
    }

    fn from_index(idx: usize, width: i32) -> Point2D<i32, U> {
        let idx: i32 = idx.try_into().unwrap();
        Self::new(idx % width, idx / width)
    }

    fn get_vector(self, other: Self) -> Vector2D<i32, U> {
        other - self
    }

    fn into_mint_f32(&self) -> Point2<f32> {
        Point2::<f32> {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl<U> PointExt<f32, U> for Point2D<f32, U> {
    fn to_index(&self, width: f32) -> usize {
        let x = self.x.round() as i32;
        let y = self.y.round() as i32;
        let w = width.round() as i32;
        (y as usize * w as usize) + x as usize
    }

    fn from_index(idx: usize, width: f32) -> Point2D<f32, U> {
        let idx: i32 = idx.try_into().unwrap();
        Self::new(idx as f32 % width, idx as f32 / width)
    }

    fn get_vector(self, other: Self) -> Vector2D<f32, U> {
        other - self
    }

    fn into_mint_f32(&self) -> Point2<f32> {
        Point2::<f32> {
            x: self.x,
            y: self.y,
        }
    }
}

pub trait VectorExt<T, U> {
    fn as_float(&self) -> Vector2D<f32, U>;
}

impl<U> VectorExt<i32, U> for Vector2D<i32, U> {
    fn as_float(&self) -> Vector2D<f32, U> {
        Vector2D::new(self.x as f32, self.y as f32)
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
    fn transform_translation_with_scale() {
        let pixel = PixelRect::new(PixelPoint::new(0, 0), PixelSize::new(8, 8));
        let screen = ScreenRect::new(ScreenPoint::new(0, 0), ScreenSize::new(8, 8));

        // coordinates mapped 1:1, but 8x8 pixels == 1x1 screen tiles
        let s2p = ScreenToPixel::from_points(screen.origin, pixel.origin).then_scale(8, 8);

        assert_eq!(
            s2p.transform_point(ScreenPoint::new(1, 0)),
            PixelPoint::new(8, 0)
        );
        assert_eq!(
            s2p.transform_point(ScreenPoint::new(2, 0)),
            PixelPoint::new(16, 0)
        );
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
                    .map(move |y| WorldPoint::new(x, y).to_index(width))
            })
            .map(|idx| map[idx] = 1)
            .collect::<Vec<_>>();
    }

    #[test]
    fn points_to_vector() {
        let start = WorldPoint::new(0, 0);
        let end = WorldPoint::new(1, 1);

        assert_eq!(start.get_vector(end), WorldVector::new(1, 1));
    }

    #[test]
    fn point_to_idx_testing() {
        let width = 2;
        let height = 2;
        let sprite_rect = SpriteRect::new(SpritePoint::new(0, 0), SpriteSize::new(width, height));

        let buf = vec![0; sprite_rect.area() as usize];
        assert_eq!(buf.len(), 4);

        assert_eq!(sprite_rect.min(), SpritePoint::new(0, 0));
        assert_eq!(sprite_rect.max(), SpritePoint::new(width, height));

        assert_eq!(sprite_rect.min().to_index(width), 0);
    }
}
