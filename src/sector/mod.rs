mod tile;
pub use tile::*;

use euclid::{Point2D, Rect, Size2D};
use fixedbitset::FixedBitSet;
use hecs::Entity;
use pathfinding::prelude::astar;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    ops::{Index, IndexMut},
};

use crate::{
    component::Cardinal,
    util::{PointExt, WorldPoint, WorldSize, WorldSpace},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Map {
    name: String,
    tiles: Vec<Tile>,
    rect: Rect<i32, WorldSpace>,
    blocked: FixedBitSet,
    revealed: FixedBitSet,
    visible: FixedBitSet,
    content: Vec<Vec<Entity>>,
}

impl Map {
    /// Create a map with a given size filled in with Wall tiles
    pub fn init(name: String, size: WorldSize, default_tile: Tile) -> Self {
        let tiles = vec![default_tile; (size.height * size.width) as usize];
        Self::new(name, size.width, size.height, tiles)
    }

    pub fn new(name: String, width: i32, height: i32, tiles: Vec<Tile>) -> Self {
        let rect = Rect::new(WorldPoint::new(0, 0), Size2D::new(width, height));
        let area = rect.size.area().try_into().unwrap();

        let blocked = FixedBitSet::with_capacity(area);
        let revealed = FixedBitSet::with_capacity(area);
        let visible = FixedBitSet::with_capacity(area);
        let content = vec![Vec::<Entity>::new(); area];
        Self {
            name,
            tiles,
            rect,
            blocked,
            revealed,
            visible,
            content,
        }
    }

    pub fn astar_path(&self, start: &WorldPoint, end: &WorldPoint) -> Option<Vec<WorldPoint>> {
        astar(
            start,
            |p| self.neighbors(p),
            |p| {
                // TODO: move this into geometry PointExt
                let p1 = Point2D::<f32, WorldSpace>::new(p.x as f32, p.y as f32);
                let p2 = Point2D::<f32, WorldSpace>::new(end.x as f32, end.y as f32);
                let distance = p1.distance_to(p2);
                distance.round() as i32
            },
            |p| p == end,
        )
        .map(|(result, _cost)| result)
    }

    // Return a Vec of all points surrounding another point
    pub fn neighbors(&self, point: &WorldPoint) -> Vec<(WorldPoint, i32)> {
        vec![
            Cardinal::SW,
            Cardinal::W,
            Cardinal::NW,
            Cardinal::N,
            Cardinal::NE,
            Cardinal::E,
            Cardinal::SE,
            Cardinal::S,
        ]
        .iter()
        .map(|vector| *point + *vector.to_vector())
        .filter(|p| !self.is_blocked(p))
        .map(|p| (p, 1))
        .collect()
    }

    pub fn contains(&self, point: WorldPoint) -> bool {
        self.rect.contains(point)
    }

    pub fn reset_content(&mut self) {
        for vec in self.content.iter_mut() {
            vec.clear();
        }
    }

    pub fn add_content(&mut self, point: &WorldPoint, entity: &Entity) {
        // TODO: point.to_index can panic if a point is negative
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        self.content[idx].push(*entity);
    }

    pub fn get_content(&self, point: &WorldPoint) -> &Vec<Entity> {
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        &self.content[idx]
    }

    pub fn reset_blocked(&mut self) {
        self.blocked.clear()
    }

    pub fn set_blocked(&mut self, point: &WorldPoint) {
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        self.blocked.insert(point.to_index(self.get_width()))
    }

    pub fn is_blocked(&self, point: &WorldPoint) -> bool {
        // If it's not in the rect, it's blocked
        if self.rect.contains(*point) {
            let idx = point.to_index(self.get_width());
            self.assert_idx_for_point(idx, point);

            // If it's blocked via map bitset, it's blocked
            if self.blocked.contains(point.to_index(self.get_width())) {
                return true;
            }

            // If it's blocked via the tile, it's blocked
            let tile = &self.tiles[idx];
            if !tile.is_passable() {
                return true;
            }

            // If it's in the map rect and not otherwise blocked,
            // it is NOT blocked
            return false;
        }
        true
    }

    pub fn reset_visible(&mut self) {
        self.visible.clear()
    }

    pub fn set_visible(&mut self, point: &WorldPoint) {
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        self.visible.insert(idx)
    }

    pub fn is_visible(&self, point: &WorldPoint) -> bool {
        assert!(
            self.rect.contains(*point),
            "Point not in world! {:?}",
            &point
        );
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        self.visible.contains(idx)
    }

    pub fn reset_revealed(&mut self) {
        self.revealed.clear()
    }

    pub fn set_revealed(&mut self, point: &WorldPoint) {
        self.revealed.insert(point.to_index(self.get_width()))
    }

    pub fn is_revealed(&self, point: &WorldPoint) -> bool {
        self.revealed.contains(point.to_index(self.get_width()))
    }

    pub fn get_rect(&self) -> &Rect<i32, WorldSpace> {
        &self.rect
    }

    pub fn get_width(&self) -> i32 {
        self.rect.width()
    }

    pub fn get_height(&self) -> i32 {
        self.rect.height()
    }

    pub fn iter_points(&self) -> impl Iterator<Item = WorldPoint> {
        let xrange = self.rect.x_range();
        let yrange = self.rect.y_range();

        xrange.flat_map(move |x| yrange.clone().map(move |y| WorldPoint::new(x, y)))
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = (WorldPoint, &Tile)> {
        self.iter_points()
            .map(|point| (point, &self.tiles[point.to_index(self.get_width())]))
    }

    pub fn get(&self, point: WorldPoint) -> Option<&Tile> {
        if self.rect.contains(point) {
            let idx = point.to_index(self.get_width());
            Some(&self.tiles[idx])
        } else {
            None
        }
    }

    fn assert_idx_for_point(&self, idx: usize, point: &WorldPoint) {
        assert!(
            self.rect.contains(*point),
            "{:?} is not a point in Map rect {:?}, and idx would panic: {:?} (vec length: {:?})",
            point,
            self.rect,
            idx,
            self.tiles.len()
        );
    }

    pub fn is_opaque_point(&self, point: &WorldPoint) -> bool {
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        self.tiles[idx].is_opaque()
    }
}

impl Index<&WorldPoint> for Map {
    type Output = Tile;

    fn index(&self, point: &WorldPoint) -> &Self::Output {
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        &self.tiles[idx]
    }
}

impl IndexMut<&WorldPoint> for Map {
    fn index_mut(&mut self, point: &WorldPoint) -> &mut Self::Output {
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        &mut self.tiles[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let map = &mut Map::init(
            String::from("test"),
            WorldSize::new(50, 50),
            Tile::Floor(FloorKind::FloorDefault),
        );

        // Check that we can traverse the entire map rect
        for x in map.rect.x_range() {
            for y in map.rect.y_range() {
                let point = &WorldPoint::new(x, y);
                map[point] = Tile::Floor(FloorKind::default());
                map.set_blocked(point);
            }
        }

        assert_eq!(map.rect.max(), WorldPoint::new(50, 50));

        // TODO: assert panic here
        // assert!(
        //     !map.is_blocked(&WorldPoint::new(50, 50)),
        //     "expect the bottom right edge to be unvisited"
        assert!(
            map.is_blocked(&WorldPoint::new(49, 49)),
            "expect (49, 49) to be the actual maximum usable point"
        );
    }

    #[test]
    fn path_open() {
        let map = Map::new(
            String::from("test"),
            5,
            5,
            vec![Tile::Floor(FloorKind::default()); 25],
        );

        let start = WorldPoint::new(0, 0);
        let end = WorldPoint::new(4, 0);

        let path = map
            .astar_path(&start, &end)
            .expect("expecting a valid path");

        assert_eq!(path.len(), 5);
    }

    #[test]
    fn path_closed() {
        let mut tiles = vec![Tile::Floor(FloorKind::default()); 25];

        // Create a wall dividing the square down the middle
        for y in 0..3 {
            let point = WorldPoint::new(2, y);
            tiles[point.to_index(5)] = Tile::Wall(WallKind::default());
        }

        let map = Map::new(String::from("test"), 5, 5, tiles);

        let start = WorldPoint::new(0, 0);
        let end = WorldPoint::new(4, 0);

        assert_eq!(map[&WorldPoint::new(2, 0)], Tile::Wall(WallKind::default()));
        assert!(!map[&WorldPoint::new(2, 0)].is_passable());
        assert!(map.is_blocked(&WorldPoint::new(2, 0)));
        assert!(map.is_blocked(&WorldPoint::new(2, 1)));
        assert!(map.is_blocked(&WorldPoint::new(2, 2)));

        let path = map.astar_path(&start, &end).expect("path");

        assert_eq!(path.len(), 7);
    }

    #[test]
    fn test_wall_bitset() {
        let w = Tile::Wall(WallKind::default());
        let f = Tile::Floor(FloorKind::default());

        #[allow(clippy::redundant_clone)]
        #[rustfmt::skip]
        let tiles: Vec<Tile> = vec![
            w, w, w, w, w,
            w, f, f, f, w,
            w, f, w, f, w,
            w, f, f, f, w,
            w, w, w, w, w,
        ];

        let mut map = Map::new(String::from("test"), 5, 5, tiles);

        let points: Vec<_> = map.iter_points().collect();
        for point in points {
            if let Some(Tile::Wall(_)) = map.get(point) {
                map[&point] = Tile::Wall(WallKind::from_map_position(&map, point));
            }
        }

        assert_eq!(
            map[&WorldPoint::new(0, 0)],
            Tile::Wall(WallKind::WallSE),
            "0, 0"
        );
        assert_eq!(
            map[&WorldPoint::new(1, 0)],
            Tile::Wall(WallKind::WallEW),
            "1, 0"
        );
        assert_eq!(
            map[&WorldPoint::new(2, 0)],
            Tile::Wall(WallKind::WallEW),
            "2, 0"
        );
        assert_eq!(
            map[&WorldPoint::new(3, 0)],
            Tile::Wall(WallKind::WallEW),
            "3, 0"
        );
        assert_eq!(
            map[&WorldPoint::new(4, 0)],
            Tile::Wall(WallKind::WallSW),
            "4, 0"
        );
        assert_eq!(
            map[&WorldPoint::new(0, 1)],
            Tile::Wall(WallKind::WallNS),
            "0, 1"
        );
        assert_eq!(
            map[&WorldPoint::new(4, 1)],
            Tile::Wall(WallKind::WallNS),
            "4, 1"
        );
        assert_eq!(
            map[&WorldPoint::new(0, 2)],
            Tile::Wall(WallKind::WallNS),
            "0, 2"
        );
        assert_eq!(
            map[&WorldPoint::new(2, 2)],
            Tile::Wall(WallKind::WallPillar),
            "2, 2"
        );
        assert_eq!(
            map[&WorldPoint::new(4, 2)],
            Tile::Wall(WallKind::WallNS),
            "4, 2"
        );
    }

    #[test]
    fn test_wall_bitset_all_sides() {
        let w = Tile::Wall(WallKind::default());

        #[allow(clippy::redundant_clone)]
        #[rustfmt::skip]
        let tiles: Vec<Tile> = vec![
            w, w, w,
            w, w, w,
            w, w, w,
        ];

        let mut map = Map::new(String::from("test"), 3, 3, tiles);

        let points: Vec<_> = map.iter_points().collect();
        for point in points {
            if let Some(Tile::Wall(_)) = map.get(point) {
                map[&point] = Tile::Wall(WallKind::from_map_position(&map, point));
            }
        }

        assert_eq!(
            map[&WorldPoint::new(0, 0)],
            Tile::Wall(WallKind::WallSE),
            "0, 0"
        );
        assert_eq!(
            map[&WorldPoint::new(1, 0)],
            Tile::Wall(WallKind::WallEWS),
            "1, 0"
        );
        assert_eq!(
            map[&WorldPoint::new(2, 0)],
            Tile::Wall(WallKind::WallSW),
            "2, 0"
        );
        assert_eq!(
            map[&WorldPoint::new(0, 1)],
            Tile::Wall(WallKind::WallNSE),
            "0, 1"
        );
        assert_eq!(
            map[&WorldPoint::new(1, 1)],
            Tile::Wall(WallKind::WallAllSides)
        );
        assert_eq!(map[&WorldPoint::new(2, 1)], Tile::Wall(WallKind::WallNSW));
    }
}
