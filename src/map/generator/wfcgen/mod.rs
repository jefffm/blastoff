//! For more info: https://www.gridbugs.org/wave-function-collapse/
//!
mod forbid;
pub use forbid::*;

pub mod seed;

use std::num::NonZeroU32;

use bracket_random::prelude::RandomNumberGenerator;
use grid_2d::Grid;
use rand::Rng;
use wfc::orientation::Orientation;
use wfc::overlapping::OverlappingPatterns;
use wfc::wrap::WrapNone;
use wfc::{retry, ForbidNothing, ForbidPattern, PropagateError, RunOwn, Wrap};

use coord_2d::{Coord, Size};

use crate::camera::Glyph;
use crate::color::{Palette, COMMON};
use crate::component::{Actor, ActorKind, Camera, Player, Position, Renderable, Viewshed};
use crate::map::{Map, Spawner, Tile};
use crate::util::{WorldSize, PLAYER};

use super::MapGenerator;

pub struct TilePattern {
    pub grid: Grid<Tile>,
    pub overlapping_patterns: OverlappingPatterns<Tile>,
}

impl TilePattern {
    pub fn new(
        grid: Grid<Tile>,
        pattern_size: NonZeroU32,
        orientation: &[Orientation],
    ) -> TilePattern {
        let overlapping_patterns =
            OverlappingPatterns::new(grid.clone(), pattern_size, orientation);
        TilePattern {
            grid,
            overlapping_patterns,
        }
    }
    pub fn from_vec(
        map: Vec<Tile>,
        size: Size,
        pattern_size: NonZeroU32,
        orientation: &[Orientation],
    ) -> TilePattern {
        let grid = Grid::new_fn(size, |Coord { x, y }| {
            map[(y * (size.width() as i32) + x) as usize]
        });
        TilePattern::new(grid, pattern_size, orientation)
    }

    pub fn run_collapse<W: Wrap, F: ForbidPattern + Send + Sync + Clone, R: Rng>(
        &self,
        output_size: Size,
        retry_times: usize,
        wrap: W,
        forbid: F,
        rng: &mut R,
    ) -> Result<Grid<Tile>, PropagateError> {
        let global_stats = self.overlapping_patterns.global_stats();

        let run = RunOwn::new_wrap_forbid(output_size, &global_stats, wrap, forbid, rng);
        let wave = run.collapse_retrying(retry::NumTimes(retry_times), rng)?;
        let wave_grid = wave.grid();

        let wave_cell_iter: Option<Vec<_>> = wave_grid
            .size()
            .coord_iter_row_major()
            .map(|coord| {
                wave_grid
                    .get(coord)
                    .expect("wave cell")
                    .chosen_pattern_id()
                    .ok()
                    .map(|pattern_id| *self.overlapping_patterns.pattern_top_left_value(pattern_id))
            })
            .collect();

        match wave_cell_iter {
            Some(cells) => Ok(Grid::new_iterator(wave_grid.size(), cells.into_iter())),
            None => Err(PropagateError::Contradiction),
        }
    }
}

pub struct WfcGen {
    seed: seed::WfcSeed,
}
impl WfcGen {
    pub fn new(seed: seed::WfcSeed) -> Self {
        Self { seed }
    }
}

impl MapGenerator for WfcGen {
    fn generate(
        &mut self,
        size: WorldSize,
        rng: &mut RandomNumberGenerator,
        mapgen_history: &mut Vec<Map>,
    ) -> Map {
        let output_size = Size::new(size.width as u32, size.height as u32);

        let pattern = self.seed.tile_pattern();
        let forbid = ForceBorderForbid::new(&pattern, self.seed.pattern_size);
        // Run Wave Function Collapse until it succeeds
        let grid = loop {
            tracing::info!("Running (or rerunning) wfc gen");
            if let Ok(grid) = pattern.run_collapse(
                output_size,
                1000,
                WrapNone,
                ForbidNothing,
                // forbid.clone(),
                rng.get_rng(), // &mut rand::thread_rng(),
            ) {
                break grid;
            }
        };

        let tilevec: Vec<Tile> = grid.iter().map(move |tile| tile.to_owned()).collect();

        let map = Map::new(
            "WFC tester".into(),
            output_size.width() as i32,
            output_size.height() as i32,
            tilevec,
        );
        mapgen_history.push(map.clone());

        map
    }
}

impl Spawner for WfcGen {
    fn spawn(&self, map: &Map, world: &mut hecs::World, _rng: &mut RandomNumberGenerator) {
        for point in map.iter_points() {
            if let Tile::Floor(_) = map[&point] {
                // Add the player
                world.spawn((
                    Position::new(point),
                    Renderable::new(
                        Glyph::new('@', COMMON.four, Palette::empty()),
                        PLAYER,
                        5,
                        None,
                    ),
                    Viewshed::default().with_init().with_range(100),
                    Player {},
                    Actor::new(0, 100, 100, 20, 0, ActorKind::Player(None)),
                ));

                // Add the camera
                world.spawn((Position::new(map.get_rect().center()), Camera {}));
                break;
            }
        }
    }
}
