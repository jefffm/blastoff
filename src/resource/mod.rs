// TODO: deprecate use of bracket rng
use bracket_random::prelude::RandomNumberGenerator;

mod viewport;
pub use viewport::*;

use crate::util::{BitmapFont, SpriteSheet};

pub struct Resources {
    pub rng: RandomNumberGenerator,
    pub viewport: Viewport,
    pub font: BitmapFont,
    pub spritesheet: SpriteSheet,
}
