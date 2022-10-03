use ggez::Context;

use crate::component::{BlocksTile, Door, Position};
use crate::overworld::SectorData;
use crate::resource::Resources;

pub fn map_indexing_system(_resources: &mut Resources, sector: &mut SectorData, ctx: &Context) {
    sector.map.reset_blocked();
    sector.map.reset_content();

    for (id, pos) in sector.world.query::<&Position>().iter() {
        sector.map.add_content(&pos.p, &id);
    }

    for (_, (pos, _blocked)) in sector.world.query::<(&Position, &BlocksTile)>().iter() {
        sector.map.set_blocked(&pos.p);
    }

    for (_, (pos, door)) in sector.world.query::<(&Position, &Door)>().iter() {
        if !door.opened {
            sector.map.set_blocked(&pos.p);
        }
    }
}
