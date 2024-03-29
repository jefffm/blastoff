//! Implement a Map Generation debugging tool that allows replaying different map generation methods
use std::fmt;

use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use hecs::World;

use crate::component::Cardinal;
use crate::game::consts::{
    get_screen_to_pixel_transform_float, SCREEN_HEIGHT_PIXELS, TILE_SIZE, UPDATE_INTERVAL_SECS,
};
use crate::input::Controls;
use crate::resource::{Resources, Viewport};
use crate::sector::Map;
use crate::util::{
    PixelPoint, Scene, SceneSwitch, ScreenFloatPoint, VectorExt, ViewportFloatPoint,
    ViewportFloatToScreen, WorldFloatPoint, WorldSpace,
};
use crate::{
    game::consts::SCREEN_RECT,
    util::{TransformExt, ViewportPoint, ViewportRect, ViewportSize, WorldToViewport},
};

const MAP_SHOW_TIME: f32 = 2.0; // seconds

#[derive(Debug, Clone, PartialEq)]
enum PlaybackState {
    Playing,
    Paused,
    Completed,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct MapGenerationCursor {
    state: PlaybackState,
    timer: f32,
    index: usize,
    length: usize,
}
impl MapGenerationCursor {
    pub fn new(length: usize, state: PlaybackState) -> Self {
        Self {
            timer: 0.0,
            index: 0,
            length,
            state,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1).min(self.length - 1);
        self.timer = 0.0;
    }

    pub fn prev(&mut self) {
        self.index = (self.index as i32 - 1).max(0) as usize;
        self.timer = 0.0;
    }

    pub fn rewind(&mut self) {
        self.index = 0;
        self.timer = 0.;
    }

    pub fn playpause(&mut self) {
        self.state = match self.state {
            PlaybackState::Playing => PlaybackState::Paused,
            PlaybackState::Paused => PlaybackState::Playing,
            PlaybackState::Completed => {
                self.rewind();
                PlaybackState::Playing
            }
        }
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
    PlayPause,
    Back,
    Forward,
    EnterPlayback,
    EnterInspect,
    Exit,
    PanN,
    PanS,
    PanE,
    PanW,
    Regenerate,
}

#[derive(Default)]
pub struct SectorGeneration {
    world: World,
    history: Vec<Map>,
    cursor: MapGenerationCursor,
    state: MapGenerationState,
    input: Option<MapGenerationInput>,
    viewport: Viewport<WorldSpace>,
    screen_transform: ViewportFloatToScreen,
}
impl SectorGeneration {
    pub fn new(world: World, history: Vec<Map>) -> Self {
        let t1 = WorldToViewport::default();
        let viewport = Viewport::new(
            ViewportRect::new(
                ViewportPoint::new(0, 0),
                ViewportSize::new(SCREEN_RECT.width() - 2, SCREEN_RECT.height() - 3),
            ),
            t1,
        );

        let screen_transform = ViewportFloatToScreen::from_points(
            ViewportFloatPoint::new(0., 0.),
            ScreenFloatPoint::new(1., 2.),
        );

        let hist_length = history.len();

        Self {
            world,
            history,
            cursor: MapGenerationCursor::new(hist_length, PlaybackState::Playing),
            state: MapGenerationState::default(),
            input: None,
            viewport,
            screen_transform,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.cursor.index >= self.history.len()
    }

    fn world_to_pixel(&self, point: WorldFloatPoint) -> PixelPoint {
        let vp = self.viewport.to_viewport_point_f32(point);
        let sp = self.screen_transform.transform_point(vp);
        get_screen_to_pixel_transform_float()
            .transform_point(sp)
            .to_i32()
    }
}

impl Scene<Resources, Controls> for SectorGeneration {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        if let Some(key) = controls.read() {
            self.input = match self.state {
                // Playback Controls
                MapGenerationState::Playback => match key {
                    KeyCode::Left => Some(MapGenerationInput::Back),
                    KeyCode::Comma => Some(MapGenerationInput::Back),
                    KeyCode::Right => Some(MapGenerationInput::Forward),
                    KeyCode::Period => Some(MapGenerationInput::Forward),
                    KeyCode::Return => Some(MapGenerationInput::Exit),
                    KeyCode::Escape => Some(MapGenerationInput::Exit),
                    KeyCode::Semicolon => Some(MapGenerationInput::EnterInspect),
                    KeyCode::I => Some(MapGenerationInput::EnterInspect),
                    KeyCode::R => Some(MapGenerationInput::Regenerate),
                    KeyCode::Space => Some(MapGenerationInput::PlayPause),
                    _ => None,
                },
                // Inspect Controls
                MapGenerationState::Inspect => match key {
                    KeyCode::Escape => Some(MapGenerationInput::EnterPlayback),
                    KeyCode::Up => Some(MapGenerationInput::PanN),
                    KeyCode::Down => Some(MapGenerationInput::PanS),
                    KeyCode::Left => Some(MapGenerationInput::PanW),
                    KeyCode::Right => Some(MapGenerationInput::PanE),
                    KeyCode::Space => Some(MapGenerationInput::PlayPause),
                    _ => None,
                },
            }
        }
    }

    fn update(
        &mut self,
        _resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        if self.is_complete() {
            self.cursor.state = PlaybackState::Completed;
        }

        match self.cursor.state {
            PlaybackState::Playing => {
                if self.cursor.timer > MAP_SHOW_TIME {
                    self.cursor.next();
                }

                self.cursor.timer += UPDATE_INTERVAL_SECS;
            }
            PlaybackState::Paused => {}
            PlaybackState::Completed => {}
        }

        match self.input.take() {
            Some(input) => match input {
                MapGenerationInput::Back => self.cursor.prev(),
                MapGenerationInput::Forward => self.cursor.next(),
                MapGenerationInput::EnterInspect => {
                    self.state = MapGenerationState::Inspect;
                }
                MapGenerationInput::EnterPlayback => self.state = MapGenerationState::Playback,
                MapGenerationInput::Exit => return SceneSwitch::Pop,
                MapGenerationInput::PanN => self.viewport.update_transform(
                    self.viewport.center().to_f32() + Cardinal::N.to_vector().as_float(),
                ),
                MapGenerationInput::PanS => self.viewport.update_transform(
                    self.viewport.center().to_f32() + Cardinal::S.to_vector().as_float(),
                ),
                MapGenerationInput::PanE => self.viewport.update_transform(
                    self.viewport.center().to_f32() + Cardinal::E.to_vector().as_float(),
                ),
                MapGenerationInput::PanW => self.viewport.update_transform(
                    self.viewport.center().to_f32() + Cardinal::W.to_vector().as_float(),
                ),
                MapGenerationInput::PlayPause => self.cursor.playpause(),
                MapGenerationInput::Regenerate => todo!("Map regeneration not implemented yet"),
            },
            None => match self.state {
                MapGenerationState::Playback => {}
                MapGenerationState::Inspect => {}
            },
        }

        SceneSwitch::None
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
        canvas: &mut Canvas,
    ) -> ggez::GameResult<()> {
        let map = &self.history[self.cursor.index];
        for point in self.viewport.visible_points() {
            if let Some(tile) = map.get(point) {
                tile.render(
                    resources,
                    self.world_to_pixel(point.to_f32()),
                    crate::sector::VisibilityKind::Torch { brightness: 100 },
                );
            }
        }

        match self.state {
            MapGenerationState::Playback => resources.font.draw_each_char(
                canvas,
                "Playback - R to regenerate, ESC to exit, Spacebar = play/pause",
                &PixelPoint::new(0, 0),
                None,
            ),
            MapGenerationState::Inspect => resources.font.draw_each_char(
                canvas,
                "Inspect (arrow keys to move)",
                &PixelPoint::new(0, 0),
                None,
            ),
        }

        match self.cursor.state {
            PlaybackState::Playing => resources.font.draw_each_char(
                canvas,
                &format!("Playing ({})", &self.cursor),
                &PixelPoint::new(0, SCREEN_HEIGHT_PIXELS - (2 * TILE_SIZE.height)),
                None,
            ),
            PlaybackState::Paused => resources.font.draw_each_char(
                canvas,
                &format!("Paused ({})", &self.cursor),
                &PixelPoint::new(0, SCREEN_HEIGHT_PIXELS - (2 * TILE_SIZE.height)),
                None,
            ),
            PlaybackState::Completed => resources.font.draw_each_char(
                canvas,
                &format!("Completed ({})", &self.cursor),
                &PixelPoint::new(0, SCREEN_HEIGHT_PIXELS - (2 * TILE_SIZE.height)),
                None,
            ),
        }

        Ok(())
    }
}
