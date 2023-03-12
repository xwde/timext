use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::CalendarDuration;
use crate::{InComplete, InDate, InOffsetDateTime, InPrimitiveDateTime, InTime};

impl Distribution<CalendarDuration> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CalendarDuration {
        let min = CalendarDuration::MIN.whole_months();
        let max = CalendarDuration::MAX.whole_months();
        CalendarDuration::months(rng.gen_range(min..=max))
    }
}

impl Distribution<InDate> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> InDate {
        InDate::from_complete(Self.sample(rng))
    }
}

impl Distribution<InTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> InTime {
        InTime::from_complete(Self.sample(rng))
    }
}

impl Distribution<InPrimitiveDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> InPrimitiveDateTime {
        InPrimitiveDateTime::from_complete(Self.sample(rng))
    }
}

impl Distribution<InOffsetDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> InOffsetDateTime {
        InOffsetDateTime::from_complete(Self.sample(rng))
    }
}
