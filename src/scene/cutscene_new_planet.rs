use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use keyframe::{
    ease,
    functions::{EaseInOut, Linear},
    keyframes,
    mint::Point2,
    AnimationSequence,
};
use macroquad::prelude::*;

use crate::{
    game::consts::{TILE_SIZE, UPDATE_INTERVAL_SECS},
    overworld::Overworld,
    resource::Resources,
    util::{easing_function, EasingEnum, PixelPoint, PointExt, Scene, SceneSwitch},
};

const CUTSCENE_LENGTH_SECS: f32 = 4.;

pub struct CutsceneNewPlanet {
    planet: Rc<RefCell<Overworld>>,
    timer: f32,
    planet_pos_animation: AnimationSequence<Point2<f32>>,
    planet_scale_animation: AnimationSequence<f32>,
}

impl CutsceneNewPlanet {
    pub fn new(planet: Rc<RefCell<Overworld>>) -> Self {
        Self {
            planet,
            timer: 0.,
            planet_pos_animation: planet_travel_position_animation(
                PixelPoint::new(0, 0),
                PixelPoint::new(20, 20),
                &EasingEnum::EaseIn,
                CUTSCENE_LENGTH_SECS,
            ),
            planet_scale_animation: planet_travel_scale_animation(
                1.,
                100.,
                &EasingEnum::EaseIn,
                CUTSCENE_LENGTH_SECS,
            ),
        }
    }
}

impl Scene<Resources> for CutsceneNewPlanet {
    fn poll_input(&mut self, _resources: &mut Resources) -> anyhow::Result<()> {
        // TODO: implement skip cutscene
        Ok(())
    }

    fn update(&mut self, _resources: &mut Resources) -> SceneSwitch<Resources> {
        if self.timer > CUTSCENE_LENGTH_SECS {
            SceneSwitch::Pop
        } else {
            self.timer += UPDATE_INTERVAL_SECS;
            SceneSwitch::None
        }
    }

    fn draw(&mut self, resources: &mut Resources) -> anyhow::Result<()> {
        let pos = self.planet_pos_animation.now_strict().unwrap();
        let scale = self.planet_scale_animation.now_strict().unwrap();
        let planet: Ref<Overworld> = (*self.planet).borrow();
        planet.info().draw(
            PixelPoint::new(pos.x.round() as i32, pos.y.round() as i32),
            &resources.assets.tileset,
            Some(scale),
        );

        // Draw text on top
        resources.assets.monospace_font.draw(
            &format!("You travel to {}", (*self.planet).borrow()),
            PixelPoint::new(5 * TILE_SIZE.width, 5 * TILE_SIZE.height),
            None,
            None,
        );

        self.planet_pos_animation
            .advance_by(get_frame_time() as f64);
        self.planet_scale_animation
            .advance_by(get_frame_time() as f64);
        Ok(())
    }

    fn draw_previous(&self) -> bool {
        false
    }
}

pub fn planet_travel_position_animation(
    start_point: PixelPoint,
    end_point: PixelPoint,
    ease_enum: &EasingEnum,
    duration: f32,
) -> AnimationSequence<Point2<f32>> {
    let start_pos: Point2<f32> = start_point.as_mint_f32();
    let end_pos: Point2<f32> = end_point.as_mint_f32();

    if let EasingEnum::EaseInOut3Point = ease_enum {
        let mid_pos = ease(Linear, start_pos, end_pos, 0.33);
        keyframes![
            (start_pos, 0.0, EaseInOut),
            (mid_pos, 0.66 * duration, EaseInOut),
            (end_pos, duration, EaseInOut)
        ]
    } else {
        keyframes![
            (start_pos, 0.0, easing_function(ease_enum)),
            (end_pos, duration, easing_function(ease_enum))
        ]
    }
}

pub fn planet_travel_scale_animation(
    start_scale: f32,
    end_scale: f32,
    ease_enum: &EasingEnum,
    duration: f32,
) -> AnimationSequence<f32> {
    if let EasingEnum::EaseInOut3Point = ease_enum {
        let mid_scale = ease(Linear, start_scale, end_scale, 0.33);
        keyframes![
            (start_scale, 0.0, EaseInOut),
            (mid_scale, 0.66 * duration, EaseInOut),
            (end_scale, duration, EaseInOut)
        ]
    } else {
        keyframes![
            (start_scale, 0.0, easing_function(ease_enum)),
            (end_scale, duration, easing_function(ease_enum))
        ]
    }
}
