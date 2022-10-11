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
    overworld::PlanetInfo,
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
        let selected_point = self.state.selection();
        let planet_infos: Vec<&(GalaxyPoint, PlanetInfo)> =
            self.galaxy.iter_planet_infos().collect();
        let selected_idx = planet_infos
            .iter()
            .position(|(point, _)| point == selected_point)
            .expect("Selected point should be a point in this galaxy");

        let carousel = Carousel::new(selected_idx, &planet_infos);
        // TODO: create carousel size constant
        let visible = carousel.visible(5);
        for (i, (point, planet_info)) in visible.enumerate() {
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

            if selected_point == point {
                canvas.draw(
                    &self.selection_rect,
                    DrawParam::default().dest(planet_pixel_point.as_mint_f32()),
                )
            }
        }

        Ok(())
    }
}

/// Translate the desired selected planet to a window of visible planets
/// Maybe something like 1-dimensional viewport rendering: the "center" is selected, while the cursor has a window size (3, 5, 7)
pub struct Carousel<'a, T> {
    selected_idx: usize,
    items: &'a [T],
}

impl<'a, T> Carousel<'a, T> {
    pub fn new(selected_idx: usize, items: &'a [T]) -> Self {
        Self {
            selected_idx,
            items,
        }
    }

    /// return a slice of visible planets with the selected planet centered
    pub fn visible(&self, size: usize) -> impl Iterator<Item = &T> {
        assert!(
            size % 2 != 0,
            "size should be an odd number (it needs to have a center)"
        );

        assert!(
            size <= self.items.len(),
            "Size should be less than the number of items",
        );
        // How many extra items to pad on each side?
        let pad_count = (size - 1) / 2;

        // Preventing overflow with addition, find the first item of the visible window
        let slice_start = ((self.selected_idx + self.items.len()) - pad_count) % self.items.len();

        self.items.iter().cycle().skip(slice_start).take(size)
    }

    pub fn selected(&self) -> &T {
        &self.items[self.selected_idx]
    }

    fn advance(&mut self) {
        self.selected_idx = (self.selected_idx + 1) % self.items.len();
    }

    fn go_back(&mut self) {
        // Prevent idx from going negative by adding the vec length to the idx first
        self.selected_idx = ((self.selected_idx + self.items.len()) - 1) % self.items.len();
    }

    pub fn next_item(&mut self) -> &T {
        self.advance();
        self.selected()
    }

    pub fn prev_item(&mut self) -> &T {
        self.go_back();
        self.selected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor() {
        let items = vec![1, 2, 3, 4, 5];
        let mut cursor = Carousel::new(1, &items);
        assert_eq!(*cursor.selected(), 2);
        assert_eq!(*cursor.next_item(), 3);
        assert_eq!(*cursor.next_item(), 4);
        assert_eq!(*cursor.next_item(), 5);
        assert_eq!(*cursor.next_item(), 1);
        assert_eq!(*cursor.prev_item(), 5);
        assert_eq!(*cursor.prev_item(), 4);
        assert_eq!(*cursor.prev_item(), 3);
        assert_eq!(*cursor.prev_item(), 2);
        assert_eq!(*cursor.prev_item(), 1);

        assert_eq!(*cursor.selected(), 1);
        assert_eq!(
            cursor.visible(5).collect::<Vec<_>>(),
            vec![&4, &5, &1, &2, &3]
        );
    }
}
