mod animation;
use animation::*;

use ggez::mint::Point2;
use keyframe::{
    ease,
    functions::{EaseInOut, Linear},
    keyframes, AnimationSequence,
};

use crate::{
    game::consts::get_screen_to_pixel_transform,
    util::{
        easing_function, EasingEnum, PixelPoint, PointExt, WorldFloatPoint, WorldPoint,
        WorldToViewport,
    },
};

use crate::{camera::Glyph, util::Sprite};

#[derive(Clone)]
pub struct Renderable {
    pub glyph: Glyph,
    pub sprite: Sprite,
    pub render_order: u32,
    pub sequence: Option<AnimationSequence<Point2<f32>>>,
}

impl Renderable {
    pub fn new(
        glyph: Glyph,
        sprite: Sprite,
        render_order: u32,
        sequence: Option<AnimationSequence<Point2<f32>>>,
    ) -> Self {
        Self {
            glyph,
            sprite,
            render_order,
            sequence,
        }
    }

    pub fn with_move(mut self, src: WorldPoint, dst: WorldPoint) -> Self {
        let sequence = animation::move_sequence(src, dst, &EasingEnum::EaseInOutCubic, 0.5);
        self.sequence = Some(sequence);
        self
    }

    pub fn update_time(&mut self, duration: f64) {
        if let Some(sequence) = &mut self.sequence {
            if sequence.finished() {
                self.sequence = None
            } else {
                sequence.advance_by(duration);
            }
        }
    }

    // TODO: right now this only supports tweening a position. Someday it should
    // also support TweenableRect which can use spritesheets.
    pub fn current_pos(&self) -> Option<WorldFloatPoint> {
        if let Some(sequence) = &self.sequence {
            let point = sequence.now_strict().expect("animation point");
            Some(WorldFloatPoint::new(point.x, point.y))
        } else {
            None
        }
    }
}
