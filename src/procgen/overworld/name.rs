use crate::{data::MarkovSeed, resource::Resources};

pub fn generate_planet_name(resources: &mut Resources) -> String {
    let seed = resources
        .load_asset::<MarkovSeed>("markov.planet")
        .read()
        .as_str()
        .to_owned();
    let seed_lines: Vec<_> = seed.lines().collect();

    resources
        .rng
        .random_slice_entry(&seed_lines)
        .unwrap()
        .to_string()
}
