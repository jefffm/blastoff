use std::num::NonZeroU32;

use coord_2d::Size;
use wfc::orientation::Orientation;

use crate::map::Tile;

use super::TilePattern;

pub struct WfcSeed {
    pattern: &'static str,
    orientation: &'static [Orientation],
    pub pattern_size: u32,
}

impl WfcSeed {
    fn input_size(&self) -> Size {
        let width = self.pattern.lines().next().unwrap().len();
        let height = self.pattern.lines().count();

        Size::new(width as u32, height as u32)
    }

    pub fn tile_pattern(&self) -> TilePattern {
        let tiles = self.pattern.chars().filter_map(Tile::from_char).collect();

        // Extract patterns from input
        TilePattern::from_vec(
            // WFC Input
            tiles,
            self.input_size(),
            // What size tiles to use examining the input?
            NonZeroU32::new(self.pattern_size).unwrap(),
            // Can we rotate tiles?
            // &[Orientation::Original, Orientation::Clockwise180],
            self.orientation,
        )
    }
}

pub const CITY: WfcSeed = WfcSeed {
    pattern: r"\
.........
.╔═╦═╗...
.║___║...
.║___║...
.║___║...
.║___║...
.╚═╩═╝...
.........
.........",
    orientation: &[Orientation::Original],
    pattern_size: 3,
};
