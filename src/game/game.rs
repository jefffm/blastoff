use bracket_lib::prelude as rltk;
use bracket_lib::prelude::*;
use legion::*;
use rand::RngCore;
use tracing::debug;
use tracing::info;

use crate::camera::Camera;
use crate::component::{Position, Renderable};
use crate::game;
use crate::game::consts;
use crate::game::draw_ui;
use crate::game::TurnsHistory;
use crate::map::Loader;
use crate::map::Map;
use crate::map::MapGenerator;
use crate::map::Simple;
use crate::player;
use crate::scene::Controller;
use crate::scene::MainMenuSelection;
use crate::scene::MapGenerationState;
use crate::system::build_systems;

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
        resources.insert(RunState::MainMenu(MainMenuSelection::NewGame));

        Self {
            schedule: build_systems(),
            resources,
            world: World::default(),
            rng: rltk::RandomNumberGenerator::seeded(rng_seed),
            controller: Controller::default(),
            maps: Vec::<Map>::default(),
        }
    }

    fn run_systems(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    pub fn handle_state(&mut self, state: RunState, ctx: &mut BTerm) -> RunState {
        match state {
            RunState::MainMenu(selection) => self.controller.main_menu(ctx, selection, false),
            RunState::PauseMenu(selection) => self.controller.pause_menu(ctx, selection),
            RunState::Initialization => {
                self.resources.insert(TurnsHistory::new());
                info!("Initializing level");
                // TODO: this is incompatible with Loader
                let map = Simple {}.generate(&mut self.rng, 1);
                self.maps.push(map);

                let mut loader = Loader::new(Simple {}, &mut self.rng);
                loader.load(1, &mut self.world, &mut self.resources);
                RunState::MapGeneration(MapGenerationState::default())
            }
            RunState::MapGeneration(map_state) => {
                info!("Map generation");
                self.controller.map_generation(ctx, map_state, &self.maps)
            }
            RunState::GameAwaitingInput => {
                player::game_turn_input(&mut self.world, &mut self.resources, ctx)
            }
            RunState::GameTurn => {
                self.run_systems();
                RunState::GameDraw
            }
            RunState::GameDraw => {
                ctx.cls();
                self.draw_game(ctx);
                RunState::GameAwaitingInput
            }
            RunState::GameOver(selection) => self.controller.game_over(ctx, selection),
        }
    }

    fn draw_game(&self, ctx: &mut BTerm) {
        let map = self.resources.get::<Map>().unwrap();
        let start_x = (consts::SCREEN_WIDTH - map.get_width() as u32) / 2;
        let start_y = 11;
        // map.draw(ctx, Point::new(start_x, start_y));
        let mut data = <(Read<Position>, Read<Renderable>)>::query()
            .iter(&self.world)
            .collect::<Vec<_>>();
        data.sort_by(|d1, d2| d2.1.render_order.cmp(&d1.1.render_order));
        for (pos, render) in data.iter() {
            ctx.set(
                // start_x as i32 + pos.p.x,
                // start_y as i32 + pos.p.y,
                pos.p.x,
                pos.p.y,
                render.glyph.fg,
                render.glyph.bg,
                render.glyph.glyph,
            );
        }
        draw_ui(&self.resources, ctx);
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        let state = self.resources.remove::<RunState>().unwrap();
        let new_state = self.handle_state(state, ctx);
        self.resources.insert(new_state);
    }
}
