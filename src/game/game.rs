use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawParam, StrokeOptions};
use ggez::{graphics, timer, Context, GameError};
use hecs::World;
use tracing::info;

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
    BitmapFont, PixelPoint, PixelSize, SpriteSize, TransformExt, ViewportPoint, ViewportToScreen,
    WorldSize,
};

use super::consts::{PIXEL_RECT, SCALING_FACTOR};
use super::{consts, process_actors, PlayGame, RunState};

pub struct Game {
    state: RunState,
    scheduler: Scheduler,
    world: World,
    resources: Resources,
    screen: Screen,
    controls: Controls,
    canvas_image: graphics::ScreenImage,
    font: BitmapFont,
    font2: BitmapFont,
}

impl Game {
    pub fn new(resources: Resources, ctx: &mut Context) -> Self {
        let font_image =
            graphics::Image::from_path(ctx, "/fonts/rex_8x8.png", true).expect("load font");
        let font = BitmapFont::from_grid(ctx, font_image, &SpriteSize::new(16, 16));
        let font_image2 =
            graphics::Image::from_path(ctx, "/fonts/yun_16x16.png", true).expect("load font");
        let font2 = BitmapFont::from_grid(ctx, font_image2, &SpriteSize::new(16, 16));
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
            canvas_image: graphics::ScreenImage::new(
                ctx,
                None,
                1. / SCALING_FACTOR,
                1. / SCALING_FACTOR,
                1,
            ),
            font,
            font2,
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

    /// Handle any new keyboard events as part of the game update loop
    fn update_from_input(&mut self) {
        let next_state = match self.state {
            RunState::MainMenu(selection) => {
                Some(input::read_mainmenu(&mut self.controls, selection))
            }
            RunState::PauseMenu(selection) => {
                Some(input::read_pausemenu(&mut self.controls, selection))
            }
            RunState::GameOver(selection) => {
                Some(input::read_gameover(&mut self.controls, selection))
            }
            RunState::Game(_) => {
                // TODO: refactor read_game to take Input and determine what to do (rather than looking at BTerm events directly)
                match input::read_game(&mut self.controls, &mut self.world, &mut self.resources) {
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
}

impl EventHandler for Game {
    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        self.controls.key_down(input);

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), GameError> {
        self.controls.key_up(input);

        Ok(())
    }

    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(consts::UPDATE_FPS) {
            let seconds = 1.0 / (consts::UPDATE_FPS as f32);

            self.update_from_input();

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
            };
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        // Example for how to do the rest: https://github.com/ggez/ggez/blob/0.8.0-rc0/examples/animation.rs
        let mut canvas =
            graphics::Canvas::from_screen_image(ctx, &mut self.canvas_image, Color::BLACK);
        // let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        match &self.state {
            // menus
            RunState::MainMenu(selection) => {
                draw_main_menu(&mut canvas, selection, &mut self.resources)
            }
            RunState::PauseMenu(selection) => {
                draw_pause_menu(&mut canvas, selection, &mut self.resources)
            }
            RunState::GameOver(selection) => {
                draw_game_over(&mut canvas, selection, &mut self.resources)
            }

            // game loop
            RunState::Game(_) => {
                self.screen
                    .draw_game(&mut canvas, &self.world, &mut self.resources)
            }

            RunState::MapGeneration(map_state) => {
                map_state.draw(&mut canvas, &self.resources.mapgen_history);
            }

            _ => {
                tracing::error!("No draw available for state {:?}", self.state);
            }
        };

        canvas.draw(
            self.font.text(
                "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz ",
                &PixelPoint::new(0, 50),
            ),
            DrawParam::default(),
        );

        canvas.draw(
            self.font2.text(
                "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz ",
                &PixelPoint::new(0, 70),
            ),
            DrawParam::default(),
        );

        canvas.finish(ctx)?;

        let mut outer_canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        outer_canvas.set_sampler(graphics::Sampler::nearest_clamp());

        let image = self.canvas_image.image(ctx);

        outer_canvas.draw(
            &image,
            graphics::DrawParam::new()
                .dest([0., 0.])
                .scale([SCALING_FACTOR, SCALING_FACTOR]),
        );

        outer_canvas.finish(ctx)?;

        timer::yield_now();
        Ok(())
    }
}
