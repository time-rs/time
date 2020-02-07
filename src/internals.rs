//! This module and its contents are not subject to stability guarantees and
//! should not be relied upon.
//!
//! These methods either exist to reduce duplication in code elsewhere or are
//! public only for usage in macros. The reasoning for a method's existence is
//! generally documented alongside the method.
//!
//! Failure to ensure that parameters to the contained functions are in range
//! will likely result in invalid behavior.

#![doc(hidden)]
#![allow(missing_debug_implementations, missing_copy_implementations)]

use crate::{days_in_year, is_leap_year, Weekday};

pub struct Time;

impl Time {
    /// Create a `Time` from its components.
    #[inline(always)]
    pub const fn from_hms_nanos_unchecked(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> crate::Time {
        crate::Time {
            hour,
            minute,
            second,
            nanosecond,
        }
    }
}

pub struct Date;

impl Date {
    // macros
    #[inline(always)]
    pub const fn from_yo_unchecked(year: i32, ordinal: u16) -> crate::Date {
        crate::Date { year, ordinal }
    }

    // reduce duplication
    #[inline]
    pub(crate) const fn from_ymd_unchecked(year: i32, month: u8, day: u8) -> crate::Date {
        /// Cumulative days through the beginning of a month in both common and
        /// leap years.
        const DAYS_CUMULATIVE_COMMON_LEAP: [[u16; 12]; 2] = [
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        let ordinal = DAYS_CUMULATIVE_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1];

        crate::Date {
            year,
            ordinal: ordinal + day as u16,
        }
    }

    // reduce duplication
    #[inline]
    pub(crate) fn from_iso_ywd_unchecked(year: i32, week: u8, weekday: Weekday) -> crate::Date {
        let ordinal = week as u16 * 7 + weekday.iso_weekday_number() as u16
            - (Self::from_yo_unchecked(year, 4)
                .weekday()
                .iso_weekday_number() as u16
                + 3);

        if ordinal < 1 {
            return Self::from_yo_unchecked(year - 1, ordinal + days_in_year(year - 1));
        }

        let days_in_cur_year = days_in_year(year);
        if ordinal > days_in_cur_year {
            Self::from_yo_unchecked(year + 1, ordinal - days_in_cur_year)
        } else {
            Self::from_yo_unchecked(year, ordinal)
        }
    }
}
