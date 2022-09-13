
use euclid::{Box2D};

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

    // TODO: use Euler clamp instead
    pub fn clamp(&mut self, rect: &Box2D<i32, WorldSpace>) {
        self.p.x = self.p.x.max(rect.min.x).min(rect.max.x);
        self.p.y = self.p.y.max(rect.min.y).min(rect.max.y);
    }
}
