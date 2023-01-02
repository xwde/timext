use crate::MonthDuration;

/// Sealed trait to prevent downstream implementations.
mod sealed {
    /// A trait that cannot be implemented by downstream users.
    pub trait Sealed {}

    impl Sealed for i32 {}

    impl Sealed for u32 {}

    impl Sealed for f32 {}
}

#[const_trait]
pub trait NumericMonthDuration: sealed::Sealed {
    fn months(self) -> MonthDuration;
    fn years(self) -> MonthDuration;
}

/// ```rust
/// # use time_monat::{MonthDuration, NumericMonthDuration};
/// assert_eq!(1.months(), MonthDuration::months(1));
/// assert_eq!(1.years(), MonthDuration::years(1));
/// assert_eq!((-1).months(), MonthDuration::months(-1));
/// assert_eq!((-1).years(), MonthDuration::years(-1));
/// ```
impl const NumericMonthDuration for i32 {
    fn months(self) -> MonthDuration {
        MonthDuration::months(self)
    }

    fn years(self) -> MonthDuration {
        MonthDuration::years(self)
    }
}

impl const NumericMonthDuration for u32 {
    fn months(self) -> MonthDuration {
        (self as i32).months()
    }

    fn years(self) -> MonthDuration {
        (self as i32).years()
    }
}

/// ```rust
/// # use time_monat::{MonthDuration, NumericMonthDuration};
/// assert_eq!((1.5).months(), MonthDuration::months(1));
/// assert_eq!((1.5).years(), MonthDuration::months(18));
/// assert_eq!((-1.5).months(), MonthDuration::months(-1));
/// assert_eq!((-1.5).years(), MonthDuration::months(-18));
/// ```
impl const NumericMonthDuration for f32 {
    fn months(self) -> MonthDuration {
        (self as i32).months()
    }

    fn years(self) -> MonthDuration {
        (self * 12.).months()
    }
}
