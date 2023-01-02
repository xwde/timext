use rand::distributions::{DistIter, DistMap, Distribution, Standard};
use rand::Rng;

use crate::duration::MonthDuration;

// TODO Impl feature Rand
// see https://github.com/martsokha/timext/issues/3
impl Distribution<MonthDuration> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MonthDuration {
        todo!()
    }

    fn sample_iter<R>(self, rng: R) -> DistIter<Self, R, MonthDuration>
        where
            R: Rng,
            Self: Sized,
    {
        todo!()
    }

    fn map<F, S>(self, func: F) -> DistMap<Self, F, MonthDuration, S>
        where
            F: Fn(MonthDuration) -> S,
            Self: Sized,
    {
        todo!()
    }
}
