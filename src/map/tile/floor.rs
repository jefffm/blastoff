use serde::{Deserialize, Serialize};

use crate::color::COMMON;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FloorKind {
    FloorDefault,
}
impl FloorKind {
    pub fn glyph(&self) -> char {
        '.'
    }

    pub fn is_passable(&self) -> bool {
        true
    }

    pub fn is_opaque(&self) -> bool {
        false
    }

    pub fn fg(&self) -> rgb::RGBA8 {
        COMMON.three
    }
}

impl Default for FloorKind {
    fn default() -> Self {
        Self::FloorDefault
    }
}
