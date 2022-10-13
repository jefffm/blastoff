use assets_manager::{AssetCache, Compound, Handle};
use bracket_random::prelude::RandomNumberGenerator;

mod viewport;
use ggez::{context::Has, graphics::GraphicsContext, GameError};
pub use viewport::*;

use crate::{
    data::Image,
    game::consts,
    util::{
        BitmapFont, SpriteSheet, SpriteSize, ViewportPoint, ViewportRect, ViewportSize, WorldSpace,
        WorldToViewport,
    },
};

pub struct Resources {
    pub rng: RandomNumberGenerator,
    pub viewport: Viewport<WorldSpace>,
    pub font: BitmapFont,
    pub spritesheet: SpriteSheet,
    pub assets: AssetCache,
}

impl Resources {
    pub fn try_new(rng_seed: u64, assets: AssetCache) -> Result<Self, GameError> {
        let rng = RandomNumberGenerator::seeded(rng_seed);
        let viewport = Viewport::new(
            ViewportRect::new(
                ViewportPoint::new(0, 0),
                ViewportSize::new(consts::VIEWPORT_WIDTH, consts::VIEWPORT_HEIGHT),
            ),
            WorldToViewport::default(),
        );

        let font_image = assets
            .load::<Image>("fonts.rex_16x16")
            .map_err(|err| GameError::ResourceLoadError(err.to_string()))?
            .read();
        let font = BitmapFont::from_grid(ctx, font_image.to_image(ctx), &SpriteSize::new(16, 16));

        // TODO: Spritesheet Definitions should be configured via yaml or something
        let spritesheet_image = assets
            .load::<Image>("tileset.monochrome-transparent")
            .map_err(|err| GameError::ResourceLoadError(err.to_string()))?
            .read();
        let spritesheet = SpriteSheet::from_grid(
            ctx,
            spritesheet_image.to_image(ctx),
            SpriteSize::new(49, 22),
        );

        Ok(Self {
            rng,
            viewport,
            font,
            spritesheet,
            assets,
        })
    }
    pub fn load_asset<T: Compound>(&self, id: &str) -> Handle<T> {
        self.assets
            .load::<T>(id)
            .unwrap_or_else(|err| panic!("asset load from {:?}: {}", id, err))
    }
}
