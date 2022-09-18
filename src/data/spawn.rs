use hecs::{Entity, World};
use serde::{Deserialize, Serialize};

use crate::{
    component::{
        Actor as ActorComponent, Behavior, Player, Position, Renderable, Viewshed, ViewshedInit,
    },
    util::WorldPoint,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    renderable: Renderable,
    viewshed: ViewshedInit,
    actor: ActorComponent,
    behavior: Behavior,
    player: Option<Player>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SpawnEntry {
    Actor(Actor),
}

trait Spawnable {
    fn into_spawn(self, point: WorldPoint, world: &mut World) -> Entity;
}

impl Spawnable for Actor {
    fn into_spawn(self, point: WorldPoint, world: &mut World) -> Entity {
        let position = Position::new(point);
        let renderable = self.renderable;
        let viewshed = Viewshed::from(&self.viewshed);
        let actor = self.actor;
        let behavior = self.behavior;

        if let Some(player) = self.player {
            world.spawn((position, renderable, viewshed, actor, behavior, player))
        } else {
            world.spawn((position, renderable, viewshed, actor, behavior))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::component::{ActorKind, BehaviorKind};

    use super::*;

    #[test]
    fn spawn() {
        let yaml = r###"\
- Actor!
  renderable:
    glyph:
      glyph: "k",
      fg:
        r: 0,
        g: 0,
        b: 0,
        a: 0,
      bg:
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    render_order: 1
  viewshed:
    range: 10
  actor:
    turns: 0,
    energy: 100,
    max_energy: 100,
    speed: 20,
    priority: 1,
    kind: !Computer !None
  behavior:
    kind: !FollowNearest
    ";
    }
}
"###;

        let values: Vec<SpawnEntry> = serde_yaml::from_str(yaml).unwrap();
        let SpawnEntry::Actor(actor) = &values[0];
        assert_eq!(actor.actor.kind(), &ActorKind::Computer(None));
    }
}
