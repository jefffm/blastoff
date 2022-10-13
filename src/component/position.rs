use std::collections::VecDeque;

use keyframe::{mint::Point2, AnimationSequence};

use crate::{
    animation,
    util::{EasingEnum, WorldFloatPoint, WorldPoint},
};

#[derive(Clone)]
pub struct Position {
    pub p: WorldPoint,
    pub sequences: VecDeque<AnimationSequence<Point2<f32>>>,
    sequence_duration_divisor: f32,
}

impl Position {
    pub fn new(point: WorldPoint) -> Self {
        Self {
            p: point,
            sequences: VecDeque::new(),
            sequence_duration_divisor: 1.,
        }
    }

    /// Move an entity to a given world float component
    pub fn move_to(&mut self, point: WorldPoint, duration_secs: f32) {
        self.set_move_now(self.render_point(), point, duration_secs);
        self.p = point.to_i32();
    }

    ///
    pub fn move_to_smooth(&mut self, point: WorldPoint, duration_secs: f32) {
        self.set_move_now_smooth(self.render_point(), point, duration_secs);
        self.p = point.to_i32();
    }

    /// Move an entity to a given world float component
    pub fn move_to_queued(&mut self, point: WorldPoint, duration_secs: f32) {
        self.set_move_queued(self.render_point(), point, duration_secs);
        self.p = point.to_i32();
    }

    /// Point to use for in-game targetting and such
    pub fn grid_point(&self) -> WorldPoint {
        self.p
    }

    /// Point to use for rendering
    pub fn render_point(&self) -> WorldFloatPoint {
        if let Some(sequence) = self.sequences.front() {
            let point = sequence.now_strict().expect("animation point");
            WorldFloatPoint::new(point.x, point.y)
        } else {
            self.p.to_f32()
        }
    }

    pub fn set_grid_point(&mut self, point: WorldPoint) {
        self.p = point
    }

    /// Add a move animation for the move
    fn set_move_now(&mut self, src: WorldFloatPoint, dst: WorldPoint, duration_secs: f32) {
        // If there is an outstanding animation, shorten the length of the animation
        let duration = if !self.sequences.is_empty() {
            // self.sequence_duration_divisor += 1.;
            duration_secs / self.sequence_duration_divisor
        } else {
            duration_secs
        };
        let sequence =
            animation::move_sequence(src, dst.to_f32(), &EasingEnum::EaseInOutCubic, duration);

        self.sequences.clear();
        self.sequences.push_back(sequence)
    }

    /// Add a move animation for the move
    fn set_move_now_smooth(&mut self, src: WorldFloatPoint, dst: WorldPoint, duration_secs: f32) {
        // If there is an outstanding animation, shorten the length of the animation
        let duration = if !self.sequences.is_empty() {
            // self.sequence_duration_divisor += 1.;
            duration_secs / self.sequence_duration_divisor
        } else {
            duration_secs
        };
        let sequence =
            animation::move_sequence(src, dst.to_f32(), &EasingEnum::EaseOutCubic, duration);

        self.sequences.clear();
        self.sequences.push_back(sequence)
    }

    /// Add a move animation for the move
    fn set_move_queued(&mut self, src: WorldFloatPoint, dst: WorldPoint, duration_secs: f32) {
        // If there is an outstanding animation, shorten the length of the animation
        let duration = if !self.sequences.is_empty() {
            // self.sequence_duration_divisor += 1.;
            duration_secs / self.sequence_duration_divisor
        } else {
            duration_secs
        };
        let sequence =
            animation::move_sequence(src, dst.to_f32(), &EasingEnum::EaseInOutCubic, duration);
        self.sequences.push_back(sequence)
    }

    /// Update the deltatime for tweening
    pub fn update_time(&mut self, duration: f64) {
        if let Some(sequence) = self.sequences.front_mut() {
            if sequence.finished() {
                self.sequences.pop_front();
                // self.sequence_duration_divisor = (self.sequence_duration_divisor - 1.).max(1.);
            } else {
                sequence.advance_by(duration);
            }
        }

        // TODO: cooldown the animation speedup
    }
}
