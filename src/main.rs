use game::consts::RESOURCE_PATH;
use game::consts::SCREEN_ASPECT_RATIO;
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
        window_height: SCREEN_HEIGHT_PIXELS * 3,
        window_width: SCREEN_WIDTH_PIXELS * 3,
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

    set_pc_assets_folder(consts::RESOURCE_PATH);

    tracing::info!("Adding 'resources' path {:?}", path);
    let cache = assets_manager::AssetCache::new(path)?;

    // Global Resources struct used for resources shared across scenes
    tracing::info!("Creating global Resources instance");
    let resources = Resources::try_new(rng_seed, cache).await?;

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
                (SCREEN_WIDTH_PIXELS as f32).recip() * 2.0,
                (SCREEN_HEIGHT_PIXELS as f32).recip() * 2.0,
            ),
            target: vec2(
                SCREEN_WIDTH_PIXELS as f32 / 2.0,
                SCREEN_HEIGHT_PIXELS as f32 / 2.0,
            ),
            ..Default::default()
        });
        clear_background(BLACK);

        // Draw the game to the canvas
        game.draw()?;

        // Draw the canvas to the window
        set_default_camera();
        clear_background(GREEN);

        let (width_deficit, height_deficit) = wh_deficit();
        draw_texture_ex(
            canvas.texture,
            width_deficit / 2.0,
            height_deficit / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    screen_width() - width_deficit,
                    screen_height() - height_deficit,
                )),
                ..Default::default()
            },
        );

        // Do the rest of the game stuff
        game.poll_input()?;
        game.update()?;

        next_frame().await
    }
}

fn wh_deficit() -> (f32, f32) {
    if (screen_width() / screen_height()) > SCREEN_ASPECT_RATIO {
        // it's too wide! put bars on the sides!
        // the height becomes the authority on how wide to draw
        let expected_width = screen_height() * SCREEN_ASPECT_RATIO;
        (screen_width() - expected_width, 0.0f32)
    } else {
        // it's too tall! put bars on the ends!
        // the width is the authority
        let expected_height = screen_width() / SCREEN_ASPECT_RATIO;
        (0.0f32, screen_height() - expected_height)
    }
}
