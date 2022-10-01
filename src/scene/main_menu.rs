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

use super::{GalaxyTravel, MenuResult};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewGame = 0,
    Continue = 1,
    Quit = 2,
}

impl fmt::Display for MainMenuSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            MainMenuSelection::NewGame => "New Game",
            MainMenuSelection::Continue => "Continue",
            MainMenuSelection::Quit => "Quit",
        };
        f.write_str(text)
    }
}

impl MainMenuSelection {
    fn print(
        &self,
        canvas: &mut Canvas,
        resources: &mut Resources,
        y: i32,
        selection: &MainMenuSelection,
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

    pub fn entries(&self, can_continue: bool) -> Vec<Self> {
        if can_continue {
            vec![Self::Continue, Self::NewGame, Self::Quit]
        } else {
            vec![Self::NewGame, Self::Quit]
        }
    }
}

pub struct MainMenu {
    state: MenuResult<MainMenuSelection>,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            state: MenuResult::new(MainMenuSelection::NewGame),
        }
    }
}
impl Scene<Resources, Controls> for MainMenu {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        let selection = self.state.selection();

        let can_continue = false; // TODO: implement save/continue
        let entries = selection.entries(can_continue);

        self.state = match controls.read() {
            None => MenuResult::Unconfirmed {
                selection: *selection,
            },
            Some(key) => match key {
                KeyCode::Escape => MenuResult::Unconfirmed {
                    selection: MainMenuSelection::Quit,
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
        resources: &mut Resources,
        ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        match self.state {
            MenuResult::Unconfirmed { selection: _ } => SceneSwitch::None,
            MenuResult::Confirmed {
                selection: selected,
            } => match selected {
                MainMenuSelection::NewGame => {
                    SceneSwitch::Push(Box::new(GalaxyTravel::create(ctx, resources)))
                }
                MainMenuSelection::Continue => SceneSwitch::None, // TODO: implement save/load/continue
                MainMenuSelection::Quit => {
                    ::std::process::exit(0);
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
        let can_continue: bool = false;
        resources.font.draw_each_char(
            canvas,
            TITLE_HEADER,
            &PixelPoint::new(PIXEL_RECT.center().x, 0),
            None,
        );

        let entries = selection.entries(can_continue);
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
