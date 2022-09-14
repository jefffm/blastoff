use crate::util::WorldPoint;

#[derive(Debug, Default)]
pub struct Viewshed {
    visible_tiles: Vec<WorldPoint>,
    range: i32,
    dirty: bool,
}

impl Viewshed {
    pub fn init(&mut self) {
        self.dirty = false;
        self.visible_tiles.clear();
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
}
