#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MenuResult<Sel> {
    Unconfirmed { selection: Sel },
    Confirmed { selection: Sel },
}

impl<Sel> MenuResult<Sel> {
    pub fn new(selection: Sel) -> Self {
        Self::Unconfirmed { selection }
    }
    pub fn selection(&self) -> &Sel {
        match self {
            MenuResult::Unconfirmed { selection } => selection,
            MenuResult::Confirmed { selection } => selection,
        }
    }
}
