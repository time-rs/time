use crate::Duration;
use core::{
    cmp::{Ord, Ordering, PartialEq, PartialOrd},
    convert::TryInto,
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};
use std::time::Instant as StdInstant;

/// A measurement of a monotonically non-decreasing clock. Opaque and useful
/// only with [`Duration`].
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
/// This implementation allows for operations with signed [`Duration`]s, but is
/// otherwise identical to [`std::time::Instant`].
#[cfg_attr(docs, doc(cfg(feature = "std")))]
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
    #[inline(always)]
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
    /// # use time::{Instant, prelude::*};
    /// # use std::thread;
    /// let instant = Instant::now();
    /// thread::sleep(100.std_milliseconds());
    /// assert!(instant.elapsed() >= 100.milliseconds());
    /// ```
    #[inline(always)]
    pub fn elapsed(self) -> Duration {
        Self::now() - self
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// ```rust
    /// # use time::{Instant, prelude::*};
    /// let now = Instant::now();
    /// assert_eq!(
    ///     now.checked_add(5.seconds()),
    ///     Some(now + 5.seconds())
    /// );
    /// assert_eq!(
    ///     now.checked_add((-5).seconds()),
    ///     Some(now + (-5).seconds())
    /// );
    /// ```
    #[inline]
    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        if duration.is_zero() {
            Some(self)
        } else if duration.is_positive() {
            self.inner.checked_add(duration.abs_std()).map(From::from)
        } else {
            // duration.is_negative()
            self.inner.checked_sub(duration.abs_std()).map(From::from)
        }
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// ```rust
    /// # use time::{Instant, prelude::*};
    /// let now = Instant::now();
    /// assert_eq!(
    ///     now.checked_sub(5.seconds()),
    ///     Some(now - 5.seconds())
    /// );
    /// assert_eq!(
    ///     now.checked_sub((-5).seconds()),
    ///     Some(now - (-5).seconds())
    /// );
    /// ```
    #[inline(always)]
    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        self.checked_add(-duration)
    }
}

#[allow(clippy::missing_docs_in_private_items)]
impl Instant {
    #[inline(always)]
    #[cfg(v01_deprecated)]
    #[cfg_attr(tarpaulin, skip)]
    #[deprecated(since = "0.2.0", note = "Use `rhs - lhs`")]
    pub fn to(&self, later: Self) -> Duration {
        later - *self
    }
}

impl From<StdInstant> for Instant {
    #[inline(always)]
    fn from(instant: StdInstant) -> Self {
        Self { inner: instant }
    }
}

impl From<Instant> for StdInstant {
    #[inline(always)]
    fn from(instant: Instant) -> Self {
        instant.inner
    }
}

impl Sub for Instant {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        match self.inner.cmp(&other.inner) {
            Ordering::Equal => Duration::zero(),
            Ordering::Greater => (self.inner - other.inner)
                .try_into()
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
            Ordering::Less => (other.inner - self.inner)
                .try_into()
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
        }
    }
}

impl Sub<StdInstant> for Instant {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, other: StdInstant) -> Self::Output {
        self - Self::from(other)
    }
}

impl Sub<Instant> for StdInstant {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, other: Instant) -> Self::Output {
        Instant::from(self) - other
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: Duration) -> Self::Output {
        self.checked_add(duration)
            .expect("overflow when adding duration to instant")
    }
}

impl Add<Duration> for StdInstant {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: Duration) -> Self::Output {
        (Instant::from(self) + duration).into()
    }
}

impl Add<StdDuration> for Instant {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: StdDuration) -> Self::Output {
        Self {
            inner: self.inner + duration,
        }
    }
}

impl AddAssign<Duration> for Instant {
    #[inline(always)]
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<Duration> for StdInstant {
    #[inline(always)]
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for Instant {
    #[inline(always)]
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: Duration) -> Self::Output {
        self.checked_sub(duration)
            .expect("overflow when subtracting duration from instant")
    }
}

impl Sub<Duration> for StdInstant {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: Duration) -> Self::Output {
        (Instant::from(self) - duration).into()
    }
}

impl Sub<StdDuration> for Instant {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: StdDuration) -> Self::Output {
        Self {
            inner: self.inner - duration,
        }
    }
}

impl SubAssign<Duration> for Instant {
    #[inline(always)]
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<Duration> for StdInstant {
    #[inline(always)]
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for Instant {
    #[inline(always)]
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl PartialEq<StdInstant> for Instant {
    #[inline(always)]
    fn eq(&self, rhs: &StdInstant) -> bool {
        self.inner.eq(rhs)
    }
}

impl PartialEq<Instant> for StdInstant {
    #[inline(always)]
    fn eq(&self, rhs: &Instant) -> bool {
        self.eq(&rhs.inner)
    }
}

impl PartialOrd<StdInstant> for Instant {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &StdInstant) -> Option<Ordering> {
        self.inner.partial_cmp(rhs)
    }
}

impl PartialOrd<Instant> for StdInstant {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &Instant) -> Option<Ordering> {
        self.partial_cmp(&rhs.inner)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    use std::thread;

    #[test]
    fn elapsed() {
        let instant = Instant::now();
        thread::sleep(100.std_milliseconds());
        assert!(instant.elapsed() >= 100.milliseconds());
    }

    #[test]
    fn checked_add() {
        let now = Instant::now();
        assert_eq!(now.checked_add(5.seconds()), Some(now + 5.seconds()));
        assert_eq!(now.checked_add((-5).seconds()), Some(now + (-5).seconds()));
    }

    #[test]
    fn checked_sub() {
        let now = Instant::now();
        assert_eq!(now.checked_sub(5.seconds()), Some(now - 5.seconds()));
        assert_eq!(now.checked_sub((-5).seconds()), Some(now - (-5).seconds()));
    }

    #[test]
    fn from_std() {
        let now_time = Instant::now();
        let now_std = StdInstant::from(now_time);
        assert_eq!(now_time, now_std);
    }

    #[test]
    fn to_std() {
        let now_std = StdInstant::now();
        let now_time = Instant::from(now_std);
        assert_eq!(now_time, now_std);
    }

    #[test]
    fn sub() {
        let start = Instant::now();
        thread::sleep(100.std_milliseconds());
        assert!(Instant::now() - start >= 100.milliseconds());
    }

    #[test]
    fn sub_std() {
        let start = StdInstant::now();
        thread::sleep(100.std_milliseconds());
        assert!(Instant::now() - start >= 100.milliseconds());
    }

    #[test]
    fn std_sub() {
        let start = Instant::now();
        thread::sleep(100.std_milliseconds());
        assert!(StdInstant::now() - start >= 100.milliseconds());
    }

    #[test]
    fn add_duration() {
        let start = Instant::now();
        thread::sleep(100.std_milliseconds());
        assert!(start + 100.milliseconds() <= Instant::now());
    }

    #[test]
    fn std_add_duration() {
        let start = StdInstant::now();
        thread::sleep(100.std_milliseconds());
        assert!(start + 100.milliseconds() <= StdInstant::now());
    }

    #[test]
    fn add_std_duration() {
        let start = Instant::now();
        thread::sleep(100.std_milliseconds());
        assert!(start + 100.std_milliseconds() <= Instant::now());
    }

    #[test]
    fn add_assign_duration() {
        let mut start = Instant::now();
        thread::sleep(100.std_milliseconds());
        start += 100.milliseconds();
        assert!(start <= Instant::now());
    }

    #[test]
    fn std_add_assign_duration() {
        let mut start = StdInstant::now();
        thread::sleep(100.std_milliseconds());
        start += 100.milliseconds();
        assert!(start <= StdInstant::now());
    }

    #[test]
    fn add_assign_std_duration() {
        let mut start = Instant::now();
        thread::sleep(100.std_milliseconds());
        start += 100.std_milliseconds();
        assert!(start <= Instant::now());
    }

    #[test]
    fn sub_duration() {
        let instant = Instant::now();
        assert!(instant - 100.milliseconds() <= Instant::now());
    }

    #[test]
    fn std_sub_duration() {
        let instant = StdInstant::now();
        assert!(instant - 100.milliseconds() <= StdInstant::now());
    }

    #[test]
    fn sub_std_duration() {
        let instant = Instant::now();
        assert!(instant - 100.std_milliseconds() <= Instant::now());
    }

    #[test]
    fn sub_assign_duration() {
        let mut instant = Instant::now();
        instant -= 100.milliseconds();
        assert!(instant <= Instant::now());
    }

    #[test]
    fn std_sub_assign_duration() {
        let mut instant = StdInstant::now();
        instant -= 100.milliseconds();
        assert!(instant <= StdInstant::now());
    }

    #[test]
    fn sub_assign_std_duration() {
        let mut instant = Instant::now();
        instant -= 100.std_milliseconds();
        assert!(instant <= Instant::now());
    }

    #[test]
    fn eq_std() {
        let now_time = Instant::now();
        let now_std = StdInstant::from(now_time);
        assert_eq!(now_time, now_std);
    }

    #[test]
    fn std_eq() {
        let now_time = Instant::now();
        let now_std = StdInstant::from(now_time);
        assert_eq!(now_std, now_time);
    }

    #[test]
    fn ord_std() {
        let now_time = Instant::now();
        let now_std = StdInstant::from(now_time) + 1.nanoseconds();
        assert!(now_time < now_std);

        let now_time = Instant::now();
        let now_std = StdInstant::from(now_time) - 1.nanoseconds();
        assert!(now_time > now_std);
    }

    #[test]
    fn std_ord() {
        let now_time = Instant::now();
        let now_std = StdInstant::from(now_time) + 1.nanoseconds();
        assert!(now_std > now_time);

        let now_time = Instant::now();
        let now_std = StdInstant::from(now_time) - 1.nanoseconds();
        assert!(now_std < now_time);
    }
}
