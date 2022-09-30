use ggez::graphics;
use keyframe::{functions::*, EasingFunction};
use keyframe_derive::CanTween;

#[derive(Debug, PartialEq)]
#[repr(i32)]
pub enum EasingEnum {
    Linear,
    EaseIn,
    EaseInOut,
    EaseOut,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    Bezier,
    EaseInOut3Point,
}

pub fn easing_function(ease_enum: &EasingEnum) -> Box<dyn EasingFunction + Send + Sync> {
    match ease_enum {
        EasingEnum::Linear => Box::new(Linear),
        EasingEnum::EaseIn => Box::new(EaseIn),
        EasingEnum::EaseInOut => Box::new(EaseInOut),
        EasingEnum::EaseOut => Box::new(EaseOut),
        EasingEnum::EaseInCubic => Box::new(EaseInCubic),
        EasingEnum::EaseOutCubic => Box::new(EaseOutCubic),
        EasingEnum::EaseInOutCubic => Box::new(EaseInOutCubic),
        EasingEnum::Bezier => Box::new(BezierCurve::from([0.6, 0.04].into(), [0.98, 0.335].into())),
        _ => panic!(),
    }
}

#[derive(CanTween, Clone, Copy)]
/// necessary because we can't implement CanTween for graphics::Rect directly, as it's a foreign type
pub struct TweenableRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl TweenableRect {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        TweenableRect { x, y, w, h }
    }
}

impl From<TweenableRect> for graphics::Rect {
    fn from(t_rect: TweenableRect) -> Self {
        graphics::Rect {
            x: t_rect.x,
            y: t_rect.y,
            w: t_rect.w,
            h: t_rect.h,
        }
    }
}

/// A fancy easing function, tweening something into one of `frames` many discrete states.
/// The `pre_easing` is applied first, thereby making other `EasingFunction`s usable in the realm of frame-by-frame animation
struct AnimationFloor {
    pre_easing: Box<dyn EasingFunction + Send + Sync>,
    frames: i32,
}
impl EasingFunction for AnimationFloor {
    #[inline]
    fn y(&self, x: f64) -> f64 {
        (self.pre_easing.y(x) * (self.frames) as f64).floor() / (self.frames - 1) as f64
    }
}
