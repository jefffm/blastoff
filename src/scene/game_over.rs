use bracket_lib::prelude::*;
use std::fmt;

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
    fn print(&self, y: i32, selection: GameOverSelection, ctx: &mut BTerm) {
        let fg = if &selection == self {
            RGB::named(WHITE)
        } else {
            RGB::named(GRAY)
        };
        ctx.print_color_centered(y, fg, RGB::named(BLACK), self.to_string());
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoSelection { selected: GameOverSelection },
    Selected { selected: GameOverSelection },
}

pub fn game_over(ctx: &mut BTerm, selection: GameOverSelection) -> GameOverResult {
    ctx.print_color_centered(11, RGB::named(DARK_RED), RGB::named(BLACK), "You are Dead");

    let entries = vec![GameOverSelection::MainMenu, GameOverSelection::Quit];
    for (i, entry) in entries.iter().enumerate() {
        entry.print(14 + i as i32, selection, ctx);
    }
    match ctx.key {
        None => GameOverResult::NoSelection {
            selected: selection,
        },
        Some(key) => match key {
            VirtualKeyCode::Escape => GameOverResult::NoSelection {
                selected: GameOverSelection::Quit,
            },
            VirtualKeyCode::Up => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                GameOverResult::NoSelection {
                    selected: entries[(idx + entries.len() - 1) % entries.len()],
                }
            }
            VirtualKeyCode::Down => {
                let idx = entries.iter().position(|&x| x == selection).unwrap();
                GameOverResult::NoSelection {
                    selected: entries[(idx + 1) % entries.len()],
                }
            }
            VirtualKeyCode::Return => GameOverResult::Selected {
                selected: selection,
            },
            _ => GameOverResult::NoSelection {
                selected: selection,
            },
        },
    }
}
