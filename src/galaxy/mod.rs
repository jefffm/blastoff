use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    overworld::Overworld,
    util::{GalaxyPoint, GalaxySize},
};

type GalaxyMap = HashMap<GalaxyPoint, Overworld>;

/// A galaxy contains coordinates pointing to each star system
// #[derive(Serialize, Deserialize)]
pub struct Galaxy {
    info: GalaxyInfo,
    map: GalaxyMap,
}

impl Galaxy {
    pub fn from_size(info: GalaxyInfo) -> Self {
        let map = HashMap::with_capacity(info.size.area() as usize);

        Self::new(info, map)
    }

    pub fn with_planets(mut self, planets: Vec<(GalaxyPoint, Overworld)>) -> Self {
        for (point, planet) in planets.into_iter() {
            self.map.insert(point, planet);
        }

        self
    }

    pub fn new(info: GalaxyInfo, map: GalaxyMap) -> Self {
        Self { info, map }
    }

    pub fn info(&self) -> &GalaxyInfo {
        &self.info
    }

    pub fn iter_content(&self) -> impl Iterator<Item = (&GalaxyPoint, &Overworld)> {
        self.map.iter()
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
