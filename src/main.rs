use bracket_lib::random::RandomNumberGenerator;
use macroquad::texture::load_texture;
use macroquad::window::next_frame;
use tracing::info;
use tracing::Level;

const VERSION: &str = "0.0.1";

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
use util::{TileAtlas, ViewportPoint, ViewportRect, ViewportSize, WorldToViewport};

use clap::Parser;
use rand::RngCore;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[clap(short, long, action, default_value_t = false)]
    mapgen_show: bool,
}

#[macroquad::main("Roguemon")]
async fn main() {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2.. => Level::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    game::env().show_map_generation = cli.mapgen_show;

    info!("linking resources");
    // Load assets.
    let texture = load_texture("assets/Tiles.png")
        .await
        .expect("loading tileset");

    // Construct TileAtlas.
    let atlas = TileAtlas::new(texture, 32., 32.);

    info!("creating context");
    info!("creating GameState");

    let rng_seed = if let Some(seed_param) = game::env().seed {
        seed_param
    } else {
        rand::thread_rng().next_u64()
    };
    info!("using rng seed: {}", rng_seed);

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
    };

    let gs = Game::new(resources);

    info!("starting main_loop");
    loop {
        gs.tick();
        next_frame().await
    }
}
