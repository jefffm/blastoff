use bracket_lib::prelude as rltk;
use bracket_lib::prelude::*;
use hecs::World;
use rand::RngCore;
use tracing::info;

use crate::game;
use crate::game::consts::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::game::TurnsHistory;
use crate::map::Loader;

use crate::map::{Bsp, Simple};
use crate::player;
use crate::resource::{Resources, Screen, Viewport};
use crate::scene::Controller;
use crate::scene::MainMenuSelection;
use crate::scene::MapGenerationState;
use crate::system::{build_systems, Scheduler};
use crate::util::{
    ScreenPoint, ScreenRect, ScreenSize, TransformExt, ViewportPoint, ViewportRect, ViewportSize,
    ViewportToScreen, WorldSize, WorldToViewport,
};

use super::RunState;

pub struct Game {
    scheduler: Scheduler,
    world: World,
    resources: Resources,
    screen: Screen,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let rng_seed = if let Some(seed_param) = game::env().seed {
            seed_param
        } else {
            rand::thread_rng().next_u64()
        };

        info!("using rng seed: {}", rng_seed);

        Self {
            scheduler: build_systems(),
            world: World::default(),
            resources: Resources {
                rng: rltk::RandomNumberGenerator::seeded(rng_seed),
                controller: Controller::default(),
                map: None,
                mapgen_history: Vec::default(),
                run_state: RunState::MainMenu(MainMenuSelection::NewGame),
                turn_history: TurnsHistory::default(),
                viewport: Viewport::new(
                    ViewportRect::new(ViewportPoint::new(0, 0), ViewportSize::new(50, 50)),
                    WorldToViewport::default(),
                ),
            },
            screen: Screen::new(
                ScreenRect::new(
                    ScreenPoint::new(0, 0),
                    ScreenSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32),
                ),
                ViewportToScreen::from_points(ViewportPoint::new(0, 0), ScreenPoint::new(2, 2)),
            ),
        }
    }

    fn run_systems(&mut self) {
        self.scheduler.execute(&mut self.world, &mut self.resources);
    }

    pub fn handle_state(
        &mut self,
        state: RunState,
        ctx: &mut BTerm,
        draw_batch: &mut DrawBatch,
    ) -> RunState {
        match state {
            RunState::MainMenu(selection) => self
                .resources
                .controller
                .main_menu(ctx, draw_batch, selection, false),
            RunState::PauseMenu(selection) => self
                .resources
                .controller
                .pause_menu(ctx, draw_batch, selection),
            RunState::Initialization => {
                info!("Initializing level");
                info!("Map generation");
                // Initialize mapgen history
                self.resources.mapgen_history = Vec::new();

                // Create the loader
                let mut loader = Loader::new(
                    Bsp::new(WorldSize::new(100, 100)),
                    &mut self.resources.rng,
                    &mut self.resources.mapgen_history,
                );

                // Load and spawn the map
                let map = loader.load(1, &mut self.world);
                self.resources.map = Some(map);

                // View Map generation (if enabled)
                RunState::MapGeneration(MapGenerationState::default())
            }
            RunState::MapGeneration(map_state) => self.resources.controller.map_generation(
                ctx,
                draw_batch,
                map_state,
                &self.resources.mapgen_history,
            ),
            RunState::GameAwaitingInput => {
                player::game_turn_input(&mut self.world, &mut self.resources, ctx)
            }
            RunState::GameTurn => {
                self.run_systems();
                RunState::GameDraw
            }
            RunState::GameDraw => {
                self.screen
                    .draw_game(&self.world, &self.resources, draw_batch);
                RunState::GameAwaitingInput
            }
            RunState::GameOver(selection) => self
                .resources
                .controller
                .game_over(ctx, draw_batch, selection),
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        // TODO: remove unnecessary clone
        let state = self.resources.run_state.clone();
        let mut draw_batch = DrawBatch::new();

        let new_state = self.handle_state(state, ctx, &mut draw_batch);

        self.resources.run_state = new_state;
        rltk::render_draw_buffer(ctx).expect("Render Draw Buffer");
    }
}
