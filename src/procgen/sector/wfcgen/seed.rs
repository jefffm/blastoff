use std::num::NonZeroU32;

use coord_2d::Size;
use wfc::orientation::Orientation;

use crate::sector::Tile;

use super::TilePattern;

pub struct WfcSeed {
    pub pattern: &'static str,
    pub orientation: &'static [Orientation],
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
    pattern: "\
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

pub const CAVE: WfcSeed = WfcSeed {
    pattern: "\
#################
##...####.####.##
#.....##...##...#
#.....##...##...#
#.....##...##...#
#####.####.####.#
#####.####......#
#####.######.####
##....###......##
##...####.####.##
#.....##...##...#
#.....##...##...#
#.....##...##...#
#####.####.####.#
#####.####......#
#####.########.##
##....########.##
##...####.####.##
#.....##...##...#
#.....##........#
#.....##...##...#
#####.####.####.#
#####......####.#
#################",
    orientation: &[
        Orientation::Original,
        // Orientation::Clockwise90,
        // Orientation::Clockwise180,
        // Orientation::Clockwise270,
    ],
    pattern_size: 3,
};

pub const CRATERS: WfcSeed = WfcSeed {
    pattern: "\
,,,,,,,,,,,,,,,,,,
,,,░,,,,,,,,,,,,*,
,,,,,,░░,,,,,,,,,,
,,░░░░░░░,,,,,,,,,
,,░▒▒▒▒▒░,,,,*,,,,
,░░▒▓▓▓▒░░,,,,*,,,
,,░▒▓▓▓▒░,,,,,,,,,
,,░▒▓▓▓▒░░,,░,,,,,
,,░▒▒▒▒▒░░,,,,,,,,
,░░░░░░░░,,,,,,,,,
,,,░░░░░░,,,,,,,,,
,░,,,░░,,,,,,,,,,,
,,,,,,,,,,,,,░░,,,
,,,,,,,,,,,,░▒▒░,,
,,,,,,,,,,,,,▒▓▒,,
,,,,*,,,,,,,░▒▒░,,
,,,,,,,,,,,,,░,,,,
,,,,,,,,,,,,,,,,,,",
    orientation: &[
        Orientation::Original,
        Orientation::Clockwise90,
        Orientation::Clockwise180,
        Orientation::Clockwise270,
    ],
    pattern_size: 3,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizes() {
        assert_eq!(CITY.pattern.chars().next().unwrap(), '.');
        assert_eq!(CITY.input_size(), Size::new(9, 9));
    }
}
