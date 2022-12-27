#![feature(const_option)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]

extern crate core;

pub use crate::duration::MonatDuration;
pub use crate::extension::MonatExt;

mod duration;
mod extension;

#[cfg(test)]
mod duration_test;
#[cfg(test)]
mod extension_test;
