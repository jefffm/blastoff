use bracket_lib::prelude::Point;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Vec2d<T> {
    vec: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> Vec2d<T> {
    pub fn new(vec: Vec<T>, height: usize, width: usize) -> Self {
        assert!(vec.len() == height * width);
        Self { vec, height, width }
    }

    pub fn row(&self, row: usize) -> &[T] {
        let i = self.width * row;
        &self.vec[i..(i + self.width)]
    }

    pub fn insert(&mut self, point: Point, item: T) {
        let idx = point.to_index(self.width);
        self.vec[idx] = item
    }

    pub fn values(&self) -> impl Iterator<Item = Point> {
        let xrange = 0..self.width;
        let yrange = 0..self.height;
        xrange.flat_map(move |x| {
            yrange
                .clone()
                .map(move |y| Point::new(x.clone(), y.clone()))
        })
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.height {
            if i != 0 {
                str.push_str(", ");
            }
            str.push_str(&format!("{:?}", &self.row(i)));
        }
        write!(f, "[{}]", str)
    }
}

impl<T> Index<Point> for Vec2d<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        let idx = point.to_index(self.width);
        &self.vec[idx]
    }
}

impl<T> IndexMut<Point> for Vec2d<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let idx = point.to_index(self.width);
        &mut self.vec[idx]
    }
}
