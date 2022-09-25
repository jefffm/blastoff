use serde::{Deserialize, Serialize};

use crate::color::COMMON;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FloorKind {
    FloorDefault,
    FloorInterior,
}

impl From<char> for FloorKind {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::FloorDefault,
            '_' => Self::FloorInterior,
            _ => panic!("invalid floor char"),
        }
    }
}

impl FloorKind {
    pub fn glyph(&self) -> char {
        match self {
            Self::FloorDefault => '.',
            Self::FloorInterior => '_',
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
