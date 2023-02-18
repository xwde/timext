use time::{Date, Month, Weekday};

use crate::error::{InComponentRange, NoComponent};
use crate::{InComplete, InPrimitiveDateTime, InTime};

#[cfg(feature = "formatting")]
use crate::{error::InFormat, InFormattable};
#[cfg(feature = "parsing")]
use crate::{error::InParse, InParsable};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct InDate {
    year: Option<i32>,
    month: Option<Month>,
    day: Option<u8>,
}

impl InDate {
    pub fn from_calendar_date(
        year: Option<i32>,
        month: Option<Month>,
        day: Option<u8>,
    ) -> Result<Self, InComponentRange> {
        // TODO soundness
        Ok(Self::from_calendar_date_unchecked(year, month, day))
    }

    fn from_calendar_date_unchecked(
        year: Option<i32>,
        month: Option<Month>,
        day: Option<u8>,
    ) -> Self {
        Self { year, month, day }
    }
}

impl InDate {
    pub fn day(self) -> Option<u8> {
        self.day
    }

    pub fn weekday(self) -> Option<Weekday> {
        self.into_complete().ok().map(|d| d.weekday())
    }

    pub fn month(self) -> Option<Month> {
        self.month
    }

    pub fn year(self) -> Option<i32> {
        self.year
    }
}

impl InDate {
    pub fn replace_day(self, day: Option<u8>) -> Result<Self, InComponentRange> {
        Self::from_calendar_date(self.year(), self.month(), day)
    }

    pub fn replace_month(self, month: Option<Month>) -> Result<Self, InComponentRange> {
        Self::from_calendar_date(self.year(), month, self.day())
    }

    pub fn replace_year(self, year: Option<i32>) -> Result<Self, InComponentRange> {
        Self::from_calendar_date(year, self.month(), self.day())
    }
}

impl InDate {
    pub fn with_time(self, time: InTime) -> InPrimitiveDateTime {
        InPrimitiveDateTime::new(self, time)
    }
}

#[cfg(feature = "formatting")]
impl InDate {
    pub fn format(self, format: &impl InFormattable) -> Result<String, InFormat> {
        format.format(Some(self), None, None)
    }
}

#[cfg(feature = "parsing")]
impl InDate {
    pub fn parse(input: &str, description: &impl InParsable) -> Result<Self, InParse> {
        description.parse_date(input.as_bytes())
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
            return Err(NoComponent::new("year").into());
        } else if self.month.is_none() {
            return Err(NoComponent::new("month").into());
        } else if self.day.is_none() {
            return Err(NoComponent::new("day").into());
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
