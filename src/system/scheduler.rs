use hecs::Entity;

use crate::{overworld::SectorData, resource::Resources};

pub type SystemFn = fn(&mut Resources, &mut SectorData);

/// This is used as a component to signify ownership
pub struct Owner(pub Entity);

/// Placeholder until we implement threading
#[derive(Default)]
pub struct SchedulerBuilder {
    steps: Vec<SystemFn>,
}

impl SchedulerBuilder {
    #[must_use]
    pub fn with_system(self, system: SystemFn) -> Self {
        let mut steps = self.steps;
        steps.push(system);

        SchedulerBuilder { steps }
    }

    pub fn add_system(&mut self, system: SystemFn) -> &mut Self {
        self.steps.push(system);
        self
    }

    #[must_use]
    pub fn with_thread_local(self, system: SystemFn) -> Self {
        let mut steps = self.steps;
        steps.push(system);

        SchedulerBuilder { steps }
    }

    pub fn add_thread_local(&mut self, system: SystemFn) -> &mut Self {
        self.steps.push(system);
        self
    }

    pub fn build(self) -> Scheduler {
        Scheduler { steps: self.steps }
    }
}

/// Placeholder until we implement threading
pub struct Scheduler {
    steps: Vec<SystemFn>,
}

impl Scheduler {
    pub fn builder() -> SchedulerBuilder {
        SchedulerBuilder::default()
    }

    pub fn execute(&mut self, resources: &mut Resources, sector: &mut SectorData) {
        for f in &mut self.steps {
            f(resources, sector);
        }
    }
}
