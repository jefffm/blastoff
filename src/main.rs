use ggez::conf;
use ggez::event;

use ggez::GameError;
use ggez::{ContextBuilder, GameResult};
use std::{env, path};
use tracing::info;
use tracing::Level;

pub mod animation;
pub mod camera;
pub mod color;
pub mod component;
pub mod data;
pub mod galaxy;
pub mod game;
pub mod input;
pub mod overworld;
pub mod procgen;
pub mod resource;
pub mod scene;
pub mod sector;
pub mod system;
pub mod util;

use game::consts;
use resource::Resources;
use scene::MainState;

use clap::Parser;
use rand::RngCore;

use crate::game::consts::SCALING_FACTOR;
use crate::game::consts::SCREEN_HEIGHT_PIXELS;
use crate::game::consts::SCREEN_WIDTH_PIXELS;
use crate::game::consts::TITLE_HEADER;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[clap(short, long, action, default_value_t = false)]
    debug: bool,

    #[clap(long)]
    seed: Option<u64>,
}

fn main() -> GameResult {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2.. => Level::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    let builder = ContextBuilder::new("roguemon", "Jeff Lynn");
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

    let rng_seed = cli.seed.unwrap_or_else(|| rand::thread_rng().next_u64());
    info!("using rng seed: {}", rng_seed);

    info!("linking resources");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|err| GameError::FilesystemError(err.to_string()))?;

    let path = path::PathBuf::from(manifest_dir).join(consts::RESOURCE_PATH);
    tracing::info!("Adding 'resources' path {:?}", path);
    let cache = assets_manager::AssetCache::new(path)?;

    // Global Resources struct used for resources shared across scenes
    let resources = Resources::try_new(&ctx, rng_seed, cache)?;

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
