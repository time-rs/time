//! Implementations of the [`quickcheck::Arbitrary`](quickcheck_dep::Arbitrary) trait.
//!
//! This enables users to write tests such as this, and have test values provided automatically:
//!
//! ```
//! # #![allow(dead_code)]
//! # use quickcheck_dep::quickcheck;
//! # #[cfg(pretend_we_didnt_rename_the_dependency)]
//! use quickcheck::quickcheck;
//! use time::Date;
//!
//! struct DateRange {
//!     from: Date,
//!     to: Date,
//! }
//!
//! impl DateRange {
//!     fn new(from: Date, to: Date) -> Result<Self, ()> {
//!         Ok(DateRange { from, to })
//!     }
//! }
//!
//! quickcheck! {
//!     fn date_range_is_well_defined(from: Date, to: Date) -> bool {
//!         let r = DateRange::new(from, to);
//!         if from <= to {
//!             r.is_ok()
//!         } else {
//!             r.is_err()
//!         }
//!     }
//! }
//! ```
//!
//! An implementation for `Instant` is intentionally omitted since its values are only meaningful in
//! relation to a [`Duration`], and obtaining an `Instant` from a [`Duration`] is very simple
//! anyway.

use crate::{
    date::{MAX_YEAR, MIN_YEAR},
    hack,
    util::days_in_year,
    Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday,
};
use alloc::boxed::Box;
use core::iter;
use quickcheck_dep::{Arbitrary, Gen};

/// Shim for the unstable clamp method.
// This method seems likely to stabilized in Rust 1.50. This will result in a NET usage date of
// 2021-08-11.
trait Clamp {
    /// Constrain `self` between `min` and `max` (inclusive).
    ///
    /// If `self` is less than `min`, returns `min`.
    /// If `self` is greater than `max`, returns `max`.
    /// Otherwise, returns `self`.
    fn clamp_(self, min: Self, max: Self) -> Self;
}

impl<T: Ord> Clamp for T {
    fn clamp_(self, min: Self, max: Self) -> Self {
        core::cmp::max(min, core::cmp::min(self, max))
    }
}

/// Obtain an arbitrary value between the minimum and maximum inclusive.
fn arbitrary_between<T>(g: &mut Gen, min: T, max: T) -> T
where
    T: PartialOrd
        + core::ops::AddAssign
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Rem<Output = T>
        + Arbitrary
        + Copy,
{
    #[allow(clippy::eq_op)]
    let zero = min - min;

    let range = max - min;
    let mut within_range = T::arbitrary(g) % range;

    if within_range < zero {
        within_range += range;
    }

    within_range + min
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for Date {
    fn arbitrary(g: &mut Gen) -> Self {
        let year = arbitrary_between(g, MIN_YEAR, MAX_YEAR);
        let ordinal = arbitrary_between(g, 1, days_in_year(year));

        Self::from_ordinal_date_unchecked(year, ordinal)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let (year, ordinal) = self.to_ordinal_date();

        let shrunk_year = year
            .shrink()
            .flat_map(move |year| Self::from_ordinal_date(year, ordinal));
        let shrunk_ordinal = ordinal
            .shrink()
            .flat_map(move |ordinal| Self::from_ordinal_date(year, ordinal));

        Box::new(shrunk_year.chain(shrunk_ordinal))
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for Duration {
    fn arbitrary(g: &mut Gen) -> Self {
        let seconds = i64::arbitrary(g);
        let mut nanoseconds = arbitrary_between(g, 0, 999_999_999);

        if seconds < 0 {
            nanoseconds *= -1;
        }

        Self {
            seconds,
            nanoseconds,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let seconds = self.seconds;
        let nanoseconds = self.nanoseconds;

        let shrunk_seconds = seconds.shrink().map(move |seconds| Self {
            seconds,
            nanoseconds,
        });
        let shrunk_nanoseconds = nanoseconds.shrink().map(move |nanoseconds| Self {
            seconds,
            nanoseconds,
        });

        Box::new(shrunk_seconds.chain(shrunk_nanoseconds))
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for Time {
    fn arbitrary(g: &mut Gen) -> Self {
        let hour = arbitrary_between(g, 0, 23);
        let minute = arbitrary_between(g, 0, 59);
        let second = arbitrary_between(g, 0, 59);
        let nanosecond = arbitrary_between(g, 0, 999_999_999);

        Self {
            hour,
            minute,
            second,
            nanosecond,
            padding: hack::Padding::Optimize,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let hour = self.hour;
        let minute = self.minute;
        let second = self.second;
        let nanosecond = self.nanosecond;

        let shrunk_hour = self.hour.shrink().map(move |hour| Self {
            hour,
            minute,
            second,
            nanosecond,
            padding: hack::Padding::Optimize,
        });
        let shrunk_minute = minute.shrink().map(move |minute| Self {
            hour,
            minute,
            second,
            nanosecond,
            padding: hack::Padding::Optimize,
        });
        let shrunk_second = second.shrink().map(move |second| Self {
            hour,
            minute,
            second,
            nanosecond,
            padding: hack::Padding::Optimize,
        });
        let shrunk_nanos = nanosecond.shrink().map(move |nanosecond| Self {
            hour,
            minute,
            second,
            nanosecond,
            padding: hack::Padding::Optimize,
        });

        Box::new(
            shrunk_hour
                .chain(shrunk_minute)
                .chain(shrunk_second)
                .chain(shrunk_nanos),
        )
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for PrimitiveDateTime {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(Date::arbitrary(g), Time::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let date = self.date;
        let time = self.time;

        let shrunk_date = date.shrink().map(move |date| Self::new(date, time));
        let shrunk_time = time.shrink().map(move |time| Self::new(date, time));

        Box::new(shrunk_date.chain(shrunk_time))
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for UtcOffset {
    fn arbitrary(g: &mut Gen) -> Self {
        let hours = arbitrary_between(g, -23, 23);
        let mut minutes = arbitrary_between(g, 0, 59);
        let mut seconds = arbitrary_between(g, 0, 59);

        if hours < 0 {
            minutes *= -1;
            seconds *= -1;
        }

        Self {
            hours,
            minutes,
            seconds,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.to_seconds().shrink().map(move |total_seconds| Self {
            hours: (total_seconds / 3_600) as _,
            minutes: ((total_seconds / 60) % 60) as _,
            seconds: (total_seconds % 60) as _,
        }))
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for OffsetDateTime {
    fn arbitrary(g: &mut Gen) -> Self {
        let datetime = PrimitiveDateTime::arbitrary(g);
        let offset = UtcOffset::arbitrary(g);
        datetime.assume_offset(offset)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let datetime = self.utc_datetime.utc_to_offset(self.offset);
        let offset = self.offset;

        let shrunk_datetime = datetime
            .shrink()
            .map(move |datetime| datetime.assume_offset(offset));
        let shrunk_offset = offset
            .shrink()
            .map(move |offset| datetime.assume_offset(offset));

        Box::new(shrunk_datetime.chain(shrunk_offset))
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for Weekday {
    fn arbitrary(g: &mut Gen) -> Self {
        use Weekday::*;
        match arbitrary_between::<u8>(g, 0, 6) {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            _ => Sunday,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            Self::Monday => Box::new(iter::empty()),
            _ => Box::new(iter::once(self.previous())),
        }
    }
}
