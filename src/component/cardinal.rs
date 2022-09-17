use crate::util::WorldVector;

static SW: WorldVector = WorldVector::new(-1, 1);
static W: WorldVector = WorldVector::new(-1, 0);
static NW: WorldVector = WorldVector::new(-1, -1);
static N: WorldVector = WorldVector::new(0, -1);
static NE: WorldVector = WorldVector::new(1, -1);
static E: WorldVector = WorldVector::new(1, 0);
static SE: WorldVector = WorldVector::new(1, 1);
static S: WorldVector = WorldVector::new(0, 1);

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
    pub fn to_vector(&self) -> &'static WorldVector {
        match self {
            Cardinal::SW => &SW,
            Cardinal::W => &W,
            Cardinal::NW => &NW,
            Cardinal::N => &N,
            Cardinal::NE => &NE,
            Cardinal::E => &E,
            Cardinal::SE => &SE,
            Cardinal::S => &S,
        }
    }

    pub fn inv(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::S => Self::N,
            Self::E => Self::W,
            Self::W => Self::E,
            Self::NE => Self::SW,
            Self::NW => Self::SE,
            Self::SW => Self::NE,
            Self::SE => Self::NW,
        }
    }
}
