use hecs::Entity;

use crate::game::Action;

#[derive(Clone, Debug, PartialEq)]
pub struct Behavior {
    kind: BehaviorKind,
}

impl Behavior {
    pub fn new(kind: BehaviorKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &BehaviorKind {
        &self.kind
    }

    pub fn set_kind(&mut self, kind: BehaviorKind) {
        self.kind = kind
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BehaviorKind {
    Wander,
    Pace(ActionCycle),
    Follow(Entity),
    Attack(Entity),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActionCycle {
    idx: usize,
    sequence: Vec<Action>,
}

impl ActionCycle {
    pub fn new(sequence: Vec<Action>) -> Self {
        Self { idx: 0, sequence }
    }

    pub fn idx(&self) -> usize {
        self.idx
    }
}

impl Iterator for ActionCycle {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx = (self.idx + 1) % self.sequence.len();
        Some(self.sequence[self.idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_action_cycle() {
        let mut seq = ActionCycle::new(vec![Action::Noop, Action::Noop, Action::Noop]);
        assert_eq!(seq.idx(), 0);
        seq.next();
        assert_eq!(seq.idx(), 1);
        seq.next();
        assert_eq!(seq.idx(), 2);
        seq.next();
        assert_eq!(seq.idx(), 0);
        seq.next();
        assert_eq!(seq.idx(), 1);
        seq.next();
        assert_eq!(seq.idx(), 2);
    }
}
