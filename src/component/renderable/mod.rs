use ggez::mint::Point2;
use keyframe::AnimationSequence;

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

    pub fn update_time(&mut self, duration: f64) {
        if let Some(sequence) = &mut self.sequence {
            if sequence.finished() {
                self.sequence = None
            } else {
                sequence.advance_by(duration);
            }
        }
    }
}
