use bracket_lib::prelude::*;
use euclid::Point2D;
use rgb::RGBA8;
use std::fmt;

use crate::{
    game::consts::{PIXEL_RECT, SCREEN_HEIGHT_PIXELS, SCREEN_RECT, SCREEN_WIDTH_PIXELS},
    map::Tile,
    resource::Resources,
    util::{blit, clear, PixelPoint, PixelSize},
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
    fn print(&self, _screen: &mut [u8], _y: i32, selection: &MainMenuSelection) {
        let _fg = if selection == self {
            RGB::named(WHITE)
        } else {
            RGB::named(GRAY)
        };
        // print_color_centered(y, self.to_string(), ColorPair::new(fg, RGB::named(BLACK)));
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
pub fn draw_main_menu(screen: &mut [u8], selection: &MainMenuSelection, resources: &mut Resources) {
    let can_continue: bool = false;

    clear(screen);

    let tree = resources
        .atlas
        .create_view(PixelPoint::new(8, 8), PixelSize::new(8, 8));

    blit(screen, &PixelPoint::new(30, 30), &tree);

    // TODO: load up bitmap fonts like

    // print_color_centered(
    //     11,
    //     TITLE_HEADER,
    //     ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
    // );

    let entries = selection.entries(can_continue);
    for (i, entry) in entries.iter().enumerate() {
        entry.print(screen, 14 + i as i32, selection);
    }
}
