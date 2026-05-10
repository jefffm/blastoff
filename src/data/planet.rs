use std::borrow::Cow;

use assets_manager::{asset::load_yaml, BoxedError, FileAsset};
use serde::{Deserialize, Serialize};

use super::{ProbabilityDistribution, SectorTypeProbability};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub enum PlanetType {
    /// Mostly empty space and not much life
    Barren,
    /// Mountains, Forests, Rivers, and Oceans
    Lush,
    /// Mostly liquid with some islands
    Aqueus,
    /// Mostly impassible mountains with winding paths and caves
    Mountains,
}

pub type PlanetTypeProbability = ProbabilityDistribution<PlanetType>;

/// Probability definitions for which sectors to create on an overworld
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetProbability {
    sector_type: SectorTypeProbability,
}

impl FileAsset for PlanetProbability {
    const EXTENSION: &'static str = "yaml";

    fn from_bytes(bytes: Cow<[u8]>) -> Result<Self, BoxedError> {
        load_yaml(&bytes)
    }
}
