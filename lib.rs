#![forbid(unsafe_code)]
#![feature(const_option)]

// TODO Make everything const
// see https://github.com/xwde/timext/issues/2

mod duration;
mod extension;
mod feature;
mod incomplete;

pub use duration::CalendarDuration;
pub use feature::*;

pub use incomplete::InComplete;

pub use incomplete::InDate;
pub use incomplete::InOffsetDateTime;
pub use incomplete::InPrimitiveDateTime;
pub use incomplete::InTime;

pub mod ext {
    pub use crate::extension::*;
}

pub mod error {
    pub use crate::incomplete::InComponentRange;
    pub use crate::incomplete::NoComponent;
}
