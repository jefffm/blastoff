//! Implement a Map Generation debugging tool that allows replaying different map generation methods
use std::fmt;

use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use hecs::World;

use crate::camera;
use crate::component::Cardinal;
use crate::game::consts::UPDATE_INTERVAL_SECS;
use crate::input::Controls;
use crate::map::Map;
use crate::resource::{Resources, Viewport};
use crate::util::{PixelPoint, Scene, SceneSwitch};
use crate::{
    game::consts::SCREEN_RECT,
    util::{
        ScreenPoint, TransformExt, ViewportPoint, ViewportRect, ViewportSize, ViewportToScreen,
        WorldToViewport,
    },
};

const MAP_SHOW_TIME: f32 = 2.0; // seconds

#[derive(Debug, Default, Clone, PartialEq)]
struct MapGenerationCursor {
    timer: f32,
    index: usize,
    length: usize,
}
impl MapGenerationCursor {
    pub fn new(length: usize) -> Self {
        Self {
            timer: 0.0,
            index: 0,
            length,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1).min(self.length);
        self.timer = 0.0;
    }

    pub fn prev(&mut self) {
        self.index = (self.index - 1).max(0);
        self.timer = 0.0;
    }
}

impl fmt::Display for MapGenerationCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("frame {} of {}", self.index + 1, self.length))
    }
}

enum MapGenerationState {
    Playback,
    Inspect,
}

impl Default for MapGenerationState {
    fn default() -> Self {
        Self::Playback
    }
}

enum MapGenerationInput {
    Back,
    Forward,
    Inspect,
    Resume,
    Exit,
    PanN,
    PanS,
    PanE,
    PanW,
}

#[derive(Default)]
pub struct MapGeneration {
    world: World,
    history: Vec<Map>,
    cursor: MapGenerationCursor,
    state: MapGenerationState,
    input: Option<MapGenerationInput>,
    viewport: Viewport,
    screen_transform: ViewportToScreen,
}
impl MapGeneration {
    pub fn new(world: World, history: Vec<Map>) -> Self {
        let t1 = WorldToViewport::default();
        let viewport = Viewport::new(
            ViewportRect::new(
                ViewportPoint::new(0, 0),
                ViewportSize::new(SCREEN_RECT.width() - 2, SCREEN_RECT.height() - 2),
            ),
            t1,
        );

        let screen_transform =
            ViewportToScreen::from_points(ViewportPoint::new(0, 0), ScreenPoint::new(2, 2));

        let hist_length = history.len();

        Self {
            world,
            history,
            cursor: MapGenerationCursor::new(hist_length),
            state: MapGenerationState::default(),
            input: None,
            viewport,
            screen_transform,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.cursor.index >= self.history.len()
    }
}

impl Scene<Resources, Controls> for MapGeneration {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        if let Some(key) = controls.read() {
            self.input = match self.state {
                // Playback Controls
                MapGenerationState::Playback => {
                    match (key, controls.control, controls.alt, controls.shift) {
                        (KeyCode::Left, _, _, false) => Some(MapGenerationInput::Back),
                        (KeyCode::Right, _, _, false) => Some(MapGenerationInput::Forward),
                        (KeyCode::Return, _, _, false) => Some(MapGenerationInput::Exit),
                        (KeyCode::Semicolon, _, _, false) => Some(MapGenerationInput::Inspect),
                        (KeyCode::I, _, _, false) => Some(MapGenerationInput::Inspect),
                        _ => None,
                    }
                }
                // Inspect Controls
                MapGenerationState::Inspect => {
                    match (key, controls.control, controls.alt, controls.shift) {
                        (KeyCode::Escape, _, _, false) => Some(MapGenerationInput::Resume),
                        (KeyCode::Up, _, _, false) => Some(MapGenerationInput::PanN),
                        (KeyCode::Down, _, _, false) => Some(MapGenerationInput::PanS),
                        (KeyCode::Left, _, _, false) => Some(MapGenerationInput::PanW),
                        (KeyCode::Right, _, _, false) => Some(MapGenerationInput::PanE),
                        _ => None,
                    }
                }
            }
        }
    }

    fn update(
        &mut self,
        _resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        self.cursor.timer += UPDATE_INTERVAL_SECS;

        match self.input.take() {
            Some(input) => match input {
                MapGenerationInput::Back => self.cursor.prev(),
                MapGenerationInput::Forward => self.cursor.next(),
                MapGenerationInput::Inspect => {
                    tracing::info!("Entering inspect mode");
                    self.state = MapGenerationState::Inspect;
                }
                MapGenerationInput::Resume => {
                    tracing::info!("Resuming playback");
                    self.state = MapGenerationState::Playback
                }
                MapGenerationInput::Exit => return SceneSwitch::Pop,
                MapGenerationInput::PanN => self
                    .viewport
                    .update_transform(self.viewport.center() + *Cardinal::N.to_vector()),
                MapGenerationInput::PanS => self
                    .viewport
                    .update_transform(self.viewport.center() + *Cardinal::S.to_vector()),
                MapGenerationInput::PanE => self
                    .viewport
                    .update_transform(self.viewport.center() + *Cardinal::E.to_vector()),
                MapGenerationInput::PanW => self
                    .viewport
                    .update_transform(self.viewport.center() + *Cardinal::W.to_vector()),
            },
            None => match self.state {
                MapGenerationState::Playback => {
                    if self.cursor.timer > MAP_SHOW_TIME {
                        self.cursor.next();
                    }

                    if self.is_complete() {
                        // If we're done, return to the debug menu
                        return SceneSwitch::Pop;
                    }
                }
                MapGenerationState::Inspect => {}
            },
        }

        SceneSwitch::None
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        ctx: &mut ggez::Context,
        canvas: &mut Canvas,
    ) -> ggez::GameResult<()> {
        // TODO: implement zooming for map debug
        camera::render_debug_map(
            ctx,
            canvas,
            &mut self.viewport,
            &self.screen_transform,
            resources,
            &self.history[self.cursor.index],
            true,
            self.cursor.index,
        );

        match self.state {
            MapGenerationState::Playback => resources.font.draw_each_char(
                canvas,
                &format!("Playback ({})", &self.cursor),
                &PixelPoint::new(0, 0),
                None,
            ),
            MapGenerationState::Inspect => resources.font.draw_each_char(
                canvas,
                &format!("Inspect ({})", &self.cursor),
                &PixelPoint::new(0, 0),
                None,
            ),
        }

        Ok(())
    }
}
