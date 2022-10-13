use macroquad::prelude::*;

use crate::color::{RGBA8Ext, EMPTY};

use crate::input::Controls;

use crate::resource::Resources;

use crate::util::SceneStack;

use crate::game::consts;

// use super::{DebugMenu, MainMenu};
use super::MainMenu;

pub struct MainState {
    scene_stack: SceneStack<Resources, Controls>,
}

impl MainState {
    pub fn new(resources: Resources) -> Self {
        Self {
            scene_stack: SceneStack::new(resources),
        }
    }

    pub fn init(&mut self) {
        self.scene_stack.push(Box::new(MainMenu::default()))
    }

    pub fn init_debug(&mut self) {
        // self.scene_stack.push(Box::new(DebugMenu::default()))
        self.scene_stack.push(Box::new(MainMenu::default()))
    }

    pub fn poll_input(&mut self) {
        self.scene_stack.poll_input();
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        self.scene_stack.update()?;

        Ok(())
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        // Draw the scene
        self.scene_stack.draw()?;

        Ok(())
    }
}
