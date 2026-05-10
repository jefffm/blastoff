//! Sector implements a local quadrant on a planet.
//!
//! This is currently a lightweight Macroquad scene: it renders the generated
//! tile map, lets the player move around, and provides a working target for the
//! overworld `Space` activation flow. The older ECS turn scheduler can be wired
//! back in once the rest of the game loop has been ported off the old ggez API.

use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::{is_key_pressed, Color, KeyCode, GRAY, WHITE};

use crate::{
    color::{COMMON, FIRE, PLANT, WATER},
    component::{Player, Position},
    game::consts::{SCREEN_HEIGHT_PIXELS, SCREEN_WIDTH_PIXELS},
    overworld::SectorData,
    resource::Resources,
    sector::{FloorKind, Tile, WallKind},
    util::{PixelPoint, Scene, SceneSwitch, WorldPoint, WorldVector},
};

const FONT_HEIGHT: i32 = 8;
const SECTOR_TILE_SIZE: i32 = 8;
const STATUS_HEIGHT_PIXELS: i32 = FONT_HEIGHT * 2;
const PLAYER_SPRITE: u32 = 218;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SectorInput {
    Move(WorldVector),
    Exit,
}

fn tile_sprite(tile: Tile) -> u32 {
    match tile {
        Tile::Floor(FloorKind::FloorDefault) => 6,
        Tile::Floor(FloorKind::FloorInterior) => 755,
        Tile::Floor(FloorKind::FloorScenery('~')) => 253,
        Tile::Floor(FloorKind::FloorScenery(',')) => 6,
        Tile::Floor(FloorKind::FloorScenery('*')) => 554,
        Tile::Floor(FloorKind::FloorScenery('░')) => 7,
        Tile::Floor(FloorKind::FloorScenery('▒')) => 8,
        Tile::Floor(FloorKind::FloorScenery('▓')) => 9,
        Tile::Floor(FloorKind::FloorScenery(_)) => 6,
        Tile::Wall(WallKind::WallPillar) => 393,
        Tile::Wall(_) => 790,
    }
}

fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Floor(FloorKind::FloorDefault) => COMMON.two,
        Tile::Floor(FloorKind::FloorInterior) => COMMON.four,
        Tile::Floor(FloorKind::FloorScenery('~')) => WATER.three,
        Tile::Floor(FloorKind::FloorScenery(',')) => COMMON.three,
        Tile::Floor(FloorKind::FloorScenery('*')) => FIRE.two,
        Tile::Floor(FloorKind::FloorScenery('░')) => COMMON.two,
        Tile::Floor(FloorKind::FloorScenery('▒')) => COMMON.three,
        Tile::Floor(FloorKind::FloorScenery('▓')) => COMMON.four,
        Tile::Floor(FloorKind::FloorScenery(_)) => PLANT.three,
        Tile::Wall(_) => COMMON.five,
    }
}

pub struct Sector {
    data: Rc<RefCell<SectorData>>,
    input: Option<SectorInput>,
}

impl Sector {
    pub fn new(data: Rc<RefCell<SectorData>>) -> Self {
        Self { data, input: None }
    }

    fn player_position(&self) -> Option<WorldPoint> {
        self.data
            .borrow()
            .world
            .query::<(&Player, &Position)>()
            .iter()
            .next()
            .map(|(_, (_, position))| position.grid_point())
    }

    fn move_player(&mut self, delta: WorldVector) {
        let mut data = self.data.borrow_mut();
        let Some(current) = data
            .world
            .query::<(&Player, &Position)>()
            .iter()
            .next()
            .map(|(_, (_, position))| position.grid_point())
        else {
            tracing::warn!("Sector scene has no player entity to move");
            return;
        };

        let next = current + delta;
        let can_move = data
            .map
            .get(next)
            .map(|tile| tile.is_passable())
            .unwrap_or(false);

        if !can_move {
            tracing::debug!(?current, ?next, "Blocked sector move");
            return;
        }

        for (_, (_, position)) in data.world.query_mut::<(&Player, &mut Position)>() {
            position.set_grid_point(next);
        }
    }
}

impl Scene<Resources> for Sector {
    fn poll_input(&mut self, _resources: &mut Resources) -> anyhow::Result<()> {
        self.input = if is_key_pressed(KeyCode::Escape) {
            Some(SectorInput::Exit)
        } else if is_key_pressed(KeyCode::Up) {
            Some(SectorInput::Move(WorldVector::new(0, -1)))
        } else if is_key_pressed(KeyCode::Down) {
            Some(SectorInput::Move(WorldVector::new(0, 1)))
        } else if is_key_pressed(KeyCode::Left) {
            Some(SectorInput::Move(WorldVector::new(-1, 0)))
        } else if is_key_pressed(KeyCode::Right) {
            Some(SectorInput::Move(WorldVector::new(1, 0)))
        } else {
            None
        };

        Ok(())
    }

    fn update(&mut self, _resources: &mut Resources) -> SceneSwitch<Resources> {
        match self.input.take() {
            Some(SectorInput::Exit) => SceneSwitch::Pop,
            Some(SectorInput::Move(delta)) => {
                self.move_player(delta);
                SceneSwitch::None
            }
            None => SceneSwitch::None,
        }
    }

    fn draw(&mut self, resources: &mut Resources) -> anyhow::Result<()> {
        let data = self.data.borrow();
        let map = &data.map;
        let player_position = self.player_position();

        let view_width = SCREEN_WIDTH_PIXELS / SECTOR_TILE_SIZE;
        let view_height = (SCREEN_HEIGHT_PIXELS - STATUS_HEIGHT_PIXELS) / SECTOR_TILE_SIZE;
        let map_width = map.get_width();
        let map_height = map.get_height();

        let origin = if let Some(player) = player_position {
            let max_x = (map_width - view_width).max(0);
            let max_y = (map_height - view_height).max(0);
            WorldPoint::new(
                (player.x - view_width / 2).clamp(0, max_x),
                (player.y - view_height / 2).clamp(0, max_y),
            )
        } else {
            WorldPoint::new(0, 0)
        };

        resources.assets.monospace_font.draw(
            "Sector view: arrows move, Esc returns to overworld",
            PixelPoint::new(0, 0),
            Some(WHITE),
            None,
        );
        resources.assets.monospace_font.draw(
            &format!(
                "map={}x{} origin={:?} player={:?}",
                map_width, map_height, origin, player_position
            ),
            PixelPoint::new(0, FONT_HEIGHT),
            Some(GRAY),
            None,
        );

        for screen_y in 0..view_height {
            for screen_x in 0..view_width {
                let map_point = WorldPoint::new(origin.x + screen_x, origin.y + screen_y);
                let pixel_point = PixelPoint::new(
                    screen_x * SECTOR_TILE_SIZE,
                    STATUS_HEIGHT_PIXELS + (screen_y * SECTOR_TILE_SIZE),
                );

                if let Some(tile) = map.get(map_point) {
                    resources.assets.tileset.draw(
                        tile_sprite(*tile),
                        pixel_point,
                        Some(tile_color(*tile)),
                        Some(SECTOR_TILE_SIZE as u8),
                        false,
                    );
                }

                if Some(map_point) == player_position {
                    resources.assets.tileset.draw(
                        PLAYER_SPRITE,
                        pixel_point,
                        Some(WHITE),
                        Some(SECTOR_TILE_SIZE as u8),
                        false,
                    );
                }
            }
        }

        Ok(())
    }
}
