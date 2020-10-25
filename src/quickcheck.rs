//! Implementations of the [`quickcheck::Arbitrary`] trait.
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
    Date,
};
use core::{cmp, convert::TryInto};
use quickcheck_dep::{Arbitrary, Gen};
use rand::Rng;
#[allow(unused_imports)]
use standback::prelude::*; // assoc_int_consts (1.43)

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

        Self::from_yo_unchecked(year, ordinal)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let (year, ordinal) = self.as_yo();

        let shrunk_year = year
            .shrink()
            .flat_map(move |year| Self::from_yo(year, ordinal));
        let shrunk_ordinal = ordinal
            .shrink()
            .flat_map(move |ordinal| Self::from_yo(year, ordinal));

        Box::new(shrunk_year.chain(shrunk_ordinal))
    }
}
