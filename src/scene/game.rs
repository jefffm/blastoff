use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;

use hecs::World;

use crate::camera::Screen;

use crate::component::{Actor, ActorKind, Player};
use crate::game::consts::VIEWPORT_SCREEN_POINT;
use crate::game::{process_actors, TurnsHistory};
use crate::input::{Controls, PlayerAction, PlayerInput, UiAction};
use crate::map::Map;
use crate::resource::Resources;

use crate::input;
use crate::system::{build_systems, Scheduler};
use crate::util::{Scene, SceneSwitch, TransformExt, ViewportPoint, ViewportToScreen};

use super::{GameOver, MainMenu, PauseMenu};

// If the RunState is a GameState, we need to do INPUT, UPDATE, and DRAW every frame
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Ticking,
    NeedPlayerInput,
}

pub struct Game {
    map: Map,
    world: World,
    input: Option<PlayerInput>,
    state: GameState,
    screen: Screen,
    scheduler: Scheduler,
    turn_history: TurnsHistory,
}

impl Game {
    pub fn new(map: Map, world: World) -> Self {
        Self {
            map,
            world,
            input: None,
            state: GameState::Ticking,
            screen: Screen::new(ViewportToScreen::from_points(
                ViewportPoint::new(0, 0),
                VIEWPORT_SCREEN_POINT,
            )),
            scheduler: build_systems(),
            turn_history: TurnsHistory::default(),
        }
    }
}

impl Game {
    /// Find the player component and set the next action on this player
    fn set_player_action(&mut self, player_action: PlayerAction) {
        for (_ent, (_player, actor)) in self.world.query_mut::<(&Player, &mut Actor)>() {
            actor.set_kind(ActorKind::Player(Some(player_action)));
        }
        self.state = GameState::Ticking
    }
}

impl Scene<Resources, Controls> for Game {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        self.input = match controls.read() {
            None => None,
            Some(key) => match (key, controls.control, controls.alt, controls.shift) {
                (KeyCode::Left, _, _, false) => Some(PlayerInput::Game(PlayerAction::MoveWest)),
                (KeyCode::Left, _, _, true) => Some(PlayerInput::Game(PlayerAction::MoveSouthWest)),
                (KeyCode::Right, _, _, false) => Some(PlayerInput::Game(PlayerAction::MoveEast)),
                (KeyCode::Right, _, _, true) => {
                    Some(PlayerInput::Game(PlayerAction::MoveNorthEast))
                }
                (KeyCode::Up, _, _, false) => Some(PlayerInput::Game(PlayerAction::MoveNorth)),
                (KeyCode::Up, _, _, true) => Some(PlayerInput::Game(PlayerAction::MoveNorthWest)),
                (KeyCode::Down, _, _, false) => Some(PlayerInput::Game(PlayerAction::MoveSouth)),
                (KeyCode::Down, _, _, true) => Some(PlayerInput::Game(PlayerAction::MoveSouthEast)),
                (KeyCode::Key1, _, _, _) => todo!("label NPCs"),
                (KeyCode::Key2, _, _, _) => todo!("label Hostiles"),
                (KeyCode::Key3, _, _, _) => todo!("label Items"),
                (KeyCode::Key4, _, _, _) => todo!("label something"),
                (KeyCode::Key5, _, _, _) => todo!("label something else"),
                (KeyCode::A, _, _, _) => todo!("skills"),
                (KeyCode::E, _, _, _) => todo!("equipment"),
                (KeyCode::I, _, _, _) => todo!("inventory"),
                (KeyCode::X, _, _, _) => todo!("character"),
                (KeyCode::L, _, _, _) => todo!("look"),
                (KeyCode::F, _, _, _) => todo!("ranged fire mode"),
                (KeyCode::Space, _, _, _) => Some(PlayerInput::Game(PlayerAction::PassTurn)),
                (KeyCode::Escape, _, _, _) => Some(PlayerInput::Ui(UiAction::PauseMenu)),
                other => {
                    tracing::debug!("unhandled keypress: {:?}", other);
                    None
                }
            },
        };
    }

    fn update(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        if let Some(input) = &self.input.take() {
            match input {
                input::PlayerInput::Ui(action) => match action {
                    input::UiAction::MainMenu => {
                        return SceneSwitch::Reinit(Box::new(MainMenu::default()));
                    }
                    input::UiAction::PauseMenu => {
                        return SceneSwitch::Push(Box::new(PauseMenu::default()));
                    }
                    input::UiAction::GameOverMenu => {
                        return SceneSwitch::Reinit(Box::new(GameOver::default()));
                    }
                },
                input::PlayerInput::Game(action) => match self.state {
                    // Skip player input when the engine asks us to
                    GameState::NeedPlayerInput => self.set_player_action(*action),
                    _ => self.state = GameState::Ticking,
                },
            }
        };

        self.state = process_actors(
            &mut self.world,
            resources,
            &self.map,
            &mut self.turn_history,
        );
        self.scheduler
            .execute(&mut self.world, resources, &mut self.map);

        SceneSwitch::None
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        ctx: &mut ggez::Context,
        canvas: &mut Canvas,
    ) -> ggez::GameResult<()> {
        self.screen
            .draw_game(ctx, canvas, &self.world, resources, &self.map);

        Ok(())
    }
}
