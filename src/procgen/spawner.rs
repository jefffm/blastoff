use bracket_random::prelude::RandomNumberGenerator;
use hecs::World;

use crate::sector::Map;

pub trait Spawner {
    fn spawn(&self, map: &Map, world: &mut World, rng: &mut RandomNumberGenerator);
}
