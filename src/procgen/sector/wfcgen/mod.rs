//! For more info: https://www.gridbugs.org/wave-function-collapse/
//!
mod forbid;
pub use forbid::*;

pub mod seed;

use std::{num::NonZeroU32, time::Instant};

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
use crate::overworld::SectorInfo;
use crate::procgen::Spawner;
use crate::resource::Resources;
use crate::sector::{FloorKind, Map, Tile};

use super::MapGenerator;

const WFC_INNER_RETRIES: usize = 100;
const WFC_OUTER_RETRIES: usize = 25;

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
        sector_info: &SectorInfo,
        resources: &mut Resources,
        mapgen_history: &mut Vec<Map>,
    ) -> Map {
        let output_size = Size::new(
            sector_info.size.width as u32,
            sector_info.size.height as u32,
        );

        let pattern = self.seed.tile_pattern();
        let pattern_size = pattern.grid.size();
        let _forbid = ForceBorderForbid::new(&pattern, self.seed.pattern_size);

        tracing::info!(
            output_width = output_size.width(),
            output_height = output_size.height(),
            input_width = pattern_size.width(),
            input_height = pattern_size.height(),
            seed_pattern_size = self.seed.pattern_size,
            orientation_count = self.seed.orientation.len(),
            inner_retries = WFC_INNER_RETRIES,
            outer_retries = WFC_OUTER_RETRIES,
            "Starting WFC sector generation"
        );

        let start = Instant::now();
        let mut last_error = None;

        for attempt in 1..=WFC_OUTER_RETRIES {
            tracing::info!(
                attempt,
                max_attempts = WFC_OUTER_RETRIES,
                "Running WFC collapse"
            );

            match pattern.run_collapse(
                output_size,
                WFC_INNER_RETRIES,
                WrapNone,
                ForbidNothing,
                // forbid.clone(),
                resources.rng.get_rng(), // &mut rand::thread_rng(),
            ) {
                Ok(grid) => {
                    tracing::info!(
                        attempt,
                        elapsed_ms = start.elapsed().as_millis(),
                        "WFC sector generation succeeded"
                    );

                    let tilevec: Vec<Tile> = grid.iter().map(move |tile| tile.to_owned()).collect();
                    let map = Map::new(
                        "WFC tester".into(),
                        output_size.width() as i32,
                        output_size.height() as i32,
                        tilevec,
                    );
                    mapgen_history.push(map.clone());

                    return map;
                }
                Err(err) => {
                    tracing::warn!(
                        attempt,
                        max_attempts = WFC_OUTER_RETRIES,
                        error = ?err,
                        elapsed_ms = start.elapsed().as_millis(),
                        "WFC collapse failed; retrying"
                    );
                    last_error = Some(err);
                }
            }
        }

        tracing::error!(
            attempts = WFC_OUTER_RETRIES,
            inner_retries = WFC_INNER_RETRIES,
            output_width = output_size.width(),
            output_height = output_size.height(),
            input_width = pattern_size.width(),
            input_height = pattern_size.height(),
            seed_pattern_size = self.seed.pattern_size,
            orientation_count = self.seed.orientation.len(),
            last_error = ?last_error,
            elapsed_ms = start.elapsed().as_millis(),
            "WFC sector generation exhausted retries; using simple fallback map"
        );

        let fallback = Map::init(
            "WFC fallback after exhausted retries".into(),
            sector_info.size,
            Tile::Floor(FloorKind::FloorDefault),
        );
        mapgen_history.push(fallback.clone());
        fallback
    }
}

impl Spawner for WfcGen {
    fn spawn(&self, map: &Map, world: &mut hecs::World, _resources: &mut Resources) {
        for point in map.iter_points() {
            if let Tile::Floor(_) = map[&point] {
                // Add the player
                world.spawn((
                    Position::new(point),
                    // Renderable::new(
                    //     Glyph::new('@', COMMON.four, Palette::empty()),
                    //     PLAYER,
                    //     5,
                    //     None,
                    // ),
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
