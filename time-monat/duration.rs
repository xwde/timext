use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonatDuration {
    months: i32,
}

impl MonatDuration {
    pub const fn new(years: i32, months: i32) -> Self {
        let months = years
            .checked_mul(12)
            .expect("overflow constructing `time_monat::MonatDuration`")
            .checked_add(months)
            .expect("overflow constructing `time_monat::MonatDuration`");

        Self { months }
    }

    pub const fn years(years: i32) -> Self {
        Self { months: years * 12 }
    }

    pub const fn months(months: i32) -> Self {
        Self { months }
    }

    pub const fn whole_years(self) -> i32 {
        self.months / 12
    }

    pub const fn whole_months(self) -> i32 {
        self.months
    }

    pub const fn subyear_months(self) -> i32 {
        self.months % 12
    }

    pub const fn abs(self) -> Self {
        Self {
            months: self.months.abs(),
        }
    }

    pub const fn is_zero(self) -> bool {
        self.months == 0
    }

    pub const fn is_positive(self) -> bool {
        self.months > 0
    }

    pub const fn is_negative(self) -> bool {
        self.months < 0
    }
}

impl const Add for MonatDuration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::months(self.months + rhs.months)
    }
}

impl const AddAssign for MonatDuration {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl const Sub for MonatDuration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::months(self.months - rhs.months)
    }
}

impl const SubAssign for MonatDuration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl const Neg for MonatDuration {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::months(-self.months)
    }
}
