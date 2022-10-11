use assets_manager::{loader::YamlLoader, Asset};
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

impl Asset for PlanetProbability {
    const EXTENSION: &'static str = "yaml";
    const EXTENSIONS: &'static [&'static str] = &[Self::EXTENSION];

    type Loader = YamlLoader;

    const HOT_RELOADED: bool = true;
}
