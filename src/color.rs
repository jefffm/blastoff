use macroquad::{color_u8, prelude::Color};

// Each Palette uses five colors
pub struct Palette {
    pub one: Color,
    pub two: Color,
    pub three: Color,
    pub four: Color,
    pub five: Color,
}

pub const EMPTY: Color = color_u8!(71, 45, 60, 0);

impl Palette {
    pub fn empty() -> Color {
        EMPTY
    }
}

pub const COMMON: Palette = Palette {
    /// black
    one: color_u8!(0, 0, 0, 255),
    // dark grey
    two: color_u8!(50, 50, 50, 255),
    // grey
    three: color_u8!(160, 160, 160, 255),
    // warm grey
    four: color_u8!(215, 215, 180, 255),
    // cool grey
    five: color_u8!(180, 215, 215, 255),
};

pub const FIRE: Palette = Palette {
    /// red
    one: color_u8!(120, 1, 22, 255),
    /// yellow
    two: color_u8!(247, 181, 56, 255),
    /// orange
    three: color_u8!(219, 124, 38, 255),
    /// redorange
    four: color_u8!(216, 87, 42, 255),
    five: color_u8!(195, 47, 39, 255),
};

pub const WATER: Palette = Palette {
    /// blue
    one: color_u8!(5, 102, 141, 255),
    /// light blue
    two: color_u8!(2, 128, 144, 255),
    /// turquoise
    three: color_u8!(0, 168, 150, 255),
    /// green
    four: color_u8!(2, 195, 154, 255),
    /// sand
    five: color_u8!(240, 243, 189, 255),
};

pub const PLANT: Palette = Palette {
    /// dark green
    one: color_u8!(19, 42, 19, 255),
    /// green
    two: color_u8!(49, 87, 44, 255),
    /// leaf
    three: color_u8!(79, 119, 45, 255),
    /// light green
    four: color_u8!(144, 169, 85, 255),
    /// yellowey
    five: color_u8!(236, 243, 158, 255),
};
