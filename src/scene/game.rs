use ggez::event::EventHandler;
use ggez::graphics::{BlendMode, Canvas, Color};
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, timer, Context, GameError};
use hecs::World;
use tracing::info;

use crate::camera::Screen;
use crate::color::{RGBA8Ext, EMPTY};
use crate::component::{Actor, ActorKind, Player};
use crate::game::consts::VIEWPORT_SCREEN_POINT;
use crate::game::{consts, process_actors, PlayGame, RunState, TurnsHistory};
use crate::input::{Controls, PlayerAction, PlayerInput, UiAction};
use crate::map::{Loader, Map, WfcGen};
use crate::resource::Resources;
use crate::scene::GameOverSelection;
use crate::scene::MapGenerationState;
use crate::scene::PauseMenuSelection;
use crate::system::{build_systems, Scheduler};
use crate::util::{Scene, SceneStack, SceneSwitch, TransformExt, ViewportPoint, ViewportToScreen};
use crate::{game, input};

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
    input: PlayerInput,
    state: GameState,
    screen: Screen,
    scheduler: Scheduler,
    turn_number: u32,
    turn_history: TurnsHistory,
}

impl Game {
    pub fn new(map: Map, world: World) -> Self {
        Self {
            map,
            world,
            input: PlayerInput::Undefined,
            state: GameState::Ticking,
            screen: Screen::new(ViewportToScreen::from_points(
                ViewportPoint::new(0, 0),
                VIEWPORT_SCREEN_POINT,
            )),
            scheduler: build_systems(),
            turn_number: 0,
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
    fn input(&mut self, resources: &mut Resources, mut controls: Controls, started: bool) {
        self.input = match controls.read() {
            None => PlayerInput::Undefined,
            Some(key) => match (key, controls.control, controls.alt, controls.shift) {
                (KeyCode::Left, _, _, false) => PlayerInput::Game(PlayerAction::MoveWest),
                (KeyCode::Left, _, _, true) => PlayerInput::Game(PlayerAction::MoveSouthWest),
                (KeyCode::Right, _, _, false) => PlayerInput::Game(PlayerAction::MoveEast),
                (KeyCode::Right, _, _, true) => PlayerInput::Game(PlayerAction::MoveNorthEast),
                (KeyCode::Up, _, _, false) => PlayerInput::Game(PlayerAction::MoveNorth),
                (KeyCode::Up, _, _, true) => PlayerInput::Game(PlayerAction::MoveNorthWest),
                (KeyCode::Down, _, _, false) => PlayerInput::Game(PlayerAction::MoveSouth),
                (KeyCode::Down, _, _, true) => PlayerInput::Game(PlayerAction::MoveSouthEast),
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
                (KeyCode::Space, _, _, _) => PlayerInput::Game(PlayerAction::PassTurn),
                (KeyCode::Escape, _, _, _) => PlayerInput::Ui(UiAction::PauseMenu),
                other => {
                    tracing::debug!("unhandled keypress: {:?}", other);
                    PlayerInput::Undefined
                }
            },
        }
    }

    fn update(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        match &self.input {
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
            input::PlayerInput::Undefined => {}
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

    fn draw(&mut self, resources: &mut Resources, canvas: &mut Canvas) -> ggez::GameResult<()> {
        self.screen
            .draw_game(canvas, &self.world, resources, &self.map);

        Ok(())
    }
}
