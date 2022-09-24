use ggez::graphics::{Canvas, DrawParam};
use ggez::input::keyboard::KeyCode;
use std::fmt;

use crate::{
    color::{RGBA8Ext, COMMON},
    game::consts::{PIXEL_RECT, SCREEN_RECT, TITLE_HEADER},
    input::Controls,
    resource::Resources,
    util::{PixelPoint, Scene, SceneSwitch},
};

use super::{MainMenu, MenuResult};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PauseMenuSelection {
    Continue = 0,
    ExitToMainMenu = 1,
}

impl fmt::Display for PauseMenuSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Continue => "Continue",
            Self::ExitToMainMenu => "Exit to main menu",
        };
        f.write_str(text)
    }
}

impl PauseMenuSelection {
    fn print(
        &self,
        canvas: &mut Canvas,
        resources: &Resources,
        y: i32,
        selection: &PauseMenuSelection,
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

    pub fn entries(&self) -> Vec<PauseMenuSelection> {
        vec![
            PauseMenuSelection::Continue,
            PauseMenuSelection::ExitToMainMenu,
        ]
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PauseMenuResult {
    NoSelection { selected: PauseMenuSelection },
    Selected { selected: PauseMenuSelection },
}
pub fn draw_pause_menu(
    _canvas: &mut Canvas,
    _selection: &PauseMenuSelection,
    _resources: &mut Resources,
) {
}

pub struct PauseMenu {
    state: MenuResult<PauseMenuSelection>,
}

impl Default for PauseMenu {
    fn default() -> Self {
        Self {
            state: MenuResult::new(PauseMenuSelection::Continue),
        }
    }
}
impl Scene<Resources, Controls> for PauseMenu {
    fn input(&mut self, _resources: &mut Resources, mut controls: Controls, _started: bool) {
        let selection = self.state.selection();
        let entries = selection.entries();

        let _result = match controls.read() {
            None => PauseMenuResult::NoSelection {
                selected: *selection,
            },
            Some(key) => match key {
                KeyCode::Escape => PauseMenuResult::NoSelection {
                    selected: PauseMenuSelection::Continue,
                },
                KeyCode::Up => {
                    let idx = entries
                        .iter()
                        .position(|&x| x == *selection)
                        .expect("MainMenuSelection");
                    PauseMenuResult::NoSelection {
                        selected: entries[(idx + entries.len() - 1) % entries.len()],
                    }
                }
                KeyCode::Down => {
                    let idx = entries
                        .iter()
                        .position(|&x| x == *selection)
                        .expect("MainMenuSelection");
                    PauseMenuResult::NoSelection {
                        selected: entries[(idx + 1) % entries.len()],
                    }
                }
                KeyCode::Return => PauseMenuResult::Selected {
                    selected: *selection,
                },
                _ => PauseMenuResult::NoSelection {
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
            MenuResult::Selected { selected } => match selected {
                PauseMenuSelection::Continue => SceneSwitch::Pop,
                PauseMenuSelection::ExitToMainMenu => {
                    SceneSwitch::Reinit(Box::new(MainMenu::default()))
                }
            },
        }
    }

    fn draw(&mut self, resources: &mut Resources, canvas: &mut Canvas) -> ggez::GameResult<()> {
        let selection = self.state.selection();
        resources.font.draw_each_char(
            canvas,
            TITLE_HEADER,
            &PixelPoint::new(PIXEL_RECT.center().x, 0),
            None,
        );

        for (i, entry) in selection.entries().iter().enumerate() {
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
