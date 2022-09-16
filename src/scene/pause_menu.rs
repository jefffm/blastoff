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
    fn print(&self, y: i32, selection: &PauseMenuSelection, draw_batch: &mut DrawBatch) {
        let fg = if selection == self {
            RGB::named(WHITE)
        } else {
            RGB::named(GRAY)
        };
        draw_batch.print_color_centered(y, self.to_string(), ColorPair::new(fg, RGB::named(BLACK)));
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
pub fn draw_pause_menu(selection: &PauseMenuSelection, draw_batch: &mut DrawBatch) {
    draw_batch.cls();
    draw_batch.print_color_centered(11, TITLE_HEADER, ColorPair::new(WHITE, BLACK));

    for (i, entry) in selection.entries().iter().enumerate() {
        entry.print(14 + i as i32, selection, draw_batch);
    }

    draw_batch.submit(0).expect("DrawBatch submit");
}
