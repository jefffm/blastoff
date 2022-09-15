use bracket_lib::prelude as rltk;
use bracket_lib::prelude::*;
bracket_lib::prelude::add_wasm_support!();
use tracing::info;

use tracing::Level;

const VERSION: &str = "0.0.1";

rltk::embedded_resource!(FONT_16X16_YUN, "../resources/fonts/yun_16x16.png");
rltk::embedded_resource!(FONT_16X16_REX, "../resources/fonts/rex_16x16.png");
rltk::embedded_resource!(FONT_14X14_REX, "../resources/fonts/rex_14x14.png");
rltk::embedded_resource!(FONT_12X12_REX, "../resources/fonts/rex_12x12.png");
rltk::embedded_resource!(FONT_8X8_REX, "../resources/fonts/rex_8x8.png");

use crate::game::Game;

pub mod camera;
pub mod component;
pub mod game;
pub mod input;
pub mod map;
pub mod resource;
pub mod scene;
pub mod system;
pub mod util;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[clap(short, long, action, default_value_t = false)]
    mapgen_show: bool,
}

fn main() -> rltk::BResult<()> {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2.. => Level::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    game::env().show_map_generation = cli.mapgen_show;

    info!("linking resources");
    rltk::link_resource!(FONT_16X16_YUN, "resources/fonts/yun_16x16.png");
    rltk::link_resource!(FONT_16X16_REX, "resources/fonts/rex_16x16.png");
    rltk::link_resource!(FONT_14X14_REX, "resources/fonts/rex_14x14.png");
    rltk::link_resource!(FONT_12X12_REX, "resources/fonts/rex_12x12.png");
    rltk::link_resource!(FONT_8X8_REX, "resources/fonts/rex_8x8.png");
    let font_16x16_yun = "fonts/yun_16x16.png";
    let font_16x16_rex = "fonts/rex_16x16.png";
    let font_14x14_rex = "fonts/rex_14x14.png";
    let font_12x12_rex = "fonts/rex_12x12.png";
    let font_8x8_rex = "fonts/rex_8x8.png";

    info!("creating context");
    let tile_size = game::consts::TILE_SIZE;
    let context = rltk::BTermBuilder::new()
        .with_dimensions(game::consts::SCREEN_WIDTH, game::consts::SCREEN_HEIGHT)
        .with_font(font_16x16_yun, 16, 16)
        .with_font(font_16x16_rex, 16, 16)
        .with_font(font_14x14_rex, 14, 14)
        .with_font(font_12x12_rex, 12, 12)
        .with_font(font_8x8_rex, 8, 8)
        .with_advanced_input(true)
        .with_fancy_console(
            // world layer
            game::consts::SCREEN_WIDTH,
            game::consts::SCREEN_HEIGHT,
            font_16x16_yun,
        )
        .with_fancy_console(
            // particles layer
            game::consts::SCREEN_WIDTH,
            game::consts::SCREEN_HEIGHT,
            font_16x16_yun,
        )
        .with_fancy_console(
            // shader layer
            game::consts::SCREEN_WIDTH,
            game::consts::SCREEN_HEIGHT,
            font_16x16_yun,
        )
        .with_fancy_console(
            game::consts::SCREEN_WIDTH,
            game::consts::SCREEN_HEIGHT,
            font_16x16_yun,
        ) // hud layer
        .with_title(format!("Roguemon v{}", VERSION))
        .with_fps_cap(60.0)
        .with_tile_dimensions(tile_size, tile_size)
        .with_vsync(true)
        .build()?;

    info!("creating GameState");
    let gs = Game::new();

    info!("starting main_loop");
    rltk::main_loop(context, gs)
}
