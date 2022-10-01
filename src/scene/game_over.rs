use ggez::graphics::{Canvas, DrawParam};
use ggez::input::keyboard::KeyCode;
use std::fmt;

use crate::{
    color::{RGBA8Ext, COMMON},
    game::consts::{PIXEL_RECT, SCREEN_RECT},
    input::Controls,
    resource::Resources,
    util::{PixelPoint, Scene, SceneSwitch},
};

use super::{MainMenu, MenuResult};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameOverSelection {
    MainMenu = 0,
    Quit = 1,
}

impl fmt::Display for GameOverSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            GameOverSelection::MainMenu => "Main Menu",
            GameOverSelection::Quit => "Quit",
        };
        f.write_str(text)
    }
}

impl GameOverSelection {
    fn print(
        &self,
        canvas: &mut Canvas,
        resources: &Resources,
        y: i32,
        selection: &GameOverSelection,
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

    pub fn entries(&self) -> Vec<GameOverSelection> {
        vec![GameOverSelection::MainMenu, GameOverSelection::Quit]
    }
}

pub struct GameOver {
    state: MenuResult<GameOverSelection>,
}
impl Default for GameOver {
    fn default() -> Self {
        Self {
            state: MenuResult::new(GameOverSelection::MainMenu),
        }
    }
}
impl Scene<Resources, Controls> for GameOver {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        let selection = self.state.selection();
        let entries = selection.entries();
        self.state = match controls.read() {
            None => MenuResult::Unconfirmed {
                selection: *selection,
            },
            Some(key) => match key {
                KeyCode::Escape => MenuResult::Unconfirmed {
                    selection: GameOverSelection::Quit,
                },
                KeyCode::Up => {
                    let idx = entries.iter().position(|&x| x == *selection).unwrap();
                    MenuResult::Unconfirmed {
                        selection: entries[(idx + entries.len() - 1) % entries.len()],
                    }
                }
                KeyCode::Down => {
                    let idx = entries.iter().position(|&x| x == *selection).unwrap();
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
        }
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
                GameOverSelection::MainMenu => SceneSwitch::Reinit(Box::new(MainMenu::default())),
                GameOverSelection::Quit => {
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
        resources.font.draw_each_char(
            canvas,
            "You are Dead",
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
