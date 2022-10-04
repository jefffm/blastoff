use assets_manager::{AssetCache, Compound, Handle};
use bracket_random::prelude::RandomNumberGenerator;

mod viewport;
pub use viewport::*;

use crate::util::{BitmapFont, SpriteSheet, WorldSpace};

pub struct Resources {
    pub rng: RandomNumberGenerator,
    pub viewport: Viewport<WorldSpace>,
    pub font: BitmapFont,
    pub spritesheet: SpriteSheet,
    pub assets: AssetCache,
}

impl Resources {
    pub fn load_asset<T: Compound>(&self, id: &str) -> Handle<T> {
        self.assets
            .load::<T>(id)
            .expect(&format!("asset load from {:?}", id))
    }
}
