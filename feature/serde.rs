use std::fmt::{Formatter, Result as FmtResult};

use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::CalendarDuration;

impl Serialize for CalendarDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (self.whole_months()).serialize(serializer)
    }
}

struct CalendarDurationVisitor;

impl<'de> Visitor<'de> for CalendarDurationVisitor {
    type Value = CalendarDuration;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a `MonthDuration`")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let months = seq.next_element()?;
        let months = months.ok_or_else(|| Error::custom("expected months"))?;
        Ok(CalendarDuration::months(months))
    }
}

impl<'de> Deserialize<'de> for CalendarDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(1, CalendarDurationVisitor)
    }
}
