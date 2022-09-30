use serde::{Deserialize, Serialize};

use crate::color::COMMON;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FloorKind {
    FloorDefault,
    FloorInterior,
    FloorScenery(char),
}

impl From<char> for FloorKind {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::FloorDefault,
            '_' => Self::FloorInterior,
            glyph => Self::FloorScenery(glyph),
        }
    }
}

impl FloorKind {
    pub fn glyph(&self) -> char {
        match self {
            Self::FloorDefault => '.',
            Self::FloorInterior => '_',
            Self::FloorScenery(glyph) => *glyph,
        }
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
