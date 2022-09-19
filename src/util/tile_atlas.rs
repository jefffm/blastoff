use macroquad::{
    prelude::{Color, Rect, Vec2},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::map::Tile;
use crate::util::ScreenPoint;

/// Is used to split one `Texture2D` into different tiles.
#[derive(Clone, Debug)]
pub struct TileAtlas {
    texture: Texture2D,
    tile_width: f32,
    tile_height: f32,
}

impl TileAtlas {
    /// Initialize the atlas from the texture and tile size.
    pub const fn new(texture: Texture2D, tile_width: f32, tile_height: f32) -> Self {
        Self {
            texture,
            tile_width,
            tile_height,
        }
    }

    /// Draw provided Tiles kind (e.g. `Tiles::Grass`) to the given position.
    pub fn draw_tile(&self, tile: &Tile, pos: &ScreenPoint, color: Color) {
        let (atlas_position_x, atlas_position_y) = tile.value();
        let params = DrawTextureParams {
            dest_size: Some(Vec2::ONE),
            source: Some(Rect {
                x: (self.tile_width + 0.2) * atlas_position_x,
                y: self.tile_height * atlas_position_y,
                w: self.tile_width - 1.0,
                h: self.tile_height - 1.0,
            }),
            rotation: std::f32::consts::PI,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        draw_texture_ex(self.texture, pos.x as f32, pos.y as f32, color, params);
    }
}
