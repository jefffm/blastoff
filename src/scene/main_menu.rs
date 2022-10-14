use std::fmt;

use macroquad::prelude::*;

use crate::{
    game::consts::{FONT_SIZE_PIXELS, HEADER_FONT_SIZE, PIXEL_RECT, TITLE_HEADER},
    resource::Resources,
    util::{Scene, SceneSwitch},
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
impl Scene<Resources> for MainMenu {
    fn poll_input(&mut self, resources: &mut Resources) -> anyhow::Result<()> {
        let selection = self.state.selection();

        let can_continue = false; // TODO: implement save/continue
        let entries = selection.entries(can_continue);

        self.state = if is_key_pressed(KeyCode::Escape) {
            MenuResult::Unconfirmed {
                selection: MainMenuSelection::Quit,
            }
        } else if is_key_pressed(KeyCode::Up) {
            let idx = entries
                .iter()
                .position(|&x| x == *selection)
                .expect("MainMenuSelection");
            MenuResult::Unconfirmed {
                selection: entries[(idx + entries.len() - 1) % entries.len()],
            }
        } else if is_key_pressed(KeyCode::Down) {
            let idx = entries
                .iter()
                .position(|&x| x == *selection)
                .expect("MainMenuSelection");
            MenuResult::Unconfirmed {
                selection: entries[(idx + 1) % entries.len()],
            }
        } else if is_key_pressed(KeyCode::Enter) {
            MenuResult::Confirmed {
                selection: *selection,
            }
        } else {
            MenuResult::Unconfirmed {
                selection: *selection,
            }
        };

        Ok(())
    }

    fn update(&mut self, resources: &mut Resources) -> SceneSwitch<Resources> {
        match self.state {
            MenuResult::Unconfirmed { selection: _ } => SceneSwitch::None,
            MenuResult::Confirmed {
                selection: selected,
            } => match selected {
                MainMenuSelection::NewGame => {
                    SceneSwitch::Push(Box::new(GalaxyTravel::create(resources)))
                }
                MainMenuSelection::Continue => SceneSwitch::None, // TODO: implement save/load/continue
                MainMenuSelection::Quit => {
                    ::std::process::exit(0);
                }
            },
        }
    }

    fn draw(&mut self, resources: &mut Resources) -> anyhow::Result<()> {
        let selection = self.state.selection();
        let can_continue: bool = false;

        draw_text_ex(
            TITLE_HEADER,
            PIXEL_RECT.center().x as f32 / 2.,
            PIXEL_RECT.center().y as f32 / 2.,
            TextParams {
                font: resources.assets.font,
                font_size: HEADER_FONT_SIZE,
                ..Default::default()
            },
        );

        let entries = selection.entries(can_continue);
        for (i, entry) in entries.iter().enumerate() {
            let color = if entry == selection { WHITE } else { GRAY };

            draw_text_ex(
                &entry.to_string(),
                PIXEL_RECT.center().x as f32 / 2.,
                FONT_SIZE_PIXELS as f32 * (i as f32 + 6.),
                TextParams {
                    font: resources.assets.font,
                    font_size: FONT_SIZE_PIXELS,
                    color,
                    ..Default::default()
                },
            );
        }

        Ok(())
    }
}
