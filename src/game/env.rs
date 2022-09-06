use super::consts;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref GAME_ENV: Mutex<GameEnv> = Mutex::new(GameEnv::new());
}

pub fn env<'a>() -> MutexGuard<'a, GameEnv> {
    GAME_ENV.lock().unwrap()
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct GameEnv {
    pub tile_size: u32,
    pub is_debug_mode: bool,
    pub seed: Option<u64>,
    pub show_map_generation: bool,
}

impl GameEnv {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GameEnv {
    fn default() -> Self {
        Self {
            tile_size: consts::TILE_SIZE,
            is_debug_mode: false,
            seed: None,
            show_map_generation: true,
        }
    }
}
