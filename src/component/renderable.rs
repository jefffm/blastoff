use crate::{camera::Glyph, util::Sprite};

#[derive(Debug, Clone, Copy)]
pub struct Renderable {
    pub glyph: Glyph,
    pub sprite: Sprite,
    pub render_order: u32,
}

impl Renderable {
    pub fn new(glyph: Glyph, sprite: Sprite, render_order: u32) -> Self {
        Self {
            glyph,
            sprite,
            render_order,
        }
    }
}
