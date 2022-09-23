use ggez::graphics::{Canvas, DrawParam};
use std::fmt;

use crate::{
    color::{RGBA8Ext, COMMON},
    game::consts::{PIXEL_RECT, SCREEN_RECT, TITLE_HEADER},
    resource::Resources,
    util::PixelPoint,
};

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
    fn print(
        &self,
        canvas: &mut Canvas,
        resources: &Resources,
        y: i32,
        selection: &PauseMenuSelection,
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
pub fn draw_pause_menu(
    canvas: &mut Canvas,
    selection: &PauseMenuSelection,
    resources: &mut Resources,
) {
    resources.font.draw_each_char(
        canvas,
        TITLE_HEADER,
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
}
