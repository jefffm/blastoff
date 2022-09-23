use ggez::graphics::{Canvas, DrawParam};
use std::fmt;

use crate::{
    color::{RGBA8Ext, COMMON},
    game::consts::{PIXEL_RECT, SCREEN_RECT, TITLE_HEADER},
    resource::Resources,
    util::PixelPoint,
};

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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}
pub fn draw_main_menu(
    canvas: &mut Canvas,
    selection: &MainMenuSelection,
    resources: &mut Resources,
) {
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
}
