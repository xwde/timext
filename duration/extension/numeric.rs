use crate::CalendarDuration;

/// Sealed trait to prevent downstream implementations.
mod sealed {
    /// A trait that cannot be implemented by downstream users.
    pub trait Sealed {}

    impl Sealed for i64 {}

    impl Sealed for f64 {}
}

pub trait NumericCalendarDuration: sealed::Sealed {
    fn months(self) -> CalendarDuration;
    fn years(self) -> CalendarDuration;
}

/// ```rust
/// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
/// assert_eq!(1.months(), CalendarDuration::months(1));
/// assert_eq!(1.years(), CalendarDuration::years(1));
/// assert_eq!((-1).months(), CalendarDuration::months(-1));
/// assert_eq!((-1).years(), CalendarDuration::years(-1));
/// ```
impl NumericCalendarDuration for i64 {
    fn months(self) -> CalendarDuration {
        CalendarDuration::months(self as i32)
    }

    fn years(self) -> CalendarDuration {
        CalendarDuration::years(self as i32)
    }
}

/// ```rust
/// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
/// assert_eq!((1.5).years(), CalendarDuration::months(18));
/// assert_eq!((-1.5).years(), CalendarDuration::months(-18));
/// ```
impl NumericCalendarDuration for f64 {
    fn months(self) -> CalendarDuration {
        i64::months(self as _)
    }

    fn years(self) -> CalendarDuration {
        i64::months((self * 12.0) as _)
    }
}
