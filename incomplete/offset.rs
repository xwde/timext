use time::UtcOffset;

use crate::IncompletePrimitiveDateTime;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct IncompleteOffsetDateTime {
    local_datetime: IncompletePrimitiveDateTime,
    offset: Option<UtcOffset>,
}

impl IncompleteOffsetDateTime {
    pub fn new(datetime: IncompletePrimitiveDateTime, offset: Option<UtcOffset>) -> Self {
        Self {
            local_datetime: datetime,
            offset,
        }
    }
}
