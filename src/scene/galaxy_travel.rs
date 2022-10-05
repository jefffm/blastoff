//! The Galaxy scene allows players to select a planet to travel to, and then initialize an Overworld Scene to push to the Scene Stack.

use ggez::{
    context::Has,
    graphics::{self, DrawParam},
    input::keyboard::KeyCode,
};

use crate::{
    color::{RGBA8Ext, FIRE},
    galaxy::Galaxy,
    game::consts::{MAX_PLANET_SPRITE_SIZE, TILE_SIZE},
    input::Controls,
    procgen::{GalaxyGenerator, OverworldProcgenLoader, StaticGalaxy, StaticPlanet},
    resource::Resources,
    util::{GalaxyPoint, PixelPoint, PointExt, Scene, SceneSwitch},
};

use super::{MenuResult, OverworldMap};

/// The GalaxyTravel Scene allows players to select a planet for landing
pub struct GalaxyTravel {
    galaxy: Galaxy,
    state: MenuResult<GalaxyPoint>,
    selection_rect: graphics::Mesh,
}

impl GalaxyTravel {
    pub fn new(
        galaxy: Galaxy,
        state: MenuResult<GalaxyPoint>,
        selection_rect: graphics::Mesh,
    ) -> Self {
        Self {
            galaxy,
            state,
            selection_rect,
        }
    }

    /// Use Resources to procgen a Galaxy of planets
    pub fn create(
        gfx: &impl Has<graphics::GraphicsContext>,
        resources: &mut Resources,
    ) -> GalaxyTravel {
        let mut loader = StaticGalaxy {};
        let galaxy = loader.generate(resources);

        // Initialize selection with whatever's first in the vec
        let state = MenuResult::Unconfirmed {
            selection: galaxy
                .iter_planet_infos()
                .map(|(point, _info)| *point)
                .next()
                .expect("any planets exist"),
        };

        let selection_rect = graphics::Mesh::new_rectangle(
            gfx,
            graphics::DrawMode::Stroke(graphics::StrokeOptions::DEFAULT),
            graphics::Rect::new_i32(
                0,
                0,
                MAX_PLANET_SPRITE_SIZE as i32,
                MAX_PLANET_SPRITE_SIZE as i32,
            ),
            FIRE.three.to_ggez_color(),
        )
        .expect("clear rect");

        Self::new(galaxy, state, selection_rect)
    }
}

impl Scene<Resources, Controls> for GalaxyTravel {
    fn input(&mut self, _resources: &mut Resources, controls: &mut Controls, _started: bool) {
        let selection = self.state.selection();
        let planets: Vec<_> = self.galaxy.iter_planet_infos().collect();

        self.state = match controls.read() {
            None => MenuResult::Unconfirmed {
                selection: *selection,
            },
            Some(key) => match key {
                KeyCode::Escape => todo!("Return to main menu"),
                KeyCode::Left => {
                    let idx = planets
                        .iter()
                        .position(|(point, _overworld)| point == selection)
                        .unwrap();

                    let (new_selection, _) = planets[(idx + planets.len() - 1) % planets.len()];

                    MenuResult::Unconfirmed {
                        selection: *new_selection,
                    }
                }
                KeyCode::Right => {
                    let idx = planets
                        .iter()
                        .position(|(point, _overworld)| point == selection)
                        .unwrap();

                    let (new_selection, _) = planets[(idx + planets.len() + 1) % planets.len()];

                    MenuResult::Unconfirmed {
                        selection: *new_selection,
                    }
                }
                KeyCode::Return => MenuResult::Confirmed {
                    selection: *selection,
                },
                _ => MenuResult::Unconfirmed {
                    selection: *selection,
                },
            },
        };
    }

    fn update(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
    ) -> SceneSwitch<Resources, Controls> {
        match self.state {
            MenuResult::Unconfirmed { selection: _ } => SceneSwitch::None,
            MenuResult::Confirmed {
                selection: galaxy_point,
            } => {
                if let Some(planet) = self.galaxy.get_planet(&galaxy_point) {
                    // If the planet already exists, don't recreate it!
                    SceneSwitch::Push(Box::new(OverworldMap::new(planet)))
                } else {
                    // Create the planet when selected
                    let overworld_gen = StaticPlanet {};
                    let mut loader = OverworldProcgenLoader::new(overworld_gen, resources);
                    let planet = self.galaxy.create_planet(&galaxy_point, &mut loader);
                    SceneSwitch::Push(Box::new(OverworldMap::new(planet)))
                }
            }
        }
    }

    fn draw(
        &mut self,
        resources: &mut Resources,
        _ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult<()> {
        // TODO: planets should be arranged in a Bootstrap Carousel-style left/right scrolley thing
        for (i, (point, planet_info)) in self.galaxy.iter_planet_infos().enumerate() {
            let y = i as i32 * MAX_PLANET_SPRITE_SIZE as i32;

            resources.font.push_text(
                &format!("{} at {:?}", planet_info, *point),
                &PixelPoint::new(2 * MAX_PLANET_SPRITE_SIZE as i32, y),
                None,
            );

            let planet_pixel_point = PixelPoint::new(1 * TILE_SIZE.width, y);
            resources
                .spritesheet
                .push_sprite(planet_info.sprite(), planet_pixel_point);

            if let MenuResult::Unconfirmed {
                selection: selected_point,
            } = self.state
            {
                if selected_point == *point {
                    canvas.draw(
                        &self.selection_rect,
                        DrawParam::default().dest(planet_pixel_point.into_mint_f32()),
                    )
                }
            }
        }

        Ok(())
    }
}
