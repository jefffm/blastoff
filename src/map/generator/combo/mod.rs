use euclid::Transform2D;

use crate::{
    camera::Glyph,
    color::{Palette, COMMON},
    component::{Actor, ActorKind, Camera, Player, Position, Renderable, Viewshed},
    map::{Map, Spawner, Tile},
    util::{TransformExt, WorldPoint, WorldSize, WorldSpace},
};

use super::MapGenerator;

/// SubMap is an internal representation of an inner MapGenerator for Combo's MapTemplate
pub struct SubMap {
    mapgen: Box<dyn MapGenerator>,
    size: WorldSize,
    dest_point: WorldPoint,
}

impl SubMap {
    pub fn new(mapgen: Box<dyn MapGenerator>, size: WorldSize, dest_point: WorldPoint) -> Self {
        Self {
            mapgen,
            size,
            dest_point,
        }
    }
    fn generate(
        &mut self,
        rng: &mut bracket_random::prelude::RandomNumberGenerator,
        mapgen_history: &mut Vec<Map>,
    ) -> Map {
        self.mapgen.generate(self.size, rng, mapgen_history)
    }
}

/// MapTemplate is used by the Combo MapGenerator to store pre-baked parameters
/// for map composition.
pub struct MapTemplate {
    size: WorldSize,
    default_tile: Tile,
    submaps: Vec<SubMap>,
}

impl MapTemplate {
    pub fn new(size: WorldSize, default_tile: Tile, submaps: Vec<SubMap>) -> Self {
        Self {
            size,
            default_tile,
            submaps,
        }
    }
}

/// Combo combines multiple MapGenerator impls and composites them together into a megamap
pub struct Combo {
    template: MapTemplate,
}

impl Combo {
    pub fn new(template: MapTemplate) -> Self {
        Self { template }
    }
}

impl MapGenerator for Combo {
    fn generate(
        &mut self,
        size: WorldSize,
        // TODO: add default_tile argument
        rng: &mut bracket_random::prelude::RandomNumberGenerator,
        mapgen_history: &mut Vec<crate::map::Map>,
    ) -> Map {
        let mut map = Map::init(String::from("meta map"), size, self.template.default_tile);

        mapgen_history.push(map.clone());

        // Composite submaps in-order onto the Combo map
        for gen in &mut self.template.submaps {
            let submap = gen.generate(rng, mapgen_history);

            // Create a transform to translate submap space into Combo map space
            let xform = Transform2D::<i32, WorldSpace, WorldSpace>::from_points(
                submap.get_rect().origin,
                gen.dest_point,
            );

            // Iterate over the submap, translate to the Combo map's
            // coordinates, and overwrite the tile if it fits.
            const MAP_DEBUG_TILES_PER_FRAME: usize = 100;
            for (i, (subpoint, subtile)) in submap.iter_tiles().enumerate() {
                let map_point = xform.transform_point(subpoint);
                if map.contains(map_point) {
                    map[&map_point] = *subtile;
                }

                if i % MAP_DEBUG_TILES_PER_FRAME == 0 {
                    mapgen_history.push(map.clone())
                }
            }

            // Create a frame after each submap is composited. Submaps will also
            // add their own frames to the map generator
            mapgen_history.push(map.clone())
        }

        map
    }
}

impl Spawner for Combo {
    fn spawn(
        &self,
        map: &Map,
        world: &mut hecs::World,
        _rng: &mut bracket_random::prelude::RandomNumberGenerator,
    ) {
        for point in map.iter_points() {
            if let Tile::Floor(_) = map[&point] {
                // Add the player
                world.spawn((
                    Position::new(point),
                    Renderable::new(Glyph::new('@', COMMON.four, Palette::empty()), 5),
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
