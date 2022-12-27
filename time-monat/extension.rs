use std::ops::{Add, AddAssign, Sub, SubAssign};

use time::util::days_in_year_month;
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime};

use crate::duration::MonatDuration;

pub trait MonatExt: Sized {
    fn checked_monat_add(self, duration: MonatDuration) -> Option<Self>;
    fn checked_monat_sub(self, duration: MonatDuration) -> Option<Self>;
    fn saturating_monat_add(self, duration: MonatDuration) -> Self;
    fn saturating_monat_sub(self, duration: MonatDuration) -> Self;
}

impl MonatExt for Date {
    fn checked_monat_add(self, duration: MonatDuration) -> Option<Self> {
        // [1, 12] + [-11, 11]
        let month = duration.subyear_months();
        let month = match month.checked_add(self.month() as i32) {
            Some(month) => month,
            None => return None,
        };

        // Aug(8) + 6 = Feb(2) or Feb(2) - 6 = Aug(8)
        let added = match month {
            x if x.is_positive() => x / 12,
            _ => -1,
        };

        debug_assert!((-1..=1).contains(&added));
        let year = self.year() + duration.whole_years() + added;

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

    fn checked_monat_sub(self, duration: MonatDuration) -> Option<Self> {
        match duration.whole_months().checked_neg() {
            Some(duration) => self.checked_monat_add(MonatDuration::months(duration)),
            None => None,
        }
    }

    fn saturating_monat_add(self, duration: MonatDuration) -> Self {
        if let Some(datetime) = self.checked_monat_add(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MIN
        } else {
            debug_assert!(duration.is_positive());
            Self::MAX
        }
    }

    fn saturating_monat_sub(self, duration: MonatDuration) -> Self {
        if let Some(datetime) = self.checked_monat_sub(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MAX
        } else {
            debug_assert!(duration.is_positive());
            Self::MIN
        }
    }
}

impl Add<MonatDuration> for Date {
    type Output = Self;

    fn add(self, rhs: MonatDuration) -> Self::Output {
        self.checked_monat_add(rhs)
            .expect("resulting value is out of range")
    }
}

impl AddAssign<MonatDuration> for Date {
    fn add_assign(&mut self, rhs: MonatDuration) {
        *self = *self + rhs;
    }
}

impl Sub<MonatDuration> for Date {
    type Output = Self;

    fn sub(self, rhs: MonatDuration) -> Self::Output {
        self.checked_monat_sub(rhs)
            .expect("resulting value is out of range")
    }
}

impl SubAssign<MonatDuration> for Date {
    fn sub_assign(&mut self, rhs: MonatDuration) {
        *self = *self - rhs;
    }
}

impl MonatExt for PrimitiveDateTime {
    fn checked_monat_add(self, duration: MonatDuration) -> Option<Self> {
        self.date()
            .checked_monat_add(duration)
            .map(|date| self.replace_date(date))
    }

    fn checked_monat_sub(self, duration: MonatDuration) -> Option<Self> {
        self.date()
            .checked_monat_sub(duration)
            .map(|date| self.replace_date(date))
    }

    fn saturating_monat_add(self, duration: MonatDuration) -> Self {
        if let Some(datetime) = self.checked_monat_add(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MIN
        } else {
            debug_assert!(duration.is_positive());
            Self::MAX
        }
    }

    fn saturating_monat_sub(self, duration: MonatDuration) -> Self {
        if let Some(datetime) = self.checked_monat_sub(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MAX
        } else {
            debug_assert!(duration.is_positive());
            Self::MIN
        }
    }
}

impl Add<MonatDuration> for PrimitiveDateTime {
    type Output = Self;

    fn add(self, rhs: MonatDuration) -> Self::Output {
        self.checked_monat_add(rhs)
            .expect("resulting value is out of range")
    }
}

impl AddAssign<MonatDuration> for PrimitiveDateTime {
    fn add_assign(&mut self, rhs: MonatDuration) {
        *self = *self + rhs;
    }
}

impl Sub<MonatDuration> for PrimitiveDateTime {
    type Output = Self;

    fn sub(self, rhs: MonatDuration) -> Self::Output {
        self.checked_monat_sub(rhs)
            .expect("resulting value is out of range")
    }
}

impl SubAssign<MonatDuration> for PrimitiveDateTime {
    fn sub_assign(&mut self, rhs: MonatDuration) {
        *self = *self - rhs;
    }
}

impl MonatExt for OffsetDateTime {
    fn checked_monat_add(self, duration: MonatDuration) -> Option<Self> {
        self.date()
            .checked_monat_add(duration)
            .map(|date| self.replace_date(date))
    }

    fn checked_monat_sub(self, duration: MonatDuration) -> Option<Self> {
        self.date()
            .checked_monat_sub(duration)
            .map(|date| self.replace_date(date))
    }

    fn saturating_monat_add(self, duration: MonatDuration) -> Self {
        if let Some(datetime) = self.checked_monat_add(duration) {
            datetime
        } else if duration.is_negative() {
            PrimitiveDateTime::MIN.assume_offset(self.offset())
        } else {
            debug_assert!(duration.is_positive());
            PrimitiveDateTime::MAX.assume_offset(self.offset())
        }
    }

    fn saturating_monat_sub(self, duration: MonatDuration) -> Self {
        if let Some(datetime) = self.checked_monat_sub(duration) {
            datetime
        } else if duration.is_negative() {
            PrimitiveDateTime::MAX.assume_offset(self.offset())
        } else {
            debug_assert!(duration.is_positive());
            PrimitiveDateTime::MIN.assume_offset(self.offset())
        }
    }
}
