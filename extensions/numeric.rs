use crate::MonthDuration;

/// Sealed trait to prevent downstream implementations.
mod sealed {
    /// A trait that cannot be implemented by downstream users.
    pub trait Sealed {}

    impl Sealed for i64 {}

    // impl Sealed for f64 {}
}

pub trait NumericMonthDuration: sealed::Sealed {
    fn months(self) -> MonthDuration;
    fn years(self) -> MonthDuration;
}

/// ```rust
/// # use time::Duration;
/// # use timext::{MonthDuration, ext::NumericMonthDuration};
/// assert_eq!(1.months(), MonthDuration::months(1));
/// assert_eq!(1.years(), MonthDuration::years(1));
/// assert_eq!((-1).months(), MonthDuration::months(-1));
/// assert_eq!((-1).years(), MonthDuration::years(-1));
/// ```
impl NumericMonthDuration for i64 {
    fn months(self) -> MonthDuration {
        MonthDuration::months(self as i32)
    }

    fn years(self) -> MonthDuration {
        MonthDuration::years(self as i32)
    }
}
