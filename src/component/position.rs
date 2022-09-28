use ggez::mint::Point2;
use keyframe::AnimationSequence;

use crate::{
    animation,
    util::{EasingEnum, PointExt, WorldFloatPoint, WorldPoint, WorldVector},
};

#[derive(Clone)]
pub struct Position {
    pub p: WorldPoint,
    pub sequence: Option<AnimationSequence<Point2<f32>>>,
    sequence_duration_divisor: f32,
}

impl Position {
    pub fn new(point: WorldPoint) -> Self {
        Self {
            p: point,
            sequence: None,
            sequence_duration_divisor: 1.,
        }
    }

    /// Move an entity to a given world float component
    pub fn move_to(&mut self, point: WorldFloatPoint, duration_secs: f32) {
        self.set_move(self.p.to_f32(), point, duration_secs);
        self.p = point.to_i32();
    }

    /// Point to use for in-game targetting and such
    pub fn grid_point(&self) -> WorldPoint {
        self.p
    }

    /// Point to use for rendering
    pub fn render_point(&self) -> WorldFloatPoint {
        if let Some(sequence) = &self.sequence {
            let point = sequence.now_strict().expect("animation point");
            WorldFloatPoint::new(point.x, point.y)
        } else {
            // tracing::info!("Animation complete! Returning world point: {:?}", self.p);
            self.p.to_f32()
        }
    }

    pub fn set_grid_point(&mut self, point: WorldPoint) {
        self.p = point
    }

    /// Add a move animation for the move
    fn set_move(&mut self, src: WorldFloatPoint, dst: WorldFloatPoint, duration_secs: f32) {
        // If there is an outstanding animation, shorten the length of the animation
        let duration = if self.sequence.is_some() {
            // self.sequence_duration_divisor += 1.;
            duration_secs / self.sequence_duration_divisor
        } else {
            duration_secs
        };
        let sequence = animation::move_sequence(src, dst, &EasingEnum::EaseInOutCubic, duration);
        self.sequence = Some(sequence);
    }

    /// Update the deltatime for tweening
    pub fn update_time(&mut self, duration: f64) {
        if let Some(sequence) = &mut self.sequence {
            if sequence.finished() {
                self.sequence = None
            } else {
                sequence.advance_by(duration);
            }
        }

        // TODO: cooldown the animation speedup
    }
}
