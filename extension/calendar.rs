use std::ops::{Add, AddAssign, Sub, SubAssign};

use time::{Date, OffsetDateTime, PrimitiveDateTime};

use crate::CalendarDuration;

/// Sealed trait to prevent downstream implementations.
mod sealed {
    use time::{Date, OffsetDateTime, PrimitiveDateTime};

    pub trait Sealed {}
    impl Sealed for Date {}
    impl Sealed for PrimitiveDateTime {}
    impl Sealed for OffsetDateTime {}
}

/// Enable arithmetic operations with [`CalendarDuration`].
pub trait CalendarExtension: sealed::Sealed + Sized {
    #[must_use]
    fn checked_calendar_add(self, duration: CalendarDuration) -> Option<Self>;
    #[must_use]
    fn checked_calendar_sub(self, duration: CalendarDuration) -> Option<Self>;
    #[must_use]
    fn saturating_calendar_add(self, duration: CalendarDuration) -> Self;
    #[must_use]
    fn saturating_calendar_sub(self, duration: CalendarDuration) -> Self;
}

impl CalendarExtension for Date {
    fn checked_calendar_add(self, duration: CalendarDuration) -> Option<Self> {
        CalendarDuration::checked_date_add(self, duration)
    }

    fn checked_calendar_sub(self, duration: CalendarDuration) -> Option<Self> {
        CalendarDuration::checked_date_sub(self, duration)
    }

    fn saturating_calendar_add(self, duration: CalendarDuration) -> Self {
        if let Some(datetime) = self.checked_calendar_add(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MIN
        } else {
            debug_assert!(duration.is_positive());
            Self::MAX
        }
    }

    fn saturating_calendar_sub(self, duration: CalendarDuration) -> Self {
        if let Some(datetime) = self.checked_calendar_sub(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MAX
        } else {
            debug_assert!(duration.is_positive());
            Self::MIN
        }
    }
}

impl Add<CalendarDuration> for Date {
    type Output = Self;

    fn add(self, rhs: CalendarDuration) -> Self::Output {
        self.checked_calendar_add(rhs)
            .expect("resulting value is out of range")
    }
}

impl AddAssign<CalendarDuration> for Date {
    fn add_assign(&mut self, rhs: CalendarDuration) {
        *self = *self + rhs;
    }
}

impl Sub<CalendarDuration> for Date {
    type Output = Self;

    fn sub(self, rhs: CalendarDuration) -> Self::Output {
        self.checked_calendar_sub(rhs)
            .expect("resulting value is out of range")
    }
}

impl SubAssign<CalendarDuration> for Date {
    fn sub_assign(&mut self, rhs: CalendarDuration) {
        *self = *self - rhs;
    }
}

impl CalendarExtension for PrimitiveDateTime {
    fn checked_calendar_add(self, duration: CalendarDuration) -> Option<Self> {
        let date = self.date().checked_calendar_add(duration)?;
        Some(self.replace_date(date))
    }

    fn checked_calendar_sub(self, duration: CalendarDuration) -> Option<Self> {
        let date = self.date().checked_calendar_sub(duration)?;
        Some(self.replace_date(date))
    }

    fn saturating_calendar_add(self, duration: CalendarDuration) -> Self {
        if let Some(datetime) = self.checked_calendar_add(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MIN
        } else {
            debug_assert!(duration.is_positive());
            Self::MAX
        }
    }

    fn saturating_calendar_sub(self, duration: CalendarDuration) -> Self {
        if let Some(datetime) = self.checked_calendar_sub(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MAX
        } else {
            debug_assert!(duration.is_positive());
            Self::MIN
        }
    }
}

impl Add<CalendarDuration> for PrimitiveDateTime {
    type Output = Self;

    fn add(self, rhs: CalendarDuration) -> Self::Output {
        self.checked_calendar_add(rhs)
            .expect("resulting value is out of range")
    }
}

impl AddAssign<CalendarDuration> for PrimitiveDateTime {
    fn add_assign(&mut self, rhs: CalendarDuration) {
        *self = *self + rhs;
    }
}

impl Sub<CalendarDuration> for PrimitiveDateTime {
    type Output = Self;

    fn sub(self, rhs: CalendarDuration) -> Self::Output {
        self.checked_calendar_sub(rhs)
            .expect("resulting value is out of range")
    }
}

impl SubAssign<CalendarDuration> for PrimitiveDateTime {
    fn sub_assign(&mut self, rhs: CalendarDuration) {
        *self = *self - rhs;
    }
}

impl CalendarExtension for OffsetDateTime {
    fn checked_calendar_add(self, duration: CalendarDuration) -> Option<Self> {
        let date = self.date().checked_calendar_add(duration)?;
        Some(self.replace_date(date))
    }

    fn checked_calendar_sub(self, duration: CalendarDuration) -> Option<Self> {
        let date = self.date().checked_calendar_sub(duration)?;
        Some(self.replace_date(date))
    }

    fn saturating_calendar_add(self, duration: CalendarDuration) -> Self {
        if let Some(datetime) = self.checked_calendar_add(duration) {
            datetime
        } else if duration.is_negative() {
            PrimitiveDateTime::MIN.assume_offset(self.offset())
        } else {
            debug_assert!(duration.is_positive());
            PrimitiveDateTime::MAX.assume_offset(self.offset())
        }
    }

    fn saturating_calendar_sub(self, duration: CalendarDuration) -> Self {
        if let Some(datetime) = self.checked_calendar_sub(duration) {
            datetime
        } else if duration.is_negative() {
            PrimitiveDateTime::MAX.assume_offset(self.offset())
        } else {
            debug_assert!(duration.is_positive());
            PrimitiveDateTime::MIN.assume_offset(self.offset())
        }
    }
}

#[cfg(test)]
mod tests {
    use time::Date;
    use time::Month::*;

    use crate::ext::CalendarExtension;
    use crate::CalendarDuration;

    #[test]
    fn add_underflow() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let md = CalendarDuration::months(i32::MIN);
        assert_eq!(d0.saturating_calendar_add(md), Date::MIN);
    }

    #[test]
    fn sub_underflow() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let md = CalendarDuration::months(i32::MAX);
        assert_eq!(d0.saturating_calendar_sub(md), Date::MIN);
    }

    #[test]
    fn add_overflow() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let md = CalendarDuration::months(i32::MAX);
        assert_eq!(d0.saturating_calendar_add(md), Date::MAX);
    }

    #[test]
    fn sub_overflow() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let md = CalendarDuration::months(i32::MIN);
        assert_eq!(d0.saturating_calendar_sub(md), Date::MAX);
    }
}
