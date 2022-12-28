#[cfg(feature = "rand")]
pub mod rand;
#[cfg(feature = "serde")]
pub mod serde;

#[cfg(test)]
#[cfg(feature = "rand")]
mod rand_test;
#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_test;
