#![forbid(unsafe_code)]

//! The collection of [time-rs/time](https://github.com/time-rs/time/) extensions
//! for calendar arithmetics, incomplete formats handling, imprecise time, and other
//! things `time` crate is not intended for.
//!
//! ## Features
//!
//! - `serde` to enable `serde::Serialize` and `serde:Deserialize` impls.
//! - `rand` to enable `rand::distributions::Distribution` impls.
//!
//! ## Examples
//!
//! - Introduces `timext:CalendarDuration` and extends `time::Date`,
//!   `time::PrimitiveDateTime`, and `time::OffsetDateTime` with several methods to
//!   enable arithmetic operations related to months and years. Additionally,
//!   attaches conversion methods to `i64` and `f64` to improve ease of use.
//!
//! ```rust
//! # use time::{Date, Month};
//! # use timext::ext::NumericCalendarDuration;
//! let d0 = Date::from_calendar_date(2023, Month::January, 31).unwrap();
//! let d1 = Date::from_calendar_date(2023, Month::February, 28).unwrap();
//! assert_eq!(d0 + 1.months(), d1);
//!
//! let d0 = Date::from_calendar_date(2024, Month::February, 29).unwrap();
//! let d1 = Date::from_calendar_date(2025, Month::February, 28).unwrap();
//! assert_eq!(d0 + 1.years(), d1);
//! ```

// TODO Make everything const
// see https://github.com/xwde/timext/issues/2

mod duration;
pub use duration::*;

mod feature;
pub use feature::*;

mod extension;
pub mod ext {
    //! Extension traits.
    pub use crate::extension::*;
}
