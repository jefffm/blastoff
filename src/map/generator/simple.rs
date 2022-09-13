use std::convert::TryInto;

use bracket_lib::prelude::*;
use hecs::World;
use tracing::debug;

use crate::camera::Glyph;
use crate::component::*;
use crate::map::Map;
use crate::map::MapGenerator;
use crate::map::Spawner;
use crate::map::Tile;
use crate::map::TileKind;
use crate::util::WorldPoint;

const MAP_WIDTH: i32 = 25;
const MAP_HEIGHT: i32 = 25;

pub struct Simple {}

/// Create an extremely simple 10x10 map surrounded by wall
impl MapGenerator for Simple {
    fn generate(&self, _rng: &mut RandomNumberGenerator, level: u32) -> Map {
        let tiles = vec![Tile::from(TileKind::Floor); MAP_HEIGHT as usize * MAP_WIDTH as usize];

        let mut map = Map::new("Simple Map".into(), MAP_WIDTH, MAP_HEIGHT, tiles, level);

        // Make the boundaries walls
        for x in 0..MAP_WIDTH {
            map[&WorldPoint::new(x, 0)] = TileKind::Wall.into();
            map[&WorldPoint::new(x, MAP_HEIGHT - 1)] = TileKind::Wall.into();
        }
        for y in 0..MAP_HEIGHT {
            map[&WorldPoint::new(0, y)] = TileKind::Wall.into();
            map[&WorldPoint::new(MAP_WIDTH - 1, y)] = TileKind::Wall.into();
        }

        map
    }
}

impl Spawner for Simple {
    fn spawn(&self, map: &Map, world: &mut World) {
        let center = map.get_rect().center();

        // Add the player
        world.spawn((
            Position::new(center),
            Renderable::new(
                Glyph::new(to_cp437('@'), RGBA::from(WHITE), RGBA::from(BLACK)),
                1,
            ),
            Player {},
        ));

        // Add the camera
        world.spawn((Position::new(center), Camera {}));

        debug!("spawn complete");
    }
}
