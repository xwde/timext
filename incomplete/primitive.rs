use crate::{IncompleteDate, IncompleteTime};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct IncompletePrimitiveDateTime {
    date: IncompleteDate,
    time: IncompleteTime,
}

impl IncompletePrimitiveDateTime {
    pub fn new(date: IncompleteDate, time: IncompleteTime) -> Self {
        Self { date, time }
    }
}
