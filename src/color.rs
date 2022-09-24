use rgb::RGBA8;

use ggez::graphics::Color;

// Each Palette uses five colors
pub struct Palette {
    pub one: RGBA8,
    pub two: RGBA8,
    pub three: RGBA8,
    pub four: RGBA8,
    pub five: RGBA8,
}

pub const EMPTY: RGBA8 = RGBA8 {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

impl Palette {
    pub fn empty() -> RGBA8 {
        EMPTY
    }
}

pub const COMMON: Palette = Palette {
    /// black
    one: RGBA8 {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    },
    // dark grey
    two: RGBA8 {
        r: 50,
        g: 50,
        b: 50,
        a: 255,
    },
    // grey
    three: RGBA8 {
        r: 160,
        g: 160,
        b: 160,
        a: 255,
    },
    // warm grey
    four: RGBA8 {
        r: 215,
        g: 215,
        b: 180,
        a: 255,
    },
    // cool grey
    five: RGBA8 {
        r: 180,
        g: 215,
        b: 215,
        a: 255,
    },
};

pub const FIRE: Palette = Palette {
    /// red
    one: RGBA8 {
        r: 120,
        g: 1,
        b: 22,
        a: 255,
    },
    /// yellow
    two: RGBA8 {
        r: 247,
        g: 181,
        b: 56,
        a: 255,
    },
    /// orange
    three: RGBA8 {
        r: 219,
        g: 124,
        b: 38,
        a: 255,
    },
    /// redorange
    four: RGBA8 {
        r: 216,
        g: 87,
        b: 42,
        a: 255,
    },
    five: RGBA8 {
        r: 195,
        g: 47,
        b: 39,
        a: 255,
    },
};

pub const WATER: Palette = Palette {
    /// blue
    one: RGBA8 {
        r: 5,
        g: 102,
        b: 141,
        a: 255,
    },
    /// light blue
    two: RGBA8 {
        r: 2,
        g: 128,
        b: 144,
        a: 255,
    },
    /// turquoise
    three: RGBA8 {
        r: 0,
        g: 168,
        b: 150,
        a: 255,
    },
    /// green
    four: RGBA8 {
        r: 2,
        g: 195,
        b: 154,
        a: 255,
    },
    /// sand
    five: RGBA8 {
        r: 240,
        g: 243,
        b: 189,
        a: 255,
    },
};

pub const PLANT: Palette = Palette {
    /// dark green
    one: RGBA8 {
        r: 19,
        g: 42,
        b: 19,
        a: 255,
    },
    /// green
    two: RGBA8 {
        r: 49,
        g: 87,
        b: 44,
        a: 255,
    },
    /// leaf
    three: RGBA8 {
        r: 79,
        g: 119,
        b: 45,
        a: 255,
    },
    /// light green
    four: RGBA8 {
        r: 144,
        g: 169,
        b: 85,
        a: 255,
    },
    /// yellowey
    five: RGBA8 {
        r: 236,
        g: 243,
        b: 158,
        a: 255,
    },
};

pub trait RGBA8Ext {
    fn to_ggez_color(&self) -> Color;
    fn from_ggez_color(color: Color) -> Self;
}

impl RGBA8Ext for RGBA8 {
    fn to_ggez_color(&self) -> Color {
        Color::from_rgba(self.r, self.g, self.b, self.a)
    }

    fn from_ggez_color(color: Color) -> Self {
        let (r, g, b, a) = color.to_rgba();
        Self { r, g, b, a }
    }
}
