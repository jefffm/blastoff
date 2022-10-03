use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    overworld::Overworld,
    util::{GalaxyPoint, GalaxySize},
};

type PlanetsVec = Vec<(GalaxyPoint, Rc<RefCell<Overworld>>)>;

/// A galaxy contains coordinates pointing to each star system
// #[derive(Serialize, Deserialize)]
pub struct Galaxy {
    info: GalaxyInfo,
    planets: PlanetsVec,
}

/// Galaxy contains a list of different planets
///
/// The planets aren't indexed, so searching for a planet is linear/not-constant
/// time. We're talking probably 10 or so planets, so it's fine. Just use
/// [`Self::iter_content`] to search.
impl Galaxy {
    pub fn from_size(info: GalaxyInfo) -> Self {
        let planets = Vec::new();

        Self::new(info, planets)
    }

    pub fn with_planets(mut self, planets: PlanetsVec) -> Self {
        self.planets.extend(planets);
        self
    }

    pub fn new(info: GalaxyInfo, planets: PlanetsVec) -> Self {
        Self { info, planets }
    }

    pub fn info(&self) -> &GalaxyInfo {
        &self.info
    }

    pub fn iter_content(&self) -> impl Iterator<Item = &(GalaxyPoint, Rc<RefCell<Overworld>>)> {
        self.planets.iter()
    }

    pub fn find(&self, point: &GalaxyPoint) -> Option<Rc<RefCell<Overworld>>> {
        self.planets
            .iter()
            .find(|(p, _)| p == point)
            .map(|(_, overworld)| overworld.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyInfo {
    name: String,
    size: GalaxySize,
}

impl GalaxyInfo {
    pub fn new(name: String, size: GalaxySize) -> Self {
        Self { name, size }
    }

    pub fn width(&self) -> i32 {
        self.size.width
    }

    pub fn height(&self) -> i32 {
        self.size.height
    }
}
