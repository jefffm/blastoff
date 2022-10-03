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
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        let selection = self.state.selection();
        let entries = selection.entries();

        self.state = match controls.read() {
            None => MenuResult::Unconfirmed {
                selection: *selection,
            },
            Some(key) => match key {
                KeyCode::Escape => MenuResult::Unconfirmed {
                    selection: PauseMenuSelection::Continue,
                },
                KeyCode::Up => {
                    let idx = entries
                        .iter()
                        .position(|&x| x == *selection)
                        .expect("MainMenuSelection");
                    MenuResult::Unconfirmed {
                        selection: entries[(idx + entries.len() - 1) % entries.len()],
                    }
                }
                KeyCode::Down => {
                    let idx = entries
                        .iter()
                        .position(|&x| x == *selection)
                        .expect("MainMenuSelection");
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
        _resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        match self.state {
            MenuResult::Unconfirmed { selection: _ } => SceneSwitch::None,
            MenuResult::Confirmed {
                selection: selected,
            } => match selected {
                PauseMenuSelection::Continue => SceneSwitch::Pop,
                PauseMenuSelection::ExitToMainMenu => {
                    SceneSwitch::Reinit(Box::new(MainMenu::default()))
                }
            },
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
