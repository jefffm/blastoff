use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{overworld::Overworld, util::GalaxyPoint};

type GalaxyMap = HashMap<GalaxyPoint, Overworld>;

/// A galaxy contains coordinates pointing to each star system
// #[derive(Serialize, Deserialize)]
pub struct Galaxy {
    info: GalaxyInfo,
    map: GalaxyMap,
}

impl Galaxy {
    pub fn new(info: GalaxyInfo, map: GalaxyMap) -> Self {
        Self { info, map }
    }

    pub fn info(&self) -> &GalaxyInfo {
        &self.info
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyInfo {
    name: String,
}

impl GalaxyInfo {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
