use assets_manager::{loader::YamlLoader, Asset};
use bracket_random::prelude::DiceType;
use serde::{Deserialize, Serialize};

use super::{Element, ElementProbability};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyProbability {
    planet_count: DiceType,
    planet_type: DiceType,
    planet_element: ElementProbability,
}

// TODO: move GalaxyProbabiliy Default into the yaml
impl Default for GalaxyProbability {
    fn default() -> Self {
        Self {
            planet_count: DiceType::new(6, 2, 0),
            planet_type: DiceType::new(6, 2, 0),
            planet_element: ElementProbability::new(vec![
                (Element::Water, 1.0),
                (Element::Fire, 2.0),
                (Element::Plant, 5.0),
            ]),
        }
    }
}

impl Asset for GalaxyProbability {
    const EXTENSION: &'static str = "yaml";
    const EXTENSIONS: &'static [&'static str] = &[Self::EXTENSION];

    type Loader = YamlLoader;

    const HOT_RELOADED: bool = true;
}

impl GalaxyProbability {
    // pub fn roll_galaxy(&self) -> Vec<PlanetInfo> {
    //     0..planet
    // }
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

        assert_eq!(galaxy_probability.planet_count, DiceType::new(6, 2, 0));
    }
}
