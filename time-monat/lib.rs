#![feature(const_trait_impl)]
#![feature(const_option)]
#![feature(const_fn_floating_point_arithmetic)]

pub use duration::MonthDuration;
pub use extensions::MonthExtension;
pub use extensions::NumericMonthDuration;

// TODO Make everything const
// see https://github.com/martsokha/timext/issues/2
mod duration;
mod extensions;
mod features;

#[cfg(feature = "serde")]
pub mod serde {
    pub use crate::features::serde;
}

#[cfg(feature = "rand")]
pub mod rand {
    pub use crate::features::rand;
}
