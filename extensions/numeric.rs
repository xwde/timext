use crate::MonthDuration;

/// Sealed trait to prevent downstream implementations.
mod sealed {
    /// A trait that cannot be implemented by downstream users.
    pub trait Sealed {}

    impl Sealed for i32 {}

    impl Sealed for u32 {}

    // impl Sealed for f32 {}
}

pub trait NumericMonthDuration: sealed::Sealed {
    fn months(self) -> MonthDuration;
    fn years(self) -> MonthDuration;
}

/// ```rust
/// # use timext::{MonthDuration, NumericMonthDuration};
/// assert_eq!(1.months(), MonthDuration::months(1));
/// assert_eq!(1.years(), MonthDuration::years(1));
/// assert_eq!((-1).months(), MonthDuration::months(-1));
/// assert_eq!((-1).years(), MonthDuration::years(-1));
/// ```
impl NumericMonthDuration for i32 {
    fn months(self) -> MonthDuration {
        MonthDuration::months(self)
    }

    fn years(self) -> MonthDuration {
        MonthDuration::years(self)
    }
}

/// ```rust
/// # use timext::{MonthDuration, NumericMonthDuration};
/// assert_eq!(1_u32.months(), MonthDuration::months(1));
/// assert_eq!(1_u32.years(), MonthDuration::years(1));
/// ```
impl NumericMonthDuration for u32 {
    fn months(self) -> MonthDuration {
        (self as i32).months()
    }

    fn years(self) -> MonthDuration {
        (self as i32).years()
    }
}
