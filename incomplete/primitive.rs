use crate::{IncompleteDate, IncompleteTime};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct IncompletePrimitiveDateTime {
    date: IncompleteDate,
    time: IncompleteTime,
}
