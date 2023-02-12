use time::error::ComponentRange;
use time::{Date, Month, Time, Weekday};

use crate::incomplete::Incomplete;
use crate::InPrimitiveDateTime;
use crate::InTime;

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
    ) -> Result<Self, ComponentRange> {
        todo!()
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
    pub fn replace_day(self, day: Option<u8>) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn replace_month(self, month: Option<Month>) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn replace_year(self, year: Option<i32>) -> Result<Self, ComponentRange> {
        todo!()
    }
}

impl InDate {
    pub fn with_time(self, time: Time) -> InPrimitiveDateTime {
        self.with_incomplete_time(time.into())
    }

    pub fn with_incomplete_time(self, time: InTime) -> InPrimitiveDateTime {
        InPrimitiveDateTime::new(self, time)
    }
}

impl Incomplete for InDate {
    type Complete = Date;

    fn from_complete(complete: Self::Complete) -> Self {
        let y = Some(complete.year());
        let m = Some(complete.month());
        let d = Some(complete.day());
        Self::from_calendar_date_unchecked(y, m, d)
    }

    fn into_complete(self) -> Result<Self::Complete, ComponentRange> {
        todo!()
    }

    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, ComponentRange> {
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
    type Error = ComponentRange;

    fn try_from(date: InDate) -> Result<Self, Self::Error> {
        date.into_complete()
    }
}
