use bracket_lib::prelude::*;

use crate::camera::Glyph;
use crate::component::*;
use crate::{
    map::{Map, Spawner, TileKind},
    util::{WorldPoint, WorldRect, WorldSize, WorldVector},
};

use super::MapGenerator;

pub struct Bsp {
    size: WorldSize,
    rooms: Vec<WorldRect>,
}

impl Bsp {
    pub fn new(size: WorldSize) -> Self {
        Self {
            size,
            rooms: Vec::new(),
        }
    }
}

/// ```
/// ###############        ###############
/// #             #        #  1   +   2  #
/// #             #        #      +      #
/// #      0      #   ->   #+++++++++++++#
/// #             #        #   3  +   4  #
/// #             #        #      +      #
/// ###############        ###############
/// ```
fn add_subrects(rect: &WorldRect, rects: &mut Vec<WorldRect>) {
    if rect.area() < 1 {
        return;
    }

    let half_size = rect.size / 2;
    rects.push(WorldRect::new(rect.min(), half_size));
    rects.push(WorldRect::new(
        WorldPoint::new(rect.center().x, rect.min().y),
        half_size,
    ));
    rects.push(WorldRect::new(
        WorldPoint::new(rect.min().x, rect.center().y),
        half_size,
    ));
    rects.push(WorldRect::new(
        WorldPoint::new(rect.center().x, rect.center().y),
        half_size,
    ));
}

/// Pick any of the rects from the passed-in vec
fn select_random_rect(rng: &mut RandomNumberGenerator, rects: &Vec<WorldRect>) -> WorldRect {
    if rects.len() == 1 {
        return rects[0];
    }

    let idx = (rng.roll_dice(1, rects.len() as i32) - 1) as usize;
    rects[idx]
}

/// ```
/// ###############        ########
/// #             #        #   1  #
/// #             #        #      #
/// #      0      #   ->   ########
/// #             #
/// #             #
/// ###############
/// ```
fn get_random_sub_rect(rng: &mut RandomNumberGenerator, rect: &WorldRect) -> WorldRect {
    let width = i32::max(3, rng.roll_dice(1, i32::min(rect.width(), 10)) - 1) + 1;
    let height = i32::max(3, rng.roll_dice(1, i32::min(rect.height(), 10)) - 1) + 1;

    WorldRect::new(
        rect.origin
            + WorldVector::new(
                rng.roll_dice(1, rect.width()) - 1,
                rng.roll_dice(1, rect.height()) - 1,
            ),
        WorldSize::new(width, height),
    )
}

fn is_possible(rect: &WorldRect, map: &Map) -> bool {
    let padding = 2;
    let expanded = WorldRect::new(
        rect.origin - WorldVector::new(padding, padding),
        rect.size + WorldSize::new(padding * 2, padding * 2),
    );

    for x in expanded.x_range() {
        for y in expanded.y_range() {
            if x < 1 {
                return false;
            }
            if y < 1 {
                return false;
            }

            // If any point in the new rectangle is outside map boundaries, abort
            let point = WorldPoint::new(x, y);
            if !map.get_rect().contains(point) {
                return false;
            }

            // If any point in the new rectangle is already carved out for a room, abort
            if map[&point] != TileKind::Wall {
                return false;
            }
        }
    }

    true
}

fn apply_room_to_map(map: &mut Map, room: &WorldRect) {
    for x in room.x_range() {
        for y in room.y_range() {
            let point = WorldPoint::new(x, y);
            map[&point] = TileKind::Floor;
        }
    }
}

fn draw_corridor(map: &mut Map, start: &WorldPoint, end: &WorldPoint) {
    let mut cursor = *start;

    while cursor != *end {
        if cursor.x < end.x {
            cursor.x += 1;
        } else if cursor.x > end.x {
            cursor.x -= 1;
        } else if cursor.y < end.y {
            cursor.y += 1;
        } else if cursor.y > end.y {
            cursor.y -= 1;
        }

        map[&cursor] = TileKind::Floor;
    }
}

impl MapGenerator for Bsp {
    fn generate(
        &mut self,
        rng: &mut RandomNumberGenerator,
        mapgen_history: &mut Vec<crate::map::Map>,
        level: u32,
    ) -> Map {
        let mut map = Map::init("bsp".into(), self.size, level);

        // Initialize with a single Rect
        let mut rects = vec![WorldRect::new(WorldPoint::new(0, 0), self.size)];

        let first_rect = rects[0];
        add_subrects(&first_rect, &mut rects);

        // Create rooms
        let mut n_rooms = 1;
        while n_rooms < 240 {
            let rect = select_random_rect(rng, &rects);
            let candidate = get_random_sub_rect(rng, &rect);

            if is_possible(&candidate, &map) {
                apply_room_to_map(&mut map, &candidate);
                self.rooms.push(candidate);

                add_subrects(&rect, &mut rects);

                // Snapshot to mapgen history
                mapgen_history.push(map.clone());
            }

            n_rooms += 1;
        }

        // Sort rooms by left coordinate
        self.rooms.sort_by(|a, b| a.origin.x.cmp(&b.origin.x));

        for i in 0..self.rooms.len() - 1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i + 1];
            let start_x = room.origin.x + (rng.roll_dice(1, room.width()) - 1);
            let start_y = room.origin.y + (rng.roll_dice(1, room.height()) - 1);
            let end_x = next_room.origin.x + (rng.roll_dice(1, next_room.width()) - 1);
            let end_y = next_room.origin.y + (rng.roll_dice(1, next_room.height()) - 1);

            draw_corridor(
                &mut map,
                &WorldPoint::new(start_x, start_y),
                &WorldPoint::new(end_x, end_y),
            );

            // Snapshot to mapgen history
            mapgen_history.push(map.clone());
        }

        map
    }
}
impl Spawner for Bsp {
    fn spawn(&self, _map: &crate::map::Map, world: &mut hecs::World) {
        let center = self.rooms[0].center();

        let mut viewshed = Viewshed::default().with_range(10);
        viewshed.set_dirty();

        // Add the player
        world.spawn((
            Position::new(center),
            Renderable::new(
                Glyph::new(to_cp437('@'), RGBA::from(WHITE), RGBA::from(BLACK)),
                1,
            ),
            viewshed,
            Player {},
            Actor::new(0, 100, 100, 20, 0, ActorKind::Player(None)),
        ));

        // Add the camera
        world.spawn((Position::new(center), Camera {}));

        // Add a monster
        let mut monster_viewshed = Viewshed::default().with_range(10);
        monster_viewshed.set_dirty();
        world.spawn((
            Position::new(self.rooms[1].center()),
            Renderable::new(
                Glyph::new(to_cp437('k'), RGBA::from(RED), RGBA::from(BLACK)),
                1,
            ),
            monster_viewshed,
            Actor::new(0, 100, 100, 25, 0, ActorKind::Computer(None)),
            Behavior::new(BehaviorKind::FollowNearest),
        ));

        tracing::debug!("spawn complete");
    }
}
