mod indate;
mod inoffset;
mod inprimitive;
mod intime;

pub use indate::*;
pub use inoffset::*;
pub use inprimitive::*;
pub use intime::*;

use crate::error::InComponentRange;

/// A set of methods required to implement an incomplete timestamp.
pub trait InComplete: Sized {
    type Complete;

    /// Converts the complete timestamp into incomplete one.
    fn from_complete(complete: Self::Complete) -> Self;

    /// Converts the incomplete timestamp into complete one.
    fn into_complete(self) -> Result<Self::Complete, InComponentRange>;

    /// Completes incomplete components of the incomplete timestamp.
    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, InComponentRange>;

    /// Completes incomplete components of the incomplete timestamp
    /// and then converts into the complete timestamp.
    fn fallback(self, fallback: Self::Complete) -> Result<Self::Complete, InComponentRange> {
        self.with_fallback(fallback)?.into_complete()
    }
}
