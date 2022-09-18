use bracket_lib::random::RandomNumberGenerator;
use hecs::World;

use super::Map;

pub trait Spawner {
    fn spawn(&self, map: &Map, world: &mut World, rng: &mut RandomNumberGenerator);
}
