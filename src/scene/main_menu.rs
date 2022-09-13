use bracket_lib::prelude::*;
use std::fmt;

use crate::game::consts::TITLE_HEADER;

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
    fn print(&self, y: i32, selection: MainMenuSelection, ctx: &mut BTerm) {
        let fg = if &selection == self {
            RGB::named(WHITE)
        } else {
            RGB::named(GRAY)
        };
        ctx.print_color_centered(y, fg, RGB::named(BLACK), self.to_string());
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}
pub fn main_menu(
    ctx: &mut BTerm,
    selection: MainMenuSelection,
    can_continue: bool,
) -> MainMenuResult {
    ctx.print_color_centered(11, RGB::named(WHITE), RGB::named(BLACK), TITLE_HEADER);

    let entries = if can_continue {
        vec![
            MainMenuSelection::Continue,
            MainMenuSelection::NewGame,
            MainMenuSelection::Quit,
        ]
    } else {
        vec![MainMenuSelection::NewGame, MainMenuSelection::Quit]
    };
    for (i, entry) in entries.iter().enumerate() {
        entry.print(14 + i as i32, selection, ctx);
    }
    match ctx.key {
        None => MainMenuResult::NoSelection {
            selected: selection,
        },
        Some(key) => match key {
            VirtualKeyCode::Escape => MainMenuResult::NoSelection {
                selected: MainMenuSelection::Quit,
            },
            VirtualKeyCode::Up => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                MainMenuResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            VirtualKeyCode::Down => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                MainMenuResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            VirtualKeyCode::Return => MainMenuResult::Selected {
                selected: selection,
            },
            _ => MainMenuResult::NoSelection {
                selected: selection,
            },
        },
    }
}
