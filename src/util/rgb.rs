use bracket_lib::prelude::RGB as BracketRGB;
use bracket_lib::prelude::RGBA;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct RGB {
    r: f32,
    g: f32,
    b: f32,
}

impl From<RGB> for BracketRGB {
    fn from(rgb: RGB) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }
    }
}

impl From<BracketRGB> for RGB {
    fn from(rgb: BracketRGB) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }
    }
}

impl From<RGBA> for RGB {
    fn from(rgb: RGBA) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }
    }
}

impl From<RGB> for RGBA {
    fn from(rgb: RGB) -> Self {
        BracketRGB::from(rgb).into()
    }
}

impl From<(u8, u8, u8)> for RGB {
    fn from(t: (u8, u8, u8)) -> Self {
        Self {
            r: t.0 as f32,
            g: t.1 as f32,
            b: t.2 as f32,
        }
    }
}
