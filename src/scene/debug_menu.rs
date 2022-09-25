use ggez::graphics::{Canvas, DrawParam};
use ggez::input::keyboard::KeyCode;
use hecs::World;
use std::fmt;

use crate::map::Map;
use crate::{
    color::{RGBA8Ext, COMMON},
    game::consts::{PIXEL_RECT, SCREEN_RECT},
    input::Controls,
    resource::Resources,
    util::{PixelPoint, Scene, SceneSwitch},
};

use super::{Initialization, MapGeneration, MenuResult};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DebugMenuSelection {
    MapGeneration = 0,
    Quit = 1,
}

impl fmt::Display for DebugMenuSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            DebugMenuSelection::MapGeneration => "Map Generation",
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
        vec![Self::MapGeneration, Self::Quit]
    }
}

pub struct DebugMenu {
    state: MenuResult<DebugMenuSelection>,
}

impl Default for DebugMenu {
    fn default() -> Self {
        Self {
            state: MenuResult::new(DebugMenuSelection::MapGeneration),
        }
    }
}
impl Scene<Resources, Controls> for DebugMenu {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        let selection = self.state.selection();

        let entries = selection.entries();

        self.state = match controls.read() {
            None => MenuResult::NoSelection {
                selected: *selection,
            },
            Some(key) => match key {
                KeyCode::Escape => MenuResult::NoSelection {
                    selected: DebugMenuSelection::Quit,
                },
                KeyCode::Up => {
                    let idx = entries.iter().position(|&x| x == *selection).expect("sel");
                    MenuResult::NoSelection {
                        selected: entries[(idx + entries.len() - 1) % entries.len()],
                    }
                }
                KeyCode::Down => {
                    let idx = entries.iter().position(|&x| x == *selection).expect("sel");
                    MenuResult::NoSelection {
                        selected: entries[(idx + 1) % entries.len()],
                    }
                }
                KeyCode::Return => MenuResult::Selected {
                    selected: *selection,
                },
                _ => MenuResult::NoSelection {
                    selected: *selection,
                },
            },
        };
    }

    fn update(
        &mut self,
        _resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        match self.state {
            MenuResult::NoSelection { selected: _ } => SceneSwitch::None,
            MenuResult::Selected { selected } => {
                let result = match selected {
                    DebugMenuSelection::MapGeneration => SceneSwitch::Push(Box::new(
                        Initialization::new(|world: World, _map: Map, history: Vec<Map>| {
                            SceneSwitch::Replace(Box::new(MapGeneration::new(world, history)))
                        }),
                    )),
                    DebugMenuSelection::Quit => {
                        ::std::process::exit(0);
                    }
                };

                // Reset the selection so that we can pop back to this menu
                self.state = MenuResult::NoSelection { selected };

                result
            }
        }
    }

    fn draw(&mut self, resources: &mut Resources, canvas: &mut Canvas) -> ggez::GameResult<()> {
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
