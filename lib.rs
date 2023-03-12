#![forbid(unsafe_code)]

//! The collection of [time-rs/time](https://github.com/time-rs/time/) extensions
//! for calendar arithmetics, incomplete formats handling, imprecise time, and other
//! things [time] crate is not intended for.
//!
//! **Also check out other `xwde` projects [here](https://github.com/xwde).**
//!
//! ## Features
//!
//! - `serde` to enable `serde::Serialize` and `serde::Deserialize` impls.
//! - `rand` to enable `rand::distributions::Distribution` impls.
//!
//! ## Examples
//!
//! - Introduces [CalendarDuration] and extends [time::Date], [time::PrimitiveDateTime],
//!   and [time::OffsetDateTime] with several methods to enable arithmetic operations
//!   related to months and years. Additionally, attaches conversion methods to `i64`
//!   and `f64` to improve ease of use.
//!
//! ```rust
//! use time::{Date, Month::*};
//! use timext::ext::NumericCalendarDuration;
//!
//! let d0 = Date::from_calendar_date(2023, January, 31);
//! let d1 = Date::from_calendar_date(2023, February, 28);
//! assert_eq!(d0.unwrap() + 1.months(), d1.unwrap());
//!
//! let d0 = Date::from_calendar_date(2024, February, 29);
//! let d1 = Date::from_calendar_date(2025, February, 28);
//! assert_eq!(d0.unwrap() + 1.years(), d1.unwrap());
//! ```
//!
//! - Implements its own [time::Time], [time::Date], [time::PrimitiveDateTime], and
//!   [time::OffsetDateTime] types, that are convertable from/to original, but allow
//!   incomplete time formats e.g. `xx:24:xx.845`, `1998-xx-02` or `2016-08 14:xx`.
//!   Also extends them with parsing & formatting capabilities.
//!
//! ```rust
//! use time::{Date, Month::*};
//! use timext::{InComplete, InDate};
//!
//! let d0 = Date::from_calendar_date(2023, January, 28);
//! let d1 = InDate::from_calendar_date(None, None, Some(28));
//! let d1 = d1.unwrap();
//!
//! let d1 = d1.replace_year(Some(2023)).unwrap();
//! let d1 = d1.replace_month(Some(January)).unwrap();
//! assert_eq!(d0.unwrap(), d1.into_complete().unwrap());
//! ```

// TODO Make everything const
// see https://github.com/xwde/timext/issues/2

mod duration;
pub use duration::*;

mod incomplete;
pub use incomplete::*;

mod feature;
pub use feature::*;

pub mod error;
mod extension;

pub mod ext {
    //! Extension traits.
    pub use crate::extension::*;
}
