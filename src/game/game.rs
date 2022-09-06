use bracket_lib::prelude as rltk;
use bracket_lib::prelude::*;
use legion::*;
use rand::RngCore;
use tracing::info;

use crate::component::*;
use crate::game;
use crate::map::Map;
use crate::map::MapGenerator;
use crate::map::Simple;
use crate::scene::Controller;
use crate::scene::GameOverSelection;
use crate::scene::MainMenuSelection;
use crate::scene::MapGenerationState;

use super::RunState;

pub struct Game {
    schedule: Schedule,
    resources: Resources,
    rng: rltk::RandomNumberGenerator,
    world: World,
    controller: Controller,
    maps: Vec<Map>,
}

impl Game {
    pub fn new() -> Self {
        let rng_seed = if let Some(seed_param) = game::env().seed {
            seed_param
        } else {
            rand::thread_rng().next_u64()
        };

        info!("using rng seed: {}", rng_seed);

        let mut resources = Resources::default();
        let schedule = Schedule::builder()
            //.add_system(update_positions_system())
            .build();

        resources.insert(RunState::MainMenu(MainMenuSelection::NewGame));

        Self {
            schedule,
            resources,
            world: World::default(),
            rng: rltk::RandomNumberGenerator::seeded(rng_seed),
            controller: Controller::default(),
            maps: Vec::<Map>::default(),
        }
    }

    pub fn create_player(&mut self) -> Entity {
        self.world.push((
            Position::new(Point::new(40, 25)),
            Renderable::new(Point::new(20, 20), WHITE, BLACK, '@'),
        ))
    }

    fn run_systems(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    pub fn handle_state(&mut self, state: RunState, ctx: &mut BTerm) -> RunState {
        match state {
            RunState::MainMenu(selection) => self.controller.main_menu(ctx, selection),
            RunState::Initialization => {
                info!("Initializing level");
                let map = Simple {}.generate(&mut self.rng);
                self.maps.push(map);
                RunState::MapGeneration(MapGenerationState::default())
            }
            RunState::MapGeneration(map_state) => {
                info!("Map generation");
                self.controller.map_generation(ctx, map_state, &self.maps)
            }
            RunState::GameAwaitingInput => {
                // player::game_input_turn(self, ctx)
                // TODO: implement game
                return RunState::GameOver(GameOverSelection::MainMenu);

                RunState::GameTurn
            }
            RunState::GameTurn => {
                self.run_systems();
                RunState::GameDraw
            }
            RunState::GameDraw => {
                self.run_systems();
                // get turn state
                ctx.cls();
                // self.draw_game()
                // match on turn state (dead? level complete? Running? (do nothing))
                RunState::GameAwaitingInput
            }
            RunState::GameOver(selection) => self.controller.game_over(ctx, selection),
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        let state = self.resources.remove::<RunState>().unwrap();
        let new_state = self.handle_state(state, ctx);
        self.resources.insert(new_state);
    }
}
