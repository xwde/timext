use time::UtcOffset;

use crate::IncompletePrimitiveDateTime;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct IncompleteOffsetDateTime {
    local_datetime: IncompletePrimitiveDateTime,
    offset: Option<UtcOffset>,
}
