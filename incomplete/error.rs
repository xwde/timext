use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use time::error::ComponentRange;

#[derive(Debug)]
pub struct NoComponent {
    component: &'static str,
}

impl NoComponent {
    pub fn new(component: &'static str) -> NoComponent {
        Self { component }
    }

    pub fn component(&self) -> &'static str {
        self.component
    }
}

impl Display for NoComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "component `{}` does not exist", self.component)
    }
}

impl Error for NoComponent {}

#[derive(Debug)]
pub enum InComponentRange {
    InComplete(NoComponent),
    ComponentRange(ComponentRange),
}

impl Display for InComponentRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            InComponentRange::InComplete(e) => Display::fmt(e, f),
            InComponentRange::ComponentRange(e) => Display::fmt(e, f),
        }
    }
}

impl Error for InComponentRange {}

impl From<NoComponent> for InComponentRange {
    fn from(error: NoComponent) -> Self {
        Self::InComplete(error)
    }
}

impl From<ComponentRange> for InComponentRange {
    fn from(error: ComponentRange) -> Self {
        Self::ComponentRange(error)
    }
}
