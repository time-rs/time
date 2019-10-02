use crate::Duration;
use crate::Sign::{Negative, Positive, Unknown, Zero};
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};
use core::time::Duration as StdDuration;
use std::time::Instant as StdInstant;

/// A measurement of a monotonically nondecreasing clock. Opaque and useful only
/// with [`Duration`].
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
/// The size of an `Instant` struct may vary depending on the target operating
/// system.
///
/// Allows for operations with signed [`Duration`]s, but is otherwise identical
/// to [`std::time::Instant`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    /// Inner representation, using `std::time::Instant`.
    inner: StdInstant,
}

impl Instant {
    /// Returns an `Instant` corresponding to "now".
    pub fn now() -> Self {
        Self {
            inner: StdInstant::now(),
        }
    }

    /// Returns the amount of time elapsed from another instant to this one.
    ///
    /// ```rust,no_run
    /// # use core::convert::TryInto;
    /// # use time::{Duration, Instant};
    /// let now = Instant::now();
    /// std::thread::sleep(Duration::second().try_into().unwrap());
    /// let new_now = Instant::now();
    /// println!("{:?}", new_now.duration_since(now));
    /// ```
    ///
    /// This implementation also supports finding the duration from a newer
    /// instant to an older, which will result in a negative `Duration`.
    ///
    /// ```rust,no_run
    /// # use core::convert::TryInto;
    /// # use time::{Duration, Instant};
    /// let now = Instant::now();
    /// std::thread::sleep(Duration::second().try_into().unwrap());
    /// let new_now = Instant::now();
    /// println!("{:?}", now.duration_since(new_now));
    /// ```
    ///
    /// If the instants are equal, `Duration::zero()` will be returned.
    ///
    /// ```rust
    /// # use time::{Duration, Instant};
    /// let now = Instant::now();
    /// assert!(now.duration_since(now).is_zero());
    /// ```
    pub fn duration_since(self, other: Self) -> Duration {
        self - other
    }

    /// Returns the amount of time elapsed since this instant was created. The
    /// duration will always be nonnegative if the instant is not synthetically
    /// created.
    ///
    /// ```rust
    /// # use core::convert::TryInto;
    /// # use time::{Duration, Instant};
    ///
    /// let instant = Instant::now();
    /// std::thread::sleep(Duration::second().try_into().unwrap());
    /// assert!(instant.elapsed() >= Duration::second());
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
    /// let in_five_seconds = now.checked_add(Duration::seconds(5));
    /// assert_eq!(in_five_seconds, Some(now + Duration::seconds(5)));
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
    /// let five_seconds_ago = now.checked_sub(Duration::seconds(5));
    /// assert_eq!(five_seconds_ago, Some(now - Duration::seconds(5)));
    /// ```
    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        self.checked_add(-duration)
    }
}

// TODO Should we actually implement `Deref`? It could lead to confusing results
// with Rust's implicit dereferencing, and the desired behavior can still be
// achieved explicitly with `.into()`.
impl Deref for Instant {
    type Target = StdInstant;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Instant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<StdInstant> for Instant {
    fn from(instant: StdInstant) -> Self {
        Self { inner: instant }
    }
}

impl From<Instant> for StdInstant {
    fn from(instant: Instant) -> Self {
        instant.inner
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        match self.inner.cmp(&other.inner) {
            Ordering::Equal => Duration::zero(),
            Ordering::Greater => (self.inner - other.inner).into(),
            Ordering::Less => (other.inner - self.inner).into(),
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        self.checked_add(duration)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self.checked_sub(duration)
            .expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}
