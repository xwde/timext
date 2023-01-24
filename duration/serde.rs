use std::fmt::Formatter;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::MonthDuration;

impl Serialize for MonthDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.whole_months())
    }
}

struct MonthDurationVisitor;

impl<'de> Visitor<'de> for MonthDurationVisitor {
    type Value = MonthDuration;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a `MonthDuration`")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(MonthDuration::months(v))
    }
}

impl<'de> Deserialize<'de> for MonthDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(MonthDurationVisitor)
    }
}
