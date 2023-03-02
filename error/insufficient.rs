use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// An error type indicating that an expected component was not found, causing a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Insufficient {
    /// Name of the component.
    name: &'static str,
}

impl Insufficient {
    /// Create `Insufficient` with the specified component name.
    pub fn new(component: &'static str) -> Insufficient {
        Self { name: component }
    }

    /// Obtain the name of the component that was not found.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl Display for Insufficient {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "component `{}` does not exist", self.name)
    }
}

impl Error for Insufficient {}
