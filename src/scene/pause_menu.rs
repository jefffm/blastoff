use bracket_lib::prelude::*;
use std::fmt;

use crate::game::consts::TITLE_HEADER;

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
    fn print(&self, y: i32, selection: PauseMenuSelection, ctx: &mut BTerm) {
        let fg = if &selection == self {
            RGB::named(WHITE)
        } else {
            RGB::named(GRAY)
        };
        ctx.print_color_centered(y, fg, RGB::named(BLACK), self.to_string());
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PauseMenuResult {
    NoSelection { selected: PauseMenuSelection },
    Selected { selected: PauseMenuSelection },
}
pub fn pause_menu(ctx: &mut BTerm, selection: PauseMenuSelection) -> PauseMenuResult {
    ctx.print_color_centered(11, RGB::named(WHITE), RGB::named(BLACK), TITLE_HEADER);

    let entries = vec![
        PauseMenuSelection::Continue,
        PauseMenuSelection::ExitToMainMenu,
    ];

    for (i, entry) in entries.iter().enumerate() {
        entry.print(14 + i as i32, selection, ctx);
    }

    match ctx.key {
        None => {
            PauseMenuResult::NoSelection {
                selected: selection,
            }
        }
        Some(key) => match key {
            VirtualKeyCode::Escape => {
                PauseMenuResult::NoSelection {
                    selected: PauseMenuSelection::Continue,
                }
            }
            VirtualKeyCode::Up => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                PauseMenuResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            VirtualKeyCode::Down => {
                let idx = entries
                    .iter()
                    .position(|&x| x == selection)
                    .expect("MainMenuSelection");
                PauseMenuResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            VirtualKeyCode::Return => {
                PauseMenuResult::Selected {
                    selected: selection,
                }
            }
            _ => {
                PauseMenuResult::NoSelection {
                    selected: selection,
                }
            }
        },
    }
}
