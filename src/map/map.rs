use bracket_lib::prelude::{Algorithm2D, BaseMap, Point};
use euclid::{Rect, Size2D};
use fixedbitset::FixedBitSet;
use hecs::Entity;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    ops::{Index, IndexMut},
};

use crate::util::{PointExt, WorldPoint, WorldSize, WorldSpace};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Map {
    name: String,
    tiles: Vec<TileKind>,
    rect: Rect<i32, WorldSpace>,
    blocked: FixedBitSet,
    revealed: FixedBitSet,
    visible: FixedBitSet,
    content: Vec<Vec<Entity>>,
    level: u32,
}

impl Map {
    /// Create a map with a given size filled in with Wall tiles
    pub fn init(name: String, size: WorldSize, level: u32) -> Self {
        let tiles = vec![TileKind::Wall; (size.height * size.width) as usize];
        Self::new(name, size.width, size.height, tiles, level)
    }

    pub fn new(name: String, width: i32, height: i32, tiles: Vec<TileKind>, level: u32) -> Self {
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
            level,
        }
    }

    pub fn contains(&self, point: WorldPoint) -> bool {
        self.rect.contains(point)
    }

    pub fn reset_content(&mut self) {
        self.content =
            vec![Vec::<Entity>::new(); (self.get_height() * self.get_width()).try_into().unwrap()];
    }

    pub fn add_content(&mut self, point: &WorldPoint, entity: &Entity) {
        // TODO: point.to_index can panic if a point is negative
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        self.content[idx].push(*entity);
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
        let idx = point.to_index(self.get_width());
        self.assert_idx_for_point(idx, point);

        self.blocked.contains(point.to_index(self.get_width()))
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

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = (WorldPoint, &TileKind)> {
        let xrange = self.rect.x_range();
        let yrange = self.rect.y_range();

        xrange.flat_map(move |x| {
            yrange.clone().map(move |y| {
                let point = WorldPoint::new(x, y);
                (point, &self.tiles[point.to_index(self.get_width())])
            })
        })
    }

    pub fn get(&self, point: WorldPoint) -> Option<&TileKind> {
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
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].handler().is_opaque()
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.get_width(), self.get_height())
    }
}

impl Index<&WorldPoint> for Map {
    type Output = TileKind;

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
        let map = &mut Map::init(String::from("test"), WorldSize::new(50, 50), 1);

        // Check that we can traverse the entire map rect
        for x in map.rect.x_range() {
            for y in map.rect.y_range() {
                let point = &WorldPoint::new(x, y);
                map[point] = TileKind::Floor;
                map.set_blocked(point);
            }
        }

        assert_eq!(map.rect.max(), WorldPoint::new(50, 50));
        assert!(
            !map.is_blocked(&WorldPoint::new(50, 50)),
            "expect the bottom right edge to be unvisited"
        );
        assert!(
            map.is_blocked(&WorldPoint::new(49, 49)),
            "expect (49, 49) to be the actual maximum usable point"
        );
    }
}
