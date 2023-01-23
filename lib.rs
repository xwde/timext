#![forbid(unsafe_code)]
#![feature(const_option)]

// TODO Make everything const
// see https://github.com/xwde/timext/issues/2

pub use duration::month::MonthDuration;
pub use incomplete::IncompleteDate;
pub use incomplete::IncompleteOffsetDateTime;
pub use incomplete::IncompletePrimitiveDateTime;
pub use incomplete::IncompleteTime;

pub mod ext {
    pub use crate::duration::extension::MonthExtension;
    pub use crate::duration::extension::NumericMonthDuration;
}

mod duration;
mod incomplete;

// use time::format_description::well_known::Iso8601;
// pub mod format_description {
//     pub mod well_known {}
// }
