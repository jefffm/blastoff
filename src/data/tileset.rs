use std::path::Path;

use anyhow::anyhow;
use macroquad::prelude::*;

use crate::util::PixelPoint;

pub struct Tileset {
    tileset: tiled::Tileset,
    texture: Texture2D,
}

impl Tileset {
    pub async fn try_from_file(path: &str) -> anyhow::Result<Self> {
        let content = load_file(path).await?;
        // TODO: Android/WASM compat for tileset path loading. Use the util library?
        let tileset = tiled::Loader::new().load_tsx_tileset_from(&*content, path)?;
        let tileset_image_path = tileset
            .image
            .to_owned()
            .ok_or_else(|| anyhow!("Expected an image for the tileset"))?
            .source;

        let path_str = tileset_image_path
            .to_str()
            .ok_or_else(|| anyhow!("Expected path to be string-able"))?;

        let texture = load_texture(path_str).await?;

        // crispy pixels please
        texture.set_filter(FilterMode::Nearest);

        Ok(Tileset { tileset, texture })
    }

    // TODO: customize sprite rendering functions

    pub fn sprite_rect(&self, ix: u32) -> Rect {
        let sw = self.tileset.tile_width as f32;
        let sh = self.tileset.tile_height as f32;
        let sx = (ix % self.tileset.columns) as f32 * (sw + self.tileset.spacing as f32)
            + self.tileset.margin as f32;
        let sy = (ix / self.tileset.columns) as f32 * (sh + self.tileset.spacing as f32)
            + self.tileset.margin as f32;

        // TODO: configure tiles margin
        Rect::new(sx, sy, sw, sh)
        // Rect::new(sx + 1.1, sy + 1.1, sw - 2.2, sh - 2.2)
    }

    pub fn spr(&self, sprite: u32, dest: &PixelPoint) {
        self.draw_sprite(sprite, dest, None, false)
    }

    pub fn spr_flip_x(&self, sprite: u32, dest: &PixelPoint) {
        self.draw_sprite(sprite, dest, None, true)
    }

    fn draw_sprite(&self, sprite: u32, dest: &PixelPoint, scale: Option<i32>, flip_x: bool) {
        let spr_rect = self.sprite_rect(sprite);

        draw_texture_ex(
            self.texture,
            dest.x as f32,
            dest.y as f32,
            WHITE,
            DrawTextureParams {
                dest_size: scale.map(|scale| vec2(scale as f32, scale as f32)),
                flip_x,
                source: Some(Rect::new(spr_rect.x, spr_rect.y, spr_rect.w, spr_rect.h)),
                ..Default::default()
            },
        );
    }
}
