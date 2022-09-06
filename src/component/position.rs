use bracket_lib::prelude::*;

#[derive(Debug, Clone)]
pub struct Position {
    position: Point,
}

impl Position {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn get_x(&self) -> i32 {
        self.position.x
    }

    pub fn get_y(&self) -> i32 {
        self.position.y
    }
}
