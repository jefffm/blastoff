mod cardinal;
pub use cardinal::*;

mod position;
pub use position::*;

mod renderable;
pub use renderable::*;

mod viewshed;
pub use viewshed::*;

mod actor;
pub use actor::*;

mod behavior;
pub use behavior::*;

mod animation;
pub use animation::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {}

#[derive(Clone, Debug)]
pub struct Player {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Activated {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Door {
    pub opened: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlocksTile {}
