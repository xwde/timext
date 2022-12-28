#![feature(const_trait_impl)]
#![feature(const_option)]
#![feature(const_fn_floating_point_arithmetic)]

extern crate core;

pub use duration::MonthDuration;
pub use extensions::date::MonthExtension;
pub use extensions::numeric::NumericMonthDuration;

mod duration;
mod features;
mod extensions;

#[cfg(test)]
mod duration_test;

#[cfg(feature = "serde")]
pub mod serde {
    use crate::features::serde;
}

#[cfg(feature = "rand")]
pub mod rand {
    use crate::features::rand;
}
