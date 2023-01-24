use time::error::ComponentRange;
use time::{Date, Month, Time, Weekday};

use crate::{IncompletePrimitiveDateTime, IncompleteTime};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct IncompleteDate {
    year: Option<i32>,
    month: Option<Month>,
    day: Option<u8>,
}

impl IncompleteDate {
    pub fn from_calendar_date(
        year: Option<i32>,
        month: Option<Month>,
        day: Option<u8>,
    ) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn from_calendar_date_unchecked(
        year: Option<i32>,
        month: Option<Month>,
        day: Option<u8>,
    ) -> Self {
        Self { year, month, day }
    }

    pub fn from_complete(date: Date) -> Self {
        let y = Some(date.year());
        let m = Some(date.month());
        let d = Some(date.day());
        Self::from_calendar_date_unchecked(y, m, d)
    }

    pub fn into_complete(self) -> Result<Date, ComponentRange> {
        todo!()
    }

    pub fn with_time(self, time: Time) -> IncompletePrimitiveDateTime {
        self.with_incomplete_time(time.into())
    }

    pub fn with_incomplete_time(self, time: IncompleteTime) -> IncompletePrimitiveDateTime {
        IncompletePrimitiveDateTime::new(self, time)
    }

    pub fn with_fallback(self, fallback: Time) -> Result<Self, ComponentRange> {
        todo!()
    }

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

impl From<Date> for IncompleteDate {
    fn from(date: Date) -> Self {
        Self::from_complete(date)
    }
}

impl TryFrom<IncompleteDate> for Date {
    type Error = ComponentRange;

    fn try_from(date: IncompleteDate) -> Result<Self, Self::Error> {
        date.into_complete()
    }
}
