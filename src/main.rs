use bracket_lib::random::RandomNumberGenerator;
use game_loop::{game_loop, Time, TimeTrait as _};

use pixels::{Error, Pixels, SurfaceTexture};

use std::io::Cursor;
use std::{time::Duration};
use tracing::info;
use tracing::Level;
use winit::{
    dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder,
};

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

use game::{consts, Game, RunState, TurnsHistory};
use resource::{Resources, Viewport};
use scene::{Controller, MainMenuSelection};
use util::{SpriteAtlas, ViewportPoint, ViewportRect, ViewportSize, WorldToViewport};

use clap::Parser;
use rand::RngCore;

use crate::game::consts::{PIXEL_RECT, TITLE_HEADER};
use crate::util::SpriteSize;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[clap(short, long, action, default_value_t = false)]
    mapgen_show: bool,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2.. => Level::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    // TODO: remove game::env mutability
    game::env().show_map_generation = cli.mapgen_show;
    // TODO: add seed parameter to CLI
    let rng_seed = game::env()
        .seed
        .unwrap_or_else(|| rand::thread_rng().next_u64());

    info!("using rng seed: {}", rng_seed);

    info!("linking resources");

    // Construct SpriteAtlas
    let atlas = SpriteAtlas::from_png(
        Cursor::new(include_bytes!(
            "../assets/tileset/monochrome-transparent.png"
        )),
        SpriteSize::new(8, 8),
    );

    info!("creating context");
    info!("creating GameState");

    let resources = Resources {
        rng: RandomNumberGenerator::seeded(rng_seed),
        controller: Controller::default(),
        map: None,
        mapgen_history: Vec::default(),
        run_state: Some(RunState::MainMenu(MainMenuSelection::NewGame)),
        turn_number: 0,
        turn_history: TurnsHistory::default(),
        viewport: Viewport::new(
            ViewportRect::new(
                ViewportPoint::new(0, 0),
                ViewportSize::new(consts::VIEWPORT_WIDTH, consts::VIEWPORT_HEIGHT),
            ),
            WorldToViewport::default(),
        ),
        atlas,
    };

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(
            consts::SCREEN_RECT.width() as f64,
            consts::SCREEN_RECT.height() as f64,
        );
        let scaled_size = LogicalSize::new(
            // TODO: configurable pixel scaling for window
            consts::SCREEN_RECT.width() as f64 * 3.0,
            consts::SCREEN_RECT.height() as f64 * 3.0,
        );
        WindowBuilder::new()
            .with_title(TITLE_HEADER)
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let canvas = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(
            PIXEL_RECT.width() as u32,
            PIXEL_RECT.height() as u32,
            surface_texture,
        )?
    };

    let game = Game::new(resources, canvas);

    info!("starting main_loop");

    game_loop(
        event_loop,
        window,
        game,
        consts::FPS as u32,
        0.1,
        move |g| g.game.handle_update(),
        move |g| {
            // Drawing
            g.game.handle_render();
            if let Err(e) = g.game.canvas.render() {
                tracing::error!("pixels.render() failed: {}", e);
                g.exit();
            }

            // Sleep the main thread to limit drawing to the fixed time step.
            // See: https://github.com/parasyte/pixels/issues/174
            let dt = consts::TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
            if dt > 0.0 {
                std::thread::sleep(Duration::from_secs_f64(dt));
            }
        },
        |g, event| {
            g.game.handle_input(event);
            if let RunState::Exiting = g.game.state() {
                g.exit();
            }
        },
    );
}
