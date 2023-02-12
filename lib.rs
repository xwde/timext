#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![feature(const_option)]

// TODO Make everything const
// see https://github.com/xwde/timext/issues/2

pub use duration::CalendarDuration;
pub use incomplete::InOffsetDateTime;
pub use incomplete::InPrimitiveDateTime;
pub use incomplete::{InDate, InTime};

pub mod ext {
    pub use crate::duration::CalendarExtension;
    pub use crate::duration::NumericCalendarDuration;
}

mod duration;
mod incomplete;

// use time::format_description::well_known::Iso8601;
// pub mod format_description {
//     pub mod well_known {}
// }
