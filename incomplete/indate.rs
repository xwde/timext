use time::{Date, Month, Weekday};

use crate::error::{InCompleteError, InComponentRange};
use crate::{InComplete, InPrimitiveDateTime, InTime};

/// An `InDate` struct represents an incomplete [time::Date] struct.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct InDate {
    year: Option<i32>,
    month: Option<Month>,
    day: Option<u8>,
}

impl InDate {
    /// Attempts to create a `InDate` from the year, month, and day.
    pub fn from_calendar_date(
        year: Option<i32>,
        month: Option<Month>,
        day: Option<u8>,
    ) -> Result<Self, InComponentRange> {
        // TODO: Soundness.
        Ok(Self::from_calendar_date_unchecked(year, month, day))
    }

    /// Creates a `InDate` from its components.
    fn from_calendar_date_unchecked(
        year: Option<i32>,
        month: Option<Month>,
        day: Option<u8>,
    ) -> Self {
        Self { year, month, day }
    }
}

impl InDate {
    /// Returns the year.
    pub fn year(self) -> Option<i32> {
        self.year
    }

    /// Returns the month.
    pub fn month(self) -> Option<Month> {
        self.month
    }

    /// Returns the weekday.
    pub fn weekday(self) -> Option<Weekday> {
        self.into_complete().ok().map(|d| d.weekday())
    }

    /// Returns the day of the month.
    pub fn day(self) -> Option<u8> {
        self.day
    }
}

impl InDate {
    /// Replaces the year.
    pub fn replace_year(self, year: Option<i32>) -> Result<Self, InComponentRange> {
        Self::from_calendar_date(year, self.month(), self.day())
    }

    /// Replaces the month of the year.
    pub fn replace_month(self, month: Option<Month>) -> Result<Self, InComponentRange> {
        Self::from_calendar_date(self.year(), month, self.day())
    }

    /// Replaces the day of the month.
    pub fn replace_day(self, day: Option<u8>) -> Result<Self, InComponentRange> {
        Self::from_calendar_date(self.year(), self.month(), day)
    }
}

impl InDate {
    /// Create a `InPrimitiveDateTime` using the existing date and the provided `InTime`.
    pub fn with_time(self, time: InTime) -> InPrimitiveDateTime {
        InPrimitiveDateTime::new(self, time)
    }
}

impl InComplete for InDate {
    type Complete = Date;

    fn from_complete(complete: Self::Complete) -> Self {
        let y = Some(complete.year());
        let m = Some(complete.month());
        let d = Some(complete.day());
        Self::from_calendar_date_unchecked(y, m, d)
    }

    fn into_complete(self) -> Result<Self::Complete, InComponentRange> {
        if self.year.is_none() {
            return Err(InCompleteError::new("year").into());
        } else if self.month.is_none() {
            return Err(InCompleteError::new("month").into());
        } else if self.day.is_none() {
            return Err(InCompleteError::new("day").into());
        }

        let y = self.year.unwrap();
        let m = self.month.unwrap();
        let d = self.day.unwrap();

        let date = Self::Complete::from_calendar_date(y, m, d);
        date.map_err(|e| e.into())
    }

    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, InComponentRange> {
        let y = Some(self.year.unwrap_or(fallback.year()));
        let m = Some(self.month.unwrap_or(fallback.month()));
        let d = Some(self.day.unwrap_or(fallback.day()));
        Self::from_calendar_date(y, m, d)
    }
}

impl From<Date> for InDate {
    fn from(date: Date) -> Self {
        Self::from_complete(date)
    }
}

impl TryFrom<InDate> for Date {
    type Error = InComponentRange;

    fn try_from(date: InDate) -> Result<Self, Self::Error> {
        date.into_complete()
    }
}
