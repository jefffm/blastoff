use bracket_lib::prelude::*;
use std::fmt;

use crate::util::clear;

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
    fn print(&self, screen: &mut [u8], y: i32, selection: &GameOverSelection) {
        let fg = if selection == self {
            RGB::named(WHITE)
        } else {
            RGB::named(GRAY)
        };
        // print_color_centered(y, self.to_string(), ColorPair::new(fg, BLACK));
    }

    pub fn entries(&self) -> Vec<GameOverSelection> {
        vec![GameOverSelection::MainMenu, GameOverSelection::Quit]
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoSelection { selected: GameOverSelection },
    Selected { selected: GameOverSelection },
}

pub fn draw_game_over(screen: &mut [u8], selection: &GameOverSelection) {
    clear(screen);
    // print_color_centered(
    //     11,
    //     "You are Dead",
    //     ColorPair::new(RGB::named(DARK_RED), RGB::named(BLACK)),
    // );

    for (i, entry) in selection.entries().iter().enumerate() {
        entry.print(screen, 14 + i as i32, selection);
    }
}
