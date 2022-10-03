use std::rc::Rc;

use crate::{
    game::consts::{get_screen_to_pixel_transform_float, SCREEN_RECT},
    input::Controls,
    overworld::Overworld,
    resource::{Resources, Viewport},
    util::{
        GalaxyVector, OverworldFloatPoint, OverworldPoint, OverworldSpace, OverworldToViewport,
        OverworldVector, PixelPoint, Scene, SceneSwitch, ScreenFloatPoint, TransformExt,
        ViewportFloatPoint, ViewportFloatToScreen, ViewportPoint, ViewportRect, ViewportSize,
        PLAYER,
    },
};

use super::CutsceneNewPlanet;

enum OverworldMapState {
    NeedsIntroCutscene,
    Ready,
}

pub enum OverworldMapInput {
    MoveN,
    MoveS,
    MoveE,
    MoveW,
}

pub struct OverworldMap {
    state: OverworldMapState,
    planet: Rc<Overworld>,
    input: Option<OverworldMapInput>,
    viewport: Viewport<OverworldSpace>,
    screen_transform: ViewportFloatToScreen,
    player_position: OverworldPoint,
}

impl OverworldMap {
    pub fn new(planet: Rc<Overworld>) -> Self {
        let t1 = OverworldToViewport::default();
        let viewport = Viewport::new(
            ViewportRect::new(
                ViewportPoint::new(0, 0),
                ViewportSize::new(SCREEN_RECT.width() - 2, SCREEN_RECT.height() - 3),
            ),
            t1,
        );

        let screen_transform = ViewportFloatToScreen::from_points(
            ViewportFloatPoint::new(0., 0.),
            ScreenFloatPoint::new(2., 2.),
        );

        let player_position = planet.center();
        Self {
            state: OverworldMapState::NeedsIntroCutscene,
            planet,
            input: None,
            viewport,
            screen_transform,
            player_position,
        }
    }

    // TODO: Use floating point and move animation
    fn move_player(&mut self, vector: OverworldVector) {
        self.player_position = self.planet.clamp(self.player_position + vector);
    }

    fn overworld_to_pixel(&self, point: OverworldFloatPoint) -> PixelPoint {
        let vp = self.viewport.to_viewport_point_f32(point);
        let sp = self.screen_transform.transform_point(vp);
        get_screen_to_pixel_transform_float()
            .transform_point(sp)
            .to_i32()
    }
}

impl Scene<Resources, Controls> for OverworldMap {
    fn input(&mut self, resources: &mut Resources, controls: &mut Controls, started: bool) {}

    fn update(
        &mut self,
        resources: &mut Resources,
        ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        match self.state {
            OverworldMapState::NeedsIntroCutscene => {
                // After we return from this scene switch, we're ready
                self.state = OverworldMapState::Ready;
                SceneSwitch::Push(Box::new(CutsceneNewPlanet::new(self.planet.clone())))
            }
            OverworldMapState::Ready => {
                if let Some(input) = self.input.take() {
                    match input {
                        OverworldMapInput::MoveN => self.move_player(OverworldVector::new(0, -1)),
                        OverworldMapInput::MoveS => self.move_player(OverworldVector::new(0, 1)),
                        OverworldMapInput::MoveE => self.move_player(OverworldVector::new(1, 0)),
                        OverworldMapInput::MoveW => self.move_player(OverworldVector::new(-1, 0)),
                    }
                }
                SceneSwitch::None
            }
        }
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult<()> {
        resources
            .font
            .push_text(&format!("{}", self.planet), &PixelPoint::new(0, 0), None);

        for overworld_point in self.viewport.visible_points() {
            if let Some(tile) = self.planet.get_tile(&overworld_point) {
                tile.render(
                    resources,
                    &self.overworld_to_pixel(overworld_point.to_f32()),
                );
            }
        }

        resources.spritesheet.push_sprite(
            PLAYER,
            self.overworld_to_pixel(self.player_position.to_f32()),
        );

        Ok(())
    }
}
