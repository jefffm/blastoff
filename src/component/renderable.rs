use crate::camera::Glyph;

#[derive(Debug, Clone)]
pub struct Renderable {
    pub glyph: Glyph,
    pub render_order: u32,
}

impl Renderable {
    pub fn new(glyph: Glyph, render_order: u32) -> Self {
        Self {
            glyph,
            render_order,
        }
    }
}
