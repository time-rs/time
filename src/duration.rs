#[cfg(feature = "std")]
use crate::Instant;
use crate::Sign::{self, Negative, Positive, Zero};
use crate::{NumberExt, OutOfRangeError};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::convert::{From, TryFrom};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use core::time::Duration as StdDuration;
#[cfg(feature = "deprecated")]
use log::warn;

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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct Duration {
    /// Is the `Duration` positive, negative, or zero?
    sign: Sign,

    /// Inner, unsigned representation of the duration.
    std: StdDuration,
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
    /// Equivalent to `Duration::seconds(0)`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::zero(), Duration::seconds(0));
    /// ```
    #[inline(always)]
    pub const fn zero() -> Self {
        Self {
            sign: Zero,
            std: StdDuration::from_secs(0),
        }
    }

    /// Equivalent to `Duration::nanoseconds(1)`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::nanosecond(), Duration::nanoseconds(1));
    /// ```
    #[inline(always)]
    pub const fn nanosecond() -> Self {
        Self::positive(StdDuration::from_nanos(1))
    }

    /// Equivalent to `Duration::microseconds(1)`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::microsecond(), Duration::microseconds(1));
    /// ```
    #[inline(always)]
    pub const fn microsecond() -> Self {
        Self::positive(StdDuration::from_micros(1))
    }

    /// Equivalent to `Duration::milliseconds(1)`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::millisecond(), Duration::milliseconds(1));
    /// ```
    #[inline(always)]
    pub const fn millisecond() -> Self {
        Self::positive(StdDuration::from_millis(1))
    }

    /// Equivalent to `Duration::seconds(1)`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::second(), Duration::seconds(1));
    /// ```
    #[inline(always)]
    pub const fn second() -> Self {
        Self::positive(StdDuration::from_secs(1))
    }

    /// Equivalent to `Duration::minutes(1)`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::minute(), Duration::minutes(1));
    /// ```
    #[inline(always)]
    pub const fn minute() -> Self {
        Self::positive(StdDuration::from_secs(SECONDS_PER_MINUTE as u64))
    }

    /// Equivalent to `Duration::hours(1)`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::hour(), Duration::hours(1));
    /// ```
    #[inline(always)]
    pub const fn hour() -> Self {
        Self::positive(StdDuration::from_secs(SECONDS_PER_HOUR as u64))
    }

    /// Equivalent to `Duration::days(1)`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::day(), Duration::days(1));
    /// ```
    #[inline(always)]
    pub const fn day() -> Self {
        Self::positive(StdDuration::from_secs(SECONDS_PER_DAY as u64))
    }

    /// Equivalent to `Duration::weeks(1)`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::week(), Duration::weeks(1));
    /// ```
    #[inline(always)]
    pub const fn week() -> Self {
        Self::positive(StdDuration::from_secs(SECONDS_PER_WEEK as u64))
    }

    /// The maximum possible duration. Adding any positive duration to this will
    /// cause an overflow.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::max_value().whole_nanoseconds(), 18_446_744_073_709_551_615_999_999_999);
    /// ```
    #[inline(always)]
    #[deprecated(
        since = "0.2.0",
        note = "If you have use case for this, please file an issue on the repository."
    )]
    pub fn max_value() -> Self {
        Self::positive(StdDuration::new(u64::max_value(), 999_999_999))
    }

    /// The minimum possible duration. Adding any negative duration to this will
    /// cause an overflow.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::min_value().whole_nanoseconds(), -18_446_744_073_709_551_615_999_999_999);
    /// ```
    #[inline(always)]
    #[deprecated(
        since = "0.2.0",
        note = "If you have use case for this, please file an issue on the repository."
    )]
    pub fn min_value() -> Self {
        Self::negative(StdDuration::new(u64::max_value(), 999_999_999))
    }

    /// Check if a duration is exactly zero.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert!(Duration::seconds(0).is_zero());
    /// assert!(!Duration::nanoseconds(1).is_zero());
    /// ```
    #[inline(always)]
    pub const fn is_zero(&self) -> bool {
        (self.std.as_secs() == 0) & (self.std.subsec_nanos() == 0)
    }

    /// Check if a duration is negative.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert!(Duration::seconds(-1).is_negative());
    /// assert!(!Duration::seconds(0).is_negative());
    /// assert!(!Duration::seconds(1).is_negative());
    /// ```
    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        self.sign.is_negative() & !self.is_zero()
    }

    /// Check if a duration is positive.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert!(Duration::seconds(1).is_positive());
    /// assert!(!Duration::seconds(0).is_positive());
    /// assert!(!Duration::seconds(-1).is_positive());
    /// ```
    #[inline(always)]
    pub const fn is_positive(&self) -> bool {
        self.sign.is_positive() & !self.is_zero()
    }

    /// Retrieve the sign of the duration.
    ///
    /// ```rust
    /// # use time::{Duration, Sign};
    /// assert_eq!(Duration::seconds(1).sign(), Sign::Positive);
    /// assert_eq!(Duration::seconds(-1).sign(), Sign::Negative);
    /// assert_eq!(Duration::zero().sign(), Sign::Zero);
    /// ```
    #[inline(always)]
    pub const fn sign(&self) -> Sign {
        self.sign
    }

    /// Get the absolute value of the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds(1).abs(), Duration::seconds(1));
    /// assert_eq!(Duration::zero().abs(), Duration::zero());
    /// assert_eq!(Duration::seconds(-1).abs(), Duration::seconds(1));
    /// ```
    #[inline(always)]
    pub fn abs(self) -> Self {
        match self.sign() {
            Zero => Self::zero(),
            Positive | Negative => Self::positive(self.std),
        }
    }

    /// Create a new `Duration` with the provided seconds and nanoseconds. If
    /// nanoseconds is at least 10^9, it will wrap to the number of seconds.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::new(1, 0), Duration::seconds(1));
    /// assert_eq!(Duration::new(-1, 0), Duration::seconds(-1));
    /// assert_eq!(Duration::new(1, 2_000_000_000), Duration::seconds(3));
    #[inline(always)]
    pub fn new(seconds: i64, nanoseconds: u32) -> Self {
        Self {
            sign: (seconds * 1_000_000_000 + nanoseconds as i64).sign(),
            std: StdDuration::new(seconds.abs() as u64, nanoseconds),
        }
    }

    /// Create a new positive `Duration` from a `std::time::Duration`.
    #[inline(always)]
    pub(crate) const fn positive(std: StdDuration) -> Self {
        Self {
            sign: Positive,
            std,
        }
    }

    /// Create a new negative `Duration` from a `std::time::Duration`.
    #[inline(always)]
    pub(crate) const fn negative(std: StdDuration) -> Self {
        Self {
            sign: Negative,
            std,
        }
    }

    /// Create a new `Duration` with the given number of weeks. Equivalent to
    /// `Duration::seconds(weeks * 604_800);
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::weeks(1), Duration::seconds(604_800));
    /// ```
    #[inline(always)]
    pub fn weeks(weeks: i64) -> Self {
        Self::seconds(weeks * SECONDS_PER_WEEK)
    }

    /// Get the number of whole weeks in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::weeks(1).whole_weeks(), 1);
    /// assert_eq!(Duration::weeks(-1).whole_weeks(), -1);
    /// assert_eq!(Duration::days(6).whole_weeks(), 0);
    /// assert_eq!(Duration::days(-6).whole_weeks(), 0);
    /// ```
    #[inline(always)]
    pub fn whole_weeks(&self) -> i64 {
        self.whole_seconds() / SECONDS_PER_WEEK
    }

    /// Create a new `Duration` with the given number of days. Equivalent to
    /// `Duration::seconds(days * 86_400);
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::days(1), Duration::seconds(86_400));
    /// ```
    #[inline(always)]
    pub fn days(days: i64) -> Self {
        Self::seconds(days * SECONDS_PER_DAY)
    }

    /// Get the number of whole days in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::days(1).whole_days(), 1);
    /// assert_eq!(Duration::days(-1).whole_days(), -1);
    /// assert_eq!(Duration::hours(23).whole_days(), 0);
    /// assert_eq!(Duration::hours(-23).whole_days(), 0);
    /// ```
    #[inline(always)]
    pub fn whole_days(&self) -> i64 {
        self.whole_seconds() / SECONDS_PER_DAY
    }

    /// Create a new `Duration` with the given number of hours. Equivalent to
    /// `Duration::seconds(hours * 3_600);
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::hours(1), Duration::seconds(3_600));
    /// ```
    #[inline(always)]
    pub fn hours(hours: i64) -> Self {
        Self::seconds(hours * SECONDS_PER_HOUR)
    }

    /// Get the number of whole hours in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::hours(1).whole_hours(), 1);
    /// assert_eq!(Duration::hours(-1).whole_hours(), -1);
    /// assert_eq!(Duration::minutes(59).whole_hours(), 0);
    /// assert_eq!(Duration::minutes(-59).whole_hours(), 0);
    /// ```
    #[inline(always)]
    pub fn whole_hours(&self) -> i64 {
        self.whole_seconds() / SECONDS_PER_HOUR
    }

    /// Create a new `Duration` with the given number of minutes. Equivalent to
    /// `Duration::seconds(minutes * 60);
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::minutes(1), Duration::seconds(60));
    /// ```
    #[inline(always)]
    pub fn minutes(minutes: i64) -> Self {
        Self::seconds(minutes * SECONDS_PER_MINUTE)
    }

    /// Get the number of whole minutes in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::minutes(1).whole_minutes(), 1);
    /// assert_eq!(Duration::minutes(-1).whole_minutes(), -1);
    /// assert_eq!(Duration::seconds(59).whole_minutes(), 0);
    /// assert_eq!(Duration::seconds(-59).whole_minutes(), 0);
    /// ```
    #[inline(always)]
    pub fn whole_minutes(&self) -> i64 {
        self.whole_seconds() / SECONDS_PER_MINUTE
    }

    /// Create a new `Duration` with the given number of seconds.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds(1), Duration::milliseconds(1_000));
    /// ```
    #[inline(always)]
    pub fn seconds(seconds: i64) -> Self {
        Self {
            sign: seconds.sign(),
            std: StdDuration::from_secs(seconds.abs() as u64),
        }
    }

    /// Get the number of whole seconds in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds(1).whole_seconds(), 1);
    /// assert_eq!(Duration::seconds(-1).whole_seconds(), -1);
    /// assert_eq!(Duration::minutes(1).whole_seconds(), 60);
    /// assert_eq!(Duration::minutes(-1).whole_seconds(), -60);
    /// ```
    #[inline(always)]
    pub fn whole_seconds(&self) -> i64 {
        self.sign * self.std.as_secs() as i64
    }

    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f64`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds_f64(0.5), Duration::milliseconds(500));
    /// assert_eq!(Duration::seconds_f64(-0.5), Duration::milliseconds(-500));
    /// ```
    #[inline(always)]
    pub fn seconds_f64(seconds: f64) -> Self {
        Self {
            sign: seconds.sign(),
            std: StdDuration::from_secs_f64(seconds.abs()),
        }
    }

    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::milliseconds(1_500).as_seconds_f64(), 1.5);
    /// assert_eq!(Duration::milliseconds(-1_500).as_seconds_f64(), -1.5);
    /// ```
    #[inline(always)]
    pub fn as_seconds_f64(&self) -> f64 {
        self.sign * self.std.as_secs_f64()
    }

    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f32`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds_f32(0.5), Duration::milliseconds(500));
    /// assert_eq!(Duration::seconds_f32(-0.5), Duration::milliseconds(-500));
    /// ```
    #[inline(always)]
    pub fn seconds_f32(seconds: f32) -> Self {
        Self {
            sign: seconds.sign(),
            std: StdDuration::from_secs_f32(seconds.abs()),
        }
    }

    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::milliseconds(1_500).as_seconds_f32(), 1.5);
    /// assert_eq!(Duration::milliseconds(-1_500).as_seconds_f32(), -1.5);
    /// ```
    #[inline(always)]
    pub fn as_seconds_f32(&self) -> f32 {
        self.sign * self.std.as_secs_f32()
    }

    /// Create a new `Duration` with the given number of milliseconds.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::milliseconds(1), Duration::seconds(1) / 1_000);
    /// assert_eq!(Duration::milliseconds(-1), Duration::seconds(-1) / 1_000);
    /// ```
    #[inline(always)]
    pub fn milliseconds(milliseconds: i64) -> Self {
        Self {
            sign: milliseconds.sign(),
            std: StdDuration::from_millis(milliseconds.abs() as u64),
        }
    }

    /// Get the number of whole milliseconds in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds(1).whole_milliseconds(), 1_000);
    /// assert_eq!(Duration::seconds(-1).whole_milliseconds(), -1_000);
    /// assert_eq!(Duration::milliseconds(1).whole_milliseconds(), 1);
    /// assert_eq!(Duration::milliseconds(-1).whole_milliseconds(), -1);
    /// ```
    #[inline(always)]
    pub fn whole_milliseconds(&self) -> i128 {
        self.sign * self.std.as_millis() as i128
    }

    /// Get the number of milliseconds past the number of whole seconds.
    ///
    /// Always in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::milliseconds(1_400).subsec_milliseconds(), 400);
    /// assert_eq!(Duration::milliseconds(-1_400).subsec_milliseconds(), 400);
    /// ```
    // Allow the lint, as the value is guaranteed to be less than 1000.
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn subsec_milliseconds(&self) -> u16 {
        self.std.subsec_millis() as u16
    }

    /// Create a new `Duration` with the given number of microseconds.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::microseconds(1), Duration::seconds(1) / 1_000_000);
    /// assert_eq!(Duration::microseconds(-1), Duration::seconds(-1) / 1_000_000);
    /// ```
    #[inline(always)]
    pub fn microseconds(microseconds: i64) -> Self {
        Self {
            sign: microseconds.sign(),
            std: StdDuration::from_micros(microseconds.abs() as u64),
        }
    }

    /// Get the number of whole microseconds in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::milliseconds(1).whole_microseconds(), 1_000);
    /// assert_eq!(Duration::milliseconds(-1).whole_microseconds(), -1_000);
    /// assert_eq!(Duration::microseconds(1).whole_microseconds(), 1);
    /// assert_eq!(Duration::microseconds(-1).whole_microseconds(), -1);
    /// ```
    #[inline(always)]
    pub fn whole_microseconds(&self) -> i128 {
        self.sign * self.std.as_micros() as i128
    }

    /// Get the number of microseconds past the number of whole seconds.
    ///
    /// Always in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::microseconds(1_000_400).subsec_microseconds(), 400);
    /// assert_eq!(Duration::microseconds(-1_000_400).subsec_microseconds(), 400);
    /// ```
    #[inline(always)]
    pub const fn subsec_microseconds(&self) -> u32 {
        self.std.subsec_micros()
    }

    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::nanoseconds(1), Duration::seconds(1) / 1_000_000_000);
    /// assert_eq!(Duration::nanoseconds(-1), Duration::seconds(-1) / 1_000_000_000);
    /// ```
    #[inline(always)]
    pub fn nanoseconds(nanoseconds: i64) -> Self {
        Self {
            sign: nanoseconds.sign(),
            std: StdDuration::from_nanos(nanoseconds.abs() as u64),
        }
    }

    /// Get the number of nanoseconds in the duration.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::microseconds(1).whole_nanoseconds(), 1_000);
    /// assert_eq!(Duration::microseconds(-1).whole_nanoseconds(), -1_000);
    /// assert_eq!(Duration::nanoseconds(1).whole_nanoseconds(), 1);
    /// assert_eq!(Duration::nanoseconds(-1).whole_nanoseconds(), -1);
    /// ```
    #[inline(always)]
    pub fn whole_nanoseconds(&self) -> i128 {
        self.sign * self.std.as_nanos() as i128
    }

    /// Get the number of nanoseconds past the number of whole seconds.
    ///
    /// Always in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::nanoseconds(1_000_000_400).subsec_nanoseconds(), 400);
    /// assert_eq!(Duration::nanoseconds(-1_000_000_400).subsec_nanoseconds(), 400);
    /// ```
    #[inline(always)]
    pub const fn subsec_nanoseconds(&self) -> u32 {
        self.std.subsec_nanos()
    }

    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds(5).checked_add(Duration::seconds(5)), Some(Duration::seconds(10)));
    /// assert_eq!(Duration::max_value().checked_add(Duration::nanosecond()), None);
    /// assert_eq!(Duration::seconds(-5).checked_add(Duration::seconds(5)), Some(Duration::zero()));
    /// ```
    #[inline]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        match (self.sign, rhs.sign) {
            (_, Zero) => Some(self),
            (Zero, _) => Some(rhs),
            (Positive, Positive) => Some(Self::positive(self.std.checked_add(rhs.std)?)),
            (Negative, Negative) => Some(Self::negative(self.std.checked_add(rhs.std)?)),
            (Positive, Negative) | (Negative, Positive) => {
                let (min, max) = if self.std < rhs.std {
                    (self, rhs)
                } else {
                    (rhs, self)
                };

                Some(Self {
                    sign: if max.std == min.std { Zero } else { max.sign },
                    std: max.std - min.std,
                })
            }
        }
    }

    /// Computes `self - rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds(5).checked_sub(Duration::seconds(5)), Some(Duration::zero()));
    /// assert_eq!(Duration::min_value().checked_sub(Duration::nanosecond()), None);
    /// assert_eq!(Duration::seconds(5).checked_sub(Duration::seconds(10)), Some(Duration::seconds(-5)));
    /// ```
    #[inline(always)]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_add(-rhs)
    }

    /// Computes `self * rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds(5).checked_mul(2), Some(Duration::seconds(10)));
    /// assert_eq!(Duration::seconds(5).checked_mul(-2), Some(Duration::seconds(-10)));
    /// assert_eq!(Duration::seconds(5).checked_mul(0), Some(Duration::zero()));
    /// assert_eq!(Duration::max_value().checked_mul(2), None);
    /// assert_eq!(Duration::min_value().checked_mul(2), None);
    /// ```
    #[inline(always)]
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        Some(Self {
            sign: self.sign * rhs.sign(),
            std: self.std.checked_mul(rhs.abs() as u32)?,
        })
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0`.
    ///
    /// ```rust
    /// # use time::Duration;
    /// assert_eq!(Duration::seconds(10).checked_div(2), Some(Duration::seconds(5)));
    /// assert_eq!(Duration::seconds(10).checked_div(-2), Some(Duration::seconds(-5)));
    /// assert_eq!(Duration::seconds(1).checked_div(0), None);
    #[inline(always)]
    pub fn checked_div(self, rhs: i32) -> Option<Self> {
        Some(Self {
            sign: self.sign * rhs.sign(),
            std: self.std.checked_div(rhs.abs() as u32)?,
        })
    }

    /// Runs a closure, returning the duration of time it took to run. The
    /// return value of the closure is provided in the second half of the tuple.
    ///
    /// This method is not available with `#![no_std]`.
    #[inline(always)]
    #[cfg(feature = "std")]
    pub fn time_fn<T: FnOnce() -> U, U>(f: T) -> (Self, U) {
        let start = Instant::now();
        let return_value = f();
        let end = Instant::now();

        (end - start, return_value)
    }
}

/// Functions that have been renamed or had signatures changed since v0.1. As
/// such, they are deprecated.
#[cfg(feature = "deprecated")]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items)]
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

    #[inline(always)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_seconds` function")]
    pub fn num_seconds(&self) -> i64 {
        self.whole_seconds()
    }

    /// [`whole_milliseconds()`] returns an `i128`, rather than panicking on
    /// overflow. To avoid panicking, this method currently limits the value to
    /// the range `i64::min_value()..=i64::max_value()`. A warning will be
    /// printed at runtime if this occurs.
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_milliseconds` function")]
    pub fn num_milliseconds(&self) -> i64 {
        let mut millis = self.whole_milliseconds();

        if millis > i64::max_value() as i128 {
            warn!(
                "The number of milliseconds exceeds `i64::max_value()`. \
                 Limiting to that value. Use the `whole_milliseconds` to \
                 return an i128."
            );
            millis = i64::max_value() as i128;
        }

        if millis < i64::min_value() as i128 {
            warn!(
                "The number of milliseconds exceeds `i64::min_value()`. \
                 Limiting to that value. Use the `whole_milliseconds` to \
                 return an i128."
            );
            millis = i64::min_value() as i128;
        }

        millis as i64
    }

    /// [`whole_microseconds()`] returns an `i128` rather than returning `None`
    /// on `i64` overflow.
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

    /// [`whole_nanoseconds()`] returns an `i128` rather than returning `None`
    /// on `i64` overflow.
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
    #[cfg(feature = "std")]
    #[deprecated(since = "0.2.0", note = "Use the `time_fn` function")]
    pub fn span<F: FnOnce()>(f: F) -> Self {
        Self::time_fn(f).0
    }

    #[inline(always)]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.0",
        note = "Use `Duration::from(value)` or `value.into()`"
    )]
    pub fn from_std(std: StdDuration) -> Result<Self, OutOfRangeError> {
        Ok(std.into())
    }

    #[inline(always)]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.0",
        note = "Use `std::time::Duration::try_from(value)` or `value.try_into()`"
    )]
    pub fn to_std(&self) -> Result<StdDuration, OutOfRangeError> {
        if self.sign.is_negative() {
            Err(OutOfRangeError::new())
        } else {
            Ok(self.std)
        }
    }
}

impl From<StdDuration> for Duration {
    #[inline(always)]
    fn from(original: StdDuration) -> Self {
        Self {
            sign: original.as_nanos().sign(),
            std: original,
        }
    }
}

impl TryFrom<Duration> for StdDuration {
    type Error = OutOfRangeError;

    #[inline(always)]
    fn try_from(duration: Duration) -> Result<Self, OutOfRangeError> {
        if duration.sign.is_negative() {
            Err(OutOfRangeError::new())
        } else {
            Ok(duration.std)
        }
    }
}

impl Add for Duration {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let secs = self.whole_seconds() + rhs.whole_seconds();
        let nanos = self.subsec_nanoseconds() + rhs.subsec_nanoseconds();

        if nanos >= 1_000_000_000 {
            Self::new(secs + 1, nanos - 1_000_000_000)
        } else {
            Self::new(secs, nanos)
        }
    }
}

impl Add<StdDuration> for Duration {
    type Output = Self;

    #[inline(always)]
    fn add(self, std_duration: StdDuration) -> Self::Output {
        self + Self::from(std_duration)
    }
}

impl Add<Duration> for StdDuration {
    type Output = Duration;

    #[inline(always)]
    fn add(self, duration: Duration) -> Self::Output {
        Duration::from(self) + duration
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
        let secs = self.whole_seconds() - rhs.whole_seconds();
        let nanos = self.subsec_nanoseconds() as i32 - rhs.subsec_nanoseconds() as i32;

        if nanos < 0 {
            Self::new(secs - 1, (nanos + 1_000_000_000) as u32)
        } else {
            Self::new(secs, nanos as u32)
        }
    }
}

impl Sub<StdDuration> for Duration {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: StdDuration) -> Self::Output {
        self - Self::from(rhs)
    }
}

impl Sub<Duration> for StdDuration {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: Duration) -> Self::Output {
        Duration::from(self) - rhs
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
        use core::convert::TryInto;

        *self = (*self - rhs).try_into().expect(
            "Cannot represent a resulting duration in std. \
             Try `let x = x - rhs;`, which will change the type.",
        );
    }
}

macro_rules! duration_mul_div {
    ($($type:ty),+) => {
        $(
            impl Mul<$type> for Duration {
                type Output = Self;

                #[inline(always)]
                #[allow(trivial_numeric_casts)]
                fn mul(self, rhs: $type) -> Self::Output {
                    Self {
                        sign: match rhs.cmp(&0) {
                            Equal => return Self::zero(),
                            Greater => self.sign,
                            Less => self.sign.negate(),
                        },
                        std: self.std * rhs.abs() as u32,
                    }
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
                    Self {
                        sign: match rhs.cmp(&0) {
                            Equal => return Self::zero(),
                            Greater => self.sign,
                            Less => self.sign.negate(),
                        },
                        std: self.std / rhs.abs() as u32,
                    }
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
duration_mul_div![i8, i16, i32, u8, u16, u32];

impl Mul<f32> for Duration {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            sign: self.sign * rhs.sign(),
            std: self.std.mul_f32(rhs.abs()),
        }
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
        Self {
            sign: self.sign * rhs.sign(),
            std: self.std.mul_f64(rhs.abs()),
        }
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
        Self {
            sign: self.sign * rhs.sign(),
            std: self.std.div_f32(rhs.abs()),
        }
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
        Self {
            sign: self.sign * rhs.sign(),
            std: self.std.div_f64(rhs.abs()),
        }
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

    // TODO Replace with `self.div_duration_f64(rhs)` when it stabilizes.
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

impl PartialEq for Duration {
    #[inline(always)]
    fn eq(&self, rhs: &Self) -> bool {
        self.sign == rhs.sign && self.std == rhs.std
    }
}

impl PartialEq<StdDuration> for Duration {
    #[inline(always)]
    fn eq(&self, rhs: &StdDuration) -> bool {
        *self == Self::from(*rhs)
    }
}

impl PartialEq<Duration> for StdDuration {
    #[inline(always)]
    fn eq(&self, rhs: &Duration) -> bool {
        Duration::from(*self) == *rhs
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
        self.partial_cmp(&Self::from(*rhs))
    }
}

impl PartialOrd<Duration> for StdDuration {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &Duration) -> Option<Ordering> {
        Duration::from(*self).partial_cmp(rhs)
    }
}

impl Ord for Duration {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self.sign, rhs.sign) {
            (Zero, Zero) => Equal,
            (Positive, Negative) | (Positive, Zero) | (Zero, Negative) => Greater,
            (Negative, Positive) | (Zero, Positive) | (Negative, Zero) => Less,
            (Positive, Positive) => self.std.cmp(&rhs.std),
            (Negative, Negative) => match self.std.cmp(&rhs.std) {
                Greater => Less,
                Equal => Equal,
                Less => Greater,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

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

    #[test]
    fn sign() {
        assert_eq!(1.seconds().sign(), Positive);
        assert_eq!((-1).seconds().sign(), Negative);
        assert_eq!(0.seconds().sign(), Zero);
    }

    #[test]
    fn abs() {
        assert_eq!(1.seconds().abs(), Duration::seconds(1));
        assert_eq!(0.seconds().abs(), Duration::zero());
        assert_eq!((-1).seconds().abs(), Duration::seconds(1));
    }

    #[test]
    fn new() {
        assert_eq!(Duration::new(1, 0), 1.seconds());
        assert_eq!(Duration::new(-1, 0), (-1).seconds());
        assert_eq!(Duration::new(1, 2_000_000_000), 3.seconds());

        assert_eq!(Duration::new(0, 0).sign(), Zero);
        assert_eq!(Duration::new(0, 1_000_000_000).sign(), Positive);
        assert_eq!(Duration::new(-1, 1_000_000_000).sign(), Zero);
        assert_eq!(Duration::new(-2, 1_000_000_000).sign(), Negative);
    }

    #[test]
    fn weeks() {
        assert_eq!(Duration::weeks(1), Duration::seconds(604_800));
        assert_eq!(Duration::weeks(2), Duration::seconds(2 * 604_800));
        assert_eq!(Duration::weeks(-1), Duration::seconds(-604_800));
        assert_eq!(Duration::weeks(-2), Duration::seconds(2 * -604_800));
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
        assert_eq!(Duration::days(1), Duration::seconds(86_400));
        assert_eq!(Duration::days(2), Duration::seconds(2 * 86_400));
        assert_eq!(Duration::days(-1), Duration::seconds(-86_400));
        assert_eq!(Duration::days(-2), Duration::seconds(2 * -86_400));
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
        assert_eq!(Duration::hours(1), Duration::seconds(3_600));
        assert_eq!(Duration::hours(2), Duration::seconds(2 * 3_600));
        assert_eq!(Duration::hours(-1), Duration::seconds(-3_600));
        assert_eq!(Duration::hours(-2), Duration::seconds(2 * -3_600));
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
        assert_eq!(Duration::minutes(1), Duration::seconds(60));
        assert_eq!(Duration::minutes(2), Duration::seconds(2 * 60));
        assert_eq!(Duration::minutes(-1), Duration::seconds(-60));
        assert_eq!(Duration::minutes(-2), Duration::seconds(2 * -60));
    }

    #[test]
    fn whole_minutes() {
        assert_eq!(Duration::minutes(1).whole_minutes(), 1);
        assert_eq!(Duration::minutes(-1).whole_minutes(), -1);
        assert_eq!(Duration::seconds(59).whole_minutes(), 0);
        assert_eq!(Duration::seconds(-59).whole_minutes(), 0);
    }

    #[test]
    fn seconds() {
        assert_eq!(Duration::seconds(1), Duration::milliseconds(1_000));
        assert_eq!(Duration::seconds(2), Duration::milliseconds(2 * 1_000));
        assert_eq!(Duration::seconds(-1), Duration::milliseconds(-1_000));
        assert_eq!(Duration::seconds(-2), Duration::milliseconds(2 * -1_000));
    }

    #[test]
    fn whole_seconds() {
        assert_eq!(Duration::seconds(1).whole_seconds(), 1);
        assert_eq!(Duration::seconds(-1).whole_seconds(), -1);
        assert_eq!(Duration::minutes(1).whole_seconds(), 60);
        assert_eq!(Duration::minutes(-1).whole_seconds(), -60);
    }

    #[test]
    fn seconds_f64() {
        assert_eq!(Duration::seconds_f64(0.5), Duration::milliseconds(500));
        assert_eq!(Duration::seconds_f64(-0.5), Duration::milliseconds(-500));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn as_seconds_f64() {
        assert_eq!(Duration::seconds(1).as_seconds_f64(), 1.0);
        assert_eq!(Duration::seconds(-1).as_seconds_f64(), -1.0);
        assert_eq!(Duration::minutes(1).as_seconds_f64(), 60.0);
        assert_eq!(Duration::minutes(-1).as_seconds_f64(), -60.0);
        assert_eq!(Duration::milliseconds(1_500).as_seconds_f64(), 1.5);
        assert_eq!(Duration::milliseconds(-1_500).as_seconds_f64(), -1.5);
    }

    #[test]
    fn seconds_f32() {
        assert_eq!(Duration::seconds_f32(0.5), Duration::milliseconds(500));
        assert_eq!(Duration::seconds_f32(-0.5), Duration::milliseconds(-500));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn as_seconds_f32() {
        assert_eq!(Duration::seconds(1).as_seconds_f32(), 1.0);
        assert_eq!(Duration::seconds(-1).as_seconds_f32(), -1.0);
        assert_eq!(Duration::minutes(1).as_seconds_f32(), 60.0);
        assert_eq!(Duration::minutes(-1).as_seconds_f32(), -60.0);
        assert_eq!(Duration::milliseconds(1_500).as_seconds_f32(), 1.5);
        assert_eq!(Duration::milliseconds(-1_500).as_seconds_f32(), -1.5);
    }

    #[test]
    fn milliseconds() {
        assert_eq!(Duration::milliseconds(1), Duration::seconds(1) / 1_000);
        assert_eq!(Duration::milliseconds(-1), Duration::seconds(-1) / 1_000);
    }

    #[test]
    fn whole_milliseconds() {
        assert_eq!(Duration::seconds(1).whole_milliseconds(), 1_000);
        assert_eq!(Duration::seconds(-1).whole_milliseconds(), -1_000);
        assert_eq!(Duration::milliseconds(1).whole_milliseconds(), 1);
        assert_eq!(Duration::milliseconds(-1).whole_milliseconds(), -1);
    }

    #[test]
    fn subsec_milliseconds() {
        assert_eq!(Duration::milliseconds(1_400).subsec_milliseconds(), 400);
        assert_eq!(Duration::milliseconds(-1_400).subsec_milliseconds(), 400);
    }

    #[test]
    fn microseconds() {
        assert_eq!(Duration::microseconds(1), Duration::seconds(1) / 1_000_000);
        assert_eq!(
            Duration::microseconds(-1),
            Duration::seconds(-1) / 1_000_000
        );
    }

    #[test]
    fn whole_microseconds() {
        assert_eq!(Duration::milliseconds(1).whole_microseconds(), 1_000);
        assert_eq!(Duration::milliseconds(-1).whole_microseconds(), -1_000);
        assert_eq!(Duration::microseconds(1).whole_microseconds(), 1);
        assert_eq!(Duration::microseconds(-1).whole_microseconds(), -1);
    }

    #[test]
    fn subsec_microseconds() {
        assert_eq!(Duration::microseconds(1_000_400).subsec_microseconds(), 400);
        assert_eq!(
            Duration::microseconds(-1_000_400).subsec_microseconds(),
            400
        );
    }

    #[test]
    fn nanoseconds() {
        assert_eq!(
            Duration::nanoseconds(1),
            Duration::seconds(1) / 1_000_000_000
        );
        assert_eq!(
            Duration::nanoseconds(-1),
            Duration::seconds(-1) / 1_000_000_000
        );
    }

    #[test]
    fn whole_nanoseconds() {
        assert_eq!(Duration::microseconds(1).whole_nanoseconds(), 1_000);
        assert_eq!(Duration::microseconds(-1).whole_nanoseconds(), -1_000);
        assert_eq!(Duration::nanoseconds(1).whole_nanoseconds(), 1);
        assert_eq!(Duration::nanoseconds(-1).whole_nanoseconds(), -1);
    }

    #[test]
    fn subsec_nanoseconds() {
        assert_eq!(
            Duration::nanoseconds(1_000_000_400).subsec_nanoseconds(),
            400
        );
        assert_eq!(
            Duration::nanoseconds(-1_000_000_400).subsec_nanoseconds(),
            400
        );
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
    #[cfg(feature = "std")]
    fn time_fn() {
        let (time, value) = Duration::time_fn(|| {
            std::thread::sleep(100.std_milliseconds());
            0
        });

        assert!(time >= 100.milliseconds());
        assert_eq!(value, 0);
    }

    #[test]
    fn from_std_duration() {
        assert_eq!(Duration::from(0.std_seconds()), 0.seconds());
        assert_eq!(Duration::from(1.std_seconds()), 1.seconds());
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
        assert_eq!(-(1.seconds()), Duration::seconds(-1));
        assert_eq!(-(-1).seconds(), Duration::second());
        assert_eq!(-(0.seconds()), Duration::zero());
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

        let mut duration = 1.std_seconds();
        assert_panics!(duration -= 2.seconds());
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
        assert_panics!(1.seconds() / 0_f32);

        assert_eq!(1.seconds() / 1_f64, 1.seconds());
        assert_eq!(1.seconds() / 2_f64, 500.milliseconds());
        assert_eq!(1.seconds() / -1_f64, (-1).seconds());
        assert_panics!(1.seconds() / 0_f64);
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
        assert_panics!(duration /= 0_f32);

        let mut duration = 1.seconds();
        duration /= 1_f64;
        assert_eq!(duration, 1.seconds());

        let mut duration = 1.seconds();
        duration /= 2_f64;
        assert_eq!(duration, 500.milliseconds());

        let mut duration = 1.seconds();
        duration /= -1_f64;
        assert_eq!(duration, (-1).seconds());

        let mut duration = 1.seconds();
        assert_panics!(duration /= 0_f64);
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
}
