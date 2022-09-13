use euclid::Box2D;

use crate::util::{WorldPoint, WorldSpace, WorldVector};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub p: WorldPoint,
}

impl Position {
    pub fn new(point: WorldPoint) -> Self {
        Self { p: point }
    }

    pub fn move_by(&mut self, vector: WorldVector) {
        self.p += vector;
    }

    pub fn move_to(&mut self, point: WorldPoint) {
        self.p = point;
    }
}
