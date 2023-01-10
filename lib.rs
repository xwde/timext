#![forbid(unsafe_code)]
#![feature(const_option)]

// TODO Make everything const
// see https://github.com/martsokha/timext/issues/2

pub use duration::MonthDuration;

#[cfg(feature = "rand")]
pub use crate::features::rand;
#[cfg(feature = "serde")]
pub use crate::features::serde;

pub mod ext {
    pub use crate::extensions::MonthExtension;
    pub use crate::extensions::NumericMonthDuration;
}

mod duration;
mod extensions;
mod features;
