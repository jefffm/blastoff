mod scheduler;
pub use scheduler::*;

mod map_indexing;
pub use map_indexing::*;

mod viewport;
pub use viewport::*;

pub fn build_systems() -> Scheduler {
    Scheduler::builder()
        .with_system(map_indexing_system)
        .with_system(viewport_system)
        .build()
}
