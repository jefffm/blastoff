#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MenuResult<Sel> {
    NoSelection { selected: Sel },
    Selected { selected: Sel },
}

impl<Sel> MenuResult<Sel> {
    pub fn new(selection: Sel) -> Self {
        Self::NoSelection {
            selected: selection,
        }
    }
    pub fn selection(&self) -> &Sel {
        match self {
            MenuResult::NoSelection { selected } => selected,
            MenuResult::Selected { selected } => selected,
        }
    }
}
