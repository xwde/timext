use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::duration::MonthDuration;

// TODO Impl feature Serde
// see https://github.com/martsokha/timext/issues/4
impl Serialize for MonthDuration {
    fn serialize<S>(&self, serializer: S) -> Result<serde::ser::Ok, dyn serde::ser::Error>
        where
            S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for MonthDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, dyn serde::de::Error>
        where
            D: Deserializer<'de>,
    {
        todo!()
    }

    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), dyn serde::de::Error>
        where
            D: Deserializer<'de>,
    {
        todo!()
    }
}
