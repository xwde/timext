use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use time::util::days_in_year_month;
use time::{Date, Month};

// TODO Represent fraction of week with opt f32?
// see https://github.com/martsokha/timext/issues/6
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MonthDuration {
    months: i32,
}

impl MonthDuration {
    pub const fn new(years: i32, months: i32) -> Self {
        let months = years
            .checked_mul(12)
            .expect("overflow constructing `timext::MonthDuration`")
            .checked_add(months)
            .expect("overflow constructing `timext::MonthDuration`");
        Self { months }
    }

    pub const fn years(years: i32) -> Self {
        let months = years
            .checked_mul(12)
            .expect("overflow constructing `timext::MonthDuration`");
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
    /// # use timext::ext::NumericMonthDuration;
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
    /// # use timext::ext::NumericMonthDuration;
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
    /// # use timext::ext::NumericMonthDuration;
    /// assert_eq!(13.months().subyear_months(), 1);
    /// assert_eq!((-13).months().subyear_months(), -1);
    /// ```
    pub const fn subyear_months(self) -> i32 {
        self.months % 12
    }

    /// Check if a duration is negative.
    ///
    /// ```rust
    /// # use timext::ext::NumericMonthDuration;
    /// assert!(0.months().is_zero());
    /// assert!(!1.months().is_zero());
    /// ```
    pub const fn is_zero(self) -> bool {
        self.months == 0
    }

    /// Check if a duration is positive.
    ///
    /// ```rust
    /// # use timext::ext::NumericMonthDuration;
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
    /// # use timext::ext::NumericMonthDuration;
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

    pub fn checked_date_add(self, date: Date) -> Option<Date> {
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

    pub fn checked_date_sub(self, date: Date) -> Option<Date> {
        let duration = self.checked_neg()?;
        duration.checked_date_add(date)
    }
}

impl MonthDuration {
    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use timext::{MonthDuration, ext::NumericMonthDuration};
    /// assert_eq!(5.months().checked_add(5.months()), Some(10.months()));
    /// assert_eq!(MonthDuration::MAX.checked_add(1.months()), None);
    /// assert_eq!((-5).months().checked_add(5.months()), Some(0.months()));
    /// ```
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.months.checked_add(rhs.months).map(Self::months)
    }

    /// Computes `self - rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use timext::{MonthDuration, ext::NumericMonthDuration};
    /// assert_eq!(5.months().checked_sub(5.months()), Some(0.months()));
    /// assert_eq!(MonthDuration::MIN.checked_sub(1.months()), None);
    /// assert_eq!(5.months().checked_sub(5.months()), Some(0.months()));
    /// ```
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.months.checked_sub(rhs.months).map(Self::months)
    }

    /// Computes `self * rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use timext::{MonthDuration, ext::NumericMonthDuration};
    /// assert_eq!(5.months().checked_mul(2), Some(10.months()));
    /// assert_eq!(5.months().checked_mul(-2), Some((-10).months()));
    /// assert_eq!(5.months().checked_mul(0), Some(0.months()));
    /// assert_eq!(MonthDuration::MAX.checked_mul(2), None);
    /// assert_eq!(MonthDuration::MIN.checked_mul(2), None);
    /// ```
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        self.months.checked_mul(rhs).map(Self::months)
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or if the result would overflow.
    ///
    /// ```rust
    /// # use timext::ext::NumericMonthDuration;
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
    /// # use timext::{MonthDuration, ext::NumericMonthDuration};
    /// assert_eq!(10.months().checked_neg(), Some((-10).months()));
    /// assert_eq!(MonthDuration::MIN.checked_neg(), None);
    /// ```
    pub fn checked_neg(self) -> Option<Self> {
        self.months.checked_neg().map(Self::months)
    }
}

impl Display for MonthDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.is_negative() {
            f.write_str("-")?;
        }

        let years = self.abs().whole_years();
        let months = self.abs().subyear_months();
        match (self.is_zero(), years, months) {
            (true, _, _) => (0.).fmt(f).and_then(|_| f.write_str("mo")),
            (_, y, _) if y.is_positive() => y.fmt(f).and_then(|_| f.write_str("y")),
            (_, _, m) if m.is_positive() => m.fmt(f).and_then(|_| f.write_str("mo")),
            (_, _, _) => unreachable!(),
        }
    }
}

impl Add for MonthDuration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding timext::MonthDuration")
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
            .expect("overflow when subtracting timext::MonthDuration")
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
            .expect("overflow when negating timext::MonthDuration")
    }
}

// TODO add arithmetic for f32/f64

macro_rules! impl_md {
    ($($t:ty),+) => {$(
        impl Div<$t> for MonthDuration {
            type Output = Self;

            fn div(self, rhs: $t) -> Self::Output {
                self.checked_div(rhs as i32)
                    .expect("overflow when dividing timext::MonthDuration")
            }
        }

        impl DivAssign<$t> for MonthDuration {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs
            }
        }

        impl Mul<$t> for MonthDuration {
            type Output = Self;

            fn mul(self, rhs: $t) -> Self::Output {
                self.checked_mul(rhs as i32)
                    .expect("overflow when multiplying timext::MonthDuration")
            }
        }

        impl Mul<MonthDuration> for $t {
            type Output = MonthDuration;

            fn mul(self, rhs: MonthDuration) -> Self::Output {
                rhs * self
            }
        }

        impl MulAssign<$t> for MonthDuration {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs
            }
        }
    )+};
}

impl_md![i8, i16, i32, u8, u16, u32];
