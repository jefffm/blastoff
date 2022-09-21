use hecs::World;
use image::DynamicImage;
use pixels::Pixels;
use tracing::info;
use winit::event::Event;
use winit_input_helper::WinitInputHelper;

use crate::component::{Actor, ActorKind, Player};
use crate::game::consts::VIEWPORT_SCREEN_POINT;
use crate::input::Controls;
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
    blit, PixelPoint, PixelSize, TransformExt, ViewportPoint, ViewportToScreen, WorldSize,
};

use super::{process_actors, PlayGame, RunState};

pub struct Game {
    state: RunState,
    scheduler: Scheduler,
    world: World,
    resources: Resources,
    screen: Screen,
    controls: Controls,
    pub input: WinitInputHelper,
    pub canvas: Pixels,
    test_img: DynamicImage,
}

impl Game {
    pub fn new(resources: Resources, canvas: Pixels) -> Self {
        let img = image::load_from_memory(include_bytes!("../../assets/demon.png")).unwrap();
        Self {
            state: RunState::MainMenu(MainMenuSelection::NewGame),
            scheduler: build_systems(),
            world: World::default(),
            resources,
            screen: Screen::new(ViewportToScreen::from_points(
                ViewportPoint::new(0, 0),
                VIEWPORT_SCREEN_POINT,
            )),
            controls: Controls::default(),
            input: WinitInputHelper::new(),
            canvas,
            test_img: img,
        }
    }

    pub fn state(&self) -> &RunState {
        &self.state
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

    pub fn handle_input(&mut self, event: &Event<()>) {
        // Let winit_input_helper collect events to build its state.
        if self.input.update(event) {
            // Update controls
            // TODO: implement update_controls()
            // self.update_controls();

            // Resize the window
            if let Some(size) = self.input.window_resized() {
                // TODO: implement pixel-perfect scaling
                self.canvas.resize_surface(size.width, size.height);
            }
        }

        let next_state = match self.state {
            RunState::MainMenu(selection) => Some(input::read_mainmenu(self.controls, selection)),
            RunState::PauseMenu(selection) => Some(input::read_pausemenu(self.controls, selection)),
            RunState::GameOver(selection) => Some(input::read_gameover(self.controls, selection)),
            RunState::Game(_) => {
                // TODO: refactor read_game to take Input and determine what to do (rather than looking at BTerm events directly)
                match input::read_game(self.controls, &mut self.world, &mut self.resources) {
                    input::PlayerInput::Ui(action) => match action {
                        input::UiAction::MainMenu => {
                            Some(RunState::MainMenu(MainMenuSelection::NewGame))
                        }
                        input::UiAction::PauseMenu => {
                            Some(RunState::PauseMenu(PauseMenuSelection::Continue))
                        }
                        input::UiAction::GameOverMenu => {
                            Some(RunState::GameOver(GameOverSelection::MainMenu))
                        }
                    },
                    input::PlayerInput::Game(action) => match self.state {
                        // Skip player input when the engine asks us to
                        RunState::Game(PlayGame::NeedPlayerInput) => {
                            Some(self.set_player_action(action))
                        }
                        _ => Some(RunState::Game(PlayGame::Ticking)),
                    },
                    input::PlayerInput::Undefined => None,
                }
            }
            _ => {
                tracing::error!("No input handling available for state {:?}", self.state);
                None
            }
        };

        if let Some(next_state) = next_state {
            self.state = next_state;
        };
    }

    pub fn handle_update(&mut self) {
        let next_state = match &self.state {
            // game loop
            RunState::Game(_) => {
                // Set the return value
                let result = self.process_actors();
                self.run_systems();
                Some(result)
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
                Some(RunState::MapGeneration(MapGenerationState::default()))
            }
            RunState::MapGeneration(m) => {
                let map_state = m.clone();
                if game::env().show_map_generation {
                    if map_state.is_complete(&self.resources.mapgen_history) {
                        // TODO: make it so that arrow keys pan around and enter allows us to continue
                        // If we're done, move on to the next state
                        Some(RunState::Game(PlayGame::Ticking))
                    } else {
                        // If we have more frames to render for map generation, pass the
                        // state onto the next tick.
                        // TODO: mapgen is borked
                        // map_state.update(ctx);
                        Some(RunState::MapGeneration(map_state))
                    }
                } else {
                    Some(RunState::Game(PlayGame::Ticking))
                }
            }

            // Skip update for all other states
            _ => None,
        };

        if let Some(s) = next_state {
            self.state = s;
        }
    }

    /// Mutable self because rendering uses the rng
    pub fn handle_render(&mut self) {
        let screen = self.canvas.get_frame();

        match &self.state {
            // menus
            RunState::MainMenu(selection) => draw_main_menu(screen, selection, &mut self.resources),
            RunState::PauseMenu(selection) => {
                draw_pause_menu(screen, selection, &mut self.resources)
            }
            RunState::GameOver(selection) => draw_game_over(screen, selection, &mut self.resources),

            // game loop
            RunState::Game(_) => self
                .screen
                .draw_game(screen, &self.world, &mut self.resources),

            RunState::MapGeneration(map_state) => {
                map_state.draw(screen, &self.resources.mapgen_history);
            }

            _ => {
                tracing::error!("No draw available for state {:?}", self.state);
            }
        };

        // blit(
        //     screen,
        //     &PixelPoint::new(0, 0),
        //     &Sprite::new(PixelSize::new(32, 36), self.test_img.as_bytes().to_vec()),
        // );
    }
}
