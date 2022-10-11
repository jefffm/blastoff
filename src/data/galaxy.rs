use assets_manager::{loader::YamlLoader, Asset};
use bracket_random::prelude::{DiceType, RandomNumberGenerator};

use serde::{Deserialize, Serialize};

use crate::{overworld::PlanetInfo, util::OverworldSize};

use super::{ElementProbability, PlanetTypeProbability, SectorProbability};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyProbability {
    planet_count: DiceType,
    planet_type: PlanetTypeProbability,
    planet_element: ElementProbability,
}

impl Asset for GalaxyProbability {
    const EXTENSION: &'static str = "yaml";
    const EXTENSIONS: &'static [&'static str] = &[Self::EXTENSION];

    type Loader = YamlLoader;

    const HOT_RELOADED: bool = true;
}

impl GalaxyProbability {
    pub fn roll_planet(&self, name: String, rng: &mut RandomNumberGenerator) -> PlanetInfo {
        let width = rng.roll_dice(3, 6);
        let height = rng.roll_dice(3, 6);
        let inner = rng.get_rng();

        let planet_type = self.planet_type.next_element(inner);
        let planet_element = self.planet_element.next_element(inner);

        let sector_probability = SectorProbability::new();

        PlanetInfo::new(
            name,
            OverworldSize::new(width, height),
            planet_type,
            planet_element,
            sector_probability,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path};

    use assets_manager::AssetCache;

    use crate::game::consts;

    use super::*;

    fn get_asset_cache() -> AssetCache {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = path::PathBuf::from(manifest_dir).join(consts::RESOURCE_PATH);
        assets_manager::AssetCache::new(path).unwrap()
    }

    #[test]
    fn test() {
        let cache = get_asset_cache();

        let galaxy_probability = cache
            .load::<GalaxyProbability>("data.galaxy_probability")
            .unwrap()
            .read();

        assert_eq!(galaxy_probability.planet_count, DiceType::new(6, 2, 2));
    }
}
