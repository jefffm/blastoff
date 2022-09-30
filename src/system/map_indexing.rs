use ggez::Context;
use hecs::World;

use crate::component::{BlocksTile, Door, Position};
use crate::resource::Resources;
use crate::sector::Map;

pub fn map_indexing_system(
    world: &mut World,
    _resources: &mut Resources,
    map: &mut Map,
    ctx: &Context,
) {
    map.reset_blocked();
    map.reset_content();

    for (id, pos) in world.query::<&Position>().iter() {
        map.add_content(&pos.p, &id);
    }

    for (_, (pos, _blocked)) in world.query::<(&Position, &BlocksTile)>().iter() {
        map.set_blocked(&pos.p);
    }

    for (_, (pos, door)) in world.query::<(&Position, &Door)>().iter() {
        if !door.opened {
            map.set_blocked(&pos.p);
        }
    }
}
