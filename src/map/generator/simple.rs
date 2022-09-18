use bracket_lib::prelude::*;
use hecs::World;
use tracing::debug;

use crate::camera::Glyph;
use crate::color::RGBA8Ext;
use crate::color::COMMON;
use crate::component::*;
use crate::map::Map;
use crate::map::MapGenerator;
use crate::map::Spawner;
use crate::map::TileKind;
use crate::util::WorldPoint;

const MAP_WIDTH: i32 = 25;
const MAP_HEIGHT: i32 = 25;

pub struct Simple {}

/// Create an extremely simple 10x10 map surrounded by wall
impl MapGenerator for Simple {
    fn generate(
        &mut self,
        _rng: &mut RandomNumberGenerator,
        mapgen_history: &mut Vec<Map>,
        level: u32,
    ) -> Map {
        let tiles = vec![TileKind::Floor; MAP_HEIGHT as usize * MAP_WIDTH as usize];

        let mut map = Map::new("Simple Map".into(), MAP_WIDTH, MAP_HEIGHT, tiles, level);

        // Make the boundaries walls
        for x in 0..MAP_WIDTH {
            map[&WorldPoint::new(x, 0)] = TileKind::Wall;
            map[&WorldPoint::new(x, MAP_HEIGHT - 1)] = TileKind::Wall;
            mapgen_history.push(map.clone());
        }
        for y in 0..MAP_HEIGHT {
            map[&WorldPoint::new(0, y)] = TileKind::Wall;
            map[&WorldPoint::new(MAP_WIDTH - 1, y)] = TileKind::Wall;
            mapgen_history.push(map.clone());
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
                Glyph::new(
                    to_cp437('@'),
                    COMMON.four.to_bracket_rgba(),
                    COMMON.one.to_bracket_rgba(),
                ),
                1,
            ),
            Player {},
        ));

        // Add the camera
        world.spawn((Position::new(center), Camera {}));

        debug!("spawn complete");
    }
}
