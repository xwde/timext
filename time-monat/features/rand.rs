use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::duration::MonthDuration;

impl Distribution<MonthDuration> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MonthDuration {
        let min = MonthDuration::MIN.whole_months();
        let max = MonthDuration::MAX.whole_months();
        MonthDuration::months(rng.gen_range(min..=max))
    }
}
