use bracket_lib::prelude as rltk;
use bracket_lib::prelude::*;
use hecs::World;
use tracing::info;

use crate::component::{Actor, ActorKind, Player};
use crate::game::consts::{SCREEN_HEIGHT, SCREEN_WIDTH, VIEWPORT_SCREEN_POINT};
use crate::map::Loader;
use crate::{game, input};

use crate::camera::Screen;
use crate::map::Bsp;
use crate::resource::Resources;
use crate::scene::GameOverSelection;
use crate::scene::MapGenerationState;
use crate::scene::PauseMenuSelection;
use crate::scene::{draw_game_over, draw_main_menu, draw_pause_menu, MainMenuSelection};
use crate::system::{build_systems, Scheduler};
use crate::util::{
    ScreenPoint, ScreenRect, ScreenSize, TransformExt, ViewportPoint, ViewportToScreen, WorldSize,
};

use super::{process_actors, PlayGame, RunState};

pub struct Game {
    scheduler: Scheduler,
    world: World,
    resources: Resources,
    screen: Screen,
}

impl Game {
    pub fn new(resources: Resources) -> Self {
        Self {
            scheduler: build_systems(),
            world: World::default(),
            resources,
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

    fn input(&mut self, state: RunState) -> RunState {
        match state {
            RunState::MainMenu(selection) => input::read_mainmenu(selection),
            RunState::PauseMenu(selection) => input::read_pausemenu(selection),
            RunState::GameOver(selection) => input::read_gameover(selection),
            RunState::Game(_) => {
                match input::read_game(&mut self.world, &mut self.resources) {
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
    fn draw(&mut self, state: &RunState) {
        match state {
            // menus
            RunState::MainMenu(selection) => draw_main_menu(selection),
            RunState::PauseMenu(selection) => draw_pause_menu(selection),
            RunState::GameOver(selection) => draw_game_over(selection),

            // game loop
            RunState::Game(_) => self.screen.draw_game(&self.world, &mut self.resources),

            RunState::MapGeneration(map_state) => {
                map_state.draw(&self.resources.mapgen_history);
            }

            _ => {
                tracing::error!("No draw available for state {:?}", state);
            }
        };
    }

    pub fn tick(&mut self) {
        let state = self.resources.take_state();
        tracing::trace!("State: {:?}", state);

        let input_result = self.input(state);
        let update_result = self.update(input_result);
        self.draw(&update_result);

        self.resources.replace_state(update_result);
    }
}
