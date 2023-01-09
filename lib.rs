#![forbid(unsafe_code)]
#![feature(const_option)]

// TODO Make everything const
// see https://github.com/martsokha/timext/issues/2

pub use duration::MonthDuration;
pub use extensions::MonthExtension;
pub use extensions::NumericMonthDuration;

#[cfg(feature = "rand")]
pub use crate::features::rand;
#[cfg(feature = "serde")]
pub use crate::features::serde;

mod duration;
mod extensions;
mod features;
