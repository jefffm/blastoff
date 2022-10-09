use serde::{Deserialize, Serialize};

use super::ProbabilityDistribution;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub enum Element {
    Water,
    Fire,
    Plant,
}

pub type ElementProbability = ProbabilityDistribution<Element>;
