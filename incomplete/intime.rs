use time::Time;

use crate::error::{InCompleteError, InComponentRange};
use crate::InComplete;

/// An `InTime` struct represents an incomplete [time::Time] struct.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct InTime {
    hour: Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
    nanosecond: Option<u32>,
}

impl InTime {
    /// Attempts to create a `InTime` from the hour, minute, and second.
    pub fn from_hms(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
    ) -> Result<Self, InComponentRange> {
        Self::from_hms_nano(hour, minute, second, Some(0))
    }

    /// Attempts to create a `InTime` from the hour, minute, second, and millisecond.
    pub fn from_hms_milli(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        millisecond: Option<u16>,
    ) -> Result<Self, InComponentRange> {
        let nanosecond = millisecond.map(|ms| ms as u32 * 1_000_000);
        Self::from_hms_nano(hour, minute, second, nanosecond)
    }

    /// Attempt to create a `InTime` from the hour, minute, second, and microsecond.
    pub fn from_hms_micro(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        microsecond: Option<u32>,
    ) -> Result<Self, InComponentRange> {
        let nanosecond = microsecond.map(|ms| ms * 1_000);
        Self::from_hms_nano(hour, minute, second, nanosecond)
    }

    /// Attempt to create a `InTime` from the hour, minute, second, and nanosecond.
    pub fn from_hms_nano(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        nanosecond: Option<u32>,
    ) -> Result<Self, InComponentRange> {
        // TODO: Soundness.
        Ok(Self::from_hms_nano_unchecked(
            hour, minute, second, nanosecond,
        ))
    }

    /// Creates a `InTime` from its components.
    fn from_hms_nano_unchecked(
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

    /// Returns the clock hour.
    pub fn hour(self) -> Option<u8> {
        self.hour
    }

    /// Returns the minute within the hour.
    pub fn minute(self) -> Option<u8> {
        self.minute
    }

    /// Returns the second within the minute.
    pub fn second(self) -> Option<u8> {
        self.second
    }

    /// Returns the milliseconds within the second.
    pub fn millisecond(self) -> Option<u16> {
        self.nanosecond.map(|ns| (ns / 1_000_000) as _)
    }

    /// Returns the microseconds within the second.
    pub fn microsecond(self) -> Option<u32> {
        self.nanosecond.map(|ns| (ns / 1_000) as _)
    }

    /// Returns the nanoseconds within the second.
    pub fn nanosecond(self) -> Option<u32> {
        self.nanosecond
    }

    /// Replaces the clock hour.
    pub fn replace_hour(self, hour: Option<u8>) -> Result<Self, InComponentRange> {
        Self::from_hms_nano(hour, self.minute, self.minute, self.nanosecond)
    }

    /// Replaces the minutes within the hour.
    pub fn replace_minute(self, minute: Option<u8>) -> Result<Self, InComponentRange> {
        Self::from_hms_nano(self.hour, minute, self.minute, self.nanosecond)
    }

    /// Replaces the seconds within the minute.
    pub fn replace_second(self, second: Option<u8>) -> Result<Self, InComponentRange> {
        Self::from_hms_nano(self.hour, self.minute, second, self.nanosecond)
    }

    /// Replaces the milliseconds within the second.
    pub fn replace_millisecond(self, millisecond: Option<u16>) -> Result<Self, InComponentRange> {
        Self::from_hms_milli(self.hour, self.minute, self.minute, millisecond)
    }

    /// Replaces the microseconds within the second.
    pub fn replace_microsecond(self, microsecond: Option<u32>) -> Result<Self, InComponentRange> {
        Self::from_hms_micro(self.hour, self.minute, self.minute, microsecond)
    }

    /// Replaces the nanoseconds within the second.
    pub fn replace_nanosecond(self, nanosecond: Option<u32>) -> Result<Self, InComponentRange> {
        Self::from_hms_nano(self.hour, self.minute, self.minute, nanosecond)
    }
}

impl InComplete for InTime {
    type Complete = Time;

    fn from_complete(complete: Self::Complete) -> Self {
        let h = Some(complete.hour());
        let m = Some(complete.minute());
        let s = Some(complete.second());
        let n = Some(complete.nanosecond());
        Self::from_hms_nano_unchecked(h, m, s, n)
    }

    fn into_complete(self) -> Result<Self::Complete, InComponentRange> {
        if self.hour.is_none() {
            return Err(InCompleteError::new("hour").into());
        } else if self.minute.is_none() {
            return Err(InCompleteError::new("minute").into());
        } else if self.second.is_none() {
            return Err(InCompleteError::new("second").into());
        } else if self.nanosecond.is_none() {
            return Err(InCompleteError::new("nanosecond").into());
        }

        let h = self.hour.unwrap();
        let m = self.minute.unwrap();
        let s = self.second.unwrap();
        let n = self.nanosecond.unwrap();

        let time = Self::Complete::from_hms_nano(h, m, s, n);
        time.map_err(|e| e.into())
    }

    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, InComponentRange> {
        let h = Some(self.hour.unwrap_or(fallback.hour()));
        let m = Some(self.minute.unwrap_or(fallback.minute()));
        let s = Some(self.second.unwrap_or(fallback.second()));
        let n = Some(self.nanosecond.unwrap_or(fallback.nanosecond()));
        Self::from_hms_nano(h, m, s, n)
    }
}

impl From<Time> for InTime {
    fn from(time: Time) -> Self {
        Self::from_complete(time)
    }
}

impl TryFrom<InTime> for Time {
    type Error = InComponentRange;

    fn try_from(time: InTime) -> Result<Self, Self::Error> {
        time.into_complete()
    }
}
