#[cfg(feature = "formatting")]
pub use self::formatting::InFormattable;
#[cfg(feature = "parsing")]
pub use self::parsing::InParsable;
#[cfg(feature = "rand")]
pub use self::rand::*;
#[cfg(feature = "serde")]
pub use self::serde::*;

#[cfg(feature = "formatting")]
mod formatting;
#[cfg(feature = "parsing")]
mod parsing;
#[cfg(feature = "rand")]
mod rand;
#[cfg(feature = "serde")]
mod serde;

pub mod error {
    #[cfg(feature = "formatting")]
    pub use self::super::formatting::error::*;
    #[cfg(feature = "parsing")]
    pub use self::super::parsing::error::*;
}
