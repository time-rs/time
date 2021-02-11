//! Implementation of [`Distribution`] for various structs.

use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::{hack, Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

impl Distribution<Time> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Time {
        Time {
            hour: rng.gen_range(0..24),
            minute: rng.gen_range(0..60),
            second: rng.gen_range(0..60),
            nanosecond: rng.gen_range(0..1_000_000_000),
            padding: hack::Padding::Optimize,
        }
    }
}

impl Distribution<Date> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Date {
        match Date::from_julian_day(
            rng.gen_range(Date::MIN.to_julian_day()..=Date::MAX.to_julian_day()),
        ) {
            Ok(date) => date,
            Err(_) => unreachable!("The value is guaranteed to be in the range of valid dates."),
        }
    }
}

impl Distribution<UtcOffset> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UtcOffset {
        let hours = rng.gen_range(-23..24);
        let mut minutes = rng.gen_range(0..60);
        let mut seconds = rng.gen_range(0..60);

        if hours < 0 {
            minutes *= -1;
            seconds *= -1;
        }

        UtcOffset {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Distribution<PrimitiveDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PrimitiveDateTime {
        PrimitiveDateTime::new(Self.sample(rng), Self.sample(rng))
    }
}

impl Distribution<OffsetDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OffsetDateTime {
        let date_time: PrimitiveDateTime = Self.sample(rng);
        date_time.assume_offset(Self.sample(rng))
    }
}

impl Distribution<Duration> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Duration {
        let seconds = Self.sample(rng);
        Duration {
            seconds,
            nanoseconds: seconds.signum() as i32 * rng.gen_range(0..1_000_000_000),
        }
    }
}

impl Distribution<Weekday> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Weekday {
        use Weekday::*;

        match rng.gen_range(0..7) {
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
