use ggez::mint::Point2;
use keyframe::{
    ease,
    functions::{EaseInOut, Linear},
    keyframes, AnimationSequence,
};

use crate::util::{easing_function, EasingEnum, PointExt, WorldFloatPoint};

pub fn move_sequence(
    start_point: WorldFloatPoint,
    end_point: WorldFloatPoint,
    ease_enum: &EasingEnum,
    duration: f32,
) -> AnimationSequence<Point2<f32>> {
    let start_pos: Point2<f32> = start_point.as_mint_f32();
    let end_pos: Point2<f32> = end_point.as_mint_f32();

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
