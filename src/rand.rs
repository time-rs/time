//! Implementation of `Distribution` for various structs.

use crate::{
    date::{MAX_YEAR, MIN_YEAR},
    util, Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<Time> for Standard {
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
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Date {
        let min_date = Date::from_yo_unchecked(MIN_YEAR, 1);
        let max_date = Date::from_yo_unchecked(MAX_YEAR, util::days_in_year(MAX_YEAR));

        Date::from_julian_day(rng.gen_range(min_date.julian_day(), max_date.julian_day() + 1))
    }
}

impl Distribution<UtcOffset> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UtcOffset {
        UtcOffset {
            seconds: rng.gen_range(-86_399, 86_400),
        }
    }
}

impl Distribution<PrimitiveDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PrimitiveDateTime {
        PrimitiveDateTime::new(Standard.sample(rng), Standard.sample(rng))
    }
}

impl Distribution<OffsetDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OffsetDateTime {
        OffsetDateTime::new_assuming_offset(Standard.sample(rng), Standard.sample(rng))
    }
}

impl Distribution<Duration> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Duration {
        let seconds = Standard.sample(rng);
        Duration::new(
            seconds,
            seconds.signum() as i32 * rng.gen_range(0, 1_000_000_000),
        )
    }
}

impl Distribution<Weekday> for Standard {
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
