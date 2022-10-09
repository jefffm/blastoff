use std::collections::BTreeMap;

use rand::Rng;
use rand_distr::{Distribution, WeightedAliasIndex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbabilityDistribution<T: Ord + Clone> {
    items: BTreeMap<T, f32>,
}

impl<T> ProbabilityDistribution<T>
where
    T: Ord + Clone,
{
    pub fn new(items: BTreeMap<T, f32>) -> Self {
        Self { items }
    }

    pub fn next_element<R: Rng>(&self, rng: &mut R) -> T {
        let dist =
            WeightedAliasIndex::new(self.items.iter().map(|item| *(item.1)).collect()).unwrap();

        // Slice the BTreeMap like a vec using the distribution
        // Not ideal for performance, but this is called once AND this makes our ser/deser format much simpler
        self.items
            .iter()
            .nth([dist.sample::<R>(rng)][0])
            .expect("weighted distribution Element selection")
            .0
            .clone()
    }
}
