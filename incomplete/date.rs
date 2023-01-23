use time::Month;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct IncompleteDate {
    year: Option<i32>,
    month: Option<Month>,
    day: Option<u8>,
}
