#![forbid(unsafe_code)]
#![feature(const_option)]

// TODO Make everything const
// see https://github.com/xwde/timext/issues/2

mod duration;
pub use duration::CalendarDuration;

mod incomplete;
pub use incomplete::InComplete;
pub use incomplete::InDate;
pub use incomplete::InOffsetDateTime;
pub use incomplete::InPrimitiveDateTime;
pub use incomplete::InTime;

mod feature;
pub use feature::*;

mod extension;
pub mod ext {
    pub use crate::extension::*;
}

pub mod error {
    pub use crate::feature::error::*;
    pub use crate::incomplete::error::*;
}
