use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use time::error::ComponentRange;

use crate::error::InCompleteError;

///
#[derive(Debug)]
pub enum InComponentRange {
    InComplete(InCompleteError),
    ComponentRange(ComponentRange),
}

impl Display for InComponentRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            InComponentRange::InComplete(e) => Display::fmt(e, f),
            InComponentRange::ComponentRange(e) => Display::fmt(e, f),
        }
    }
}

impl From<InCompleteError> for InComponentRange {
    fn from(error: InCompleteError) -> Self {
        Self::InComplete(error)
    }
}

impl From<ComponentRange> for InComponentRange {
    fn from(error: ComponentRange) -> Self {
        Self::ComponentRange(error)
    }
}

impl Error for InComponentRange {}
