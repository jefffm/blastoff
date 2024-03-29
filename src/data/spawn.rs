use hecs::{Entity, World};
use rgb::RGBA8;
use serde::Deserialize;

use crate::{
    camera::Glyph,
    component::{
        Actor as ActorComponent, ActorKind, InitialBehavior, Player, Position, Renderable, Viewshed,
    },
    util::WorldPoint,
};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Actor {
    name: String,
    glyph: char,
    fg: RGBA8,
    bg: RGBA8,
    #[serde(default)]
    zorder: u32,
    view_range: i32,
    energy_capacity: i32,
    movement_cost: i32,
    behavior: InitialBehavior,

    #[serde(default)]
    is_player: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum SpawnEntry {
    Actor(Actor),
}

trait Spawnable {
    fn into_spawn(self, point: WorldPoint, world: &mut World) -> Entity;
}

impl Spawnable for Actor {
    fn into_spawn(self, point: WorldPoint, world: &mut World) -> Entity {
        let position = Position::new(point);
        let renderable = Renderable {
            glyph: Glyph::new(self.glyph, self.fg, self.bg),
            sprite: 0, // TODO parse sprites from yaml datafiles somehow
            render_order: self.zorder,
            sequence: None,
        };
        let viewshed = Viewshed::default().with_range(self.view_range).with_init();

        if self.is_player {
            let actor = ActorComponent::new(
                0,
                self.energy_capacity,
                self.energy_capacity,
                self.movement_cost,
                1,
                ActorKind::Player(None),
            );
            world.spawn((position, renderable, viewshed, actor, Player {}))
        } else {
            let actor = ActorComponent::new(
                0,
                self.energy_capacity,
                self.energy_capacity,
                self.movement_cost,
                1,
                ActorKind::Player(None),
            );

            let behavior = self.behavior;
            world.spawn((position, renderable, viewshed, actor, behavior))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn() {
        let yaml = r###"
        
---
- !Actor
  name: Kobold
  glyph: "k"
  fg:
    r: 0
    g: 0
    b: 0
    a: 255
  bg:
    r: 255
    g: 255
    b: 255
    a: 255
  view_range: 10
  energy_capacity: 100
  movement_cost: 20
  behavior: !FollowNearest
"###;

        let values: Vec<SpawnEntry> = serde_yaml::from_str(yaml).unwrap();
        let SpawnEntry::Actor(actor) = &values[0];
        assert_eq!(&actor.name, "Kobold");
        assert_eq!(actor.behavior, InitialBehavior::FollowNearest);
    }
}
