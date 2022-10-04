use std::{cell::RefCell, rc::Rc};

use ggez::input::keyboard::KeyCode;

use crate::{
    game::consts::{get_screen_to_pixel_transform_float, SCREEN_RECT, SECTOR_SIZE},
    input::Controls,
    overworld::Overworld,
    procgen::{
        seed::{self},
        Combo, MapTemplate, SectorProcgenLoader, SubMap, WfcGen,
    },
    resource::{Resources, Viewport},
    sector::{FloorKind, Tile},
    util::{
        OverworldFloatPoint, OverworldPoint, OverworldSpace, OverworldToViewport, OverworldVector,
        PixelPoint, Scene, SceneSwitch, ScreenFloatPoint, TransformExt, ViewportFloatPoint,
        ViewportFloatToScreen, ViewportPoint, ViewportRect, ViewportSize, WorldPoint, WorldSize,
        PLAYER,
    },
};

use super::{CutsceneNewPlanet, Sector};

enum OverworldMapState {
    NeedsIntroCutscene,
    Ready,
}

pub enum OverworldMapInput {
    MoveN,
    MoveS,
    MoveE,
    MoveW,
    Activate,
}

pub struct OverworldMap {
    state: OverworldMapState,
    planet: Rc<RefCell<Overworld>>,
    input: Option<OverworldMapInput>,
    viewport: Viewport<OverworldSpace>,
    screen_transform: ViewportFloatToScreen,
    player_position: OverworldPoint,
}

impl OverworldMap {
    pub fn new(planet: Rc<RefCell<Overworld>>) -> Self {
        let t1 = OverworldToViewport::default();
        let viewport = Viewport::new(
            ViewportRect::new(
                ViewportPoint::new(0, 0),
                ViewportSize::new(SCREEN_RECT.width() - 2, SCREEN_RECT.height() - 3),
            ),
            t1,
        );

        let screen_transform = ViewportFloatToScreen::from_points(
            ViewportFloatPoint::new(0., 0.),
            ScreenFloatPoint::new(2., 2.),
        );

        let player_position = (*planet).borrow().center();
        Self {
            state: OverworldMapState::NeedsIntroCutscene,
            planet,
            input: None,
            viewport,
            screen_transform,
            player_position,
        }
    }

    // TODO: Use floating point and move animation
    fn move_player(&mut self, vector: OverworldVector) {
        self.player_position = (*self.planet).borrow().clamp(self.player_position + vector);
    }

    fn overworld_to_pixel(&self, point: OverworldFloatPoint) -> PixelPoint {
        let vp = self.viewport.to_viewport_point_f32(point);
        let sp = self.screen_transform.transform_point(vp);
        get_screen_to_pixel_transform_float()
            .transform_point(sp)
            .to_i32()
    }
}

impl Scene<Resources, Controls> for OverworldMap {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        if let Some(key) = controls.read() {
            self.input = match self.state {
                OverworldMapState::NeedsIntroCutscene => None,
                OverworldMapState::Ready => match key {
                    // KeyCode::Escape => Some(OverworldMapInput::Exit),
                    KeyCode::Up => Some(OverworldMapInput::MoveN),
                    KeyCode::Down => Some(OverworldMapInput::MoveS),
                    KeyCode::Left => Some(OverworldMapInput::MoveW),
                    KeyCode::Right => Some(OverworldMapInput::MoveE),
                    KeyCode::Space => Some(OverworldMapInput::Activate),
                    _ => None,
                },
            }
        }
    }

    fn update(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        match self.state {
            OverworldMapState::NeedsIntroCutscene => {
                // After we return from this scene switch, we're ready
                self.state = OverworldMapState::Ready;
                SceneSwitch::Push(Box::new(CutsceneNewPlanet::new(self.planet.clone())))
            }
            OverworldMapState::Ready => {
                if let Some(input) = self.input.take() {
                    match input {
                        OverworldMapInput::MoveN => self.move_player(OverworldVector::new(0, -1)),
                        OverworldMapInput::MoveS => self.move_player(OverworldVector::new(0, 1)),
                        OverworldMapInput::MoveE => self.move_player(OverworldVector::new(1, 0)),
                        OverworldMapInput::MoveW => self.move_player(OverworldVector::new(-1, 0)),
                        OverworldMapInput::Activate => {
                            // TODO: this logic needs to move somewhere else (like how we had it in LoadingScreen)
                            let mapgen = Combo::new(MapTemplate::new(
                                SECTOR_SIZE,
                                Tile::Floor(FloorKind::FloorScenery('~')),
                                vec![
                                    // First create an entire map of craters
                                    SubMap::new(
                                        Box::new(WfcGen::new(seed::CRATERS)),
                                        SECTOR_SIZE,
                                        WorldPoint::new(0, 0),
                                    ),
                                    // Then, create a city in the middle
                                    SubMap::new(
                                        Box::new(WfcGen::new(seed::CITY)),
                                        WorldSize::new(50, 50),
                                        WorldPoint::new(25, 25),
                                    ),
                                ],
                            ));

                            // TODO: this isn't plumbed correctly
                            // TODO: Make history optional so that it's only used for the debug view
                            let mut history = Vec::new();

                            // Create the loader
                            let mut loader =
                                SectorProcgenLoader::new(mapgen, resources, &mut history);

                            let sector = {
                                let mut planet = (*self.planet).borrow_mut();
                                planet.get_sector(&self.player_position).unwrap_or_else(|| {
                                    planet.create_sector(&self.player_position, &mut loader)
                                })
                            };

                            // early return
                            return SceneSwitch::Push(Box::new(Sector::new(sector)));
                        }
                    }
                }
                SceneSwitch::None
            }
        }
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
        _canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult<()> {
        resources.font.push_text(
            &format!("{}", (*self.planet).borrow()),
            &PixelPoint::new(0, 0),
            None,
        );

        for overworld_point in self.viewport.visible_points() {
            if let Some(tile) = (*self.planet).borrow().get_tile(&overworld_point) {
                tile.render(
                    resources,
                    &self.overworld_to_pixel(overworld_point.to_f32()),
                );
            }
        }

        resources.spritesheet.push_sprite(
            PLAYER,
            self.overworld_to_pixel(self.player_position.to_f32()),
        );

        Ok(())
    }
}