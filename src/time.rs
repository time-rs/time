use crate::{error, Duration};
use const_fn::const_fn;
use core::{
    convert::TryFrom,
    fmt::{self, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};

/// The clock time within a given date. Nanosecond precision.
///
/// All minutes are assumed to have exactly 60 seconds; no attempt is made to
/// handle leap seconds (either positive or negative).
///
/// When comparing two `Time`s, they are assumed to be in the same calendar
/// date.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(into = "crate::serde::Time", try_from = "crate::serde::Time")
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Time {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) hour: u8,
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) minute: u8,
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) second: u8,
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) nanosecond: u32,
}

impl Time {
    /// Create a `Time` from its components.
    #[doc(hidden)]
    pub const fn from_hms_nanos_unchecked(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Self {
        Self {
            hour,
            minute,
            second,
            nanosecond,
        }
    }

    /// Create a `Time` that is exactly midnight.
    ///
    /// ```rust
    /// # use time::Time;
    /// # use time_macros::time;
    /// assert_eq!(Time::midnight(), time!("0:00"));
    /// ```
    pub const fn midnight() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            nanosecond: 0,
        }
    }

    /// Attempt to create a `Time` from the hour, minute, and second.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms(1, 2, 3).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms(24, 0, 0).is_err()); // 24 isn't a valid hour.
    /// assert!(Time::from_hms(0, 60, 0).is_err()); // 60 isn't a valid minute.
    /// assert!(Time::from_hms(0, 0, 60).is_err()); // 60 isn't a valid second.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_hms(hour: u8, minute: u8, second: u8) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => 23);
        ensure_value_in_range!(minute in 0 => 59);
        ensure_value_in_range!(second in 0 => 59);
        Ok(Self {
            hour,
            minute,
            second,
            nanosecond: 0,
        })
    }

    /// Attempt to create a `Time` from the hour, minute, second, and millisecond.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_milli(1, 2, 3, 4).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_milli(24, 0, 0, 0).is_err()); // 24 isn't a valid hour.
    /// assert!(Time::from_hms_milli(0, 60, 0, 0).is_err()); // 60 isn't a valid minute.
    /// assert!(Time::from_hms_milli(0, 0, 60, 0).is_err()); // 60 isn't a valid second.
    /// assert!(Time::from_hms_milli(0, 0, 0, 1_000).is_err()); // 1_000 isn't a valid millisecond.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_hms_milli(
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => 23);
        ensure_value_in_range!(minute in 0 => 59);
        ensure_value_in_range!(second in 0 => 59);
        ensure_value_in_range!(millisecond in 0 => 999);
        Ok(Self {
            hour,
            minute,
            second,
            nanosecond: millisecond as u32 * 1_000_000,
        })
    }

    /// Attempt to create a `Time` from the hour, minute, second, and microsecond.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_micro(1, 2, 3, 4).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_micro(24, 0, 0, 0).is_err()); // 24 isn't a valid hour.
    /// assert!(Time::from_hms_micro(0, 60, 0, 0).is_err()); // 60 isn't a valid minute.
    /// assert!(Time::from_hms_micro(0, 0, 60, 0).is_err()); // 60 isn't a valid second.
    /// assert!(Time::from_hms_micro(0, 0, 0, 1_000_000).is_err()); // 1_000_000 isn't a valid microsecond.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_hms_micro(
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => 23);
        ensure_value_in_range!(minute in 0 => 59);
        ensure_value_in_range!(second in 0 => 59);
        ensure_value_in_range!(microsecond in 0 => 999_999);
        Ok(Self {
            hour,
            minute,
            second,
            nanosecond: microsecond * 1_000,
        })
    }

    /// Attempt to create a `Time` from the hour, minute, second, and nanosecond.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_nano(1, 2, 3, 4).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_nano(24, 0, 0, 0).is_err()); // 24 isn't a valid hour.
    /// assert!(Time::from_hms_nano(0, 60, 0, 0).is_err()); // 60 isn't a valid minute.
    /// assert!(Time::from_hms_nano(0, 0, 60, 0).is_err()); // 60 isn't a valid second.
    /// assert!(Time::from_hms_nano(0, 0, 0, 1_000_000_000).is_err()); // 1_000_000_000 isn't a valid nanosecond.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_hms_nano(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => 23);
        ensure_value_in_range!(minute in 0 => 59);
        ensure_value_in_range!(second in 0 => 59);
        ensure_value_in_range!(nanosecond in 0 => 999_999_999);
        Ok(Self {
            hour,
            minute,
            second,
            nanosecond,
        })
    }

    /// Get the clock hour.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!("0:00:00").hour(), 0);
    /// assert_eq!(time!("23:59:59").hour(), 23);
    /// ```
    pub const fn hour(self) -> u8 {
        self.hour
    }

    /// Get the minute within the hour.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!("0:00:00").minute(), 0);
    /// assert_eq!(time!("23:59:59").minute(), 59);
    /// ```
    pub const fn minute(self) -> u8 {
        self.minute
    }

    /// Get the second within the minute.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!("0:00:00").second(), 0);
    /// assert_eq!(time!("23:59:59").second(), 59);
    /// ```
    pub const fn second(self) -> u8 {
        self.second
    }

    /// Get the milliseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!("0:00").millisecond(), 0);
    /// assert_eq!(time!("23:59:59.999").millisecond(), 999);
    /// ```
    pub const fn millisecond(self) -> u16 {
        (self.nanosecond() / 1_000_000) as u16
    }

    /// Get the microseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!("0:00").microsecond(), 0);
    /// assert_eq!(time!("23:59:59.999_999").microsecond(), 999_999);
    /// ```
    pub const fn microsecond(self) -> u32 {
        self.nanosecond() / 1_000
    }

    /// Get the nanoseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!("0:00").nanosecond(), 0);
    /// assert_eq!(time!("23:59:59.999_999_999").nanosecond(), 999_999_999);
    /// ```
    pub const fn nanosecond(self) -> u32 {
        self.nanosecond
    }

    /// Get the number of nanoseconds since midnight.
    pub(crate) const fn nanoseconds_since_midnight(self) -> u64 {
        self.hour() as u64 * 60 * 60 * 1_000_000_000
            + self.minute() as u64 * 60 * 1_000_000_000
            + self.second() as u64 * 1_000_000_000
            + self.nanosecond() as u64
    }

    /// Add the sub-day time of the [`Duration`] to the `Time`. Wraps on
    /// overflow, returning the necessary adjustment to the date value as the
    /// first element of the tuple.
    pub(crate) fn adjusting_add(self, duration: Duration) -> (Duration, Self) {
        let mut nanoseconds = self.nanosecond as i32 + duration.subsec_nanoseconds();
        let mut seconds = self.second as i8 + (duration.whole_seconds() % 60) as i8;
        let mut minutes = self.minute as i8 + (duration.whole_minutes() % 60) as i8;
        let mut hours = self.hour as i8 + (duration.whole_hours() % 24) as i8;
        let mut date_adjustment = Duration::zero();

        // Provide a fast path for values that are already valid. The optimizer
        // is able to eliminate duplicated comparisons, so the added cost of
        // this is extremely little.
        if !(0..1_000_000_000).contains(&nanoseconds)
            || !(0..60).contains(&seconds)
            || !(0..60).contains(&minutes)
            || !(0..24).contains(&hours)
        {
            if nanoseconds >= 1_000_000_000 {
                nanoseconds -= 1_000_000_000;
                seconds += 1;
            } else if nanoseconds < 0 {
                nanoseconds += 1_000_000_000;
                seconds -= 1;
            }
            if seconds >= 60 {
                seconds -= 60;
                minutes += 1;
            } else if seconds < 0 {
                seconds += 60;
                minutes -= 1;
            }
            if minutes >= 60 {
                minutes -= 60;
                hours += 1;
            } else if minutes < 0 {
                minutes += 60;
                hours -= 1;
            }
            if hours >= 24 {
                hours -= 24;
                date_adjustment = Duration::day()
            } else if hours < 0 {
                hours += 24;
                date_adjustment = -Duration::day()
            }
        }

        (
            date_adjustment,
            Self::from_hms_nanos_unchecked(
                hours as u8,
                minutes as u8,
                seconds as u8,
                nanoseconds as u32,
            ),
        )
    }
}

impl Display for Time {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Add<Duration> for Time {
    type Output = Self;

    /// Add the sub-day time of the [`Duration`] to the `Time`. Wraps on
    /// overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::time;
    /// assert_eq!(time!("12:00") + 2.hours(), time!("14:00"));
    /// assert_eq!(time!("0:00:01") + (-2).seconds(), time!("23:59:59"));
    /// ```
    fn add(self, duration: Duration) -> Self::Output {
        self.adjusting_add(duration).1
    }
}

impl Add<StdDuration> for Time {
    type Output = Self;

    /// Add the sub-day time of the [`std::time::Duration`] to the `Time`. Wraps
    /// on overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalStdDuration;
    /// # use time_macros::time;
    /// assert_eq!(time!("12:00") + 2.std_hours(), time!("14:00"));
    /// assert_eq!(time!("23:59:59") + 2.std_seconds(), time!("0:00:01"));
    /// ```
    fn add(self, duration: StdDuration) -> Self::Output {
        self + Duration::try_from(duration)
            .expect("overflow converting `core::time::Duration` to `time::Duration`")
    }
}

impl AddAssign<Duration> for Time {
    /// Add the sub-day time of the [`Duration`] to the existing `Time`. Wraps
    /// on overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::time;
    /// let mut time = time!("12:00");
    /// time += 2.hours();
    /// assert_eq!(time, time!("14:00"));
    ///
    /// let mut time = time!("0:00:01");
    /// time += (-2).seconds();
    /// assert_eq!(time, time!("23:59:59"));
    /// ```
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for Time {
    /// Add the sub-day time of the [`std::time::Duration`] to the existing
    /// `Time`. Wraps on overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalStdDuration;
    /// # use time_macros::time;
    /// let mut time = time!("12:00");
    /// time += 2.std_hours();
    /// assert_eq!(time, time!("14:00"));
    ///
    /// let mut time = time!("23:59:59");
    /// time += 2.std_seconds();
    /// assert_eq!(time, time!("0:00:01"));
    /// ```
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Time {
    type Output = Self;

    /// Subtract the sub-day time of the [`Duration`] from the `Time`. Wraps on
    /// overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::time;
    /// assert_eq!(time!("14:00") - 2.hours(), time!("12:00"));
    /// assert_eq!(time!("23:59:59") - (-2).seconds(), time!("0:00:01"));
    /// ```
    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl Sub<StdDuration> for Time {
    type Output = Self;

    /// Subtract the sub-day time of the [`std::time::Duration`] from the
    /// `Time`.
    /// Wraps on overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalStdDuration;
    /// # use time_macros::time;
    /// assert_eq!(time!("14:00") - 2.std_hours(), time!("12:00"));
    /// assert_eq!(time!("0:00:01") - 2.std_seconds(), time!("23:59:59"));
    /// ```
    fn sub(self, duration: StdDuration) -> Self::Output {
        self - Duration::try_from(duration)
            .expect("overflow converting `core::time::Duration` to `time::Duration`")
    }
}

impl SubAssign<Duration> for Time {
    /// Subtract the sub-day time of the [`Duration`] from the existing `Time`.
    /// Wraps on overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::time;
    /// let mut time = time!("14:00");
    /// time -= 2.hours();
    /// assert_eq!(time, time!("12:00"));
    ///
    /// let mut time = time!("23:59:59");
    /// time -= (-2).seconds();
    /// assert_eq!(time, time!("0:00:01"));
    /// ```
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for Time {
    /// Subtract the sub-day time of the [`std::time::Duration`] from the
    /// existing `Time`. Wraps on overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalStdDuration;
    /// # use time_macros::time;
    /// let mut time = time!("14:00");
    /// time -= 2.std_hours();
    /// assert_eq!(time, time!("12:00"));
    ///
    /// let mut time = time!("0:00:01");
    /// time -= 2.std_seconds();
    /// assert_eq!(time, time!("23:59:59"));
    /// ```
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<Time> for Time {
    type Output = Duration;

    /// Subtract two `Time`s, returning the [`Duration`] between. This assumes
    /// both `Time`s are in the same calendar day.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::time;
    /// assert_eq!(time!("0:00") - time!("0:00"), 0.seconds());
    /// assert_eq!(time!("1:00") - time!("0:00"), 1.hours());
    /// assert_eq!(time!("0:00") - time!("1:00"), (-1).hours());
    /// assert_eq!(time!("0:00") - time!("23:00"), (-23).hours());
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Duration::nanoseconds(
            self.nanoseconds_since_midnight() as i64 - rhs.nanoseconds_since_midnight() as i64,
        )
    }
}
