use ggez::graphics::{Canvas, DrawParam};
use std::fmt;

use crate::{
    color::{RGBA8Ext, COMMON},
    game::consts::{PIXEL_RECT, SCREEN_RECT},
    resource::Resources,
    util::PixelPoint,
};

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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoSelection { selected: GameOverSelection },
    Selected { selected: GameOverSelection },
}

pub fn draw_game_over(
    canvas: &mut Canvas,
    selection: &GameOverSelection,
    resources: &mut Resources,
) {
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
}
