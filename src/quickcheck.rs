//! Implementations of the [`quickcheck::Arbitrary`](quickcheck_dep::Arbitrary)
//! trait.
//!
//! This enables users to write tests such as this, and have test values
//! provided automatically:
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
//! An implementation for `Instant` is intentionally omitted since its values
//! are only meaningful in relation to a [`Duration`], and obtaining an
//! `Instant` from a [`Duration`] is very simple anyway.

use crate::{
    date::{MAX_YEAR, MIN_YEAR},
    util::days_in_year,
    Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday,
};
use core::{cmp, convert::TryInto, i32, iter, u16, u32, u8};
use quickcheck_dep::{Arbitrary, Gen};
use rand::Rng;

/// Shim for the unstable clamp method.
// This method seems likely to stabilized in Rust 1.50. This will result in a
// NET usage date of 2021-08-11.
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

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for Date {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let year_size = g.size().try_into().unwrap_or(i32::MAX);
        let year = if year_size == 0 {
            0
        } else {
            g.gen_range(
                cmp::max(MIN_YEAR, -year_size),
                cmp::min(MAX_YEAR, year_size),
            )
        };

        let ordinal_size = cmp::min(g.size().try_into().unwrap_or(u16::MAX), days_in_year(year));
        let ordinal = g.gen_range(1, cmp::max(2, ordinal_size + 1));

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
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let seconds = i64::arbitrary(g);
        let nanoseconds: i32 = g.gen_range(
            0,
            g.size()
                .try_into()
                .unwrap_or(i32::MAX)
                .clamp_(1, 1_000_000_000),
        );
        Self {
            seconds,
            nanoseconds: nanoseconds * (seconds.signum() as i32),
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
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let hour = g.gen_range(0, g.size().try_into().unwrap_or(u8::MAX).clamp_(1, 24));
        let minute = g.gen_range(0, g.size().try_into().unwrap_or(u8::MAX).clamp_(1, 60));
        let second = g.gen_range(0, g.size().try_into().unwrap_or(u8::MAX).clamp_(1, 60));
        let nanosecond = g.gen_range(
            0,
            g.size()
                .try_into()
                .unwrap_or(u32::MAX)
                .clamp_(1, 1_000_000_000),
        );
        Self {
            hour,
            minute,
            second,
            nanosecond,
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
        });
        let shrunk_minute = minute.shrink().map(move |minute| Self {
            hour,
            minute,
            second,
            nanosecond,
        });
        let shrunk_second = second.shrink().map(move |second| Self {
            hour,
            minute,
            second,
            nanosecond,
        });
        let shrunk_nanos = nanosecond.shrink().map(move |nanosecond| Self {
            hour,
            minute,
            second,
            nanosecond,
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
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
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
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let size = g
            .size()
            .try_into()
            .unwrap_or(i32::MAX)
            .clamp_(1, 60 * 60 * 24);
        let offset = g.gen_range(-cmp::max(0, size - 1), size);
        Self { seconds: offset }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.seconds.shrink().map(|seconds| Self { seconds }))
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for OffsetDateTime {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
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
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        use Weekday::*;
        match g.gen_range(0, g.size().clamp_(1, 7)) {
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
