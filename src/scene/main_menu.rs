use std::fmt;

use macroquad::prelude::*;

use crate::{
    game::consts::{PIXEL_RECT, TILE_SIZE, TITLE_HEADER},
    input::Controls,
    resource::Resources,
    util::{Scene, SceneSwitch},
};

use super::MenuResult;

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
impl Scene<Resources, Controls> for MainMenu {
    fn input(&mut self, resources: &mut Resources) {
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
        } else if is_key_pressed(KeyCode::Return) {
            MenuResult::Confirmed {
                selection: *selection,
            }
        } else {
            MenuResult::Unconfirmed {
                selection: *selection,
            }
        };
    }

    fn update(&mut self, resources: &mut Resources) -> SceneSwitch<Resources, Controls> {
        match self.state {
            MenuResult::Unconfirmed { selection: _ } => SceneSwitch::None,
            MenuResult::Confirmed {
                selection: selected,
            } => match selected {
                MainMenuSelection::NewGame => {
                    // TODO: make next scene work again
                    // SceneSwitch::Push(Box::new(GalaxyTravel::create(resources)))
                    SceneSwitch::None
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

        draw_text(
            TITLE_HEADER,
            PIXEL_RECT.center().x,
            PIXEL_RECT.center().y,
            TILE_SIZE as f32,
            WHITE,
        );

        let entries = selection.entries(can_continue);
        for (i, entry) in entries.iter().enumerate() {
            let text_pos = resources.font.char_size.height * (i as i32 + 2); // 2-line gap because title
            draw_text(
                &selection.to_string(),
                text_pos.x,
                text_pos.y,
                TILE_SIZE as f32,
                WHITE,
            );
        }

        Ok(())
    }
}
