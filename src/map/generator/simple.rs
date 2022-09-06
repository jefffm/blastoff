use bracket_lib::prelude::Point;
use bracket_lib::prelude::RandomNumberGenerator;
use tracing::debug;

use crate::map::Map;
use crate::map::MapGenerator;
use crate::map::Tile;
use crate::map::TileKind;
use crate::util::Vec2d;

const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;

pub struct Simple {}

impl MapGenerator for Simple {
    fn generate(&self, rng: &mut RandomNumberGenerator) -> Map {
        let mut tiles = Vec2d::new(
            vec![Tile::from(TileKind::Floor); MAP_HEIGHT * MAP_WIDTH],
            MAP_HEIGHT,
            MAP_WIDTH,
        );

        // Make the boundaries walls
        for x in 0..MAP_WIDTH {
            tiles.insert(Point::new(x, 0), TileKind::Wall.into());
            tiles.insert(Point::new(x, MAP_HEIGHT - 1), TileKind::Wall.into());
        }
        for y in 0..MAP_HEIGHT {
            tiles.insert(Point::new(0, y), TileKind::Wall.into());
            tiles.insert(Point::new(MAP_WIDTH - 1, y), TileKind::Wall.into());
        }

        Map::new("Simple Map".into(), MAP_WIDTH, MAP_HEIGHT, 1, tiles)
    }
}
