use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bracket_random::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    data::GalaxyProbability,
    overworld::{Overworld, PlanetInfo},
    util::{GalaxyPoint, GalaxySize},
};

pub type PlanetInfoVec = Vec<(GalaxyPoint, PlanetInfo)>;
pub type PlanetMap = HashMap<GalaxyPoint, Rc<RefCell<Overworld>>>;

/// A galaxy contains coordinates pointing to each star system
// #[derive(Serialize, Deserialize)]
pub struct Galaxy {
    info: GalaxyInfo,
    planet_infos: PlanetInfoVec,
    planet_map: PlanetMap,
}

/// Galaxy contains a list of different planets
///
/// The planets aren't indexed, so searching for a planet is linear/not-constant
/// time. We're talking probably 10 or so planets, so it's fine. Just use
/// [`Self::iter_content`] to search.
impl Galaxy {
    pub fn from_size(info: GalaxyInfo) -> Self {
        let planets = Vec::new();
        let planet_map = HashMap::new();

        Self::new(info, planets, planet_map)
    }

    pub fn new(info: GalaxyInfo, planets: PlanetInfoVec, planet_map: PlanetMap) -> Self {
        Self {
            info,
            planet_infos: planets,
            planet_map,
        }
    }

    pub fn with_planet_infos(mut self, planets: PlanetInfoVec) -> Self {
        self.planet_infos.extend(planets);
        self
    }

    pub fn info(&self) -> &GalaxyInfo {
        &self.info
    }

    /// The entire galaxy is always populated with all possible points and their
    /// corresponding PlanetInfos
    pub fn iter_planet_infos(&self) -> impl Iterator<Item = &(GalaxyPoint, PlanetInfo)> {
        self.planet_infos.iter()
    }

    pub fn get_planet(&self, point: &GalaxyPoint) -> Option<Rc<RefCell<Overworld>>> {
        self.planet_map.get(point).map(|planet| planet.clone())
    }

    pub fn iter_planets(&self) -> impl Iterator<Item = (&GalaxyPoint, &Rc<RefCell<Overworld>>)> {
        self.planet_map.iter()
    }

    pub fn find(&self, point: &GalaxyPoint) -> Option<PlanetInfo> {
        self.planet_infos
            .iter()
            .find(|(p, _)| p == point)
            .map(|(_, overworld)| overworld.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyInfo {
    name: String,
    size: GalaxySize,
    probability: GalaxyProbability,
}

impl GalaxyInfo {
    pub fn new(name: String, size: GalaxySize, probability: GalaxyProbability) -> Self {
        Self {
            name,
            size,
            probability,
        }
    }

    pub fn width(&self) -> i32 {
        self.size.width
    }

    pub fn height(&self) -> i32 {
        self.size.height
    }
}
