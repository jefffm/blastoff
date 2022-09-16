use crate::{game::Action, input};

#[derive(Debug, Clone, PartialEq)]
pub struct Actor {
    turns: u32,
    energy: i32,
    max_energy: i32,
    speed: i32,
    priority: i32,
    kind: ActorKind,
}

impl Actor {
    pub fn new(
        turns: u32,
        energy: i32,
        max_energy: i32,
        speed: i32,
        priority: i32,
        kind: ActorKind,
    ) -> Self {
        Self {
            turns,
            energy,
            max_energy,
            speed,
            priority,
            kind,
        }
    }

    pub fn energy(&self) -> i32 {
        self.energy
    }

    /// Turn priority is energy * priority
    pub fn priority(&self) -> i32 {
        self.energy * self.priority
    }

    pub fn increase_priority(&mut self) {
        self.priority += 1
    }

    /// Recover energy up to a maximum of zero
    pub fn recover_energy(&mut self) {
        self.energy = std::cmp::min(self.energy + self.speed, self.max_energy);
    }

    pub fn speed(&self) -> i32 {
        self.speed
    }

    pub fn kind(&self) -> &ActorKind {
        &self.kind
    }

    pub fn set_kind(&mut self, kind: ActorKind) {
        self.kind = kind
    }

    pub fn use_energy(&mut self, energy: i32) {
        self.energy -= energy
    }

    pub fn take_turn(&mut self) {
        self.turns += 1;
        self.priority = 0;
    }

    pub fn turns(&self) -> u32 {
        self.turns
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActorKind {
    Player(Option<input::PlayerAction>),
    Computer(Option<Action>),
}
