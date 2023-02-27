#![forbid(unsafe_code)]
#![feature(const_option)]

// TODO Make everything const
// see https://github.com/xwde/timext/issues/2

mod duration;
pub use duration::CalendarDuration;

mod feature;
pub use feature::*;

mod extension;
pub mod ext {
    pub use crate::extension::*;
}
