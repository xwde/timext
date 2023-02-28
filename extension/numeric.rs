use crate::CalendarDuration;

/// Sealed trait to prevent downstream implementations.
mod sealed {
    pub trait Sealed {}
    impl Sealed for i64 {}
    impl Sealed for f64 {}
}

/// Create [`CalendarDuration`] from numeric literals.
pub trait NumericCalendarDuration: sealed::Sealed {
    fn months(self) -> CalendarDuration;
    fn years(self) -> CalendarDuration;
}

impl NumericCalendarDuration for i64 {
    /// Creates a new `CalendarDuration` with provided months.
    ///
    /// ```rust
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// assert_eq!(1.months(), CalendarDuration::months(1));
    /// assert_eq!((-1).months(), CalendarDuration::months(-1));
    /// ```
    fn months(self) -> CalendarDuration {
        CalendarDuration::months(self as i32)
    }

    /// Creates a new `CalendarDuration` with provided years.
    ///
    /// ```rust
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// assert_eq!(1.years(), CalendarDuration::years(1));
    /// assert_eq!((-1).years(), CalendarDuration::years(-1));
    /// ```
    fn years(self) -> CalendarDuration {
        CalendarDuration::years(self as i32)
    }
}

impl NumericCalendarDuration for f64 {
    /// Creates a new `CalendarDuration` with provided months.
    ///
    /// ```rust
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// assert_eq!((1.0).months(), CalendarDuration::months(1));
    /// assert_eq!((-1.0).months(), CalendarDuration::months(-1));
    /// ```
    fn months(self) -> CalendarDuration {
        i64::months(self as _)
    }

    /// Creates a new `CalendarDuration` with provided years.
    ///
    /// ```rust
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// assert_eq!((1.5).years(), CalendarDuration::months(18));
    /// assert_eq!((-1.5).years(), CalendarDuration::months(-18));
    /// ```
    fn years(self) -> CalendarDuration {
        i64::months((self * 12.0) as _)
    }
}
