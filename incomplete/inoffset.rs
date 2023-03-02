use time::{Month, OffsetDateTime, PrimitiveDateTime, UtcOffset, Weekday};

use crate::error::{InComponentRange, Insufficient};
use crate::{InComplete, InDate, InPrimitiveDateTime, InTime};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct InOffsetDateTime {
    datetime: InPrimitiveDateTime,
    offset: Option<UtcOffset>,
}

impl InOffsetDateTime {
    pub fn new(datetime: InPrimitiveDateTime, offset: Option<UtcOffset>) -> Self {
        Self { datetime, offset }
    }
}

impl InOffsetDateTime {
    pub fn offset(self) -> Option<UtcOffset> {
        self.offset
    }

    pub fn date(self) -> InDate {
        self.datetime.date()
    }

    pub fn year(self) -> Option<i32> {
        self.datetime.year()
    }

    pub fn month(self) -> Option<Month> {
        self.datetime.month()
    }

    pub fn weekday(self) -> Option<Weekday> {
        self.datetime.weekday()
    }

    pub fn day(self) -> Option<u8> {
        self.datetime.day()
    }

    pub fn time(self) -> InTime {
        self.datetime.time()
    }

    pub fn hour(self) -> Option<u8> {
        self.datetime.hour()
    }

    pub fn minute(self) -> Option<u8> {
        self.datetime.minute()
    }

    pub fn second(self) -> Option<u8> {
        self.datetime.second()
    }

    pub fn millisecond(self) -> Option<u16> {
        self.datetime.millisecond()
    }

    pub fn microsecond(self) -> Option<u32> {
        self.datetime.microsecond()
    }

    pub fn nanosecond(self) -> Option<u32> {
        self.datetime.nanosecond()
    }
}

impl InOffsetDateTime {
    pub fn replace_offset(self, offset: Option<UtcOffset>) -> Result<Self, InComponentRange> {
        Ok(Self::new(self.datetime, offset))
    }

    pub fn replace_date_time(
        self,
        datetime: InPrimitiveDateTime,
    ) -> Result<Self, InComponentRange> {
        Ok(Self::new(datetime, self.offset))
    }

    pub fn replace_date(self, date: InDate) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_date(date)?)
    }

    pub fn replace_year(self, year: Option<i32>) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_year(year)?)
    }

    pub fn replace_month(self, month: Option<Month>) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_month(month)?)
    }

    pub fn replace_day(self, day: Option<u8>) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_day(day)?)
    }

    pub fn replace_time(self, time: InTime) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_time(time)?)
    }

    pub fn replace_hour(self, hour: Option<u8>) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_hour(hour)?)
    }

    pub fn replace_minute(self, minute: Option<u8>) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_minute(minute)?)
    }

    pub fn replace_second(self, second: Option<u8>) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_second(second)?)
    }

    pub fn replace_millisecond(self, millisecond: Option<u16>) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_millisecond(millisecond)?)
    }

    pub fn replace_microsecond(self, microsecond: Option<u32>) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_microsecond(microsecond)?)
    }

    pub fn replace_nanosecond(self, nanosecond: Option<u32>) -> Result<Self, InComponentRange> {
        self.replace_date_time(self.datetime.replace_nanosecond(nanosecond)?)
    }
}

impl InComplete for InOffsetDateTime {
    type Complete = OffsetDateTime;

    fn from_complete(complete: Self::Complete) -> Self {
        let dt = PrimitiveDateTime::new(complete.date(), complete.time());
        Self::new(
            InPrimitiveDateTime::from_complete(dt),
            Some(complete.offset()),
        )
    }

    fn into_complete(self) -> Result<Self::Complete, InComponentRange> {
        match self.offset {
            Some(offset) => {
                let dt = self.datetime.into_complete()?;
                Ok(dt.assume_offset(offset))
            }
            None => Err(Insufficient::new("offset").into()),
        }
    }

    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, InComponentRange> {
        let dt = PrimitiveDateTime::new(fallback.date(), fallback.time());
        let dt = self.datetime.with_fallback(dt)?;
        let os = Some(self.offset.unwrap_or(fallback.offset()));
        Ok(Self::new(dt, os))
    }
}

impl From<OffsetDateTime> for InOffsetDateTime {
    fn from(datetime: OffsetDateTime) -> Self {
        Self::from_complete(datetime)
    }
}

impl TryFrom<InOffsetDateTime> for OffsetDateTime {
    type Error = InComponentRange;

    fn try_from(datetime: InOffsetDateTime) -> Result<Self, Self::Error> {
        datetime.into_complete()
    }
}
