use bracket_lib::prelude as rltk;
use bracket_lib::prelude::*;
use hecs::World;
use rand::RngCore;
use tracing::info;

use crate::component::{Actor, ActorKind, Player};
use crate::game::consts::{
    SCREEN_HEIGHT, SCREEN_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_SCREEN_POINT, VIEWPORT_WIDTH,
};
use crate::game::TurnsHistory;
use crate::map::Loader;
use crate::{game, input};

use crate::camera::Screen;
use crate::map::Bsp;
use crate::resource::{Resources, Viewport};
use crate::scene::MapGenerationState;
use crate::scene::PauseMenuSelection;
use crate::scene::{draw_game_over, draw_main_menu, draw_pause_menu, MainMenuSelection};
use crate::scene::{Controller, GameOverSelection};
use crate::system::{build_systems, Scheduler};
use crate::util::{
    ScreenPoint, ScreenRect, ScreenSize, TransformExt, ViewportPoint, ViewportRect, ViewportSize,
    ViewportToScreen, WorldSize, WorldToViewport,
};

use super::{process_actors, PlayGame, RunState};

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
                run_state: Some(RunState::MainMenu(MainMenuSelection::NewGame)),
                turn_number: 0,
                turn_history: TurnsHistory::default(),
                viewport: Viewport::new(
                    ViewportRect::new(
                        ViewportPoint::new(0, 0),
                        ViewportSize::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
                    ),
                    WorldToViewport::default(),
                ),
            },
            screen: Screen::new(
                ScreenRect::new(
                    ScreenPoint::new(0, 0),
                    ScreenSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32),
                ),
                ViewportToScreen::from_points(ViewportPoint::new(0, 0), VIEWPORT_SCREEN_POINT),
            ),
        }
    }

    fn run_systems(&mut self) {
        self.scheduler.execute(&mut self.world, &mut self.resources);
    }

    fn process_actors(&mut self) -> RunState {
        process_actors(&mut self.world, &mut self.resources)
    }

    fn set_player_action(&mut self, player_action: input::PlayerAction) -> RunState {
        // Find the player component and set the next action on this player
        for (_ent, (_player, actor)) in self.world.query_mut::<(&Player, &mut Actor)>() {
            actor.set_kind(ActorKind::Player(Some(player_action)));
        }
        RunState::Game(PlayGame::Ticking)
    }

    fn input(&mut self, ctx: &mut BTerm, state: RunState) -> RunState {
        match state {
            RunState::MainMenu(selection) => input::read_mainmenu(selection, ctx),
            RunState::PauseMenu(selection) => input::read_pausemenu(selection, ctx),
            RunState::GameOver(selection) => input::read_gameover(selection, ctx),
            RunState::Game(_) => {
                match input::read_game(&mut self.world, &mut self.resources, ctx) {
                    input::PlayerInput::Ui(action) => match action {
                        input::UiAction::MainMenu => RunState::MainMenu(MainMenuSelection::NewGame),
                        input::UiAction::PauseMenu => {
                            RunState::PauseMenu(PauseMenuSelection::Continue)
                        }
                        input::UiAction::GameOverMenu => {
                            RunState::GameOver(GameOverSelection::MainMenu)
                        }
                    },
                    input::PlayerInput::Game(action) => match state {
                        // Skip player input when the engine asks us to
                        RunState::Game(PlayGame::NeedPlayerInput) => self.set_player_action(action),
                        _ => RunState::Game(PlayGame::Ticking),
                    },
                    input::PlayerInput::Undefined => state,
                }
            }
            _ => {
                tracing::error!("No input handling available for state {:?}", state);
                state
            }
        }
    }

    fn update(&mut self, ctx: &BTerm, state: RunState) -> RunState {
        match state {
            // game loop
            RunState::Game(_) => {
                // Set the return value
                let result = self.process_actors();
                self.run_systems();
                result
            }
            RunState::Initialization => {
                info!("Initializing level");
                info!("Map generation");
                // Initialize mapgen history
                self.resources.mapgen_history = Vec::new();

                // Create the loader
                let mut loader = Loader::new(
                    Bsp::new(WorldSize::new(50, 50)),
                    &mut self.resources.rng,
                    &mut self.resources.mapgen_history,
                );

                // Load and spawn the map
                let map = loader.load(1, &mut self.world);
                self.resources.map = Some(map);

                // View Map generation (if enabled)
                RunState::MapGeneration(MapGenerationState::default())
            }
            RunState::MapGeneration(mut map_state) => {
                if game::env().show_map_generation {
                    if map_state.is_complete(&self.resources.mapgen_history) {
                        // TODO: make it so that arrow keys pan around and enter allows us to continue
                        // If we're done, move on to the next state
                        RunState::Game(PlayGame::Ticking)
                    } else {
                        // If we have more frames to render for map generation, pass the
                        // state onto the next tick.
                        map_state.update(ctx);
                        RunState::MapGeneration(map_state)
                    }
                } else {
                    RunState::Game(PlayGame::Ticking)
                }
            }

            // Skip update for all other states
            _ => state,
        }
    }

    /// Mutable self because rendering uses the rng
    fn draw(&mut self, ctx: &mut BTerm, state: &RunState) {
        let draw_batch = &mut DrawBatch::new();
        match state {
            // menus
            RunState::MainMenu(selection) => draw_main_menu(selection, draw_batch),
            RunState::PauseMenu(selection) => draw_pause_menu(selection, draw_batch),
            RunState::GameOver(selection) => draw_game_over(selection, draw_batch),

            // game loop
            RunState::Game(_) => {
                self.screen
                    .draw_game(&self.world, &mut self.resources, ctx, draw_batch)
            }

            RunState::MapGeneration(map_state) => {
                map_state.draw(ctx, draw_batch, &self.resources.mapgen_history);
            }

            _ => {
                tracing::error!("No draw available for state {:?}", state);
            }
        };
        rltk::render_draw_buffer(ctx).expect("Render Draw Buffer");
    }
}

impl rltk::GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        let state = self.resources.take_state();
        tracing::trace!("State: {:?}", state);

        let input_result = self.input(ctx, state);
        let update_result = self.update(ctx, input_result);
        self.draw(ctx, &update_result);

        self.resources.replace_state(update_result);
    }
}

pub struct GameHandler {}
impl GameHandler {}
