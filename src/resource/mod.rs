// TODO: deprecate use of bracket rng
use bracket_random::prelude::RandomNumberGenerator;

mod viewport;
pub use viewport::*;

use crate::util::{BitmapFont, SpriteSheet, WorldSpace};

pub struct Resources {
    pub rng: RandomNumberGenerator,
    pub viewport: Viewport<WorldSpace>,
    pub font: BitmapFont,
    pub spritesheet: SpriteSheet,
}
