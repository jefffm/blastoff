use euclid::{Rect, Size2D};
use fixedbitset::FixedBitSet;
use hecs::Entity;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    ops::{Index, IndexMut},
};

use crate::{
    map::Tile,
    util::{PointExt, WorldPoint, WorldSpace},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Map {
    name: String,
    tiles: Vec<Tile>,
    rect: Rect<f32, WorldSpace>,
    blocked: FixedBitSet,
    revealed: FixedBitSet,
    visible: FixedBitSet,
    content: Vec<Vec<Entity>>,
    level: u32,
}

impl Map {
    pub fn new(name: String, width: f32, height: f32, tiles: Vec<Tile>, level: u32) -> Self {
        let rect = Rect::new(WorldPoint::new(0.0, 0.0), Size2D::new(width, height));
        let area = rect.size.area().ceil() as usize;

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

    pub fn reset_content(&mut self) {
        self.content =
            vec![Vec::<Entity>::new(); (self.get_height() * self.get_width()).ceil() as usize];
    }

    pub fn add_content(&mut self, point: &WorldPoint, entity: &Entity) {
        let idx = point.to_index(self.get_width());
        assert!(
            self.rect.contains(*point),
            "{:?} is not a point in Map rect {:?}, and idx would panic: {:?}",
            point,
            self.rect,
            idx
        );

        self.content[idx].push(*entity);
    }

    pub fn reset_blocked(&mut self) {
        self.blocked.clear()
    }

    pub fn set_blocked(&mut self, point: &WorldPoint) {
        self.blocked.insert(point.to_index(self.get_width()))
    }

    pub fn is_blocked(&self, point: &WorldPoint) -> bool {
        self.blocked.contains(point.to_index(self.get_width()))
    }

    pub fn reset_visible(&mut self) {
        self.visible.clear()
    }

    pub fn set_visible(&mut self, point: &WorldPoint) {
        self.visible.insert(point.to_index(self.get_width()))
    }

    pub fn is_visible(&self, point: &WorldPoint) -> bool {
        self.visible.contains(point.to_index(self.get_width()))
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

    pub fn get_rect(&self) -> &Rect<f32, WorldSpace> {
        &self.rect
    }

    pub fn get_width(&self) -> f32 {
        self.rect.width()
    }

    pub fn get_height(&self) -> f32 {
        self.rect.height()
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = (WorldPoint, &Tile)> {
        let min = self.rect.min();
        let max = self.rect.max();

        // TODO: this range seems like it could have problems
        let xrange = (min.x.floor() as u32)..(max.x.ceil() as u32);
        let yrange = (min.y.floor() as u32)..(max.y.ceil() as u32);

        xrange.flat_map(move |x| {
            yrange.clone().map(move |y| {
                let point = WorldPoint::new(x as f32, y as f32);
                (point, &self.tiles[point.to_index(self.get_width())])
            })
        })
    }
}

impl Index<&WorldPoint> for Map {
    type Output = Tile;

    fn index(&self, point: &WorldPoint) -> &Self::Output {
        &self.tiles[point.to_index(self.get_width())]
    }
}

impl IndexMut<&WorldPoint> for Map {
    fn index_mut(&mut self, point: &WorldPoint) -> &mut Self::Output {
        let idx = point.to_index(self.get_width());
        &mut self.tiles[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::map::TileKind;

    #[test]
    fn map_test() {
        // Create a 5x5 grid with the border surrounded by wall
        #[rustfmt::skip]
        let tiles = vec![
            TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(),
            TileKind::Wall.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Wall.into(),
            TileKind::Wall.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Wall.into(),
            TileKind::Wall.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Floor.into(), TileKind::Wall.into(),
            TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(), TileKind::Wall.into(),
        ];

        let map = Map::new(String::from("test"), 5.0, 5.0, tiles, 1);

        // Check that we can traverse the entire map rect
        for (point, tile) in map.iter_tiles() {
            map.is_blocked(&point);
        }
    }
}
