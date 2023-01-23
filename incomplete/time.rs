use time::error::ComponentRange;
use time::Time;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct IncompleteTime {
    hour: Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
    nanoseconds: Option<u32>,
}

impl IncompleteTime {
    pub fn into_time(self, complete: Time) -> Result<Time, ComponentRange> {
        let h = self.hour.unwrap_or(complete.hour());
        let m = self.minute.unwrap_or(complete.minute());
        let s = self.second.unwrap_or(complete.second());
        let ns = self.nanoseconds.unwrap_or(complete.nanosecond());
        Time::from_hms_nano(h, m, s, ns)
    }
}

impl IncompleteTime {
    pub fn from_hms(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
    ) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn from_hms_milli(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        millisecond: Option<u16>,
    ) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn from_hms_micro(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        microsecond: Option<u32>,
    ) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn from_hms_nano(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        nanosecond: Option<u32>,
    ) -> Result<Self, ComponentRange> {
        todo!()
    }
}

impl IncompleteTime {
    pub fn as_hms(self) -> (Option<u8>, Option<u8>, Option<u8>) {
        (self.hour, self.minute, self.second)
    }

    pub fn as_hms_milli(self) -> (Option<u8>, Option<u8>, Option<u8>, Option<u16>) {
        (self.hour, self.minute, self.second, self.millisecond())
    }

    pub fn as_hms_micro(self) -> (Option<u8>, Option<u8>, Option<u8>, Option<u32>) {
        (self.hour, self.minute, self.second, self.microsecond())
    }

    pub fn as_hms_nano(self) -> (Option<u8>, Option<u8>, Option<u8>, Option<u32>) {
        (self.hour, self.minute, self.second, self.nanoseconds)
    }
}

impl IncompleteTime {
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
        self.nanoseconds.map(|ns| (ns / 1_000_000) as _)
    }

    pub fn microsecond(self) -> Option<u32> {
        self.nanoseconds.map(|ns| (ns / 1_000) as _)
    }

    pub fn nanosecond(self) -> Option<u32> {
        self.nanoseconds
    }
}

impl IncompleteTime {
    pub fn replace_hour(self, hour: Option<u8>) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn replace_minute(self, minute: Option<u8>) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn replace_second(self, second: Option<u8>) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn replace_millisecond(self, millisecond: Option<u16>) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn replace_microsecond(self, microsecond: Option<u32>) -> Result<Self, ComponentRange> {
        todo!()
    }

    pub fn replace_nanosecond(self, nanosecond: Option<u32>) -> Result<Self, ComponentRange> {
        todo!()
    }
}
