use assets_manager::{AssetCache, Compound, Handle};
use bracket_random::prelude::RandomNumberGenerator;

mod viewport;
use macroquad::texture::{load_texture, FilterMode};
pub use viewport::*;

mod assets;
pub use assets::*;

use crate::{
    data::{BitmapFont, Tileset},
    game::consts,
    util::{
        PixelSize, SpriteSize, ViewportPoint, ViewportRect, ViewportSize, WorldSpace,
        WorldToViewport,
    },
};

pub struct Resources {
    pub rng: RandomNumberGenerator,
    pub viewport: Viewport<WorldSpace>,

    // TODO: deprecate AssetCache (not wasm/android compatible)
    pub assets_cache: AssetCache,
    pub assets: Assets,
}

impl Resources {
    pub async fn try_new(rng_seed: u64, assets_cache: AssetCache) -> anyhow::Result<Self> {
        let rng = RandomNumberGenerator::seeded(rng_seed);
        let viewport = Viewport::new(
            ViewportRect::new(
                ViewportPoint::new(0, 0),
                ViewportSize::new(consts::VIEWPORT_WIDTH, consts::VIEWPORT_HEIGHT),
            ),
            WorldToViewport::default(),
        );

        tracing::info!("Loading Tileset");
        let tileset = Tileset::try_from_file("tileset/tileset_transparent.tsx").await?;

        tracing::info!("Loading Font");
        let monospace_font_texture = load_texture("fonts/zx_evolution_8x8.png").await?;
        monospace_font_texture.set_filter(FilterMode::Nearest);
        let monospace_font = BitmapFont::new(
            monospace_font_texture,
            PixelSize::new(8, 8),
            SpriteSize::new(16, 16),
        );

        let assets = Assets {
            tileset,
            monospace_font,
        };

        Ok(Self {
            rng,
            viewport,
            assets_cache,
            assets,
        })
    }

    pub fn load_asset<T: Compound>(&self, id: &str) -> Handle<T> {
        self.assets_cache
            .load::<T>(id)
            .unwrap_or_else(|err| panic!("asset load from {:?}: {}", id, err))
    }
}
