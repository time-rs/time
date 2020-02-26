//! Implementation of `Distribution` for various structs.

use crate::{
    date::{MAX_YEAR, MIN_YEAR},
    days_in_year,
    internal_prelude::*,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<Time> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Time {
        Time {
            hour: rng.gen_range(0, 24),
            minute: rng.gen_range(0, 60),
            second: rng.gen_range(0, 60),
            nanosecond: rng.gen_range(0, 1_000_000_000),
        }
    }
}

impl Distribution<Date> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Date {
        let year = rng.gen_range(MIN_YEAR, MAX_YEAR + 1);
        Date {
            year,
            ordinal: rng.gen_range(1, days_in_year(year) + 1),
        }
    }
}

impl Distribution<UtcOffset> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UtcOffset {
        UtcOffset {
            seconds: rng.gen_range(-86_399, 86_400),
        }
    }
}

impl Distribution<PrimitiveDateTime> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PrimitiveDateTime {
        PrimitiveDateTime {
            date: Standard.sample(rng),
            time: Standard.sample(rng),
        }
    }
}

impl Distribution<OffsetDateTime> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OffsetDateTime {
        OffsetDateTime {
            utc_datetime: Standard.sample(rng),
            offset: Standard.sample(rng),
        }
    }
}

impl Distribution<Duration> for Standard {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Duration {
        let seconds = Standard.sample(rng);
        Duration {
            seconds,
            nanoseconds: seconds.signum() as i32 * rng.gen_range(0, 1_000_000_000),
        }
    }
}

impl Distribution<Weekday> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Weekday {
        use Weekday::*;

        match rng.gen_range(0, 7) {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            6 => Sunday,
            _ => unreachable!("values are 0 to 6 inclusive"),
        }
    }
}
