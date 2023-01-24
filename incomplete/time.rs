use time::error::ComponentRange;
use time::Time;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct IncompleteTime {
    hour: Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
    nanosecond: Option<u32>,
}

impl IncompleteTime {
    pub fn from_hms(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
    ) -> Result<Self, ComponentRange> {
        Self::from_hms_nano(hour, minute, second, None)
    }

    pub fn from_hms_milli(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        millisecond: Option<u16>,
    ) -> Result<Self, ComponentRange> {
        let nanosecond = millisecond.map(|ms| ms as u32 * 1_000_000);
        Self::from_hms_nano(hour, minute, second, nanosecond)
    }

    pub fn from_hms_micro(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        microsecond: Option<u32>,
    ) -> Result<Self, ComponentRange> {
        let nanosecond = microsecond.map(|ms| ms * 1_000);
        Self::from_hms_nano(hour, minute, second, nanosecond)
    }

    pub fn from_hms_nano(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        nanosecond: Option<u32>,
    ) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn from_hms_nano_unchecked(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        nanosecond: Option<u32>,
    ) -> Self {
        Self {
            hour,
            minute,
            second,
            nanosecond,
        }
    }

    pub fn from_complete(time: Time) -> Self {
        let h = Some(time.hour());
        let m = Some(time.minute());
        let s = Some(time.second());
        let n = Some(time.nanosecond());
        Self::from_hms_nano_unchecked(h, m, s, n)
    }

    pub fn into_complete(self) -> Result<Time, ComponentRange> {
        todo!()
    }

    pub fn with_fallback(self, fallback: Time) -> Result<Self, ComponentRange> {
        let h = Some(self.hour.unwrap_or(fallback.hour()));
        let m = Some(self.minute.unwrap_or(fallback.minute()));
        let s = Some(self.second.unwrap_or(fallback.second()));
        let n = Some(self.nanosecond.unwrap_or(fallback.nanosecond()));
        Ok(Self::from_hms_nano_unchecked(h, m, s, n))
    }

    pub fn hour(self) -> Option<u8> {
        self.hour
    }

    pub fn minute(self) -> Option<u8> {
        self.minute
    }

    pub fn second(self) -> Option<u8> {
        self.second
    }

    pub fn millisecond(self) -> Option<u16> {
        self.nanosecond.map(|ns| (ns / 1_000_000) as _)
    }

    pub fn microsecond(self) -> Option<u32> {
        self.nanosecond.map(|ns| (ns / 1_000) as _)
    }

    pub fn nanosecond(self) -> Option<u32> {
        self.nanosecond
    }

    pub fn replace_hour(self, hour: Option<u8>) -> Result<Self, ComponentRange> {
        Self::from_hms_nano(hour, self.minute, self.minute, self.nanosecond)
    }

    pub fn replace_minute(self, minute: Option<u8>) -> Result<Self, ComponentRange> {
        Self::from_hms_nano(self.hour, minute, self.minute, self.nanosecond)
    }

    pub fn replace_second(self, second: Option<u8>) -> Result<Self, ComponentRange> {
        Self::from_hms_nano(self.hour, self.minute, second, self.nanosecond)
    }

    pub fn replace_millisecond(self, millisecond: Option<u16>) -> Result<Self, ComponentRange> {
        let nanosecond = millisecond.map(|ms| ms as u32 * 1_000_000);
        Self::from_hms_nano(self.hour, self.minute, self.minute, nanosecond)
    }

    pub fn replace_microsecond(self, microsecond: Option<u32>) -> Result<Self, ComponentRange> {
        let nanosecond = microsecond.map(|ms| ms * 1_000);
        Self::from_hms_nano(self.hour, self.minute, self.minute, nanosecond)
    }

    pub fn replace_nanosecond(self, nanosecond: Option<u32>) -> Result<Self, ComponentRange> {
        Self::from_hms_nano(self.hour, self.minute, self.minute, nanosecond)
    }
}

impl From<Time> for IncompleteTime {
    fn from(time: Time) -> Self {
        Self::from_complete(time)
    }
}

impl TryFrom<IncompleteTime> for Time {
    type Error = ComponentRange;

    fn try_from(time: IncompleteTime) -> Result<Self, Self::Error> {
        time.into_complete()
    }
}
