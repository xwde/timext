use crate::MonthDuration;

mod sealed {
    pub trait Sealed {}

    impl Sealed for i32 {}

    impl Sealed for f32 {}
}

#[const_trait]
pub trait NumericMonthDuration: sealed::Sealed {
    fn months(self) -> MonthDuration;
    fn years(self) -> MonthDuration;
}

impl const NumericMonthDuration for i32 {
    fn months(self) -> MonthDuration {
        MonthDuration::months(self)
    }

    fn years(self) -> MonthDuration {
        MonthDuration::years(self)
    }
}

impl const NumericMonthDuration for f32 {
    fn months(self) -> MonthDuration {
        (self as i32).months()
    }

    fn years(self) -> MonthDuration {
        (self * 12.).months()
    }
}
