use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use time::{Date, Month};
use time::util::days_in_year_month;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MonthDuration {
    // TODO Replace with f32
    // see https://github.com/martsokha/timext/issues/6
    months: i32,
}

impl MonthDuration {
    pub const fn new(years: i32, months: i32) -> Self {
        let months = years
            .checked_mul(12)
            .expect("overflow constructing `time-monat::MonthDuration`")
            .checked_add(months)
            .expect("overflow constructing `time-monat::MonthDuration`");
        Self { months }
    }

    pub const fn years(years: i32) -> Self {
        let months = years
            .checked_mul(12)
            .expect("overflow constructing `time-monat::MonthDuration`");
        Self { months }
    }

    pub const fn months(months: i32) -> Self {
        Self { months }
    }

    pub const MIN: Self = Self::months(i32::MIN);
    pub const MAX: Self = Self::months(i32::MAX);
}

impl MonthDuration {
    /// Get the number of whole years in the duration.
    ///
    /// ```rust
    /// # use time_monat::NumericMonthDuration;
    /// assert_eq!(1.years().whole_years(), 1);
    /// assert_eq!((-1).years().whole_years(), -1);
    /// assert_eq!(6.months().whole_years(), 0);
    /// assert_eq!((-6).months().whole_years(), 0);
    /// ```
    pub const fn whole_years(self) -> i32 {
        self.months / 12
    }

    /// Get the number of whole months in the duration.
    ///
    /// ```rust
    /// # use time_monat::NumericMonthDuration;
    /// assert_eq!(1.months().whole_months(), 1);
    /// assert_eq!((-1).months().whole_months(), -1);
    /// assert_eq!(6.months().whole_years(), 0);
    /// assert_eq!((-6).months().whole_years(), 0);
    /// ```
    pub const fn whole_months(self) -> i32 {
        self.months
    }

    /// Get the number of months past the number of whole years.
    ///
    /// ```rust
    /// # use time_monat::NumericMonthDuration;
    /// assert_eq!(13.months().subyear_months(), 1);
    /// assert_eq!((-13).months().subyear_months(), -1);
    /// ```
    pub const fn subyear_months(self) -> i32 {
        self.months % 12
    }

    /// Check if a duration is negative.
    ///
    /// ```rust
    /// # use time_monat::NumericMonthDuration;
    /// assert!(0.months().is_zero());
    /// assert!(!1.months().is_zero());
    /// ```
    pub const fn is_zero(self) -> bool {
        self.months == 0
    }

    /// Check if a duration is positive.
    ///
    /// ```rust
    /// # use time_monat::NumericMonthDuration;
    /// assert!(1.months().is_positive());
    /// assert!(!0.months().is_positive());
    /// assert!(!(-1).months().is_positive());
    /// ```
    pub const fn is_positive(self) -> bool {
        self.months.is_positive()
    }

    /// Check if a duration is negative.
    ///
    /// ```rust
    /// # use time_monat::NumericMonthDuration;
    /// assert!((-1).months().is_negative());
    /// assert!(!0.months().is_negative());
    /// assert!(!1.months().is_negative());
    /// ```
    pub const fn is_negative(self) -> bool {
        self.months.is_negative()
    }
}

impl MonthDuration {
    pub const fn abs(self) -> Self {
        Self::months(self.whole_months().abs())
    }

    pub fn checked_date_add(self, date: &Date) -> Option<Date> {
        // [1, 12] + [-11, 11]
        let month = self.subyear_months();
        let month = month.checked_add(date.month() as i32)?;

        // Aug(8) + 6 = Feb(2) or Feb(2) - 6 = Aug(8)
        let added = match month {
            x if x.is_positive() => x / 12,
            _ => -1,
        };

        debug_assert!((-1..=1).contains(&added));
        let year = self
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

    pub fn checked_date_sub(self, date: &Date) -> Option<Date> {
        let duration = self.checked_neg()?;
        duration.checked_date_add(date)
    }
}

impl MonthDuration {
    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time_monat::{MonthDuration, NumericMonthDuration};
    /// assert_eq!(5.months().checked_add(5.months()), Some(10.months()));
    /// assert_eq!(MonthDuration::MAX.checked_add(1.months()), None);
    /// assert_eq!((-5).months().checked_add(5.months()), Some(0.months()));
    /// ```
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let months = self.months.checked_add(rhs.months)?;
        Some(Self { months })
    }

    /// Computes `self - rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time_monat::{MonthDuration, NumericMonthDuration};
    /// assert_eq!(5.months().checked_sub(5.months()), Some(0.months()));
    /// assert_eq!(MonthDuration::MIN.checked_sub(1.months()), None);
    /// assert_eq!(5.months().checked_sub(5.months()), Some(0.months()));
    /// ```
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let months = self.months.checked_sub(rhs.months)?;
        Some(Self { months })
    }

    /// Computes `self * rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time_monat::{MonthDuration, NumericMonthDuration};
    /// assert_eq!(5.months().checked_mul(2), Some(10.months()));
    /// assert_eq!(5.months().checked_mul(-2), Some((-10).months()));
    /// assert_eq!(5.months().checked_mul(0), Some(0.months()));
    /// assert_eq!(MonthDuration::MAX.checked_mul(2), None);
    /// assert_eq!(MonthDuration::MIN.checked_mul(2), None);
    /// ```
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        let months = self.months.checked_mul(rhs)?;
        Some(Self { months })
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or if the result would overflow.
    ///
    /// ```rust
    /// # use time_monat::NumericMonthDuration;
    /// assert_eq!(10.months().checked_div(2), Some(5.months()));
    /// assert_eq!(10.months().checked_div(-2), Some((-5).months()));
    /// assert_eq!(1.months().checked_div(0), None);
    /// ```
    pub fn checked_div(self, rhs: i32) -> Option<Self> {
        let months = self.months.checked_div(rhs)?;
        Some(Self { months })
    }

    /// Computes `-self`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time_monat::{MonthDuration, NumericMonthDuration};
    /// assert_eq!(10.months().checked_neg(), Some((-10).months()));
    /// assert_eq!(MonthDuration::MIN.checked_neg(), None);
    /// ```
    pub fn checked_neg(self) -> Option<Self> {
        let months = self.months.checked_neg()?;
        Some(Self { months })
    }
}

impl Display for MonthDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_negative() {
            f.write_str("-")?;
        }

        if f.precision().is_some() {
            if self.is_zero() {
                return (0.).fmt(f).and_then(|_| f.write_str("mo"));
            }

            if self.whole_years() >= 1 {
                return self.whole_years().fmt(f).and_then(|_| f.write_str("y"));
            }

            if self.whole_months() >= 1 {
                return self.whole_months().fmt(f).and_then(|_| f.write_str("mo"));
            }
        } else {
            if self.is_zero() {
                return f.write_str("0mo");
            }

            self.whole_years().fmt(f).and_then(|_| f.write_str("y"))?;
            self.whole_months().fmt(f).and_then(|_| f.write_str("mo"))?;
        }

        Ok(())
    }
}

// TODO Mut/Div u8, u16,i8, i16, f32 traits
// see https://github.com/martsokha/timext/issues/5

impl Add for MonthDuration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding MonthDuration")
    }
}

impl AddAssign for MonthDuration {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for MonthDuration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting MonthDuration")
    }
}

impl SubAssign for MonthDuration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Neg for MonthDuration {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.checked_neg()
            .expect("overflow when negating MonthDuration")
    }
}

impl Div<i32> for MonthDuration {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self.checked_div(rhs)
            .expect("overflow when dividing MonthDuration")
    }
}

impl DivAssign<i32> for MonthDuration {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs
    }
}

impl Mul<i32> for MonthDuration {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self.checked_mul(rhs)
            .expect("overflow when multiplying MonthDuration")
    }
}

impl MulAssign<i32> for MonthDuration {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs
    }
}
