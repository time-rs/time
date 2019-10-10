use crate::Duration;
use crate::Sign::{Negative, Positive, Unknown, Zero};
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration as StdDuration;
use std::time::Instant as StdInstant;

/// A measurement of a monotonically nondecreasing clock. Opaque and useful only
/// with [`Duration`](Duration).
///
/// Instants are always guaranteed to be no less than any previously measured
/// instant when created, and are often useful for tasks such as measuring
/// benchmarks or timing how long an operation takes.
///
/// Note, however, that instants are not guaranteed to be **steady**. In other
/// words, each tick of the underlying clock may not be the same length (e.g.
/// some seconds may be longer than others). An instant may jump forwards or
/// experience time dilation (slow down or speed up), but it will never go
/// backwards.
///
/// Instants are opaque types that can only be compared to one another. There is
/// no method to get "the number of seconds" from an instant. Instead, it only
/// allows measuring the duration between two instants (or comparing two
/// instants).
///
/// Allows for operations with signed [`Duration`](Duration)s, but is otherwise
/// identical to [`std::time::Instant`](std::time::Instant).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    /// Inner representation, using `std::time::Instant`.
    inner: StdInstant,
}

impl Instant {
    /// Returns an `Instant` corresponding to "now".
    ///
    /// ```rust,no_run
    /// # use time::Instant;
    /// println!("{:?}", Instant::now());
    /// ```
    pub fn now() -> Self {
        Self {
            inner: StdInstant::now(),
        }
    }

    /// Returns the amount of time elapsed since this instant was created. The
    /// duration will always be nonnegative if the instant is not synthetically
    /// created.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::{Duration, Instant};
    /// # use std::thread;
    /// let instant = Instant::now();
    /// thread::sleep(Duration::milliseconds(100).try_into().unwrap());
    /// assert!(instant.elapsed() >= Duration::milliseconds(100));
    /// ```
    pub fn elapsed(self) -> Duration {
        Self::now() - self
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// ```rust
    /// # use time::{Duration, Instant};
    /// let now = Instant::now();
    /// assert_eq!(now.checked_add(Duration::seconds(5)), Some(now + Duration::seconds(5)));
    /// assert_eq!(now.checked_add(Duration::seconds(-5)), Some(now + Duration::seconds(-5)));
    /// ```
    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        match duration.sign() {
            Zero => Some(self),
            Negative => self
                .inner
                .checked_sub(StdDuration::try_from(duration.abs()).unwrap())
                .map(From::from),
            Positive => self
                .inner
                .checked_add(StdDuration::try_from(duration.abs()).unwrap())
                .map(From::from),
            Unknown => unreachable!("A `Duration` cannot have an unknown sign"),
        }
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// ```rust
    /// # use time::{Duration, Instant};
    /// let now = Instant::now();
    /// assert_eq!(now.checked_sub(Duration::seconds(5)), Some(now - Duration::seconds(5)));
    /// assert_eq!(now.checked_sub(Duration::seconds(-5)), Some(now - Duration::seconds(-5)));
    /// ```
    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        self.checked_add(-duration)
    }
}

#[allow(clippy::missing_docs_in_private_items)]
impl Instant {
    #[deprecated(since = "0.2.0", note = "Use `rhs - lhs`")]
    pub fn to(&self, later: Self) -> Duration {
        later - *self
    }
}

impl From<StdInstant> for Instant {
    /// Convert a `std::time::Instant` to a `time::Instant`.
    ///
    /// ```rust
    /// # use time::Instant;
    /// let now_time = Instant::now();
    /// let now_std = std::time::Instant::from(now_time);
    /// assert_eq!(now_time, now_std.into());
    /// ```
    fn from(instant: StdInstant) -> Self {
        Self { inner: instant }
    }
}

impl From<Instant> for StdInstant {
    /// Convert a `time::Instant` to a `std::time::Instant`.
    ///
    /// ```rust
    /// # use time::Instant;
    /// let now_std = std::time::Instant::now();
    /// let now_time = Instant::from(now_std);
    /// assert_eq!(now_time, now_std.into());
    /// ```
    fn from(instant: Instant) -> Self {
        instant.inner
    }
}

impl Sub for Instant {
    type Output = Duration;

    /// Subtract two `Instant`s, resulting in the `Duration` between them.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::{Duration, Instant};
    /// # use std::thread;
    /// let start = Instant::now();
    /// thread::sleep(Duration::milliseconds(100).try_into().unwrap());
    /// assert!(Instant::now() - start >= Duration::milliseconds(100));
    /// ```
    fn sub(self, other: Self) -> Self::Output {
        match self.inner.cmp(&other.inner) {
            Ordering::Equal => Duration::zero(),
            Ordering::Greater => (self.inner - other.inner).into(),
            Ordering::Less => (other.inner - self.inner).into(),
        }
    }
}

impl Sub<StdInstant> for Instant {
    type Output = Duration;

    /// Subtract two `Instant`s, resulting in the `Duration` between them.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::{Duration, Instant};
    /// # use std::thread;
    /// let start = std::time::Instant::now();
    /// thread::sleep(Duration::milliseconds(100).try_into().unwrap());
    /// assert!(Instant::now() - start >= Duration::milliseconds(100));
    /// ```
    fn sub(self, other: StdInstant) -> Self::Output {
        self - Self::from(other)
    }
}

impl Sub<Instant> for StdInstant {
    type Output = Duration;

    /// Subtract two `Instant`s, resulting in the `Duration` between them.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::{Duration, Instant};
    /// # use std::thread;
    /// let start = Instant::now();
    /// thread::sleep(Duration::milliseconds(100).try_into().unwrap());
    /// assert!(std::time::Instant::now() - start >= Duration::milliseconds(100));
    /// ```
    fn sub(self, other: Instant) -> Self::Output {
        Instant::from(self) - other
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    /// Add a `Duration` to an `Instant`, resulting in an `Instant` representing
    /// the new point in time.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::{Duration, Instant};
    /// # use std::thread;
    /// let start = Instant::now();
    /// thread::sleep(Duration::milliseconds(100).try_into().unwrap());
    /// assert!(start + Duration::milliseconds(100) <= Instant::now());
    /// ```
    fn add(self, duration: Duration) -> Self::Output {
        self.checked_add(duration)
            .expect("overflow when adding duration to instant")
    }
}

impl Add<StdDuration> for Instant {
    type Output = Self;

    /// Add a `std::time::Duration` to an `Instant`, resulting in an `Instant`
    /// representing the new point in time.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::Instant;
    /// # use std::thread;
    /// # use core::time::Duration;
    /// let start = Instant::now();
    /// thread::sleep(Duration::from_millis(100));
    /// assert!(start + Duration::from_millis(100) <= Instant::now());
    /// ```
    fn add(self, duration: StdDuration) -> Self::Output {
        self + Duration::from(duration)
    }
}

impl AddAssign<Duration> for Instant {
    /// Add a `Duration` to an `Instant`, resulting in an `Instant` representing
    /// the new point in time.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::{Duration, Instant};
    /// # use std::thread;
    /// let mut start = Instant::now();
    /// thread::sleep(Duration::milliseconds(100).try_into().unwrap());
    /// start += Duration::milliseconds(100);
    /// assert!(start <= Instant::now());
    /// ```
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for Instant {
    /// Add a `std::time::Duration` to an `Instant`, resulting in an `Instant`
    /// representing the new point in time.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::Instant;
    /// # use std::thread;
    /// # use core::time::Duration;
    /// let mut start = Instant::now();
    /// thread::sleep(Duration::from_millis(100));
    /// start += Duration::from_millis(100);
    /// assert!(start <= Instant::now());
    /// ```
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    /// Subtract a `Duration` from an `Instant`, resulting in an `Instant`
    /// representing the new point in time.
    ///
    /// ```rust
    /// # use time::{Duration, Instant};
    /// let instant = Instant::now();
    /// assert!(instant - Duration::milliseconds(100) <= Instant::now());
    /// ```
    fn sub(self, duration: Duration) -> Self::Output {
        self.checked_sub(duration)
            .expect("overflow when subtracting duration from instant")
    }
}

impl Sub<StdDuration> for Instant {
    type Output = Self;

    /// Subtract a `std::time::Duration` from an `Instant`, resulting in an
    /// `Instant` representing the new point in time.
    ///
    /// ```rust,no_run
    /// # use time::Instant;
    /// # use core::time::Duration;
    /// let instant = Instant::now();
    /// assert!(instant - Duration::from_millis(100) <= Instant::now());
    /// ```
    fn sub(self, duration: StdDuration) -> Self::Output {
        self - Duration::from(duration)
    }
}

impl SubAssign<Duration> for Instant {
    /// Subtract a `Duration` from an `Instant`, resulting in an `Instant`
    /// representing the new point in time.
    ///
    /// ```rust
    /// # use time::{Duration, Instant};
    /// let mut instant = Instant::now();
    /// instant -= Duration::milliseconds(100);
    /// assert!(instant <= Instant::now());
    /// ```
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for Instant {
    /// Subtract a `std::time::Duration` from an `Instant`, resulting in an
    /// `Instant` representing the new point in time.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::Instant;
    /// # use std::thread;
    /// # use core::time::Duration;
    /// let mut instant = Instant::now();
    /// instant -= Duration::from_millis(100);
    /// assert!(instant <= Instant::now());
    /// ```
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}
