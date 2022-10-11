use ggez::graphics::{Canvas, DrawParam};
use ggez::input::keyboard::KeyCode;
use std::fmt;

use crate::data::{Element, PlanetType};
use crate::game::consts::{SECTOR_HEIGHT, SECTOR_SIZE, SECTOR_WIDTH};
use crate::overworld::{OverworldTile, PlanetInfo, SectorInfo};
use crate::procgen::{seed, Combo, MapTemplate, SectorProcgenLoader, SubMap, WfcGen};
use crate::sector::{FloorKind, Tile};
use crate::util::{OverworldSize, WorldPoint, WorldSize};
use crate::{
    color::{RGBA8Ext, COMMON},
    game::consts::{PIXEL_RECT, SCREEN_RECT},
    input::Controls,
    resource::Resources,
    util::{PixelPoint, Scene, SceneSwitch},
};

use super::{MenuResult, SectorGeneration};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DebugMenuSelection {
    SectorGeneration = 0,
    // OverworldGeneration = 1,
    // GalaxyGeneration = 2,
    Quit = 3,
}

impl fmt::Display for DebugMenuSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            DebugMenuSelection::SectorGeneration => "Sector Generation",
            DebugMenuSelection::Quit => "Quit",
        };
        f.write_str(text)
    }
}

impl DebugMenuSelection {
    fn print(
        &self,
        canvas: &mut Canvas,
        resources: &mut Resources,
        y: i32,
        selection: &DebugMenuSelection,
    ) {
        let fg = if selection == self {
            COMMON.five
        } else {
            COMMON.three
        };
        resources.font.draw_each_char(
            canvas,
            &self.to_string(),
            &PixelPoint::new(SCREEN_RECT.center().x, y),
            Some(DrawParam::default().color(fg.to_ggez_color())),
        );
    }

    pub fn entries(&self) -> Vec<Self> {
        vec![Self::SectorGeneration, Self::Quit]
    }
}

pub struct DebugMenu {
    state: MenuResult<DebugMenuSelection>,
}

impl Default for DebugMenu {
    fn default() -> Self {
        Self {
            state: MenuResult::new(DebugMenuSelection::SectorGeneration),
        }
    }
}
impl Scene<Resources, Controls> for DebugMenu {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        let selection = self.state.selection();
        let entries = selection.entries();

        self.state = match controls.read() {
            None => MenuResult::Unconfirmed {
                selection: *selection,
            },
            Some(key) => match key {
                KeyCode::Escape => MenuResult::Unconfirmed {
                    selection: DebugMenuSelection::Quit,
                },
                KeyCode::Up => {
                    let idx = entries.iter().position(|&x| x == *selection).expect("sel");
                    MenuResult::Unconfirmed {
                        selection: entries[(idx + entries.len() - 1) % entries.len()],
                    }
                }
                KeyCode::Down => {
                    let idx = entries.iter().position(|&x| x == *selection).expect("sel");
                    MenuResult::Unconfirmed {
                        selection: entries[(idx + 1) % entries.len()],
                    }
                }
                KeyCode::Return => MenuResult::Confirmed {
                    selection: *selection,
                },
                _ => MenuResult::Unconfirmed {
                    selection: *selection,
                },
            },
        };
    }

    fn update(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        match self.state {
            MenuResult::Unconfirmed { selection: _ } => SceneSwitch::None,
            MenuResult::Confirmed {
                selection: selected,
            } => {
                let result = match selected {
                    DebugMenuSelection::SectorGeneration => {
                        // TODO: map debug needs a way to test overworlds and planets too
                        let planet_info = PlanetInfo::new(
                            "Test Planet".to_owned(),
                            OverworldSize::new(100, 100),
                            PlanetType::Barren,
                            Element::Fire,
                        );

                        let sector_info = SectorInfo::new(
                            planet_info,
                            OverworldTile::City,
                            WorldSize::new(SECTOR_WIDTH, SECTOR_HEIGHT),
                        );

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

                        let mut history = Vec::new();

                        // Create the loader
                        let mut loader = SectorProcgenLoader::new(mapgen, resources, &mut history);

                        let mut world = hecs::World::new();
                        let _sector = {
                            // Create a new Sector and spawn to a fresh ECS world
                            loader.load(&sector_info, &mut world)
                        };

                        tracing::info!("Pushing MapGeneration to the Scene stack");
                        SceneSwitch::Push(Box::new(SectorGeneration::new(world, history)))
                    }
                    DebugMenuSelection::Quit => {
                        ::std::process::exit(0);
                    }
                };

                // Reset the selection so that we can pop back to this menu
                self.state = MenuResult::Unconfirmed {
                    selection: selected,
                };

                result
            }
        }
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
        canvas: &mut Canvas,
    ) -> ggez::GameResult<()> {
        let selection = self.state.selection();
        resources.font.draw_each_char(
            canvas,
            "Debug Menu",
            &PixelPoint::new(PIXEL_RECT.center().x, 0),
            None,
        );

        let entries = selection.entries();
        for (i, entry) in entries.iter().enumerate() {
            entry.print(
                canvas,
                resources,
                resources.font.char_size.height * (i as i32 + 2), // 2-line gap because title
                selection,
            );
        }

        Ok(())
    }
}
