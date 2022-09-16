mod scheduler;
pub use scheduler::*;

mod map_indexing;
pub use map_indexing::*;

mod viewport;
pub use viewport::*;

mod visibility;
pub use visibility::*;

mod ai;
pub use ai::*;

pub fn build_systems() -> Scheduler {
    Scheduler::builder()
        .with_system(visibility_system)
        .with_system(map_indexing_system)
        .with_system(viewport_system)
        .with_system(ai_system)
        .build()
}
