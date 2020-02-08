#![allow(trivial_numeric_casts, clippy::cast_possible_truncation)]

use crate::Duration;
use core::time::Duration as StdDuration;

/// Create `Duration`s from primitive and core numeric types.
///
/// This trait can be imported with `use time::prelude::*`.
///
/// Due to limitations in rustc, these methods are currently _not_ `const fn`.
/// See [RFC 2632](https://github.com/rust-lang/rfcs/pull/2632) for details.
///
/// # Examples
///
/// Basic construction of `Duration`s.
///
/// ```rust
/// # use time::{Duration, NumericalDuration};
/// assert_eq!(5.nanoseconds(), Duration::nanoseconds(5));
/// assert_eq!(5.microseconds(), Duration::microseconds(5));
/// assert_eq!(5.milliseconds(), Duration::milliseconds(5));
/// assert_eq!(5.seconds(), Duration::seconds(5));
/// assert_eq!(5.minutes(), Duration::minutes(5));
/// assert_eq!(5.hours(), Duration::hours(5));
/// assert_eq!(5.days(), Duration::days(5));
/// assert_eq!(5.weeks(), Duration::weeks(5));
/// ```
///
/// Signed integers work as well!
///
/// ```rust
/// # use time::{Duration, NumericalDuration};
/// assert_eq!((-5).nanoseconds(), Duration::nanoseconds(-5));
/// assert_eq!((-5).microseconds(), Duration::microseconds(-5));
/// assert_eq!((-5).milliseconds(), Duration::milliseconds(-5));
/// assert_eq!((-5).seconds(), Duration::seconds(-5));
/// assert_eq!((-5).minutes(), Duration::minutes(-5));
/// assert_eq!((-5).hours(), Duration::hours(-5));
/// assert_eq!((-5).days(), Duration::days(-5));
/// assert_eq!((-5).weeks(), Duration::weeks(-5));
/// ```
///
/// Just like any other `Duration`, they can be added, subtracted, etc.
///
/// ```rust
/// # use time::NumericalDuration;
/// assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
/// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
/// ```
///
/// When called on floating point values, any remainder of the floating point
/// value will be truncated. Keep in mind that floating point numbers are
/// inherently imprecise and have limited capacity.
pub trait NumericalDuration {
    /// Create a `Duration` from the number of nanoseconds.
    fn nanoseconds(self) -> Duration;
    /// Create a `Duration` from the number of microseconds.
    fn microseconds(self) -> Duration;
    /// Create a `Duration` from the number of milliseconds.
    fn milliseconds(self) -> Duration;
    /// Create a `Duration` from the number of seconds.
    fn seconds(self) -> Duration;
    /// Create a `Duration` from the number of minutes.
    fn minutes(self) -> Duration;
    /// Create a `Duration` from the number of hours.
    fn hours(self) -> Duration;
    /// Create a `Duration` from the number of days.
    fn days(self) -> Duration;
    /// Create a `Duration` from the number of weeks.
    fn weeks(self) -> Duration;
}

macro_rules! impl_numerical_duration {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalDuration for $type {
                #[inline(always)]
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self as i64)
                }

                #[inline(always)]
                fn microseconds(self) -> Duration {
                    Duration::microseconds(self as i64)
                }

                #[inline(always)]
                fn milliseconds(self) -> Duration {
                    Duration::milliseconds(self as i64)
                }

                #[inline(always)]
                fn seconds(self) -> Duration {
                    Duration::seconds(self as i64)
                }

                #[inline(always)]
                fn minutes(self) -> Duration {
                    Duration::minutes(self as i64)
                }

                #[inline(always)]
                fn hours(self) -> Duration {
                    Duration::hours(self as i64)
                }

                #[inline(always)]
                fn days(self) -> Duration {
                    Duration::days(self as i64)
                }

                #[inline(always)]
                fn weeks(self) -> Duration {
                    Duration::weeks(self as i64)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_duration_nonzero {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalDuration for $type {
                #[inline(always)]
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self.get() as i64)
                }

                #[inline(always)]
                fn microseconds(self) -> Duration {
                    Duration::microseconds(self.get() as i64)
                }

                #[inline(always)]
                fn milliseconds(self) -> Duration {
                    Duration::milliseconds(self.get() as i64)
                }

                #[inline(always)]
                fn seconds(self) -> Duration {
                    Duration::seconds(self.get() as i64)
                }

                #[inline(always)]
                fn minutes(self) -> Duration {
                    Duration::minutes(self.get() as i64)
                }

                #[inline(always)]
                fn hours(self) -> Duration {
                    Duration::hours(self.get() as i64)
                }

                #[inline(always)]
                fn days(self) -> Duration {
                    Duration::days(self.get() as i64)
                }

                #[inline(always)]
                fn weeks(self) -> Duration {
                    Duration::weeks(self.get() as i64)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_duration_float {
    ($($type:ty),* $(,)?) => {
        $(
            #[allow(clippy::cast_sign_loss)]
            impl NumericalDuration for $type {
                #[inline(always)]
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self as i64)
                }

                #[inline]
                fn microseconds(self) -> Duration {
                    Duration::nanoseconds((self * 1_000.) as i64)
                }

                #[inline]
                fn milliseconds(self) -> Duration {
                    Duration::nanoseconds((self * 1_000_000.) as i64)
                }

                #[inline]
                fn seconds(self) -> Duration {
                    Duration::nanoseconds((self * 1_000_000_000.) as i64)
                }

                #[inline]
                fn minutes(self) -> Duration {
                    Duration::nanoseconds((self * 60_000_000_000.) as i64)
                }

                #[inline]
                fn hours(self) -> Duration {
                    Duration::nanoseconds((self * 3_600_000_000_000.) as i64)
                }

                #[inline]
                fn days(self) -> Duration {
                    Duration::nanoseconds((self * 86_400_000_000_000.) as i64)
                }

                #[inline]
                fn weeks(self) -> Duration {
                    Duration::nanoseconds((self * 604_800_000_000_000.) as i64)
                }
            }
        )*
    };
}

impl_numerical_duration![u8, u16, u32, i8, i16, i32, i64];
impl_numerical_duration_nonzero![
    core::num::NonZeroU8,
    core::num::NonZeroU16,
    core::num::NonZeroU32,
    core::num::NonZeroI8,
    core::num::NonZeroI16,
    core::num::NonZeroI32,
    core::num::NonZeroI64,
];
impl_numerical_duration_float![f32, f64];

/// Create `std::time::Duration`s from primitive and core numeric types.
///
/// This trait can be imported (alongside others) with `use time::prelude::*`.
///
/// Due to limitations in rustc, these methods are currently _not_ `const fn`.
/// See [RFC 2632](https://github.com/rust-lang/rfcs/pull/2632) for details.
///
/// # Examples
///
/// Basic construction of `std::time::Duration`s.
///
/// ```rust
/// # use time::NumericalStdDuration;
/// # use core::time::Duration;
/// assert_eq!(5.std_nanoseconds(), Duration::from_nanos(5));
/// assert_eq!(5.std_microseconds(), Duration::from_micros(5));
/// assert_eq!(5.std_milliseconds(), Duration::from_millis(5));
/// assert_eq!(5.std_seconds(), Duration::from_secs(5));
/// assert_eq!(5.std_minutes(), Duration::from_secs(5 * 60));
/// assert_eq!(5.std_hours(), Duration::from_secs(5 * 3_600));
/// assert_eq!(5.std_days(), Duration::from_secs(5 * 86_400));
/// assert_eq!(5.std_weeks(), Duration::from_secs(5 * 604_800));
/// ```
///
/// Just like any other `std::time::Duration`, they can be added, subtracted,
/// etc.
///
/// ```rust
/// # use time::NumericalStdDuration;
/// assert_eq!(
///     2.std_seconds() + 500.std_milliseconds(),
///     2_500.std_milliseconds()
/// );
/// assert_eq!(
///     2.std_seconds() - 500.std_milliseconds(),
///     1_500.std_milliseconds()
/// );
/// ```
///
/// When called on floating point values, any remainder of the floating point
/// value will be truncated. Keep in mind that floating point numbers are
/// inherently imprecise and have limited capacity.
pub trait NumericalStdDuration {
    /// Create a `std::time::Duration` from the number of nanoseconds.
    fn std_nanoseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of microseconds.
    fn std_microseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of milliseconds.
    fn std_milliseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of seconds.
    fn std_seconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of minutes.
    fn std_minutes(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of hours.
    fn std_hours(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of days.
    fn std_days(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of weeks.
    fn std_weeks(self) -> StdDuration;
}

macro_rules! impl_numerical_std_duration {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalStdDuration for $type {
                #[inline(always)]
                fn std_nanoseconds(self) -> StdDuration {
                    StdDuration::from_nanos(self as u64)
                }

                #[inline(always)]
                fn std_microseconds(self) -> StdDuration {
                    StdDuration::from_micros(self as u64)
                }

                #[inline(always)]
                fn std_milliseconds(self) -> StdDuration {
                    StdDuration::from_millis(self as u64)
                }

                #[inline(always)]
                fn std_seconds(self) -> StdDuration {
                    StdDuration::from_secs(self as u64)
                }

                #[inline(always)]
                fn std_minutes(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 60)
                }

                #[inline(always)]
                fn std_hours(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 3_600)
                }

                #[inline(always)]
                fn std_days(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 86_400)
                }

                #[inline(always)]
                fn std_weeks(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 604_800)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_std_duration_nonzero {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalStdDuration for $type {
                #[inline(always)]
                fn std_nanoseconds(self) -> StdDuration {
                    StdDuration::from_nanos(self.get() as u64)
                }

                #[inline(always)]
                fn std_microseconds(self) -> StdDuration {
                    StdDuration::from_micros(self.get() as u64)
                }

                #[inline(always)]
                fn std_milliseconds(self) -> StdDuration {
                    StdDuration::from_millis(self.get() as u64)
                }

                #[inline(always)]
                fn std_seconds(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64)
                }

                #[inline(always)]
                fn std_minutes(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 60)
                }

                #[inline(always)]
                fn std_hours(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 3_600)
                }

                #[inline(always)]
                fn std_days(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 86_400)
                }

                #[inline(always)]
                fn std_weeks(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 604_800)
                }
            }
        )*
    };
}

impl_numerical_std_duration![u8, u16, u32, u64];
impl_numerical_std_duration_nonzero![
    core::num::NonZeroU8,
    core::num::NonZeroU16,
    core::num::NonZeroU32,
    core::num::NonZeroU64,
];

/// Implement on `i32` because that's the default type for integers. This
/// performs a runtime check and panics if the value is negative.
#[allow(clippy::cast_sign_loss)]
impl NumericalStdDuration for i32 {
    #[inline(always)]
    fn std_nanoseconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_nanos(self as u64)
    }

    #[inline(always)]
    fn std_microseconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_micros(self as u64)
    }

    #[inline(always)]
    fn std_milliseconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_millis(self as u64)
    }

    #[inline(always)]
    fn std_seconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64)
    }

    #[inline(always)]
    fn std_minutes(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 60)
    }

    #[inline(always)]
    fn std_hours(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 3_600)
    }

    #[inline(always)]
    fn std_days(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 86_400)
    }

    #[inline(always)]
    fn std_weeks(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 604_800)
    }
}

/// Implement on `f64` because that's the default type for floats. This performs
/// a runtime check and panics if the value is negative.
#[allow(clippy::cast_sign_loss)]
impl NumericalStdDuration for f64 {
    #[inline(always)]
    fn std_nanoseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos(self as u64)
    }

    #[inline]
    fn std_microseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000.) as u64)
    }

    #[inline]
    fn std_milliseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000.) as u64)
    }

    #[inline]
    fn std_seconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000_000.) as u64)
    }

    #[inline]
    fn std_minutes(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 60_000_000_000.) as u64)
    }

    #[inline]
    fn std_hours(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 3_600_000_000_000.) as u64)
    }

    #[inline]
    fn std_days(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 86_400_000_000_000.) as u64)
    }

    #[inline]
    fn std_weeks(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 604_800_000_000_000.) as u64)
    }
}

/// Create `std::time::Duration`s from primitive and core numeric types. Unless
/// you are always expecting a `std::time::Duration`, you should prefer to use
/// [`NumericalStdDuration`] for clarity.
///
/// Due to limitations in rustc, these methods are currently _not_ `const fn`.
/// See [this RFC](https://github.com/rust-lang/rfcs/pull/2632) for details.
///
/// # Examples
///
/// Basic construction of `std::time::Duration`s.
///
/// ```rust
/// # use time::NumericalStdDurationShort;
/// # use core::time::Duration;
/// assert_eq!(5.nanoseconds(), Duration::from_nanos(5));
/// assert_eq!(5.microseconds(), Duration::from_micros(5));
/// assert_eq!(5.milliseconds(), Duration::from_millis(5));
/// assert_eq!(5.seconds(), Duration::from_secs(5));
/// assert_eq!(5.minutes(), Duration::from_secs(5 * 60));
/// assert_eq!(5.hours(), Duration::from_secs(5 * 3_600));
/// assert_eq!(5.days(), Duration::from_secs(5 * 86_400));
/// assert_eq!(5.weeks(), Duration::from_secs(5 * 604_800));
/// ```
///
/// Just like any other `std::time::Duration`, they can be added, subtracted,
/// etc.
///
/// ```rust
/// # use time::NumericalStdDurationShort;
/// assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
/// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
/// ```
///
/// When called on floating point values, any remainder of the floating point
/// value will be truncated. Keep in mind that floating point numbers are
/// inherently imprecise and have limited capacity.
pub trait NumericalStdDurationShort {
    /// Create a `std::time::Duration` from the number of nanoseconds.
    fn nanoseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of microseconds.
    fn microseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of milliseconds.
    fn milliseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of seconds.
    fn seconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of minutes.
    fn minutes(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of hours.
    fn hours(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of days.
    fn days(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of weeks.
    fn weeks(self) -> StdDuration;
}

macro_rules! impl_numerical_std_duration {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalStdDurationShort for $type {
                #[inline(always)]
                fn nanoseconds(self) -> StdDuration {
                    StdDuration::from_nanos(self as u64)
                }

                #[inline(always)]
                fn microseconds(self) -> StdDuration {
                    StdDuration::from_micros(self as u64)
                }

                #[inline(always)]
                fn milliseconds(self) -> StdDuration {
                    StdDuration::from_millis(self as u64)
                }

                #[inline(always)]
                fn seconds(self) -> StdDuration {
                    StdDuration::from_secs(self as u64)
                }

                #[inline(always)]
                fn minutes(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 60)
                }

                #[inline(always)]
                fn hours(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 3_600)
                }

                #[inline(always)]
                fn days(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 86_400)
                }

                #[inline(always)]
                fn weeks(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 604_800)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_std_duration_nonzero {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalStdDurationShort for $type {
                #[inline(always)]
                fn nanoseconds(self) -> StdDuration {
                    StdDuration::from_nanos(self.get() as u64)
                }

                #[inline(always)]
                fn microseconds(self) -> StdDuration {
                    StdDuration::from_micros(self.get() as u64)
                }

                #[inline(always)]
                fn milliseconds(self) -> StdDuration {
                    StdDuration::from_millis(self.get() as u64)
                }

                #[inline(always)]
                fn seconds(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64)
                }

                #[inline(always)]
                fn minutes(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 60)
                }

                #[inline(always)]
                fn hours(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 3_600)
                }

                #[inline(always)]
                fn days(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 86_400)
                }

                #[inline(always)]
                fn weeks(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 604_800)
                }
            }
        )*
    };
}

impl_numerical_std_duration![u8, u16, u32, u64];
impl_numerical_std_duration_nonzero![
    core::num::NonZeroU8,
    core::num::NonZeroU16,
    core::num::NonZeroU32,
    core::num::NonZeroU64,
];

/// Implement on `i32` because that's the default type for integers. This
/// performs a runtime check and panics if the value is negative.
#[allow(clippy::cast_sign_loss)]
impl NumericalStdDurationShort for i32 {
    #[inline(always)]
    fn nanoseconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_nanos(self as u64)
    }

    #[inline(always)]
    fn microseconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_micros(self as u64)
    }

    #[inline(always)]
    fn milliseconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_millis(self as u64)
    }

    #[inline(always)]
    fn seconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64)
    }

    #[inline(always)]
    fn minutes(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 60)
    }

    #[inline(always)]
    fn hours(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 3_600)
    }

    #[inline(always)]
    fn days(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 86_400)
    }

    #[inline(always)]
    fn weeks(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 604_800)
    }
}

/// Implement on `f64` because that's the default type for floats. This performs
/// a runtime check and panics if the value is negative.
#[allow(clippy::cast_sign_loss)]
impl NumericalStdDurationShort for f64 {
    #[inline(always)]
    fn nanoseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos(self as u64)
    }

    #[inline]
    fn microseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000.) as u64)
    }

    #[inline]
    fn milliseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000.) as u64)
    }

    #[inline]
    fn seconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000_000.) as u64)
    }

    #[inline]
    fn minutes(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 60_000_000_000.) as u64)
    }

    #[inline]
    fn hours(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 3_600_000_000_000.) as u64)
    }

    #[inline]
    fn days(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 86_400_000_000_000.) as u64)
    }

    #[inline]
    fn weeks(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 604_800_000_000_000.) as u64)
    }
}

#[cfg(test)]
mod test_numerical_duration {
    use super::{Duration, NumericalDuration};

    #[test]
    fn unsigned() {
        assert_eq!(5.nanoseconds(), Duration::nanoseconds(5));
        assert_eq!(5.microseconds(), Duration::microseconds(5));
        assert_eq!(5.milliseconds(), Duration::milliseconds(5));
        assert_eq!(5.seconds(), Duration::seconds(5));
        assert_eq!(5.minutes(), Duration::minutes(5));
        assert_eq!(5.hours(), Duration::hours(5));
        assert_eq!(5.days(), Duration::days(5));
        assert_eq!(5.weeks(), Duration::weeks(5));
    }

    #[test]
    fn signed() {
        assert_eq!((-5).nanoseconds(), Duration::nanoseconds(-5));
        assert_eq!((-5).microseconds(), Duration::microseconds(-5));
        assert_eq!((-5).milliseconds(), Duration::milliseconds(-5));
        assert_eq!((-5).seconds(), Duration::seconds(-5));
        assert_eq!((-5).minutes(), Duration::minutes(-5));
        assert_eq!((-5).hours(), Duration::hours(-5));
        assert_eq!((-5).days(), Duration::days(-5));
        assert_eq!((-5).weeks(), Duration::weeks(-5));
    }

    #[test]
    fn float() {
        // Ensure values truncate rather than round.
        assert_eq!(1.9.nanoseconds(), Duration::nanoseconds(1));

        assert_eq!(1.0.nanoseconds(), Duration::nanoseconds(1));
        assert_eq!(1.0.microseconds(), Duration::microseconds(1));
        assert_eq!(1.0.milliseconds(), Duration::milliseconds(1));
        assert_eq!(1.0.seconds(), Duration::seconds(1));
        assert_eq!(1.0.minutes(), Duration::minutes(1));
        assert_eq!(1.0.hours(), Duration::hours(1));
        assert_eq!(1.0.days(), Duration::days(1));
        assert_eq!(1.0.weeks(), Duration::weeks(1));

        assert_eq!(1.5.nanoseconds(), Duration::nanoseconds(1));
        assert_eq!(1.5.microseconds(), Duration::nanoseconds(1_500));
        assert_eq!(1.5.milliseconds(), Duration::microseconds(1_500));
        assert_eq!(1.5.seconds(), Duration::milliseconds(1_500));
        assert_eq!(1.5.minutes(), Duration::seconds(90));
        assert_eq!(1.5.hours(), Duration::minutes(90));
        assert_eq!(1.5.days(), Duration::hours(36));
        assert_eq!(1.5.weeks(), Duration::hours(252));
    }

    #[test]
    fn arithmetic() {
        assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
        assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
    }
}

#[cfg(test)]
mod test_numerical_std_duration {
    use super::NumericalStdDuration;
    use core::time::Duration;

    #[test]
    fn unsigned() {
        assert_eq!(5.std_nanoseconds(), Duration::from_nanos(5));
        assert_eq!(5.std_microseconds(), Duration::from_micros(5));
        assert_eq!(5.std_milliseconds(), Duration::from_millis(5));
        assert_eq!(5.std_seconds(), Duration::from_secs(5));
        assert_eq!(5.std_minutes(), Duration::from_secs(5 * 60));
        assert_eq!(5.std_hours(), Duration::from_secs(5 * 3_600));
        assert_eq!(5.std_days(), Duration::from_secs(5 * 86_400));
        assert_eq!(5.std_weeks(), Duration::from_secs(5 * 604_800));
    }

    #[test]
    fn float() {
        // Ensure values truncate rather than round.
        assert_eq!(1.9.std_nanoseconds(), Duration::from_nanos(1));

        assert_eq!(1.0.std_nanoseconds(), Duration::from_nanos(1));
        assert_eq!(1.0.std_microseconds(), Duration::from_micros(1));
        assert_eq!(1.0.std_milliseconds(), Duration::from_millis(1));
        assert_eq!(1.0.std_seconds(), Duration::from_secs(1));
        assert_eq!(1.0.std_minutes(), Duration::from_secs(60));
        assert_eq!(1.0.std_hours(), Duration::from_secs(3_600));
        assert_eq!(1.0.std_days(), Duration::from_secs(86_400));
        assert_eq!(1.0.std_weeks(), Duration::from_secs(604_800));

        assert_eq!(1.5.std_nanoseconds(), Duration::from_nanos(1));
        assert_eq!(1.5.std_microseconds(), Duration::from_nanos(1_500));
        assert_eq!(1.5.std_milliseconds(), Duration::from_micros(1_500));
        assert_eq!(1.5.std_seconds(), Duration::from_millis(1_500));
        assert_eq!(1.5.std_minutes(), Duration::from_secs(90));
        assert_eq!(1.5.std_hours(), Duration::from_secs(90 * 60));
        assert_eq!(1.5.std_days(), Duration::from_secs(36 * 3_600));
        assert_eq!(1.5.std_weeks(), Duration::from_secs(252 * 3_600));
    }

    #[test]
    fn arithmetic() {
        assert_eq!(
            2.std_seconds() + 500.std_milliseconds(),
            2_500.std_milliseconds()
        );
        assert_eq!(
            2.std_seconds() - 500.std_milliseconds(),
            1_500.std_milliseconds()
        );
    }
}

#[cfg(test)]
mod test_numerical_std_duration_short {
    use super::NumericalStdDurationShort;
    use core::time::Duration;

    #[test]
    fn unsigned() {
        assert_eq!(5.nanoseconds(), Duration::from_nanos(5));
        assert_eq!(5.microseconds(), Duration::from_micros(5));
        assert_eq!(5.milliseconds(), Duration::from_millis(5));
        assert_eq!(5.seconds(), Duration::from_secs(5));
        assert_eq!(5.minutes(), Duration::from_secs(5 * 60));
        assert_eq!(5.hours(), Duration::from_secs(5 * 3_600));
        assert_eq!(5.days(), Duration::from_secs(5 * 86_400));
        assert_eq!(5.weeks(), Duration::from_secs(5 * 604_800));
    }

    #[test]
    fn float() {
        // Ensure values truncate rather than round.
        assert_eq!(1.9.nanoseconds(), Duration::from_nanos(1));

        assert_eq!(1.0.nanoseconds(), Duration::from_nanos(1));
        assert_eq!(1.0.microseconds(), Duration::from_micros(1));
        assert_eq!(1.0.milliseconds(), Duration::from_millis(1));
        assert_eq!(1.0.seconds(), Duration::from_secs(1));
        assert_eq!(1.0.minutes(), Duration::from_secs(60));
        assert_eq!(1.0.hours(), Duration::from_secs(3_600));
        assert_eq!(1.0.days(), Duration::from_secs(86_400));
        assert_eq!(1.0.weeks(), Duration::from_secs(604_800));

        assert_eq!(1.5.nanoseconds(), Duration::from_nanos(1));
        assert_eq!(1.5.microseconds(), Duration::from_nanos(1_500));
        assert_eq!(1.5.milliseconds(), Duration::from_micros(1_500));
        assert_eq!(1.5.seconds(), Duration::from_millis(1_500));
        assert_eq!(1.5.minutes(), Duration::from_secs(90));
        assert_eq!(1.5.hours(), Duration::from_secs(90 * 60));
        assert_eq!(1.5.days(), Duration::from_secs(36 * 3_600));
        assert_eq!(1.5.weeks(), Duration::from_secs(252 * 3_600));
    }

    #[test]
    fn arithmetic() {
        assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
        assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
    }
}
