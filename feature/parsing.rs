use time::parsing::Parsed;

use crate::error::{InParse, InTryFromParsed};
use crate::{InDate, InOffsetDateTime, InPrimitiveDateTime, InTime};

pub mod error {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

    #[derive(Debug)]
    pub struct InParse {}

    impl Display for InParse {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            todo!()
        }
    }

    // impl From<Parse> for InParse
    // impl From<InComponentRange> for InParse
    impl From<InTryFromParsed> for InParse {
        fn from(value: InTryFromParsed) -> Self {
            todo!()
        }
    }

    impl Error for InParse {}

    #[derive(Debug)]
    pub struct InTryFromParsed {}

    impl Display for InTryFromParsed {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            todo!()
        }
    }

    impl Error for InTryFromParsed {}
}

pub mod timestamp {
    use time::parsing::Parsed;

    use crate::error::InParse;
    use crate::InParsable;

    pub struct W3C {}

    impl W3C {}

    impl InParsable for W3C {
        fn parse_into<'a>(
            &self,
            input: &'a [u8],
            parsed: &mut Parsed,
        ) -> Result<&'a [u8], InParse> {
            todo!()
        }
    }
}

pub trait InParsable: Sized {
    fn parse_into<'a>(&self, input: &'a [u8], parsed: &mut Parsed) -> Result<&'a [u8], InParse>;

    fn parse(&self, input: &[u8]) -> Result<Parsed, InParse> {
        let mut parsed = Parsed::new();
        if self.parse_into(input, &mut parsed)?.is_empty() {
            Ok(parsed)
        } else {
            // Err(Parse::UnexpectedTrailingCharacters)
            todo!()
        }
    }

    fn parse_date(&self, input: &[u8]) -> Result<InDate, InParse> {
        Ok(self.parse(input)?.try_into()?)
    }

    fn parse_time(&self, input: &[u8]) -> Result<InTime, InParse> {
        Ok(self.parse(input)?.try_into()?)
    }

    fn parse_date_time(&self, input: &[u8]) -> Result<InPrimitiveDateTime, InParse> {
        Ok(self.parse(input)?.try_into()?)
    }

    fn parse_offset_date_time(&self, input: &[u8]) -> Result<InOffsetDateTime, InParse> {
        Ok(self.parse(input)?.try_into()?)
    }
}

impl TryFrom<Parsed> for InDate {
    type Error = InTryFromParsed;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Parsed> for InTime {
    type Error = InTryFromParsed;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Parsed> for InPrimitiveDateTime {
    type Error = InTryFromParsed;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Parsed> for InOffsetDateTime {
    type Error = InTryFromParsed;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        todo!()
    }
}
