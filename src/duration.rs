use crate::internal_prelude::*;
use core::{
    cmp::Ordering::{self, Equal, Greater, Less},
    convert::{TryFrom, TryInto},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    time::Duration as StdDuration,
};

/// A span of time with nanosecond precision.
///
/// Each `Duration` is composed of a whole number of seconds and a fractional
/// part represented in nanoseconds.
///
/// `Duration` implements many traits, including [`Add`], [`Sub`], [`Mul`], and
/// [`Div`], among others.
///
/// This implementation allows for negative durations, unlike
/// [`core::time::Duration`].
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    serde,
    serde(from = "crate::serde::Duration", into = "crate::serde::Duration")
)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Duration {
    /// Number of whole seconds.
    pub(crate) seconds: i64,
    /// Number of nanoseconds within the second. The sign always matches the
    /// `seconds` field.
    pub(crate) nanoseconds: i32, // always -10^9 < nanoseconds < 10^9
}

/// The number of seconds in one minute.
const SECONDS_PER_MINUTE: i64 = 60;

/// The number of seconds in one hour.
const SECONDS_PER_HOUR: i64 = 60 * SECONDS_PER_MINUTE;

/// The number of seconds in one day.
const SECONDS_PER_DAY: i64 = 24 * SECONDS_PER_HOUR;

/// The number of seconds in one week.
const SECONDS_PER_WEEK: i64 = 7 * SECONDS_PER_DAY;

impl Duration {
    /// Equivalent to `0.seconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::zero(), 0.seconds());
    /// ```
    #[inline(always)]
    pub const fn zero() -> Self {
        Self::seconds(0)
    }

    /// Equivalent to `1.nanoseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::nanosecond(), 1.nanoseconds());
    /// ```
    #[inline(always)]
    pub const fn nanosecond() -> Self {
        Self::nanoseconds(1)
    }

    /// Equivalent to `1.microseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::microsecond(), 1.microseconds());
    /// ```
    #[inline(always)]
    pub const fn microsecond() -> Self {
        Self::microseconds(1)
    }

    /// Equivalent to `1.milliseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::millisecond(), 1.milliseconds());
    /// ```
    #[inline(always)]
    pub const fn millisecond() -> Self {
        Self::milliseconds(1)
    }

    /// Equivalent to `1.seconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::second(), 1.seconds());
    /// ```
    #[inline(always)]
    pub const fn second() -> Self {
        Self::seconds(1)
    }

    /// Equivalent to `1.minutes()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::minute(), 1.minutes());
    /// ```
    #[inline(always)]
    pub const fn minute() -> Self {
        Self::minutes(1)
    }

    /// Equivalent to `1.hours()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::hour(), 1.hours());
    /// ```
    #[inline(always)]
    pub const fn hour() -> Self {
        Self::hours(1)
    }

    /// Equivalent to `1.days()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::day(), 1.days());
    /// ```
    #[inline(always)]
    pub const fn day() -> Self {
        Self::days(1)
    }

    /// Equivalent to `1.weeks()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::week(), 1.weeks());
    /// ```
    #[inline(always)]
    pub const fn week() -> Self {
        Self::weeks(1)
    }

    /// The maximum possible duration. Adding any positive duration to this will
    /// cause an overflow.
    ///
    /// The value returned by this method may change at any time.
    #[inline(always)]
    pub const fn max_value() -> Self {
        Self::new(i64::max_value(), 999_999_999)
    }

    /// The minimum possible duration. Adding any negative duration to this will
    /// cause an overflow.
    ///
    /// The value returned by this method may change at any time.
    #[inline(always)]
    pub const fn min_value() -> Self {
        Self::new(i64::min_value(), -999_999_999)
    }

    /// Check if a duration is exactly zero.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert!(0.seconds().is_zero());
    /// assert!(!1.nanoseconds().is_zero());
    /// ```
    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        (self.seconds == 0) & (self.nanoseconds == 0)
    }

    /// Check if a duration is negative.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert!((-1).seconds().is_negative());
    /// assert!(!0.seconds().is_negative());
    /// assert!(!1.seconds().is_negative());
    /// ```
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        (self.seconds < 0) | (self.nanoseconds < 0)
    }

    /// Check if a duration is positive.
    ///
    /// ```rust
    /// # use time::{prelude::*};
    /// assert!(1.seconds().is_positive());
    /// assert!(!0.seconds().is_positive());
    /// assert!(!(-1).seconds().is_positive());
    /// ```
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        (self.seconds > 0) | (self.nanoseconds > 0)
    }

    /// Get the sign of the duration.
    ///
    /// ```rust
    /// # use time::{Sign, prelude::*};
    /// assert_eq!(1.seconds().sign(), Sign::Positive);
    /// assert_eq!((-1).seconds().sign(), Sign::Negative);
    /// assert_eq!(0.seconds().sign(), Sign::Zero);
    /// ```
    #[deprecated(
        since = "0.2.7",
        note = "To obtain the sign of a `Duration`, you should use the `is_positive`, \
                `is_negative`, and `is_zero` methods."
    )]
    #[allow(deprecated)]
    #[inline(always)]
    pub fn sign(self) -> crate::Sign {
        use crate::Sign::*;

        if self.nanoseconds > 0 {
            Positive
        } else if self.nanoseconds < 0 {
            Negative
        } else if self.seconds > 0 {
            Positive
        } else if self.seconds < 0 {
            Negative
        } else {
            Zero
        }
    }

    /// Get the absolute value of the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.seconds().abs(), 1.seconds());
    /// assert_eq!(0.seconds().abs(), 0.seconds());
    /// assert_eq!((-1).seconds().abs(), 1.seconds());
    /// ```
    #[inline(always)]
    #[rustversion::attr(since(1.39), const)]
    pub fn abs(self) -> Self {
        Self {
            seconds: self.seconds.abs(),
            nanoseconds: self.nanoseconds.abs(),
        }
    }

    /// Convert the existing `Duration` to a `std::time::Duration` and its sign.
    // This doesn't actually require the standard library, but is currently only
    // used when it's enabled.
    #[inline(always)]
    #[cfg(std)]
    pub(crate) fn abs_std(self) -> StdDuration {
        StdDuration::new(self.seconds.abs() as u64, self.nanoseconds.abs() as u32)
    }

    /// Create a new `Duration` with the provided seconds and nanoseconds. If
    /// nanoseconds is at least 10<sup>9</sup>, it will wrap to the number of
    /// seconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::new(1, 0), 1.seconds());
    /// assert_eq!(Duration::new(-1, 0), (-1).seconds());
    /// assert_eq!(Duration::new(1, 2_000_000_000), 3.seconds());
    /// ```
    #[inline(always)]
    pub const fn new(seconds: i64, nanoseconds: i32) -> Self {
        Self {
            seconds: seconds + nanoseconds as i64 / 1_000_000_000,
            nanoseconds: nanoseconds % 1_000_000_000,
        }
    }

    /// Create a new `Duration` with the given number of weeks. Equivalent to
    /// `Duration::seconds(weeks * 604_800)`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::weeks(1), 604_800.seconds());
    /// ```
    #[inline(always)]
    pub const fn weeks(weeks: i64) -> Self {
        Self::seconds(weeks * SECONDS_PER_WEEK)
    }

    /// Get the number of whole weeks in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.weeks().whole_weeks(), 1);
    /// assert_eq!((-1).weeks().whole_weeks(), -1);
    /// assert_eq!(6.days().whole_weeks(), 0);
    /// assert_eq!((-6).days().whole_weeks(), 0);
    /// ```
    #[inline(always)]
    pub const fn whole_weeks(self) -> i64 {
        self.whole_seconds() / SECONDS_PER_WEEK
    }

    /// Create a new `Duration` with the given number of days. Equivalent to
    /// `Duration::seconds(days * 86_400)`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::days(1), 86_400.seconds());
    /// ```
    #[inline(always)]
    pub const fn days(days: i64) -> Self {
        Self::seconds(days * SECONDS_PER_DAY)
    }

    /// Get the number of whole days in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.days().whole_days(), 1);
    /// assert_eq!((-1).days().whole_days(), -1);
    /// assert_eq!(23.hours().whole_days(), 0);
    /// assert_eq!((-23).hours().whole_days(), 0);
    /// ```
    #[inline(always)]
    pub const fn whole_days(self) -> i64 {
        self.whole_seconds() / SECONDS_PER_DAY
    }

    /// Create a new `Duration` with the given number of hours. Equivalent to
    /// `Duration::seconds(hours * 3_600)`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::hours(1), 3_600.seconds());
    /// ```
    #[inline(always)]
    pub const fn hours(hours: i64) -> Self {
        Self::seconds(hours * SECONDS_PER_HOUR)
    }

    /// Get the number of whole hours in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.hours().whole_hours(), 1);
    /// assert_eq!((-1).hours().whole_hours(), -1);
    /// assert_eq!(59.minutes().whole_hours(), 0);
    /// assert_eq!((-59).minutes().whole_hours(), 0);
    /// ```
    #[inline(always)]
    pub const fn whole_hours(self) -> i64 {
        self.whole_seconds() / SECONDS_PER_HOUR
    }

    /// Create a new `Duration` with the given number of minutes. Equivalent to
    /// `Duration::seconds(minutes * 60)`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::minutes(1), 60.seconds());
    /// ```
    #[inline(always)]
    pub const fn minutes(minutes: i64) -> Self {
        Self::seconds(minutes * SECONDS_PER_MINUTE)
    }

    /// Get the number of whole minutes in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.minutes().whole_minutes(), 1);
    /// assert_eq!((-1).minutes().whole_minutes(), -1);
    /// assert_eq!(59.seconds().whole_minutes(), 0);
    /// assert_eq!((-59).seconds().whole_minutes(), 0);
    /// ```
    #[inline(always)]
    pub const fn whole_minutes(self) -> i64 {
        self.whole_seconds() / SECONDS_PER_MINUTE
    }

    /// Create a new `Duration` with the given number of seconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::seconds(1), 1_000.milliseconds());
    /// ```
    #[inline(always)]
    pub const fn seconds(seconds: i64) -> Self {
        Self {
            seconds,
            nanoseconds: 0,
        }
    }

    /// Get the number of whole seconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.seconds().whole_seconds(), 1);
    /// assert_eq!((-1).seconds().whole_seconds(), -1);
    /// assert_eq!(1.minutes().whole_seconds(), 60);
    /// assert_eq!((-1).minutes().whole_seconds(), -60);
    /// ```
    #[inline(always)]
    pub const fn whole_seconds(self) -> i64 {
        self.seconds
    }

    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f64`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::seconds_f64(0.5), 0.5.seconds());
    /// assert_eq!(Duration::seconds_f64(-0.5), -0.5.seconds());
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn seconds_f64(seconds: f64) -> Self {
        Self {
            seconds: seconds as i64,
            nanoseconds: ((seconds % 1.) * 1_000_000_000.) as i32,
        }
    }

    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.5.seconds().as_seconds_f64(), 1.5);
    /// assert_eq!((-1.5).seconds().as_seconds_f64(), -1.5);
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_seconds_f64(self) -> f64 {
        self.seconds as f64 + self.nanoseconds as f64 / 1_000_000_000.
    }

    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f32`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::seconds_f32(0.5), 0.5.seconds());
    /// assert_eq!(Duration::seconds_f32(-0.5), (-0.5).seconds());
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn seconds_f32(seconds: f32) -> Self {
        Self {
            seconds: seconds as i64,
            nanoseconds: ((seconds % 1.) * 1_000_000_000.) as i32,
        }
    }

    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.5.seconds().as_seconds_f32(), 1.5);
    /// assert_eq!((-1.5).seconds().as_seconds_f32(), -1.5);
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_seconds_f32(self) -> f32 {
        self.seconds as f32 + self.nanoseconds as f32 / 1_000_000_000.
    }

    /// Create a new `Duration` with the given number of milliseconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::milliseconds(1), 1_000.microseconds());
    /// assert_eq!(Duration::milliseconds(-1), (-1_000).microseconds());
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn milliseconds(milliseconds: i64) -> Self {
        Self {
            seconds: milliseconds / 1_000,
            nanoseconds: ((milliseconds % 1_000) * 1_000_000) as i32,
        }
    }

    /// Get the number of whole milliseconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.seconds().whole_milliseconds(), 1_000);
    /// assert_eq!((-1).seconds().whole_milliseconds(), -1_000);
    /// assert_eq!(1.milliseconds().whole_milliseconds(), 1);
    /// assert_eq!((-1).milliseconds().whole_milliseconds(), -1);
    /// ```
    #[inline(always)]
    pub const fn whole_milliseconds(self) -> i128 {
        self.seconds as i128 * 1_000 + self.nanoseconds as i128 / 1_000_000
    }

    /// Get the number of milliseconds past the number of whole seconds.
    ///
    /// Always in the range `-1_000..1_000`.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.4.seconds().subsec_milliseconds(), 400);
    /// assert_eq!((-1.4).seconds().subsec_milliseconds(), -400);
    /// ```
    // Allow the lint, as the value is guaranteed to be less than 1000.
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn subsec_milliseconds(self) -> i16 {
        (self.nanoseconds / 1_000_000) as i16
    }

    /// Create a new `Duration` with the given number of microseconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::microseconds(1), 1_000.nanoseconds());
    /// assert_eq!(Duration::microseconds(-1), (-1_000).nanoseconds());
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn microseconds(microseconds: i64) -> Self {
        Self {
            seconds: microseconds / 1_000_000,
            nanoseconds: ((microseconds % 1_000_000) * 1_000) as i32,
        }
    }

    /// Get the number of whole microseconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.milliseconds().whole_microseconds(), 1_000);
    /// assert_eq!((-1).milliseconds().whole_microseconds(), -1_000);
    /// assert_eq!(1.microseconds().whole_microseconds(), 1);
    /// assert_eq!((-1).microseconds().whole_microseconds(), -1);
    /// ```
    #[inline(always)]
    pub const fn whole_microseconds(self) -> i128 {
        self.seconds as i128 * 1_000_000 + self.nanoseconds as i128 / 1_000
    }

    /// Get the number of microseconds past the number of whole seconds.
    ///
    /// Always in the range `-1_000_000..1_000_000`.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.0004.seconds().subsec_microseconds(), 400);
    /// assert_eq!((-1.0004).seconds().subsec_microseconds(), -400);
    /// ```
    #[inline(always)]
    pub const fn subsec_microseconds(self) -> i32 {
        self.nanoseconds / 1_000
    }

    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::nanoseconds(1), 1.microseconds() / 1_000);
    /// assert_eq!(Duration::nanoseconds(-1), (-1).microseconds() / 1_000);
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn nanoseconds(nanoseconds: i64) -> Self {
        Self {
            seconds: nanoseconds / 1_000_000_000,
            nanoseconds: (nanoseconds % 1_000_000_000) as i32,
        }
    }

    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// As the input range cannot be fully mapped to the output, this should
    /// only be used where it's known to result in a valid value.
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn nanoseconds_i128(nanoseconds: i128) -> Self {
        Self {
            seconds: (nanoseconds / 1_000_000_000) as i64,
            nanoseconds: (nanoseconds % 1_000_000_000) as i32,
        }
    }

    /// Get the number of nanoseconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.microseconds().whole_nanoseconds(), 1_000);
    /// assert_eq!((-1).microseconds().whole_nanoseconds(), -1_000);
    /// assert_eq!(1.nanoseconds().whole_nanoseconds(), 1);
    /// assert_eq!((-1).nanoseconds().whole_nanoseconds(), -1);
    /// ```
    #[inline(always)]
    pub const fn whole_nanoseconds(self) -> i128 {
        self.seconds as i128 * 1_000_000_000 + self.nanoseconds as i128
    }

    /// Get the number of nanoseconds past the number of whole seconds.
    ///
    /// The returned value will always be in the range
    /// `-1_000_000_000..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.000_000_400.seconds().subsec_nanoseconds(), 400);
    /// assert_eq!((-1.000_000_400).seconds().subsec_nanoseconds(), -400);
    /// ```
    #[inline(always)]
    pub const fn subsec_nanoseconds(self) -> i32 {
        self.nanoseconds
    }

    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
    /// assert_eq!(Duration::max_value().checked_add(1.nanoseconds()), None);
    /// assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
    /// ```
    #[inline]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let mut seconds = self.seconds.checked_add(rhs.seconds)?;
        let mut nanoseconds = self.nanoseconds + rhs.nanoseconds;

        if nanoseconds >= 1_000_000_000 || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= 1_000_000_000;
            seconds = seconds.checked_add(1)?;
        } else if nanoseconds <= -1_000_000_000 || seconds > 0 && nanoseconds < 0 {
            nanoseconds += 1_000_000_000;
            seconds = seconds.checked_sub(1)?;
        }

        // Ensure that the signs match _unless_ one of them is zero.
        debug_assert_ne!(seconds.signum() * nanoseconds.signum() as i64, -1);
        debug_assert!((-999_999_999..1_000_000_000).contains(&nanoseconds));

        Some(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Computes `self - rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(5.seconds().checked_sub(5.seconds()), Some(Duration::zero()));
    /// assert_eq!(Duration::min_value().checked_sub(1.nanoseconds()), None);
    /// assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
    /// ```
    #[inline(always)]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_add(-rhs)
    }

    /// Computes `self * rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
    /// assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
    /// assert_eq!(5.seconds().checked_mul(0), Some(0.seconds()));
    /// assert_eq!(Duration::max_value().checked_mul(2), None);
    /// assert_eq!(Duration::min_value().checked_mul(2), None);
    /// ```
    #[inline(always)]
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        // Multiply nanoseconds as i64, because it cannot overflow that way.
        let total_nanos = self.nanoseconds as i64 * rhs as i64;
        let extra_secs = total_nanos / 1_000_000_000;
        #[allow(clippy::cast_possible_truncation)]
        let nanoseconds = (total_nanos % 1_000_000_000) as i32;
        let seconds = self
            .seconds
            .checked_mul(rhs as i64)?
            .checked_add(extra_secs)?;

        Some(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0`.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
    /// assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
    /// assert_eq!(1.seconds().checked_div(0), None);
    #[inline(always)]
    pub fn checked_div(self, rhs: i32) -> Option<Self> {
        if rhs == 0 {
            return None;
        }

        let seconds = self.seconds / (rhs as i64);
        let carry = self.seconds - seconds * (rhs as i64);
        let extra_nanos = carry * 1_000_000_000 / (rhs as i64);
        #[allow(clippy::cast_possible_truncation)]
        let nanoseconds = self.nanoseconds / rhs + (extra_nanos as i32);

        Some(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Runs a closure, returning the duration of time it took to run. The
    /// return value of the closure is provided in the second part of the tuple.
    #[inline(always)]
    #[cfg(std)]
    #[cfg_attr(docs, doc(cfg(feature = "std")))]
    pub fn time_fn<T>(f: impl FnOnce() -> T) -> (Self, T) {
        let start = Instant::now();
        let return_value = f();
        let end = Instant::now();

        (end - start, return_value)
    }
}

/// Functions that have been renamed or had signatures changed since v0.1. As
/// such, they are deprecated.
#[cfg(v01_deprecated)]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items, clippy::missing_const_for_fn)]
impl Duration {
    #[inline(always)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_weeks` function")]
    pub fn num_weeks(&self) -> i64 {
        self.whole_weeks()
    }

    #[inline(always)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_days` function")]
    pub fn num_days(&self) -> i64 {
        self.whole_days()
    }

    #[inline(always)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_hours` function")]
    pub fn num_hours(&self) -> i64 {
        self.whole_hours()
    }

    #[inline(always)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_minutes` function")]
    pub fn num_minutes(&self) -> i64 {
        self.whole_minutes()
    }

    #[allow(clippy::missing_const_for_fn)]
    #[inline(always)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_seconds` function")]
    pub fn num_seconds(&self) -> i64 {
        self.whole_seconds()
    }

    /// [`Duration::whole_milliseconds`] returns an `i128`, rather than
    /// panicking on overflow. To avoid panicking, this method currently limits
    /// the value to the range `i64::min_value()..=i64::max_value()`.
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    #[deprecated(
        since = "0.2.0",
        note = "Use the `whole_milliseconds` function. The value is clamped between \
                `i64::min_value()` and `i64::max_value()`."
    )]
    pub fn num_milliseconds(&self) -> i64 {
        let millis = self.whole_milliseconds();

        if millis > i64::max_value() as i128 {
            return i64::max_value();
        }

        if millis < i64::min_value() as i128 {
            return i64::min_value();
        }

        millis as i64
    }

    /// [`Duration::whole_microseconds`] returns an `i128` rather than returning
    /// `None` on `i64` overflow.
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_microseconds` function")]
    pub fn num_microseconds(&self) -> Option<i64> {
        let micros = self.whole_microseconds();

        if micros.abs() > i64::max_value() as i128 {
            None
        } else {
            Some(micros as i64)
        }
    }

    /// [`Duration::whole_nanoseconds`] returns an `i128` rather than returning
    /// `None` on `i64` overflow.
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_nanoseconds` function")]
    pub fn num_nanoseconds(&self) -> Option<i64> {
        let nanos = self.whole_nanoseconds();

        if nanos.abs() > i64::max_value() as i128 {
            None
        } else {
            Some(nanos as i64)
        }
    }

    #[inline(always)]
    #[cfg(std)]
    #[deprecated(since = "0.2.0", note = "Use the `time_fn` function")]
    pub fn span<F: FnOnce()>(f: F) -> Self {
        Self::time_fn(f).0
    }

    #[inline(always)]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.0",
        note = "Use `Duration::try_from(value)` or `value.try_into()`"
    )]
    pub fn from_std(std: StdDuration) -> Result<Self, ConversionRangeError> {
        std.try_into()
    }

    #[inline(always)]
    #[allow(deprecated, clippy::cast_sign_loss)]
    #[deprecated(
        since = "0.2.0",
        note = "Use `std::time::Duration::try_from(value)` or `value.try_into()`"
    )]
    pub fn to_std(&self) -> Result<StdDuration, ConversionRangeError> {
        (*self).try_into()
    }
}

impl TryFrom<StdDuration> for Duration {
    type Error = ConversionRangeError;

    #[inline(always)]
    fn try_from(original: StdDuration) -> Result<Self, ConversionRangeError> {
        Ok(Self::new(
            original
                .as_secs()
                .try_into()
                .map_err(|_| ConversionRangeError::new())?,
            original
                .subsec_nanos()
                .try_into()
                .map_err(|_| ConversionRangeError::new())?,
        ))
    }
}

impl TryFrom<Duration> for StdDuration {
    type Error = ConversionRangeError;

    #[inline(always)]
    fn try_from(duration: Duration) -> Result<Self, ConversionRangeError> {
        Ok(Self::new(
            duration
                .seconds
                .try_into()
                .map_err(|_| ConversionRangeError::new())?,
            duration
                .nanoseconds
                .try_into()
                .map_err(|_| ConversionRangeError::new())?,
        ))
    }
}

impl Add for Duration {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding durations")
    }
}

impl Add<StdDuration> for Duration {
    type Output = Self;

    #[inline(always)]
    fn add(self, std_duration: StdDuration) -> Self::Output {
        self + Self::try_from(std_duration)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
    }
}

impl Add<Duration> for StdDuration {
    type Output = Duration;

    #[inline(always)]
    fn add(self, rhs: Duration) -> Self::Output {
        rhs + self
    }
}

impl AddAssign for Duration {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<StdDuration> for Duration {
    #[inline(always)]
    fn add_assign(&mut self, rhs: StdDuration) {
        *self = *self + rhs;
    }
}

impl Neg for Duration {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        -1 * self
    }
}

impl Sub for Duration {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting durations")
    }
}

impl Sub<StdDuration> for Duration {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: StdDuration) -> Self::Output {
        self - Self::try_from(rhs)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
    }
}

impl Sub<Duration> for StdDuration {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: Duration) -> Self::Output {
        Duration::try_from(self)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
            - rhs
    }
}

impl SubAssign for Duration {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<StdDuration> for Duration {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: StdDuration) {
        *self = *self - rhs;
    }
}

impl SubAssign<Duration> for StdDuration {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = (*self - rhs).try_into().expect(
            "Cannot represent a resulting duration in std. Try `let x = x - rhs;`, which will \
             change the type.",
        );
    }
}

macro_rules! duration_mul_div_int {
    ($($type:ty),+) => {
        $(
            impl Mul<$type> for Duration {
                type Output = Self;

                #[inline(always)]
                #[allow(trivial_numeric_casts)]
                fn mul(self, rhs: $type) -> Self::Output {
                    Self::nanoseconds_i128(
                        self.whole_nanoseconds()
                            .checked_mul(rhs as i128)
                            .expect("overflow when multiplying duration")
                    )
                }
            }

            impl MulAssign<$type> for Duration {
                #[inline(always)]
                fn mul_assign(&mut self, rhs: $type) {
                    *self = *self * rhs;
                }
            }

            impl Mul<Duration> for $type {
                type Output = Duration;

                #[inline(always)]
                fn mul(self, rhs: Duration) -> Self::Output {
                    rhs * self
                }
            }

            impl Div<$type> for Duration {
                type Output = Self;

                #[inline(always)]
                #[allow(trivial_numeric_casts)]
                fn div(self, rhs: $type) -> Self::Output {
                    Self::nanoseconds_i128(self.whole_nanoseconds() / rhs as i128)
                }
            }

            impl DivAssign<$type> for Duration {
                #[inline(always)]
                fn div_assign(&mut self, rhs: $type) {
                    *self = *self / rhs;
                }
            }
        )+
    };
}
duration_mul_div_int![i8, i16, i32, u8, u16, u32];

impl Mul<f32> for Duration {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::seconds_f32(self.as_seconds_f32() * rhs)
    }
}

impl MulAssign<f32> for Duration {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Mul<Duration> for f32 {
    type Output = Duration;

    #[inline(always)]
    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for Duration {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::seconds_f64(self.as_seconds_f64() * rhs)
    }
}

impl MulAssign<f64> for Duration {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Mul<Duration> for f64 {
    type Output = Duration;

    #[inline(always)]
    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Duration {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Self::seconds_f32(self.as_seconds_f32() / rhs)
    }
}

impl DivAssign<f32> for Duration {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Div<f64> for Duration {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f64) -> Self::Output {
        Self::seconds_f64(self.as_seconds_f64() / rhs)
    }
}

impl DivAssign<f64> for Duration {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Div<Duration> for Duration {
    type Output = f64;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        self.as_seconds_f64() / rhs.as_seconds_f64()
    }
}

impl Div<StdDuration> for Duration {
    type Output = f64;

    #[inline(always)]
    fn div(self, rhs: StdDuration) -> Self::Output {
        self.as_seconds_f64() / rhs.as_secs_f64()
    }
}

impl Div<Duration> for StdDuration {
    type Output = f64;

    #[inline(always)]
    fn div(self, rhs: Duration) -> Self::Output {
        self.as_secs_f64() / rhs.as_seconds_f64()
    }
}

impl PartialEq<StdDuration> for Duration {
    #[inline(always)]
    fn eq(&self, rhs: &StdDuration) -> bool {
        Ok(*self) == Self::try_from(*rhs)
    }
}

impl PartialEq<Duration> for StdDuration {
    #[inline(always)]
    fn eq(&self, rhs: &Duration) -> bool {
        rhs == self
    }
}

impl PartialOrd for Duration {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl PartialOrd<StdDuration> for Duration {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &StdDuration) -> Option<Ordering> {
        if rhs.as_secs() > i64::max_value() as u64 {
            return Some(Greater);
        }

        match self.seconds.partial_cmp(&(rhs.as_secs() as i64)) {
            Some(Less) => Some(Less),
            Some(Equal) => self.nanoseconds.partial_cmp(&(rhs.subsec_nanos() as i32)),
            Some(Greater) => Some(Greater),
            None => None,
        }
    }
}

impl PartialOrd<Duration> for StdDuration {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &Duration) -> Option<Ordering> {
        match rhs.partial_cmp(self) {
            Some(Less) => Some(Greater),
            Some(Equal) => Some(Equal),
            Some(Greater) => Some(Less),
            None => None,
        }
    }
}

impl Ord for Duration {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.seconds.cmp(&rhs.seconds) {
            Less => Less,
            Equal => self.nanoseconds.cmp(&rhs.nanoseconds),
            Greater => Greater,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_values() {
        assert_eq!(Duration::zero(), 0.seconds());
        assert_eq!(Duration::nanosecond(), 1.nanoseconds());
        assert_eq!(Duration::microsecond(), 1.microseconds());
        assert_eq!(Duration::millisecond(), 1.milliseconds());
        assert_eq!(Duration::second(), 1.seconds());
        assert_eq!(Duration::minute(), 60.seconds());
        assert_eq!(Duration::hour(), 3_600.seconds());
        assert_eq!(Duration::day(), 86_400.seconds());
        assert_eq!(Duration::week(), 604_800.seconds());
    }

    #[test]
    fn is_zero() {
        assert!(!(-1).nanoseconds().is_zero());
        assert!(0.seconds().is_zero());
        assert!(!1.nanoseconds().is_zero());
    }

    #[test]
    fn is_negative() {
        assert!((-1).seconds().is_negative());
        assert!(!0.seconds().is_negative());
        assert!(!1.seconds().is_negative());
    }

    #[test]
    fn is_positive() {
        assert!(!(-1).seconds().is_positive());
        assert!(!0.seconds().is_positive());
        assert!(1.seconds().is_positive());
    }

    #[allow(deprecated)]
    #[test]
    fn sign() {
        use crate::Sign::*;
        assert_eq!(1.seconds().sign(), Positive);
        assert_eq!((-1).seconds().sign(), Negative);
        assert_eq!(0.seconds().sign(), Zero);
    }

    #[test]
    fn abs() {
        assert_eq!(1.seconds().abs(), 1.seconds());
        assert_eq!(0.seconds().abs(), 0.seconds());
        assert_eq!((-1).seconds().abs(), 1.seconds());
    }

    #[test]
    fn new() {
        assert_eq!(Duration::new(1, 0), 1.seconds());
        assert_eq!(Duration::new(-1, 0), (-1).seconds());
        assert_eq!(Duration::new(1, 2_000_000_000), 3.seconds());

        assert!(Duration::new(0, 0).is_zero());
        assert!(Duration::new(0, 1_000_000_000).is_positive());
        assert!(Duration::new(-1, 1_000_000_000).is_zero());
        assert!(Duration::new(-2, 1_000_000_000).is_negative());
    }

    #[test]
    fn weeks() {
        assert_eq!(Duration::weeks(1), 604_800.seconds());
        assert_eq!(Duration::weeks(2), (2 * 604_800).seconds());
        assert_eq!(Duration::weeks(-1), (-604_800).seconds());
        assert_eq!(Duration::weeks(-2), (2 * -604_800).seconds());
    }

    #[test]
    fn whole_weeks() {
        assert_eq!(Duration::weeks(1).whole_weeks(), 1);
        assert_eq!(Duration::weeks(-1).whole_weeks(), -1);
        assert_eq!(Duration::days(6).whole_weeks(), 0);
        assert_eq!(Duration::days(-6).whole_weeks(), 0);
    }

    #[test]
    fn days() {
        assert_eq!(Duration::days(1), 86_400.seconds());
        assert_eq!(Duration::days(2), (2 * 86_400).seconds());
        assert_eq!(Duration::days(-1), (-86_400).seconds());
        assert_eq!(Duration::days(-2), (2 * -86_400).seconds());
    }

    #[test]
    fn whole_days() {
        assert_eq!(Duration::days(1).whole_days(), 1);
        assert_eq!(Duration::days(-1).whole_days(), -1);
        assert_eq!(Duration::hours(23).whole_days(), 0);
        assert_eq!(Duration::hours(-23).whole_days(), 0);
    }

    #[test]
    fn hours() {
        assert_eq!(Duration::hours(1), 3_600.seconds());
        assert_eq!(Duration::hours(2), (2 * 3_600).seconds());
        assert_eq!(Duration::hours(-1), (-3_600).seconds());
        assert_eq!(Duration::hours(-2), (2 * -3_600).seconds());
    }

    #[test]
    fn whole_hours() {
        assert_eq!(Duration::hours(1).whole_hours(), 1);
        assert_eq!(Duration::hours(-1).whole_hours(), -1);
        assert_eq!(Duration::minutes(59).whole_hours(), 0);
        assert_eq!(Duration::minutes(-59).whole_hours(), 0);
    }

    #[test]
    fn minutes() {
        assert_eq!(Duration::minutes(1), 60.seconds());
        assert_eq!(Duration::minutes(2), (2 * 60).seconds());
        assert_eq!(Duration::minutes(-1), (-60).seconds());
        assert_eq!(Duration::minutes(-2), (2 * -60).seconds());
    }

    #[test]
    fn whole_minutes() {
        assert_eq!(1.minutes().whole_minutes(), 1);
        assert_eq!((-1).minutes().whole_minutes(), -1);
        assert_eq!(59.seconds().whole_minutes(), 0);
        assert_eq!((-59).seconds().whole_minutes(), 0);
    }

    #[test]
    fn seconds() {
        assert_eq!(Duration::seconds(1), 1_000.milliseconds());
        assert_eq!(Duration::seconds(2), (2 * 1_000).milliseconds());
        assert_eq!(Duration::seconds(-1), (-1_000).milliseconds());
        assert_eq!(Duration::seconds(-2), (2 * -1_000).milliseconds());
    }

    #[test]
    fn whole_seconds() {
        assert_eq!(1.seconds().whole_seconds(), 1);
        assert_eq!((-1).seconds().whole_seconds(), -1);
        assert_eq!(1.minutes().whole_seconds(), 60);
        assert_eq!((-1).minutes().whole_seconds(), -60);
    }

    #[test]
    fn seconds_f64() {
        assert_eq!(Duration::seconds_f64(0.5), 0.5.seconds());
        assert_eq!(Duration::seconds_f64(-0.5), (-0.5).seconds());
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn as_seconds_f64() {
        assert_eq!(1.seconds().as_seconds_f64(), 1.0);
        assert_eq!((-1).seconds().as_seconds_f64(), -1.0);
        assert_eq!(1.minutes().as_seconds_f64(), 60.0);
        assert_eq!((-1).minutes().as_seconds_f64(), -60.0);
        assert_eq!(1.5.seconds().as_seconds_f64(), 1.5);
        assert_eq!((-1.5).seconds().as_seconds_f64(), -1.5);
    }

    #[test]
    fn seconds_f32() {
        assert_eq!(Duration::seconds_f32(0.5), 0.5.seconds());
        assert_eq!(Duration::seconds_f32(-0.5), (-0.5).seconds());
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn as_seconds_f32() {
        assert_eq!(1.seconds().as_seconds_f32(), 1.0);
        assert_eq!((-1).seconds().as_seconds_f32(), -1.0);
        assert_eq!(1.minutes().as_seconds_f32(), 60.0);
        assert_eq!((-1).minutes().as_seconds_f32(), -60.0);
        assert_eq!(1.5.seconds().as_seconds_f32(), 1.5);
        assert_eq!((-1.5).seconds().as_seconds_f32(), -1.5);
    }

    #[test]
    fn milliseconds() {
        assert_eq!(Duration::milliseconds(1), 1_000.microseconds());
        assert_eq!(Duration::milliseconds(-1), (-1000).microseconds());
    }

    #[test]
    fn whole_milliseconds() {
        assert_eq!(1.seconds().whole_milliseconds(), 1_000);
        assert_eq!((-1).seconds().whole_milliseconds(), -1_000);
        assert_eq!(1.milliseconds().whole_milliseconds(), 1);
        assert_eq!((-1).milliseconds().whole_milliseconds(), -1);
    }

    #[test]
    fn subsec_milliseconds() {
        assert_eq!(1.4.seconds().subsec_milliseconds(), 400);
        assert_eq!((-1.4).seconds().subsec_milliseconds(), -400);
    }

    #[test]
    fn microseconds() {
        assert_eq!(Duration::microseconds(1), 1_000.nanoseconds());
        assert_eq!(Duration::microseconds(-1), (-1_000).nanoseconds());
    }

    #[test]
    fn whole_microseconds() {
        assert_eq!(1.milliseconds().whole_microseconds(), 1_000);
        assert_eq!((-1).milliseconds().whole_microseconds(), -1_000);
        assert_eq!(1.microseconds().whole_microseconds(), 1);
        assert_eq!((-1).microseconds().whole_microseconds(), -1);
    }

    #[test]
    fn subsec_microseconds() {
        assert_eq!(1.0004.seconds().subsec_microseconds(), 400);
        assert_eq!((-1.0004).seconds().subsec_microseconds(), -400);
    }

    #[test]
    fn nanoseconds() {
        assert_eq!(Duration::nanoseconds(1), 1.microseconds() / 1_000);
        assert_eq!(Duration::nanoseconds(-1), (-1).microseconds() / 1_000);
    }

    #[test]
    fn whole_nanoseconds() {
        assert_eq!(1.microseconds().whole_nanoseconds(), 1_000);
        assert_eq!((-1).microseconds().whole_nanoseconds(), -1_000);
        assert_eq!(1.nanoseconds().whole_nanoseconds(), 1);
        assert_eq!((-1).nanoseconds().whole_nanoseconds(), -1);
    }

    #[test]
    fn subsec_nanoseconds() {
        assert_eq!(1.0000004.seconds().subsec_nanoseconds(), 400);
        assert_eq!((-1.0000004).seconds().subsec_nanoseconds(), -400);
    }

    #[test]
    #[allow(deprecated)]
    fn checked_add() {
        assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
        assert_eq!(Duration::max_value().checked_add(1.nanoseconds()), None);
        assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
    }

    #[test]
    #[allow(deprecated)]
    fn checked_sub() {
        assert_eq!(5.seconds().checked_sub(5.seconds()), Some(0.seconds()));
        assert_eq!(Duration::min_value().checked_sub(1.nanoseconds()), None);
        assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
    }

    #[test]
    #[allow(deprecated)]
    fn checked_mul() {
        assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
        assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
        assert_eq!(5.seconds().checked_mul(0), Some(Duration::zero()));
        assert_eq!(Duration::max_value().checked_mul(2), None);
        assert_eq!(Duration::min_value().checked_mul(2), None);
    }

    #[test]
    fn checked_div() {
        assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
        assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
        assert_eq!(1.seconds().checked_div(0), None);
    }

    #[test]
    #[cfg(std)]
    fn time_fn() {
        let (time, value) = Duration::time_fn(|| {
            std::thread::sleep(100.std_milliseconds());
            0
        });

        assert!(time >= 100.milliseconds());
        assert_eq!(value, 0);
    }

    #[test]
    fn try_from_std_duration() {
        assert_eq!(Duration::try_from(0.std_seconds()), Ok(0.seconds()));
        assert_eq!(Duration::try_from(1.std_seconds()), Ok(1.seconds()));
    }

    #[test]
    fn try_to_std_duration() {
        assert_eq!(StdDuration::try_from(0.seconds()), Ok(0.std_seconds()));
        assert_eq!(StdDuration::try_from(1.seconds()), Ok(1.std_seconds()));
        assert!(StdDuration::try_from((-1).seconds()).is_err());
    }

    #[test]
    fn add() {
        assert_eq!(1.seconds() + 1.seconds(), 2.seconds());
        assert_eq!(500.milliseconds() + 500.milliseconds(), 1.seconds());
        assert_eq!(1.seconds() + (-1).seconds(), 0.seconds());
    }

    #[test]
    fn add_std() {
        assert_eq!(1.seconds() + 1.std_seconds(), 2.seconds());
        assert_eq!(500.milliseconds() + 500.std_milliseconds(), 1.seconds());
        assert_eq!((-1).seconds() + 1.std_seconds(), 0.seconds());
    }

    #[test]
    fn std_add() {
        assert_eq!(1.std_seconds() + 1.seconds(), 2.seconds());
        assert_eq!(500.std_milliseconds() + 500.milliseconds(), 1.seconds());
        assert_eq!(1.std_seconds() + (-1).seconds(), 0.seconds());
    }

    #[test]
    fn add_assign() {
        let mut duration = 1.seconds();
        duration += 1.seconds();
        assert_eq!(duration, 2.seconds());

        let mut duration = 500.milliseconds();
        duration += 500.milliseconds();
        assert_eq!(duration, 1.seconds());

        let mut duration = 1.seconds();
        duration += (-1).seconds();
        assert_eq!(duration, 0.seconds());
    }

    #[test]
    fn add_assign_std() {
        let mut duration = 1.seconds();
        duration += 1.std_seconds();
        assert_eq!(duration, 2.seconds());

        let mut duration = 500.milliseconds();
        duration += 500.std_milliseconds();
        assert_eq!(duration, 1.seconds());

        let mut duration = (-1).seconds();
        duration += 1.std_seconds();
        assert_eq!(duration, 0.seconds());
    }

    #[test]
    fn neg() {
        assert_eq!(-(1.seconds()), (-1).seconds());
        assert_eq!(-(-1).seconds(), 1.seconds());
        assert_eq!(-(0.seconds()), 0.seconds());
    }

    #[test]
    fn sub() {
        assert_eq!(1.seconds() - 1.seconds(), 0.seconds());
        assert_eq!(1_500.milliseconds() - 500.milliseconds(), 1.seconds());
        assert_eq!(1.seconds() - (-1).seconds(), 2.seconds());
    }

    #[test]
    fn sub_std() {
        assert_eq!(1.seconds() - 1.std_seconds(), 0.seconds());
        assert_eq!(1_500.milliseconds() - 500.std_milliseconds(), 1.seconds());
        assert_eq!((-1).seconds() - 1.std_seconds(), (-2).seconds());
    }

    #[test]
    fn std_sub() {
        assert_eq!(1.std_seconds() - 1.seconds(), 0.seconds());
        assert_eq!(1_500.std_milliseconds() - 500.milliseconds(), 1.seconds());
        assert_eq!(1.std_seconds() - (-1).seconds(), 2.seconds());
    }

    #[test]
    fn sub_assign() {
        let mut duration = 1.seconds();
        duration -= 1.seconds();
        assert_eq!(duration, 0.seconds());

        let mut duration = 1_500.milliseconds();
        duration -= 500.milliseconds();
        assert_eq!(duration, 1.seconds());

        let mut duration = 1.seconds();
        duration -= (-1).seconds();
        assert_eq!(duration, 2.seconds());
    }

    #[test]
    fn sub_assign_std() {
        let mut duration = 1.seconds();
        duration -= 1.std_seconds();
        assert_eq!(duration, 0.seconds());

        let mut duration = 1_500.milliseconds();
        duration -= 500.std_milliseconds();
        assert_eq!(duration, 1.seconds());

        let mut duration = (-1).seconds();
        duration -= 1.std_seconds();
        assert_eq!(duration, (-2).seconds());
    }

    #[test]
    fn std_sub_assign() {
        let mut duration = 1.std_seconds();
        duration -= 1.seconds();
        assert_eq!(duration, 0.seconds());

        let mut duration = 1_500.std_milliseconds();
        duration -= 500.milliseconds();
        assert_eq!(duration, 1.seconds());

        #[cfg(std)]
        {
            let mut duration = 1.std_seconds();
            assert_panics!(duration -= 2.seconds());
        }
    }

    #[test]
    fn mul_int() {
        assert_eq!(1.seconds() * 2, 2.seconds());
        assert_eq!(1.seconds() * -2, (-2).seconds());
    }

    #[test]
    fn mul_int_assign() {
        let mut duration = 1.seconds();
        duration *= 2;
        assert_eq!(duration, 2.seconds());

        let mut duration = 1.seconds();
        duration *= -2;
        assert_eq!(duration, (-2).seconds());
    }

    #[test]
    fn int_mul() {
        assert_eq!(2 * 1.seconds(), 2.seconds());
        assert_eq!(-2 * 1.seconds(), (-2).seconds());
    }

    #[test]
    fn div_int() {
        assert_eq!(1.seconds() / 2, 500.milliseconds());
        assert_eq!(1.seconds() / -2, (-500).milliseconds());
    }

    #[test]
    fn div_int_assign() {
        let mut duration = 1.seconds();
        duration /= 2;
        assert_eq!(duration, 500.milliseconds());

        let mut duration = 1.seconds();
        duration /= -2;
        assert_eq!(duration, (-500).milliseconds());
    }

    #[test]
    fn mul_float() {
        assert_eq!(1.seconds() * 1.5_f32, 1_500.milliseconds());
        assert_eq!(1.seconds() * 2.5_f32, 2_500.milliseconds());
        assert_eq!(1.seconds() * -1.5_f32, (-1_500).milliseconds());
        assert_eq!(1.seconds() * 0_f32, 0.seconds());

        assert_eq!(1.seconds() * 1.5_f64, 1_500.milliseconds());
        assert_eq!(1.seconds() * 2.5_f64, 2_500.milliseconds());
        assert_eq!(1.seconds() * -1.5_f64, (-1_500).milliseconds());
        assert_eq!(1.seconds() * 0_f64, 0.seconds());
    }

    #[test]
    fn float_mul() {
        assert_eq!(1.5_f32 * 1.seconds(), 1_500.milliseconds());
        assert_eq!(2.5_f32 * 1.seconds(), 2_500.milliseconds());
        assert_eq!(-1.5_f32 * 1.seconds(), (-1_500).milliseconds());
        assert_eq!(0_f32 * 1.seconds(), 0.seconds());

        assert_eq!(1.5_f64 * 1.seconds(), 1_500.milliseconds());
        assert_eq!(2.5_f64 * 1.seconds(), 2_500.milliseconds());
        assert_eq!(-1.5_f64 * 1.seconds(), (-1_500).milliseconds());
        assert_eq!(0_f64 * 1.seconds(), 0.seconds());
    }

    #[test]
    fn mul_float_assign() {
        let mut duration = 1.seconds();
        duration *= 1.5_f32;
        assert_eq!(duration, 1_500.milliseconds());

        let mut duration = 1.seconds();
        duration *= 2.5_f32;
        assert_eq!(duration, 2_500.milliseconds());

        let mut duration = 1.seconds();
        duration *= -1.5_f32;
        assert_eq!(duration, (-1_500).milliseconds());

        let mut duration = 1.seconds();
        duration *= 0_f32;
        assert_eq!(duration, 0.seconds());

        let mut duration = 1.seconds();
        duration *= 1.5_f64;
        assert_eq!(duration, 1_500.milliseconds());

        let mut duration = 1.seconds();
        duration *= 2.5_f64;
        assert_eq!(duration, 2_500.milliseconds());

        let mut duration = 1.seconds();
        duration *= -1.5_f64;
        assert_eq!(duration, (-1_500).milliseconds());

        let mut duration = 1.seconds();
        duration *= 0_f64;
        assert_eq!(duration, 0.seconds());
    }

    #[test]
    fn div_float() {
        assert_eq!(1.seconds() / 1_f32, 1.seconds());
        assert_eq!(1.seconds() / 2_f32, 500.milliseconds());
        assert_eq!(1.seconds() / -1_f32, (-1).seconds());

        assert_eq!(1.seconds() / 1_f64, 1.seconds());
        assert_eq!(1.seconds() / 2_f64, 500.milliseconds());
        assert_eq!(1.seconds() / -1_f64, (-1).seconds());
    }

    #[test]
    fn div_float_assign() {
        let mut duration = 1.seconds();
        duration /= 1_f32;
        assert_eq!(duration, 1.seconds());

        let mut duration = 1.seconds();
        duration /= 2_f32;
        assert_eq!(duration, 500.milliseconds());

        let mut duration = 1.seconds();
        duration /= -1_f32;
        assert_eq!(duration, (-1).seconds());

        let mut duration = 1.seconds();
        duration /= 1_f64;
        assert_eq!(duration, 1.seconds());

        let mut duration = 1.seconds();
        duration /= 2_f64;
        assert_eq!(duration, 500.milliseconds());

        let mut duration = 1.seconds();
        duration /= -1_f64;
        assert_eq!(duration, (-1).seconds());
    }

    #[test]
    fn partial_eq() {
        assert_eq!(1.seconds(), 1.seconds());
        assert_eq!(0.seconds(), 0.seconds());
        assert_eq!((-1).seconds(), (-1).seconds());
        assert_ne!(1.minutes(), (-1).minutes());
        assert_ne!(40.seconds(), 1.minutes());
    }

    #[test]
    fn partial_eq_std() {
        assert_eq!(1.seconds(), 1.std_seconds());
        assert_eq!(0.seconds(), 0.std_seconds());
        assert_ne!((-1).seconds(), 1.std_seconds());
        assert_ne!((-1).minutes(), 1.std_minutes());
        assert_ne!(40.seconds(), 1.std_minutes());
    }

    #[test]
    fn std_partial_eq() {
        assert_eq!(1.std_seconds(), 1.seconds());
        assert_eq!(0.std_seconds(), 0.seconds());
        assert_ne!(1.std_seconds(), (-1).seconds());
        assert_ne!(1.std_minutes(), (-1).minutes());
        assert_ne!(40.std_seconds(), 1.minutes());
    }

    #[test]
    fn partial_ord() {
        assert_eq!(0.seconds().partial_cmp(&0.seconds()), Some(Equal));
        assert_eq!(1.seconds().partial_cmp(&0.seconds()), Some(Greater));
        assert_eq!(1.seconds().partial_cmp(&(-1).seconds()), Some(Greater));
        assert_eq!((-1).seconds().partial_cmp(&1.seconds()), Some(Less));
        assert_eq!(0.seconds().partial_cmp(&(-1).seconds()), Some(Greater));
        assert_eq!(0.seconds().partial_cmp(&1.seconds()), Some(Less));
        assert_eq!((-1).seconds().partial_cmp(&0.seconds()), Some(Less));
        assert_eq!(1.minutes().partial_cmp(&1.seconds()), Some(Greater));
        assert_eq!((-1).minutes().partial_cmp(&(-1).seconds()), Some(Less));
    }

    #[test]
    fn partial_ord_std() {
        assert_eq!(0.seconds().partial_cmp(&0.std_seconds()), Some(Equal));
        assert_eq!(1.seconds().partial_cmp(&0.std_seconds()), Some(Greater));
        assert_eq!((-1).seconds().partial_cmp(&1.std_seconds()), Some(Less));
        assert_eq!(0.seconds().partial_cmp(&1.std_seconds()), Some(Less));
        assert_eq!((-1).seconds().partial_cmp(&0.std_seconds()), Some(Less));
        assert_eq!(1.minutes().partial_cmp(&1.std_seconds()), Some(Greater));
    }

    #[test]
    fn std_partial_ord() {
        assert_eq!(0.std_seconds().partial_cmp(&0.seconds()), Some(Equal));
        assert_eq!(1.std_seconds().partial_cmp(&0.seconds()), Some(Greater));
        assert_eq!(1.std_seconds().partial_cmp(&(-1).seconds()), Some(Greater));
        assert_eq!(0.std_seconds().partial_cmp(&(-1).seconds()), Some(Greater));
        assert_eq!(0.std_seconds().partial_cmp(&1.seconds()), Some(Less));
        assert_eq!(1.std_minutes().partial_cmp(&1.seconds()), Some(Greater));
    }

    #[test]
    fn ord() {
        assert_eq!(0.seconds(), 0.seconds());
        assert!(1.seconds() > 0.seconds());
        assert!(1.seconds() > (-1).seconds());
        assert!((-1).seconds() < 1.seconds());
        assert!(0.seconds() > (-1).seconds());
        assert!(0.seconds() < 1.seconds());
        assert!((-1).seconds() < 0.seconds());
        assert!(1.minutes() > 1.seconds());
        assert!((-1).minutes() < (-1).seconds());
    }

    #[test]
    fn arithmetic_regression() {
        let added = 1.6.seconds() + 1.6.seconds();
        assert_eq!(added.whole_seconds(), 3);
        assert_eq!(added.subsec_milliseconds(), 200);

        let subtracted = 1.6.seconds() - (-1.6).seconds();
        assert_eq!(subtracted.whole_seconds(), 3);
        assert_eq!(subtracted.subsec_milliseconds(), 200);
    }
}
