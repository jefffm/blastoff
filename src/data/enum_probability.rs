use std::collections::BTreeMap;

use rand::Rng;
use rand_distr::{Distribution, WeightedAliasIndex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumProbability<Enum: Ord + Copy> {
    items: BTreeMap<Enum, f32>,
}

impl<Enum> EnumProbability<Enum>
where
    Enum: Ord + Copy,
{
    pub fn new(items: BTreeMap<Enum, f32>) -> Self {
        Self { items }
    }

    pub fn next_element<R: Rng>(&self, rng: &mut R) -> Enum {
        let dist =
            WeightedAliasIndex::new(self.items.iter().map(|item| *item.1).collect()).unwrap();

        // Slice the BTreeMap like a vec using the distribution
        // Not ideal for performance, but this is called once AND this makes our ser/deser format much simpler
        *self
            .items
            .iter()
            .nth([dist.sample::<R>(rng)][0])
            .expect("weighted distribution Element selection")
            .0
    }
}
