mod error;

mod indate;
mod inoffset;
mod inprimitive;
mod intime;

pub use error::*;

pub use indate::*;
pub use inoffset::*;
pub use inprimitive::*;
pub use intime::*;

pub trait InComplete: Sized {
    type Complete;

    fn from_complete(complete: Self::Complete) -> Self;
    fn into_complete(self) -> Result<Self::Complete, InComponentRange>;
    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, InComponentRange>;

    fn fallback(self, fallback: Self::Complete) -> Result<Self::Complete, InComponentRange> {
        self.with_fallback(fallback)?.into_complete()
    }
}
