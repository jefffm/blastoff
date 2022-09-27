use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{ContextBuilder, GameResult};
use std::{env, path};
use tracing::info;
use tracing::Level;

use bracket_random::prelude::RandomNumberGenerator;

pub mod camera;
pub mod color;
pub mod component;
pub mod data;
pub mod game;
pub mod input;
pub mod map;
pub mod resource;
pub mod scene;
pub mod system;
pub mod util;

use game::consts;
use resource::{Resources, Viewport};
use scene::MainState;
use util::{ViewportPoint, ViewportRect, ViewportSize, WorldToViewport};

use clap::Parser;
use rand::RngCore;

use crate::game::consts::SCALING_FACTOR;
use crate::game::consts::SCREEN_HEIGHT_PIXELS;
use crate::game::consts::SCREEN_WIDTH_PIXELS;
use crate::game::consts::TITLE_HEADER;
use crate::util::BitmapFont;
use crate::util::SpriteSheet;
use crate::util::SpriteSize;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[clap(short, long, action, default_value_t = false)]
    debug: bool,
}

fn main() -> GameResult {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2.. => Level::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    // TODO: add resources path using cargo manifest dir https://github.com/joetsoi/OpenMoonstone/blob/master/rust/src/main.rs#L108-L113
    let mut builder = ContextBuilder::new("roguemon", "Jeff Lynn");
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let path = path::PathBuf::from(manifest_dir).join(consts::RESOURCE_PATH);
        tracing::info!("Adding 'resources' path {:?}", path);
        builder = builder.add_resource_path(path);
    }
    let (mut ctx, event_loop) = builder
        .window_setup(conf::WindowSetup::default().title(TITLE_HEADER).vsync(true))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(
                    SCREEN_WIDTH_PIXELS as f32 * SCALING_FACTOR,
                    SCREEN_HEIGHT_PIXELS as f32 * SCALING_FACTOR,
                )
                .fullscreen_type(conf::FullscreenType::Windowed),
        )
        .build()
        .expect("aieee, could not create ggez context!");

    // TODO: add seed parameter to CLI
    let rng_seed = game::env()
        .seed
        .unwrap_or_else(|| rand::thread_rng().next_u64());

    info!("using rng seed: {}", rng_seed);

    info!("linking resources");
    // TODO: load sprite sheet like https://github.com/ggez/ggez/blob/0.8.0-rc0/examples/animation.rs#L237

    info!("creating GameState");
    let font_image = graphics::Image::from_path(&ctx, "/fonts/rex_16x16.png").expect("load font");
    let font = BitmapFont::from_grid(&ctx, font_image, &SpriteSize::new(16, 16));

    let spritesheet_image = graphics::Image::from_path(&ctx, "/tileset/colored-transparent.png")
        .expect("load spritesheet");
    let spritesheet = SpriteSheet::from_grid(&ctx, spritesheet_image, SpriteSize::new(49, 22));

    let resources = Resources {
        rng: RandomNumberGenerator::seeded(rng_seed),
        viewport: Viewport::new(
            ViewportRect::new(
                ViewportPoint::new(0, 0),
                ViewportSize::new(consts::VIEWPORT_WIDTH, consts::VIEWPORT_HEIGHT),
            ),
            WorldToViewport::default(),
        ),
        font,
        spritesheet,
    };

    let mut game = MainState::new(resources, &mut ctx);

    // Push an initial scene to the SceneStack and prepare it for playing
    if cli.debug {
        game.init_debug()
    } else {
        game.init();
    }

    info!("starting main_loop");

    event::run(ctx, event_loop, game)
}
