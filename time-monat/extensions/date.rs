use std::ops::{Add, AddAssign, Sub, SubAssign};

use time::{Date, Month, OffsetDateTime, PrimitiveDateTime};
use time::util::days_in_year_month;

use crate::duration::MonthDuration;

mod sealed {
    use time::{Date, OffsetDateTime, PrimitiveDateTime};

    pub trait Sealed {}

    impl Sealed for Date {}

    impl Sealed for PrimitiveDateTime {}

    impl Sealed for OffsetDateTime {}
}

pub trait MonthExtension: sealed::Sealed + Sized {
    fn checked_calendar_add(&self, duration: MonthDuration) -> Option<Self>;
    fn checked_calendar_sub(&self, duration: MonthDuration) -> Option<Self>;
    fn saturating_calendar_add(&self, duration: MonthDuration) -> Self;
    fn saturating_calendar_sub(&self, duration: MonthDuration) -> Self;
}

impl MonthExtension for Date {
    fn checked_calendar_add(&self, duration: MonthDuration) -> Option<Self> {
        // [1, 12] + [-11, 11]
        let month = duration.subyear_months();
        let month = month.checked_add(self.month() as i32)?;

        // Aug(8) + 6 = Feb(2) or Feb(2) - 6 = Aug(8)
        let added = match month {
            x if x.is_positive() => x / 12,
            _ => -1,
        };

        debug_assert!((-1..=1).contains(&added));
        let year = duration
            .whole_years()
            .checked_add(added)?
            .checked_add(self.year())?;

        // Feb(2) - 6 = -4; 12 - 4 = Aug(8)
        let month = match month {
            x if x.is_positive() => x % 12,
            x => 12 + x,
        };

        debug_assert!((1..=12).contains(&month));
        let month = Month::try_from(month as u8).unwrap();
        let day = days_in_year_month(year, month).min(self.day());
        Date::from_calendar_date(year, month, day).ok()
    }

    fn checked_calendar_sub(&self, duration: MonthDuration) -> Option<Self> {
        let months = duration.whole_months().checked_neg()?;
        self.checked_calendar_add(MonthDuration::months(months))
    }

    fn saturating_calendar_add(&self, duration: MonthDuration) -> Self {
        if let Some(datetime) = self.checked_calendar_add(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MIN
        } else {
            debug_assert!(duration.is_positive());
            Self::MAX
        }
    }

    fn saturating_calendar_sub(&self, duration: MonthDuration) -> Self {
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

impl Add<MonthDuration> for Date {
    type Output = Self;

    fn add(self, rhs: MonthDuration) -> Self::Output {
        self.checked_calendar_add(rhs)
            .expect("resulting value is out of range")
    }
}

impl AddAssign<MonthDuration> for Date {
    fn add_assign(&mut self, rhs: MonthDuration) {
        *self = *self + rhs;
    }
}

impl Sub<MonthDuration> for Date {
    type Output = Self;

    fn sub(self, rhs: MonthDuration) -> Self::Output {
        self.checked_calendar_sub(rhs)
            .expect("resulting value is out of range")
    }
}

impl SubAssign<MonthDuration> for Date {
    fn sub_assign(&mut self, rhs: MonthDuration) {
        *self = *self - rhs;
    }
}

impl MonthExtension for PrimitiveDateTime {
    fn checked_calendar_add(&self, duration: MonthDuration) -> Option<Self> {
        let date = self.date().checked_calendar_add(duration)?;
        Some(self.replace_date(date))
    }

    fn checked_calendar_sub(&self, duration: MonthDuration) -> Option<Self> {
        let date = self.date().checked_calendar_sub(duration)?;
        Some(self.replace_date(date))
    }

    fn saturating_calendar_add(&self, duration: MonthDuration) -> Self {
        if let Some(datetime) = self.checked_calendar_add(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MIN
        } else {
            debug_assert!(duration.is_positive());
            Self::MAX
        }
    }

    fn saturating_calendar_sub(&self, duration: MonthDuration) -> Self {
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

impl Add<MonthDuration> for PrimitiveDateTime {
    type Output = Self;

    fn add(self, rhs: MonthDuration) -> Self::Output {
        self.checked_calendar_add(rhs)
            .expect("resulting value is out of range")
    }
}

impl AddAssign<MonthDuration> for PrimitiveDateTime {
    fn add_assign(&mut self, rhs: MonthDuration) {
        *self = *self + rhs;
    }
}

impl Sub<MonthDuration> for PrimitiveDateTime {
    type Output = Self;

    fn sub(self, rhs: MonthDuration) -> Self::Output {
        self.checked_calendar_sub(rhs)
            .expect("resulting value is out of range")
    }
}

impl SubAssign<MonthDuration> for PrimitiveDateTime {
    fn sub_assign(&mut self, rhs: MonthDuration) {
        *self = *self - rhs;
    }
}

impl MonthExtension for OffsetDateTime {
    fn checked_calendar_add(&self, duration: MonthDuration) -> Option<Self> {
        let date = self.date().checked_calendar_add(duration)?;
        Some(self.replace_date(date))
    }

    fn checked_calendar_sub(&self, duration: MonthDuration) -> Option<Self> {
        let date = self.date().checked_calendar_sub(duration)?;
        Some(self.replace_date(date))
    }

    fn saturating_calendar_add(&self, duration: MonthDuration) -> Self {
        if let Some(datetime) = self.checked_calendar_add(duration) {
            datetime
        } else if duration.is_negative() {
            PrimitiveDateTime::MIN.assume_offset(self.offset())
        } else {
            debug_assert!(duration.is_positive());
            PrimitiveDateTime::MAX.assume_offset(self.offset())
        }
    }

    fn saturating_calendar_sub(&self, duration: MonthDuration) -> Self {
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
