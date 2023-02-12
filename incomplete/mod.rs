use time::error::ComponentRange;

pub use indate::*;
pub use inoffset::*;
pub use inprimitive::*;
pub use intime::*;

mod indate;
mod inoffset;
mod inprimitive;
mod intime;

pub trait Incomplete: Sized {
    type Complete;

    fn from_complete(complete: Self::Complete) -> Self;
    fn into_complete(self) -> Result<Self::Complete, ComponentRange>;
    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, ComponentRange>;

    fn fallback(self, fallback: Self::Complete) -> Result<Self::Complete, ComponentRange> {
        self.with_fallback(fallback)?.into_complete()
    }
}
