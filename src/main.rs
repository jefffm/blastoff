use std::{env, path};
use tracing::info;
use tracing::Level;

pub mod scene;
// pub mod ation;
// pub mod camera;
pub mod color;
// pub mod component;
// pub mod data;
// pub mod galaxy;
pub mod game;
// pub mod input;
// pub mod overworld;
// pub mod procgen;
pub mod resource;
// pub mod sector;
// pub mod system;
pub mod util;

use clap::Parser;
use macroquad::prelude::*;

use game::consts::{self, SCREEN_HEIGHT_PIXELS, SCREEN_WIDTH_PIXELS, TITLE_HEADER};
use resource::Resources;
use scene::MainState;

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

fn window_conf() -> Conf {
    Conf {
        window_title: if cfg!(debug_assertions) {
            concat!(env!("CARGO_CRATE_NAME"), " v", env!("CARGO_PKG_VERSION"))
        } else {
            TITLE_HEADER
        }
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_height: SCREEN_HEIGHT_PIXELS,
        window_width: SCREEN_WIDTH_PIXELS,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2.. => Level::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    let rng_seed = cli
        .seed
        .unwrap_or_else(|| rand::gen_range(0, std::u64::MAX));

    info!("using rng seed: {}", rng_seed);

    // TODO: std::env isn't supported on WASM.
    //#[cfg(not(target_arch = "wasm32"))]
    info!("linking resources");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    let path = path::PathBuf::from(manifest_dir).join(consts::RESOURCE_PATH);
    tracing::info!("Adding 'resources' path {:?}", path);
    let cache = assets_manager::AssetCache::new(path)?;

    // Global Resources struct used for resources shared across scenes
    let resources = Resources::try_new(rng_seed, cache)?;

    let mut game = MainState::new(resources);

    // Push an initial scene to the SceneStack and prepare it for playing
    if cli.debug {
        game.init_debug()
    } else {
        game.init();
    }

    info!("starting main_loop");

    let canvas = render_target(SCREEN_WIDTH_PIXELS as u32, SCREEN_HEIGHT_PIXELS as u32);
    canvas.texture.set_filter(FilterMode::Nearest);
    loop {
        // Render to the Canvas
        set_camera(&Camera2D {
            render_target: Some(canvas),
            zoom: vec2(
                (SCREEN_WIDTH_PIXELS).recip() * 2.0,
                (SCREEN_HEIGHT_PIXELS).recip() * 2.0,
            ),
            target: vec2(SCREEN_WIDTH_PIXELS / 2.0, SCREEN_WIDTH_PIXELS / 2.0),
            ..Default::default()
        });
        clear_background(WHITE);

        // Done rendering to the canvas; go back to our normal camera
        // to size the canvas
        set_default_camera();
        clear_background(BLACK);

        game.poll_input()?;
        game.update()?;
        game.draw()?;

        next_frame().await
    }

    Ok(())
}
