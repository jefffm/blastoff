use std::rc::Rc;

use crate::{
    input::Controls,
    overworld::Overworld,
    resource::Resources,
    util::{PixelPoint, Scene, SceneSwitch},
};

use super::CutsceneNewPlanet;

enum OverworldMapState {
    NeedsIntroCutscene,
    Ready,
}

pub struct OverworldMap {
    state: OverworldMapState,
    planet: Rc<Overworld>,
}

impl OverworldMap {
    pub fn new(planet: Rc<Overworld>) -> Self {
        Self {
            state: OverworldMapState::NeedsIntroCutscene,
            planet,
        }
    }
}

impl Scene<Resources, Controls> for OverworldMap {
    fn input(&mut self, resources: &mut Resources, controls: &mut Controls, started: bool) {
        // TODO: implement skip cutscene
    }

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
            OverworldMapState::Ready => SceneSwitch::None,
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

        for (pos, tile) in self.planet.iter_tiles() {
            // translate point to viewport
            // draw it
        }

        Ok(())
    }
}
