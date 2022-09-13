use crate::util::WorldVector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cardinal {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}
impl Cardinal {
    pub fn to_vector(&self) -> WorldVector {
        match self {
            Cardinal::SW => WorldVector::new(-1.0, 1.0),
            Cardinal::W => WorldVector::new(-1.0, 0.0),
            Cardinal::NW => WorldVector::new(-1.0, -1.0),
            Cardinal::N => WorldVector::new(0.0, -1.0),
            Cardinal::NE => WorldVector::new(1.0, -1.0),
            Cardinal::E => WorldVector::new(1.0, 0.0),
            Cardinal::SE => WorldVector::new(1.0, 1.0),
            Cardinal::S => WorldVector::new(0.0, 1.0),
        }
    }
    pub fn inv(&self) -> Cardinal {
        match self {
            Cardinal::N => Cardinal::S,
            Cardinal::S => Cardinal::N,
            Cardinal::E => Cardinal::W,
            Cardinal::W => Cardinal::E,
            Cardinal::NE => Cardinal::SW,
            Cardinal::NW => Cardinal::SE,
            Cardinal::SW => Cardinal::NE,
            Cardinal::SE => Cardinal::NW,
        }
    }
}
