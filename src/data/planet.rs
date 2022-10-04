use serde::{Deserialize, Serialize};

use super::EnumProbability;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub enum PlanetType {
    Barren,
    Ice,
    Terran,
    Water,
    Plant,
    Fire,
}

pub type PlanetTypeProbability = EnumProbability<PlanetType>;
