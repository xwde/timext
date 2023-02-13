#![forbid(unsafe_code)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::nursery)]
#![feature(const_option)]

// TODO Make everything const
// see https://github.com/xwde/timext/issues/2

pub use duration::CalendarDuration;
pub use incomplete::InCompleteTimeFormat;
pub use incomplete::InDate;
pub use incomplete::InOffsetDateTime;
pub use incomplete::InPrimitiveDateTime;
pub use incomplete::InTime;

pub mod ext {
    pub use crate::duration::extension::CalendarExtension;
    pub use crate::duration::extension::NumericCalendarDuration;
}

pub mod error {
    pub use crate::incomplete::error::InComponentRange;
    pub use crate::incomplete::error::NoComponent;
}

mod duration;
mod incomplete;
