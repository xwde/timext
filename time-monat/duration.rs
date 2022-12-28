use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonthDuration {
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
}

impl MonthDuration {
    pub const fn whole_years(self) -> i32 {
        self.months / 12
    }

    pub const fn whole_months(self) -> i32 {
        self.months
    }

    pub const fn subyear_months(self) -> i32 {
        self.months % 12
    }

    pub const fn is_zero(self) -> bool {
        self.months == 0
    }

    pub const fn is_positive(self) -> bool {
        self.months.is_positive()
    }

    pub const fn is_negative(self) -> bool {
        self.months.is_negative()
    }
}

impl MonthDuration {
    pub const fn abs(self) -> Self {
        Self::months(self.whole_months().abs())
    }
}

impl MonthDuration {
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let months = self.months.checked_add(rhs.months)?;
        Some(Self { months })
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let months = self.months.checked_sub(rhs.months)?;
        Some(Self { months })
    }
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        let months = self.months.checked_sub(rhs)?;
        Some(Self { months })
    }

    pub fn checked_div(self, rhs: i32) -> Option<Self> {
        let months = self.months.checked_div(rhs)?;
        Some(Self { months })
    }

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
