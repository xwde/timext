#![forbid(unsafe_code)]
#![feature(const_option)]

// TODO Make everything const
// see https://github.com/martsokha/timext/issues/2

pub use duration::MonthDuration;

pub mod ext {
    pub use crate::extensions::MonthExtension;
    pub use crate::extensions::NumericMonthDuration;
}

mod duration;
mod extensions;

// use time::format_description::well_known::Iso8601;
// pub mod format_description {
//     pub mod well_known {}
// }
