use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use time::error::ComponentRange;

use crate::error::Insufficient;

///
#[derive(Debug)]
pub enum InComponentRange {
    Insufficient(Insufficient),
    ComponentRange(ComponentRange),
}

impl Display for InComponentRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            InComponentRange::Insufficient(e) => Display::fmt(e, f),
            InComponentRange::ComponentRange(e) => Display::fmt(e, f),
        }
    }
}

impl From<Insufficient> for InComponentRange {
    fn from(error: Insufficient) -> Self {
        Self::Insufficient(error)
    }
}

impl From<ComponentRange> for InComponentRange {
    fn from(error: ComponentRange) -> Self {
        Self::ComponentRange(error)
    }
}

impl Error for InComponentRange {}
