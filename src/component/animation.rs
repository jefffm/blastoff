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

#[derive(Clone, Default)]
pub struct Animation {
    sequence: Option<AnimationSequence<Point2<f32>>>,
}

/// The rendering system looks at this to determine which rect at which position to render
impl Animation {
    pub fn with_move(mut self, src: WorldPoint, dst: WorldPoint) -> Self {
        let sequence = move_sequence(src, dst, &EasingEnum::EaseInOutCubic, 0.5);
        self.sequence = Some(sequence);
        self
    }

    pub fn update(&mut self, duration: f64) {
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

fn move_sequence(
    mut start_point: WorldPoint,
    mut end_point: WorldPoint,
    ease_enum: &EasingEnum,
    duration: f32,
) -> AnimationSequence<Point2<f32>> {
    let start_pos: Point2<f32> = start_point.into_mint_f32();
    let end_pos: Point2<f32> = end_point.into_mint_f32();

    if let EasingEnum::EaseInOut3Point = ease_enum {
        let mid_pos = ease(Linear, start_pos, end_pos, 0.33);
        keyframes![
            (start_pos, 0.0, EaseInOut),
            (mid_pos, 0.66 * duration, EaseInOut),
            (end_pos, duration, EaseInOut)
        ]
    } else {
        keyframes![
            (start_pos, 0.0, easing_function(ease_enum)),
            (end_pos, duration, easing_function(ease_enum))
        ]
    }
}
