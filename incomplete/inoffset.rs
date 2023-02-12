use time::error::ComponentRange;
use time::{Month, OffsetDateTime, UtcOffset, Weekday};

use crate::incomplete::Incomplete;
use crate::{InDate, InPrimitiveDateTime, InTime};

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

impl Incomplete for InOffsetDateTime {
    type Complete = OffsetDateTime;

    fn from_complete(complete: Self::Complete) -> Self {
        todo!()
    }

    fn into_complete(self) -> Result<Self::Complete, ComponentRange> {
        todo!()
    }

    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, ComponentRange> {
        todo!()
    }
}

impl From<OffsetDateTime> for InOffsetDateTime {
    fn from(datetime: OffsetDateTime) -> Self {
        Self::from_complete(datetime)
    }
}

impl TryFrom<InOffsetDateTime> for OffsetDateTime {
    type Error = ComponentRange;

    fn try_from(datetime: InOffsetDateTime) -> Result<Self, Self::Error> {
        datetime.into_complete()
    }
}
