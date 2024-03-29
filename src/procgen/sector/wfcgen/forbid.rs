use std::collections::HashSet;

use coord_2d::Coord;
use rand::Rng;

use wfc::{ForbidInterface, ForbidPattern, PatternId, Wrap};

use super::TilePattern;

/// Used to prevent wrapping on the edges in the result
///
///
/// # Examples
/// A fully functional example can be found under `examples/anchor.rs`
/// ```
/// let pattern = TilePattern::from_csv(input_path,
///     NonZeroU32::new(pattern_size).unwrap(),
///     &[Orientation::Original]).expect("Error while creating pattern");
///
/// let forbid = ForceBorderForbid::new(&pattern, pattern_size);
/// let grid = pattern.run_collapse(output_size, attempts,
///  WrapXY, forbid, &mut rand::thread_rng()).expect("Error in WFC");
/// ```
#[derive(Clone)]
pub struct ForceBorderForbid {
    pattern_ids: HashSet<PatternId>,
    offset: i32,
}

impl ForbidPattern for ForceBorderForbid {
    fn forbid<W: Wrap, R: Rng>(&mut self, fi: &mut ForbidInterface<W>, rng: &mut R) {
        let output_size = fi.wave_size();
        (0..(output_size.width() as i32))
            .map(|x| Coord::new(x, output_size.height() as i32 - self.offset as i32))
            .chain(
                (0..(output_size.width() as i32))
                    .map(|y| Coord::new(output_size.width() as i32 - self.offset as i32, y)),
            )
            .for_each(|coord| {
                self.pattern_ids.iter().for_each(|&pattern_id| {
                    fi.forbid_all_patterns_except(coord, pattern_id, rng)
                        .unwrap();
                });
            });
    }
}
impl ForceBorderForbid {
    pub fn new(pattern: &TilePattern, pattern_size: u32) -> ForceBorderForbid {
        let input_size = pattern.overlapping_patterns.grid().size();
        let bottom_right_offset = pattern_size - (pattern_size / 2);
        let id_grid = pattern.overlapping_patterns.id_grid();
        let bottom_right_coord = Coord::new(
            input_size.width() as i32 - bottom_right_offset as i32,
            input_size.height() as i32 - bottom_right_offset as i32,
        );
        let bottom_right_ids = id_grid
            .get_checked(bottom_right_coord)
            .iter()
            .cloned()
            .collect::<HashSet<_>>();

        ForceBorderForbid {
            pattern_ids: bottom_right_ids,
            offset: bottom_right_offset as i32,
        }
    }
}
