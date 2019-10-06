use crate::Duration;
use core::ops::{Add, AddAssign, Sub, SubAssign};

/// The number of nanoseconds in one day.
const NANOS_PER_DAY: u64 = 24 * 60 * 60 * 1_000_000_000;

/// The clock time within a given date. Nanosecond precision.
///
/// All minutes are assumed to have exactly 60 seconds; no attempt is made to
/// handle leap seconds (either positive or negative).
///
/// As order is dependent on context (is noon before or after midnight?), this
/// type does not implement `PartialOrd` or `Ord`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time {
    #[allow(clippy::missing_docs_in_private_items)]
    hours: u8,
    #[allow(clippy::missing_docs_in_private_items)]
    minutes: u8,
    #[allow(clippy::missing_docs_in_private_items)]
    seconds: u8,
    #[allow(clippy::missing_docs_in_private_items)]
    nanoseconds: u32,
}

impl Time {
    /// Create a `Time` from the hour, minute, and second.
    ///
    /// ```rust
    /// # use time::Time;
    /// let time = Time::from_hms(1, 2, 3);
    /// assert_eq!(time.hours(), 1);
    /// assert_eq!(time.minutes(), 2);
    /// assert_eq!(time.seconds(), 3);
    /// assert_eq!(time.nanoseconds(), 0);
    /// ```
    ///
    /// Panics if any component is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms(24, 0, 0); // 24 isn't a valid hour.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms(0, 60, 0); // 60 isn't a valid minute.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms(0, 0, 60); // 60 isn't a valid second.
    /// ```
    pub fn from_hms(hours: u8, minutes: u8, seconds: u8) -> Self {
        assert_value_in_range!(hours in 0 => exclusive 24);
        assert_value_in_range!(minutes in 0 => exclusive 60);
        assert_value_in_range!(seconds in 0 => exclusive 60);
        Self {
            hours,
            minutes,
            seconds,
            nanoseconds: 0,
        }
    }

    /// Create a `Time` from the hour, minute, second, and millisecond.
    ///
    /// ```rust
    /// # use time::Time;
    /// let time = Time::from_hms_milli(1, 2, 3, 4);
    /// assert_eq!(time.hours(), 1);
    /// assert_eq!(time.minutes(), 2);
    /// assert_eq!(time.seconds(), 3);
    /// assert_eq!(time.milliseconds(), 4);
    /// assert_eq!(time.nanoseconds(), 4_000_000);
    /// ```
    ///
    /// Panics if any component is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_milli(24, 0, 0, 0); // 24 isn't a valid hour.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_milli(0, 60, 0, 0); // 60 isn't a valid minute.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_milli(0, 0, 60, 0); // 60 isn't a valid second.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_milli(0, 0, 0, 1_000); // 1_000 isn't a valid millisecond.
    /// ```
    pub fn from_hms_milli(hours: u8, minutes: u8, seconds: u8, milliseconds: u16) -> Self {
        assert_value_in_range!(hours in 0 => exclusive 24);
        assert_value_in_range!(minutes in 0 => exclusive 60);
        assert_value_in_range!(seconds in 0 => exclusive 60);
        assert_value_in_range!(milliseconds in 0 => exclusive 1_000);
        Self {
            hours,
            minutes,
            seconds,
            nanoseconds: milliseconds as u32 * 1_000_000,
        }
    }

    /// Create a `Time` from the hour, minute, second, and microsecond.
    ///
    /// ```rust
    /// # use time::Time;
    /// let time = Time::from_hms_micro(1, 2, 3, 4);
    /// assert_eq!(time.hours(), 1);
    /// assert_eq!(time.minutes(), 2);
    /// assert_eq!(time.seconds(), 3);
    /// assert_eq!(time.microseconds(), 4);
    /// assert_eq!(time.nanoseconds(), 4_000);
    /// ```
    ///
    /// Panics if any component is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_micro(24, 0, 0, 0); // 24 isn't a valid hour.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_micro(0, 60, 0, 0); // 60 isn't a valid minute.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_micro(0, 0, 60, 0); // 60 isn't a valid second.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_micro(0, 0, 0, 1_000_000); // 1_000_000 isn't a valid microsecond.
    /// ```
    pub fn from_hms_micro(hours: u8, minutes: u8, seconds: u8, microseconds: u32) -> Self {
        assert_value_in_range!(hours in 0 => exclusive 24);
        assert_value_in_range!(minutes in 0 => exclusive 60);
        assert_value_in_range!(seconds in 0 => exclusive 60);
        assert_value_in_range!(microseconds in 0 => exclusive 1_000_000);
        Self {
            hours,
            minutes,
            seconds,
            nanoseconds: microseconds * 1_000,
        }
    }

    /// Create a `Time` from the hour, minute, second, and nanosecond.
    ///
    /// ```rust
    /// # use time::Time;
    /// let time = Time::from_hms_nano(1, 2, 3, 4);
    /// assert_eq!(time.hours(), 1);
    /// assert_eq!(time.minutes(), 2);
    /// assert_eq!(time.seconds(), 3);
    /// assert_eq!(time.nanoseconds(), 4);
    /// ```
    ///
    /// Panics if any component is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_nano(24, 0, 0, 0); // 24 isn't a valid hour.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_nano(0, 60, 0, 0); // 60 isn't a valid minute.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_nano(0, 0, 60, 0); // 60 isn't a valid second.
    /// ```
    ///
    /// ```rust,should_panic
    /// # use time::Time;
    /// Time::from_hms_nano(0, 0, 0, 1_000_000_000); // 1_000_000_000 isn't a valid nanosecond.
    /// ```
    pub fn from_hms_nano(hours: u8, minutes: u8, seconds: u8, nanoseconds: u32) -> Self {
        assert_value_in_range!(hours in 0 => exclusive 24);
        assert_value_in_range!(minutes in 0 => exclusive 60);
        assert_value_in_range!(seconds in 0 => exclusive 60);
        assert_value_in_range!(nanoseconds in 0 => exclusive 1_000_000_000);
        Self {
            hours,
            minutes,
            seconds,
            nanoseconds,
        }
    }

    /// Returns the clock hour.
    ///
    /// The returned value will always be in the range `0..=23`.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert_eq!(Time::from_hms(0, 0, 0).hours(), 0);
    /// assert_eq!(Time::from_hms(23, 59, 59).hours(), 23);
    /// ```
    pub const fn hours(self) -> u8 {
        self.hours
    }

    /// Returns the minute within the hour.
    ///
    /// The returned value will always be in the range `0..=60`.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert_eq!(Time::from_hms(0, 0, 0).minutes(), 0);
    /// assert_eq!(Time::from_hms(23, 59, 59).minutes(), 59);
    /// ```
    pub const fn minutes(self) -> u8 {
        self.minutes
    }

    /// Returns the second within the minute.
    ///
    /// The returned value will always be in the range `0..=60`.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert_eq!(Time::from_hms(0, 0, 0).seconds(), 0);
    /// assert_eq!(Time::from_hms(23, 59, 59).seconds(), 59);
    /// ```
    pub const fn seconds(self) -> u8 {
        self.seconds
    }

    /// Return the milliseconds within the second.
    ///
    /// The returned value will always be in the range `0..=1_000`.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert_eq!(Time::from_hms_milli(0, 0, 0, 0).milliseconds(), 0);
    /// assert_eq!(Time::from_hms_milli(23, 59, 59, 999).milliseconds(), 999);
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub const fn milliseconds(self) -> u16 {
        (self.nanoseconds / 1_000_000) as u16
    }

    /// Return the microseconds within the second.
    ///
    /// The returned value will always be in the range `0..=1_000_000`.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert_eq!(Time::from_hms_micro(0, 0, 0, 0).microseconds(), 0);
    /// assert_eq!(Time::from_hms_micro(23, 59, 59, 999_999).microseconds(), 999_999);
    /// ```
    pub const fn microseconds(self) -> u32 {
        self.nanoseconds / 1_000
    }

    /// Return the nanoseconds within the second.
    ///
    /// The returned value will always be in the range `0..=1_000_000_000`.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert_eq!(Time::from_hms_nano(0, 0, 0, 0).nanoseconds(), 0);
    /// assert_eq!(Time::from_hms_nano(23, 59, 59, 999_999_999).nanoseconds(), 999_999_999);
    /// ```
    pub const fn nanoseconds(self) -> u32 {
        self.nanoseconds
    }

    /// Return the number of nanoseconds since midnight.
    const fn nanoseconds_since_midnight(self) -> u64 {
        self.hours as u64 * 60 * 60 * 1_000_000_000
            + self.minutes as u64 * 60 * 1_000_000_000
            + self.seconds as u64 * 1_000_000_000
            + self.nanoseconds as u64
    }

    /// Create a `Time` from the number of nanoseconds since midnight.
    const fn from_nanoseconds_since_midnight(mut nanoseconds: u64) -> Self {
        #![allow(clippy::cast_possible_truncation)]

        nanoseconds %= 86_400 * 1_000_000_000;

        Self {
            hours: (nanoseconds / 1_000_000_000 / 60 / 60) as u8,
            minutes: (nanoseconds / 1_000_000_000 / 60 % 60) as u8,
            seconds: (nanoseconds / 1_000_000_000 % 60) as u8,
            nanoseconds: (nanoseconds % 1_000_000_000) as u32,
        }
    }
}

impl Add<Duration> for Time {
    type Output = Self;

    /// Add the sub-day time of the `Duration` to the `Time`. Wraps on overflow
    /// and underflow.
    ///
    /// ```rust
    /// # use time::{Duration, Time};
    /// assert_eq!(Time::from_hms(12, 0, 0) + Duration::hours(2), Time::from_hms(14, 0, 0));
    /// assert_eq!(Time::from_hms(0, 0, 1) + Duration::seconds(-2), Time::from_hms(23, 59, 59));
    /// ```
    fn add(self, duration: Duration) -> Self::Output {
        #[allow(clippy::cast_possible_truncation)]
        Self::from_nanoseconds_since_midnight(
            self.nanoseconds_since_midnight()
                + duration
                    .whole_nanoseconds()
                    .rem_euclid(NANOS_PER_DAY as i128) as u64,
        )
    }
}

impl AddAssign<Duration> for Time {
    /// Add the sub-day time of the `Duration` to the existing `Time`. Wraps on
    /// overflow and underflow.
    ///
    /// ```rust
    /// # use time::{Duration, Time};
    ///
    /// let mut time = Time::from_hms(12, 0, 0);
    /// time += Duration::hours(2);
    /// assert_eq!(time, Time::from_hms(14, 0, 0));
    ///
    /// let mut time = Time::from_hms(0, 0, 1);
    /// time += Duration::seconds(-2);
    /// assert_eq!(time, Time::from_hms(23, 59, 59));
    /// ```
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Time {
    type Output = Self;

    /// Subtract the sub-day time of the `Duration` from the `Time`. Wraps on
    /// overflow and underflow.
    ///
    /// ```rust
    /// # use time::{Duration, Time};
    /// assert_eq!(Time::from_hms(14, 0, 0) - Duration::hours(2), Time::from_hms(12, 0, 0));
    /// assert_eq!(Time::from_hms(23, 59, 59) - Duration::seconds(-2), Time::from_hms(0, 0, 1));
    /// ```
    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl SubAssign<Duration> for Time {
    /// Subtract the sub-day time of the `Duration` fromthe existing `Time`.
    /// Wraps on overflow and underflow.
    ///
    /// ```rust
    /// # use time::{Duration, Time};
    ///
    /// let mut time = Time::from_hms(14, 0, 0);
    /// time -= Duration::hours(2);
    /// assert_eq!(time, Time::from_hms(12, 0, 0));
    ///
    /// let mut time = Time::from_hms(23, 59, 59);
    /// time -= Duration::seconds(-2);
    /// assert_eq!(time, Time::from_hms(0, 0, 1));
    /// ```
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration
    }
}

impl Sub<Time> for Time {
    type Output = Duration;

    /// Subtract two `Time`s, returning the `Duration` between. This assumes
    /// both `Time`s are in the same calendar day.
    ///
    /// ```rust
    /// use time::{Duration, Time};
    ///
    /// assert_eq!(Time::from_hms(0, 0, 0) - Time::from_hms(0, 0, 0), Duration::zero());
    /// assert_eq!(Time::from_hms(1, 0, 0) - Time::from_hms(0, 0, 0), Duration::hour());
    /// assert_eq!(Time::from_hms(0, 0, 0) - Time::from_hms(1, 0, 0), Duration::hours(-1));
    /// assert_eq!(Time::from_hms(0, 0, 0) - Time::from_hms(23, 0, 0), Duration::hours(-23));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Duration::nanoseconds(
            self.nanoseconds_since_midnight() as i64 - rhs.nanoseconds_since_midnight() as i64,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nanoseconds_since_midnight() {
        let time = Time::from_hms(0, 0, 0);
        assert_eq!(time.nanoseconds_since_midnight(), 0);
        assert_eq!(Time::from_nanoseconds_since_midnight(0), time);

        let time = Time::from_hms_nano(23, 59, 59, 999_999_999);
        assert_eq!(time.nanoseconds_since_midnight(), NANOS_PER_DAY - 1);
        assert_eq!(
            Time::from_nanoseconds_since_midnight(NANOS_PER_DAY - 1),
            time
        );
    }
}
