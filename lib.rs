#![feature(const_option)]

extern crate core;

pub use duration::MonthDuration;
pub use extensions::MonthExtension;
pub use extensions::NumericMonthDuration;

#[cfg(feature = "rand")]
pub use crate::features::rand;
#[cfg(feature = "serde")]
pub use crate::features::serde;

// TODO Make everything const
// see https://github.com/martsokha/timext/issues/2
mod duration;
mod extensions;
mod features;
