use ggez::event::EventHandler;
use ggez::graphics::{BlendMode, Color};
use ggez::{graphics, timer, Context, GameError};

use crate::color::{RGBA8Ext, EMPTY};

use crate::input::Controls;

use crate::resource::Resources;

use crate::util::SceneStack;

use crate::game::consts;

use super::MainMenu;

pub struct MainState {
    controls: Controls,
    canvas_image: graphics::ScreenImage,
    scene_stack: SceneStack<Resources, Controls>,
}

impl MainState {
    pub fn new(resources: Resources, ctx: &mut Context) -> Self {
        Self {
            controls: Controls::default(),
            canvas_image: graphics::ScreenImage::new(
                ctx,
                None,
                1. / consts::SCALING_FACTOR,
                1. / consts::SCALING_FACTOR,
                1,
            ),
            scene_stack: SceneStack::new(ctx, resources),
        }
    }

    pub fn init(&mut self) {
        self.scene_stack.push(Box::new(MainMenu::default()))
    }
}

impl EventHandler for MainState {
    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        self.controls.key_down(input);
        self.scene_stack.input(&mut self.controls, true);

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), GameError> {
        self.controls.key_up(input);
        self.scene_stack.input(&mut self.controls, true);

        Ok(())
    }

    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(consts::UPDATE_FPS) {
            self.scene_stack.update(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas =
            graphics::Canvas::from_screen_image(ctx, &mut self.canvas_image, EMPTY.to_ggez_color());
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        canvas.set_blend_mode(BlendMode::REPLACE);

        self.scene_stack.draw(&mut canvas);

        // Write out the InstanceArrays for BitmapFonts and SpriteSheets
        // TODO: how can we share Resources with MainState as well as everything else?
        // canvas.draw(
        //     &self.resources.font,
        //     graphics::DrawParam::new().dest([0., 0.]),
        // );
        canvas.finish(ctx)?;

        let mut outer_canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        outer_canvas.set_sampler(graphics::Sampler::nearest_clamp());

        let image = self.canvas_image.image(ctx);

        outer_canvas.draw(
            &image,
            graphics::DrawParam::new()
                .dest([0., 0.])
                .scale([consts::SCALING_FACTOR, consts::SCALING_FACTOR]),
        );

        outer_canvas.finish(ctx)?;

        timer::yield_now();
        Ok(())
    }
}
