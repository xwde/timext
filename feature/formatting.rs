use std::io::Write;
use time::UtcOffset;

use crate::error::InFormat;
use crate::{InDate, InTime};

pub mod error {
    use std::error::Error;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    #[derive(Debug)]
    pub struct InFormat {}

    impl Display for InFormat {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            todo!()
        }
    }

    impl Error for InFormat {}
}

pub trait InFormattable: Sized {
    fn format_into(
        &self,
        output: &mut impl Write,
        date: Option<InDate>,
        time: Option<InTime>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, InFormat>;

    fn format(
        &self,
        date: Option<InDate>,
        time: Option<InTime>,
        offset: Option<UtcOffset>,
    ) -> Result<String, InFormat> {
        let mut buf = Vec::new();
        self.format_into(&mut buf, date, time, offset)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }
}
