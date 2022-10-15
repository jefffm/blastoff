use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use crate::{
    game::consts::{get_screen_to_pixel_transform_float, SCREEN_RECT, SECTOR_SIZE},
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
    },
};

// use super::{CutsceneNewPlanet, Sector};
use super::CutsceneNewPlanet;

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

impl Scene<Resources> for OverworldMap {
    fn poll_input(&mut self, _resources: &mut Resources) -> anyhow::Result<()> {
        self.input = match self.state {
            OverworldMapState::NeedsIntroCutscene => None,
            OverworldMapState::Ready => {
                // TODO: how to implement "key held down goes fast"?
                if is_key_pressed(KeyCode::Up) {
                    Some(OverworldMapInput::MoveN)
                } else if is_key_pressed(KeyCode::Down) {
                    Some(OverworldMapInput::MoveS)
                } else if is_key_pressed(KeyCode::Left) {
                    Some(OverworldMapInput::MoveW)
                } else if is_key_pressed(KeyCode::Right) {
                    Some(OverworldMapInput::MoveE)
                } else if is_key_pressed(KeyCode::Space) {
                    Some(OverworldMapInput::Activate)
                } else {
                    None
                }
            }
        };

        Ok(())
    }

    fn update(&mut self, resources: &mut Resources) -> SceneSwitch<Resources> {
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
                            // return SceneSwitch::Push(Box::new(Sector::new(sector)));
                        }
                    }
                }
                SceneSwitch::None
            }
        }
    }

    fn draw(&mut self, resources: &mut Resources) -> anyhow::Result<()> {
        resources.assets.monospace_font.draw(
            &format!("{}", (*self.planet).borrow()),
            PixelPoint::new(0, 0),
            None,
            None,
        );

        for overworld_point in self.viewport.visible_points() {
            if let Some(tile) = (*self.planet).borrow().get_sector_info(&overworld_point) {
                tile.render(
                    resources,
                    &self.overworld_to_pixel(overworld_point.to_f32()),
                );
            }
        }

        resources
            .assets
            .tileset
            .spr(469, &self.overworld_to_pixel(self.player_position.to_f32()));

        // Coordinate debugging
        resources.assets.monospace_font.draw(
            &format!("{:?}", self.player_position),
            PixelPoint::new(0, 8),
            None,
            None,
        );

        Ok(())
    }
}
