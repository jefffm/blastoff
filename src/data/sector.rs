use assets_manager::{loader::YamlLoader, Asset};
use serde::{Deserialize, Serialize};

use super::ProbabilityDistribution;

/// Each Sector is associated with an ElementType as well as a SectorType
/// SectorType is a generalization of each ElementType's tiles
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub enum SectorType {
    Barren,
    Civilization,
    ImpassibleTerrain,
    ImpassibleLiquid,
    PassibleLiquid,
}

/// Probability Distributions for each sector type on the Overworld for a given planet
/// Note that this may be used loosely during Overworld generation
/// (so as to make geographic features look nice)
pub type SectorTypeProbability = ProbabilityDistribution<SectorType>;

/// Probability definitions for what to create in each sector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorProbability {
    // TODO: not sure what is needed in sector probability (items? level distribution?)
}

impl Asset for SectorProbability {
    const EXTENSION: &'static str = "yaml";
    const EXTENSIONS: &'static [&'static str] = &[Self::EXTENSION];

    type Loader = YamlLoader;

    const HOT_RELOADED: bool = true;
}
