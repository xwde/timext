use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use time::util::days_in_year_month;
use time::{Date, Month};

/// TODO Represent fraction of week with opt f32
/// see https://github.com/xwde/timext/issues/6
///
/// ```rust
/// # use time::{Date, Month};
/// # use timext::ext::NumericCalendarDuration;
///
/// let d0 = Date::from_calendar_date(2023, Month::January, 31).unwrap();
/// let d1 = Date::from_calendar_date(2023, Month::February, 28).unwrap();
/// assert_eq!(d0 + 1.months(), d1);
///
/// let d0 = Date::from_calendar_date(2024, Month::February, 29).unwrap();
/// let d1 = Date::from_calendar_date(2025, Month::February, 28).unwrap();///
/// assert_eq!(d0 + 1.years(), d1);
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct CalendarDuration {
    months: i32,
}

impl CalendarDuration {
    /// Creates a new `CalendarDuration` with provided years and months.
    ///
    /// ```rust
    /// # use timext::CalendarDuration;
    /// let d0 = CalendarDuration::new(2, 24);
    /// assert_eq!(d0.whole_years(), 4);
    /// assert_eq!(d0.whole_months(), 48);
    /// ```
    #[must_use]
    pub const fn new(years: i32, months: i32) -> Self {
        let months = years
            .checked_mul(12)
            .expect("overflow constructing `timext::CalendarDuration`")
            .checked_add(months)
            .expect("overflow constructing `timext::CalendarDuration`");
        Self { months }
    }

    /// Creates a new `CalendarDuration` with provided years.
    ///
    /// ```rust
    /// # use timext::CalendarDuration;
    /// let d0 = CalendarDuration::years(2);
    /// assert_eq!(d0.whole_years(), 2);
    /// assert_eq!(d0.whole_months(), 24);
    /// ```
    #[must_use]
    pub const fn years(years: i32) -> Self {
        Self::new(years, 0)
    }

    /// Creates a new `CalendarDuration` with provided months.
    ///
    /// ```rust
    /// # use timext::CalendarDuration;
    /// let d0 = CalendarDuration::months(24);
    /// assert_eq!(d0.whole_years(), 2);
    /// assert_eq!(d0.whole_months(), 24);
    /// ```
    #[must_use]
    pub const fn months(months: i32) -> Self {
        Self::new(0, months)
    }

    pub const MIN: Self = Self::months(i32::MIN);
    pub const MAX: Self = Self::months(i32::MAX);
}

impl CalendarDuration {
    /// Returns the number of whole years in the `CalendarDuration`.
    ///
    /// ```rust
    /// # use timext::ext::NumericCalendarDuration;
    /// assert_eq!(1.years().whole_years(), 1);
    /// assert_eq!((-1).years().whole_years(), -1);
    /// assert_eq!(6.months().whole_years(), 0);
    /// assert_eq!((-6).months().whole_years(), 0);
    /// ```
    #[must_use]
    pub const fn whole_years(self) -> i32 {
        self.months / 12
    }

    /// Returns the number of whole months in the `CalendarDuration`.
    ///
    /// ```rust
    /// # use timext::ext::NumericCalendarDuration;
    /// assert_eq!(1.months().whole_months(), 1);
    /// assert_eq!((-1).months().whole_months(), -1);
    /// assert_eq!(6.months().whole_years(), 0);
    /// assert_eq!((-6).months().whole_years(), 0);
    /// ```
    #[must_use]
    pub const fn whole_months(self) -> i32 {
        self.months
    }

    /// Returns the number of months past the number of whole years.
    ///
    /// ```rust
    /// # use timext::ext::NumericCalendarDuration;
    /// assert_eq!(13.months().subyear_months(), 1);
    /// assert_eq!((-13).months().subyear_months(), -1);
    /// ```
    #[must_use]
    pub const fn subyear_months(self) -> i32 {
        self.months % 12
    }

    /// Checks if a duration is negative.
    ///
    /// ```rust
    /// # use timext::ext::NumericCalendarDuration;
    /// assert!(0.months().is_zero());
    /// assert!(!1.months().is_zero());
    /// ```
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.months == 0
    }

    /// Checks if a `CalendarDuration` is positive.
    ///
    /// ```rust
    /// # use timext::ext::NumericCalendarDuration;
    /// assert!(1.months().is_positive());
    /// assert!(!0.months().is_positive());
    /// assert!(!(-1).months().is_positive());
    /// ```
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.months.is_positive()
    }

    /// Checks if a `CalendarDuration` is negative.
    ///
    /// ```rust
    /// # use timext::ext::NumericCalendarDuration;
    /// assert!((-1).months().is_negative());
    /// assert!(!0.months().is_negative());
    /// assert!(!1.months().is_negative());
    /// ```
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.months.is_negative()
    }
}

impl CalendarDuration {
    /// Returns the absolute value of the duration.
    ///
    /// ```rust
    /// # use timext::ext::NumericCalendarDuration;
    /// assert_eq!(1.months().abs(), 1.months());
    /// assert_eq!(0.months().abs(), 0.months());
    /// assert_eq!((-1).months().abs(), 1.months());
    /// ```
    #[must_use]
    pub const fn abs(self) -> Self {
        Self::months(self.whole_months().abs())
    }
}

impl CalendarDuration {
    /// Returns the sum of provided `Date` and `CalendarDuration`.
    ///
    /// ```rust
    /// # use time::{Date, Month::{October, September}};
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// let d0 = Date::from_calendar_date(2018, September, 1).unwrap();
    /// let d1 = Date::from_calendar_date(2018, October, 1).unwrap();
    /// let rs = CalendarDuration::checked_date_add(d0, 1.months());
    /// assert_eq!(rs.unwrap(), d1);
    /// ```
    #[must_use]
    pub fn checked_date_add(date: Date, duration: Self) -> Option<Date> {
        // [1, 12] + [-11, 11]
        let month = duration.subyear_months();
        let month = month.checked_add(date.month() as i32)?;

        // Aug(8) + 6 = Feb(2) or Feb(2) - 6 = Aug(8)
        let added = match month {
            x if x.is_positive() => x / 12,
            _ => -1,
        };

        debug_assert!((-1..=1).contains(&added));
        let year = duration
            .whole_years()
            .checked_add(added)?
            .checked_add(date.year())?;

        // Feb(2) - 6 = -4; 12 - 4 = Aug(8)
        let month = match month {
            x if x.is_positive() => x % 12,
            x => 12 + x,
        };

        debug_assert!((1..=12).contains(&month));
        let month = Month::try_from(month as u8).unwrap();
        let day = days_in_year_month(year, month).min(date.day());
        Date::from_calendar_date(year, month, day).ok()
    }

    /// Returns the difference of provided `Date` and `CalendarDuration`.
    ///
    /// ```rust
    /// # use time::{Date, Month::{August, September}};
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// let d0 = Date::from_calendar_date(2018, September, 1).unwrap();
    /// let d1 = Date::from_calendar_date(2018, August, 1).unwrap();
    /// let rs = CalendarDuration::checked_date_sub(d0, 1.months());
    /// assert_eq!(rs.unwrap(), d1);
    /// ```
    #[must_use]
    pub fn checked_date_sub(date: Date, duration: Self) -> Option<Date> {
        let duration = duration.checked_neg()?;
        Self::checked_date_add(date, duration)
    }
}

impl CalendarDuration {
    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// assert_eq!(5.months().checked_add(5.months()), Some(10.months()));
    /// assert_eq!(CalendarDuration::MAX.checked_add(1.months()), None);
    /// assert_eq!((-5).months().checked_add(5.months()), Some(0.months()));
    /// ```
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.months.checked_add(rhs.months).map(Self::months)
    }

    /// Computes `self - rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// assert_eq!(5.months().checked_sub(5.months()), Some(0.months()));
    /// assert_eq!(CalendarDuration::MIN.checked_sub(1.months()), None);
    /// assert_eq!(5.months().checked_sub(5.months()), Some(0.months()));
    /// ```
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.months.checked_sub(rhs.months).map(Self::months)
    }

    /// Computes `self * rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// assert_eq!(5.months().checked_mul(2), Some(10.months()));
    /// assert_eq!(5.months().checked_mul(-2), Some((-10).months()));
    /// assert_eq!(5.months().checked_mul(0), Some(0.months()));
    /// assert_eq!(CalendarDuration::MAX.checked_mul(2), None);
    /// assert_eq!(CalendarDuration::MIN.checked_mul(2), None);
    /// ```
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        self.months.checked_mul(rhs).map(Self::months)
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or if the result would overflow.
    ///
    /// ```rust
    /// # use timext::ext::NumericCalendarDuration;
    /// assert_eq!(10.months().checked_div(2), Some(5.months()));
    /// assert_eq!(10.months().checked_div(-2), Some((-5).months()));
    /// assert_eq!(1.months().checked_div(0), None);
    /// ```
    pub fn checked_div(self, rhs: i32) -> Option<Self> {
        self.months.checked_div(rhs).map(Self::months)
    }

    /// Computes `-self`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use timext::{CalendarDuration, ext::NumericCalendarDuration};
    /// assert_eq!(10.months().checked_neg(), Some((-10).months()));
    /// assert_eq!(CalendarDuration::MIN.checked_neg(), None);
    /// ```
    pub fn checked_neg(self) -> Option<Self> {
        self.months.checked_neg().map(Self::months)
    }
}

impl Display for CalendarDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.is_negative() {
            f.write_str("-")?;
        }

        let years = self.abs().whole_years();
        let months = self.abs().subyear_months();
        match (self.is_zero(), years, months) {
            (true, _, _) => (0f32).fmt(f).and_then(|_| f.write_str("mo")),
            (_, y, _) if y.is_positive() => y.fmt(f).and_then(|_| f.write_str("y")),
            (_, _, m) if m.is_positive() => m.fmt(f).and_then(|_| f.write_str("mo")),
            (_, _, _) => unreachable!(),
        }
    }
}

impl Add for CalendarDuration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding timext::CalendarDuration")
    }
}

impl AddAssign for CalendarDuration {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for CalendarDuration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting timext::CalendarDuration")
    }
}

impl SubAssign for CalendarDuration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for CalendarDuration {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.checked_neg()
            .expect("overflow when negating timext::CalendarDuration")
    }
}

// TODO add arithmetic for f32/f64

macro_rules! impl_md {
    ($($t:ty),+) => {$(
        impl Div<$t> for CalendarDuration {
            type Output = Self;

            fn div(self, rhs: $t) -> Self::Output {
                self.checked_div(rhs as i32)
                    .expect("overflow when dividing timext::CalendarDuration")
            }
        }

        impl DivAssign<$t> for CalendarDuration {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs
            }
        }

        impl Mul<$t> for CalendarDuration {
            type Output = Self;

            fn mul(self, rhs: $t) -> Self::Output {
                self.checked_mul(rhs as i32)
                    .expect("overflow when multiplying timext::CalendarDuration")
            }
        }

        impl Mul<CalendarDuration> for $t {
            type Output = CalendarDuration;

            fn mul(self, rhs: CalendarDuration) -> Self::Output {
                rhs * self
            }
        }

        impl MulAssign<$t> for CalendarDuration {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs
            }
        }
    )+};
}

impl_md![i8, i16, i32, u8, u16, u32];

#[cfg(test)]
mod tests {
    use time::Date;
    use time::Month::*;

    use crate::ext::NumericCalendarDuration;

    #[test]
    fn sub_one() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let d1 = Date::from_calendar_date(2022, December, 1).unwrap();
        assert_eq!(d0 - 13.months(), d1);
    }

    #[test]
    fn sub_many() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let d1 = Date::from_calendar_date(2019, December, 1).unwrap();
        assert_eq!(d0 - 49.months(), d1);
    }

    #[test]
    fn sub_max() {
        let d0 = Date::from_calendar_date(2024, January, 1).unwrap();
        let d1 = Date::from_calendar_date(2023, February, 1).unwrap();
        assert_eq!(d0 - 11.months(), d1);
    }

    #[test]
    fn add_one() {
        let d0 = Date::from_calendar_date(2024, December, 1).unwrap();
        let d1 = Date::from_calendar_date(2026, January, 1).unwrap();
        assert_eq!(d0 + 13.months(), d1);
    }

    #[test]
    fn add_many() {
        let d0 = Date::from_calendar_date(2024, December, 1).unwrap();
        let d1 = Date::from_calendar_date(2029, January, 1).unwrap();
        assert_eq!(d0 + 49.months(), d1);
    }

    #[test]
    fn add_max() {
        let d0 = Date::from_calendar_date(2024, February, 1).unwrap();
        let d1 = Date::from_calendar_date(2025, January, 1).unwrap();
        assert_eq!(d0 + 11.months(), d1);
    }

    #[test]
    fn add_leap() {
        let d0 = Date::from_calendar_date(2024, February, 29).unwrap();
        let d1 = Date::from_calendar_date(2025, February, 28).unwrap();
        assert_eq!(d0 + 12.months(), d1);
    }

    #[test]
    fn sub_leap() {
        let d0 = Date::from_calendar_date(2024, February, 29).unwrap();
        let d1 = Date::from_calendar_date(2023, February, 28).unwrap();
        assert_eq!(d0 - 12.months(), d1);
    }
}
