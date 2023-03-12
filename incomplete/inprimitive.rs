use time::{Month, PrimitiveDateTime, UtcOffset, Weekday};

use crate::error::InComponentRange;
use crate::{InComplete, InDate, InOffsetDateTime, InTime};

/// An `InPrimitiveDateTime` struct represents an incomplete [time::PrimitiveDateTime] struct.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct InPrimitiveDateTime {
    date: InDate,
    time: InTime,
}

impl InPrimitiveDateTime {
    /// Create a new `InPrimitiveDateTime` from the provided `InDate` and `InTime`.
    pub fn new(date: InDate, time: InTime) -> Self {
        Self { date, time }
    }
}

impl InPrimitiveDateTime {
    /// Returns the date.
    pub fn date(self) -> InDate {
        self.date
    }

    /// Returns the year.
    pub fn year(self) -> Option<i32> {
        self.date.year()
    }

    /// Returns the month.
    pub fn month(self) -> Option<Month> {
        self.date.month()
    }

    /// Returns the weekday.
    pub fn weekday(self) -> Option<Weekday> {
        self.date.weekday()
    }

    /// Returns the day of the month.
    pub fn day(self) -> Option<u8> {
        self.date.day()
    }

    /// Returns the time.
    pub fn time(self) -> InTime {
        self.time
    }

    /// Returns the clock hour.
    pub fn hour(self) -> Option<u8> {
        self.time.hour()
    }

    /// Returns the minute within the hour.
    pub fn minute(self) -> Option<u8> {
        self.time.minute()
    }

    /// Returns the second within the minute.
    pub fn second(self) -> Option<u8> {
        self.time.second()
    }

    /// Returns the milliseconds within the second.
    pub fn millisecond(self) -> Option<u16> {
        self.time.millisecond()
    }

    /// Returns the microseconds within the second.
    pub fn microsecond(self) -> Option<u32> {
        self.time.microsecond()
    }

    /// Returns the nanoseconds within the second.
    pub fn nanosecond(self) -> Option<u32> {
        self.time.nanosecond()
    }
}

impl InPrimitiveDateTime {
    /// Replaces the date.
    pub fn replace_date(self, date: InDate) -> Result<Self, InComponentRange> {
        Ok(date.with_time(self.time))
    }

    /// Replaces the year.
    pub fn replace_year(self, year: Option<i32>) -> Result<Self, InComponentRange> {
        self.replace_date(self.date.replace_year(year)?)
    }

    /// Replaces the month of the year.
    pub fn replace_month(self, month: Option<Month>) -> Result<Self, InComponentRange> {
        self.replace_date(self.date.replace_month(month)?)
    }

    /// Replaces the day of the month.
    pub fn replace_day(self, day: Option<u8>) -> Result<Self, InComponentRange> {
        self.replace_date(self.date.replace_day(day)?)
    }

    /// Returns the time.
    pub fn replace_time(self, time: InTime) -> Result<Self, InComponentRange> {
        Ok(self.date.with_time(time))
    }

    /// Returns the clock hour.
    pub fn replace_hour(self, hour: Option<u8>) -> Result<Self, InComponentRange> {
        self.replace_time(self.time.replace_hour(hour)?)
    }

    /// Returns the minute within the hour.
    pub fn replace_minute(self, minute: Option<u8>) -> Result<Self, InComponentRange> {
        self.replace_time(self.time.replace_minute(minute)?)
    }

    /// Returns the second within the minute.
    pub fn replace_second(self, second: Option<u8>) -> Result<Self, InComponentRange> {
        self.replace_time(self.time.replace_second(second)?)
    }

    /// Returns the milliseconds within the second.
    pub fn replace_millisecond(self, millisecond: Option<u16>) -> Result<Self, InComponentRange> {
        self.replace_time(self.time.replace_millisecond(millisecond)?)
    }

    /// Returns the microseconds within the second.
    pub fn replace_microsecond(self, microsecond: Option<u32>) -> Result<Self, InComponentRange> {
        self.replace_time(self.time.replace_microsecond(microsecond)?)
    }

    /// Returns the nanoseconds within the second.
    pub fn replace_nanosecond(self, nanosecond: Option<u32>) -> Result<Self, InComponentRange> {
        self.replace_time(self.time.replace_nanosecond(nanosecond)?)
    }
}

impl InPrimitiveDateTime {
    /// Assuming that the existing `InPrimitiveDateTime` represents a moment
    /// in the provided `UtcOffset`, returns an `InOffsetDateTime`.
    pub fn assume_offset(self, offset: Option<UtcOffset>) -> InOffsetDateTime {
        InOffsetDateTime::new(self, offset)
    }

    /// Assuming that the existing `InPrimitiveDateTime` represents a moment
    /// in UTC, returns an `InOffsetDateTime`.
    pub fn assume_utc(self) -> InOffsetDateTime {
        self.assume_offset(Some(UtcOffset::UTC))
    }
}

impl InComplete for InPrimitiveDateTime {
    type Complete = PrimitiveDateTime;

    fn from_complete(complete: Self::Complete) -> Self {
        let d = InDate::from_complete(complete.date());
        let t = InTime::from_complete(complete.time());
        Self::new(d, t)
    }

    fn into_complete(self) -> Result<Self::Complete, InComponentRange> {
        let d = self.date.into_complete()?;
        let t = self.time.into_complete()?;
        Ok(Self::Complete::new(d, t))
    }

    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, InComponentRange> {
        let d = self.date.with_fallback(fallback.date())?;
        let t = self.time.with_fallback(fallback.time())?;
        Ok(Self::new(d, t))
    }
}

impl From<PrimitiveDateTime> for InPrimitiveDateTime {
    fn from(datetime: PrimitiveDateTime) -> Self {
        Self::from_complete(datetime)
    }
}

impl TryFrom<InPrimitiveDateTime> for PrimitiveDateTime {
    type Error = InComponentRange;

    fn try_from(datetime: InPrimitiveDateTime) -> Result<Self, Self::Error> {
        datetime.into_complete()
    }
}
