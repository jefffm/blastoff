use rand::Rng;
use rand_distr::{Distribution, WeightedAliasIndex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Element {
    Water,
    Fire,
    Plant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementProbability {
    items: Vec<(Element, f32)>,
}

impl ElementProbability {
    pub fn new(mut items: Vec<(Element, f32)>) -> Self {
        items.dedup_by_key(|(k, _)| *k);
        Self { items }
    }

    pub fn next_element<R: Rng>(&self, rng: &mut R) -> Element {
        let dist = WeightedAliasIndex::new(self.items.iter().map(|item| item.1).collect()).unwrap();
        self.items[dist.sample::<R>(rng)].0
    }
}
