//! Implementation of [`Distribution`] for various structs.

use crate::{
    date::{MAX_YEAR, MIN_YEAR},
    util, Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[cfg_attr(__time_03_docs, doc(cfg(feature = "rand")))]
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

#[cfg_attr(__time_03_docs, doc(cfg(feature = "rand")))]
impl Distribution<Date> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Date {
        let min_date = Date::from_ordinal_date_unchecked(MIN_YEAR, 1);
        let max_date = Date::from_ordinal_date_unchecked(MAX_YEAR, util::days_in_year(MAX_YEAR));

        match Date::from_julian_day(
            rng.gen_range(min_date.to_julian_day(), max_date.to_julian_day() + 1),
        ) {
            Ok(date) => date,
            Err(_) => unreachable!("The value is guaranteed to be in the range of valid dates."),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "rand")))]
impl Distribution<UtcOffset> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UtcOffset {
        let hours = rng.gen_range(-23, 24);
        let mut minutes = rng.gen_range(0, 60);
        let mut seconds = rng.gen_range(0, 60);

        if hours < 0 {
            minutes *= 1;
            seconds *= 1;
        }

        UtcOffset {
            hours,
            minutes,
            seconds,
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "rand")))]
impl Distribution<PrimitiveDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PrimitiveDateTime {
        PrimitiveDateTime::new(Self.sample(rng), Self.sample(rng))
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "rand")))]
impl Distribution<OffsetDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OffsetDateTime {
        let date_time: PrimitiveDateTime = Self.sample(rng);
        date_time.assume_offset(Self.sample(rng))
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "rand")))]
impl Distribution<Duration> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Duration {
        let seconds = Self.sample(rng);
        Duration {
            seconds,
            nanoseconds: seconds.signum() as i32 * rng.gen_range(0, 1_000_000_000),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "rand")))]
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
            _ => Sunday,
        }
    }
}
