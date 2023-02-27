use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::CalendarDuration;

impl Distribution<CalendarDuration> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CalendarDuration {
        let min = CalendarDuration::MIN.whole_months();
        let max = CalendarDuration::MAX.whole_months();
        CalendarDuration::months(rng.gen_range(min..=max))
    }
}
