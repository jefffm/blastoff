use crate::util::WorldPoint;

#[derive(Debug, Default)]
pub struct Viewshed {
    visible_tiles: Vec<WorldPoint>,
    range: i32,
    dirty: bool,
}

impl Viewshed {
    pub fn init(&mut self) {
        self.set_dirty();
        self.visible_tiles.clear();
    }

    pub fn insert(&mut self, point: WorldPoint) {
        self.visible_tiles.push(point)
    }

    pub fn set(&mut self, points: Vec<WorldPoint>) {
        self.visible_tiles = points;
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn range(&self) -> i32 {
        self.range
    }

    pub fn points(&self) -> impl Iterator<Item = &WorldPoint> {
        self.visible_tiles.iter()
    }

    pub fn with_range(mut self, range: i32) -> Self {
        self.range = range;
        self
    }

    pub fn with_init(mut self) -> Self {
        self.init();
        self
    }

    pub fn contains(&self, point: &WorldPoint) -> bool {
        self.points().any(|tile| tile == point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewshed() {
        let mut viewshed = Viewshed::default();
        viewshed.init();

        assert!(!viewshed.contains(&WorldPoint::new(6, 14)));

        let points: Vec<_> = [
            (6, 14),
            (8, 13),
            (11, 22),
            (11, 11),
            (13, 9),
            (10, 20),
            (10, 13),
            (14, 11),
            (10, 14),
            (13, 7),
            (9, 17),
            (9, 11),
            (7, 4),
        ]
        .into_iter()
        .map(move |(x, y)| WorldPoint::new(x, y))
        .collect();

        viewshed.set(points);

        assert!(viewshed.contains(&WorldPoint::new(6, 14)));
    }
}
