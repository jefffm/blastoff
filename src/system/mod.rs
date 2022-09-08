use legion::*;

mod map_indexing;
pub use map_indexing::*;

mod viewport;
pub use viewport::*;

pub fn build_systems() -> Schedule {
    Schedule::builder()
        .add_system(map_indexing_system())
        .add_system(viewport_system())
        .build()
}
