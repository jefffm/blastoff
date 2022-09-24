//! For more info: https://www.gridbugs.org/wave-function-collapse/
//!
use std::num::NonZeroU32;
use std::option;

use bracket_random::prelude::RandomNumberGenerator;
use grid_2d::Grid;
use rand::Rng;
use wfc::orientation::Orientation;
use wfc::overlapping::OverlappingPatterns;
use wfc::wrap::WrapXY;
use wfc::{retry, ForbidNothing, ForbidPattern, PropagateError, RunOwn, Wrap};

use coord_2d::{Coord, Size};

use crate::camera::Glyph;
use crate::color::{Palette, COMMON};
use crate::component::{Actor, ActorKind, Camera, Player, Position, Renderable, Viewshed};
use crate::map::{Map, Spawner, Tile};

use super::MapGenerator;

const TEST_HOUSE: &str = r"\
.........
.╔═╦═╗...
.║○○○║...
.║○○○║...
.║○○○║...
.║○○○║...
.╚═╩═╝...
.........
.........
";
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
        let grid = Grid::new_fn(wave_grid.size(), |coord| {
            *self
                .overlapping_patterns
                .pattern_top_left_value(wave_grid.get(coord).unwrap().chosen_pattern_id().unwrap())
        });
        Ok(grid)
    }
}

pub struct WfcGen {}

impl MapGenerator for WfcGen {
    fn generate(
        &mut self,
        rng: &mut RandomNumberGenerator,
        mapgen_history: &mut Vec<Map>,
        level: u32,
    ) -> Map {
        let output_size = Size::new(50, 50);
        let tiles = TEST_HOUSE.chars().filter_map(Tile::from_char).collect();

        // Extract patterns from input
        let pattern = TilePattern::from_vec(
            tiles,
            Size::new(9, 9),
            NonZeroU32::new(3).unwrap(),
            &[Orientation::Original, Orientation::Clockwise180],
        );

        // Run Wave Function Collapse
        let grid = pattern
            .run_collapse(
                output_size,
                1000,
                WrapXY,
                ForbidNothing,
                rng.get_rng(), // &mut rand::thread_rng(),
            )
            .expect("Error in WFC");

        let tilevec: Vec<Tile> = grid.iter().map(move |tile| tile.to_owned()).collect();

        let map = Map::new(
            "WFC tester".into(),
            output_size.width() as i32,
            output_size.height() as i32,
            tilevec,
            1,
        );
        mapgen_history.push(map.clone());

        map
    }
}

impl Spawner for WfcGen {
    fn spawn(&self, map: &Map, world: &mut hecs::World, rng: &mut RandomNumberGenerator) {
        for point in map.iter_points() {
            if let Tile::Floor(_) = map[&point] {
                // Add the player
                world.spawn((
                    Position::new(point),
                    Renderable::new(Glyph::new('@', COMMON.four, Palette::empty()), 5),
                    Viewshed::default().with_init().with_range(10),
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
