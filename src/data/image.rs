use std::sync::Arc;

use assets_manager::{
    loader::{ImageLoader, LoadFrom},
    Asset,
};
use ggez::{
    context::Has,
    graphics::{self, ImageFormat},
};
use image::DynamicImage;

pub struct Image(pub Arc<DynamicImage>);

impl Image {
    /// Return the loaded image asset as a ggez::graphics::Image loaded into the wgpu context
    pub fn to_image(&self, gfx: &impl Has<graphics::GraphicsContext>) -> graphics::Image {
        let rgba8 = self.0.to_rgba8();
        let (width, height) = (rgba8.width(), rgba8.height());

        graphics::Image::from_pixels(
            gfx,
            rgba8.as_ref(),
            ImageFormat::Rgba8UnormSrgb,
            width,
            height,
        )
    }
}

impl From<DynamicImage> for Image {
    fn from(img: DynamicImage) -> Self {
        tracing::warn!("here");
        Self(Arc::new(img))
    }
}

impl Asset for Image {
    const EXTENSION: &'static str = "png";
    type Loader = LoadFrom<DynamicImage, ImageLoader>;

    const EXTENSIONS: &'static [&'static str] = &[Self::EXTENSION];

    /// Hot reload isn't going to work until we wire it up in bitmap_font and spritesheet
    const HOT_RELOADED: bool = false;
}
