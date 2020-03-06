use crate::{
    format::parse::{parse, ParsedItems},
    internal_prelude::*,
    internals,
};
use core::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::{self, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};

// Some methods could be `const fn` due to the internal structure of `Date`, but
// are explicitly not (and have linting disabled) as it could lead to
// compatibility issues down the road if the internal structure is changed.

/// The number of days in a month in both common and leap years.
const DAYS_IN_MONTH_COMMON_LEAP: [[u16; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

/// Get the number of days in the month of a given year.
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
const fn days_in_year_month(year: i32, month: u8) -> u8 {
    DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1] as u8
}

/// Returns if the provided year is a leap year in the proleptic Gregorian
/// calendar. Uses [astronomical year numbering](https://en.wikipedia.org/wiki/Astronomical_year_numbering).
///
/// ```rust
/// # use time::is_leap_year;
/// assert!(!is_leap_year(1900));
/// assert!(is_leap_year(2000));
/// assert!(is_leap_year(2004));
/// assert!(!is_leap_year(2005));
/// assert!(!is_leap_year(2100));
/// ```
#[inline(always)]
pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) & ((year % 100 != 0) | (year % 400 == 0))
}

/// Get the number of calendar days in a given year.
///
/// The returned value will always be either 365 or 366.
///
/// ```rust
/// # use time::days_in_year;
/// assert_eq!(days_in_year(1900), 365);
/// assert_eq!(days_in_year(2000), 366);
/// assert_eq!(days_in_year(2004), 366);
/// assert_eq!(days_in_year(2005), 365);
/// assert_eq!(days_in_year(2100), 365);
/// ```
#[inline(always)]
pub const fn days_in_year(year: i32) -> u16 {
    365 + is_leap_year(year) as u16
}

/// Get the number of weeks in the ISO year.
///
/// The returned value will always be either 52 or 53.
///
/// ```rust
/// # use time::weeks_in_year;
/// assert_eq!(weeks_in_year(2019), 52);
/// assert_eq!(weeks_in_year(2020), 53);
/// ```
#[inline(always)]
pub fn weeks_in_year(year: i32) -> u8 {
    let weekday = internals::Date::from_yo_unchecked(year, 1).weekday();

    if (weekday == Thursday) || (weekday == Wednesday && is_leap_year(year)) {
        53
    } else {
        52
    }
}

/// The minimum valid year.
pub(crate) const MIN_YEAR: i32 = -100_000;
/// The maximum valid year.
pub(crate) const MAX_YEAR: i32 = 100_000;

/// Calendar date.
///
/// Years between `-100_000` and `+100_000` inclusive are guaranteed to be
/// representable. Any values outside this range may have incidental support
/// that can change at any time without notice. If you need support outside this
/// range, please [file an issue](https://github.com/time-rs/time/issues/new)
/// with your use case.
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    serde,
    serde(try_from = "crate::serde::Date", into = "crate::serde::Date")
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) year: i32,
    /// The day of the year.
    ///
    /// - 1 January => 1
    /// - 31 December => 365/366
    pub(crate) ordinal: u16,
}

impl Date {
    /// Create a `Date` from the year, month, and day.
    ///
    /// ```rust
    /// # use time::{Date, date};
    /// assert_eq!(Date::from_ymd(2019, 1, 1), date!(2019-001));
    /// assert_eq!(Date::from_ymd(2019, 12, 31), date!(2019-365));
    /// ```
    ///
    /// Panics if the date is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Date;
    /// Date::from_ymd(2019, 2, 29); // 2019 isn't a leap year.
    /// ```
    #[inline]
    #[cfg(panicking_api)]
    #[cfg_attr(docs, doc(cfg(feature = "panicking-api")))]
    #[deprecated(
        since = "0.2.3",
        note = "For dates knowable at compile-time, use the `date!` macro. For situations where a \
                value isn't known, use `Date::try_from_ymd`."
    )]
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Self {
        assert_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        assert_value_in_range!(month in 1 => 12);
        assert_value_in_range!(day in 1 => days_in_year_month(year, month), given year, month);

        internals::Date::from_ymd_unchecked(year, month, day)
    }

    /// Attempt to create a `Date` from the year, month, and day.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_ymd(2019, 1, 1).is_ok());
    /// assert!(Date::try_from_ymd(2019, 12, 31).is_ok());
    /// ```
    ///
    /// Returns `None` if the date is not valid.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_ymd(2019, 2, 29).is_err()); // 2019 isn't a leap year.
    /// ```
    #[inline]
    pub fn try_from_ymd(year: i32, month: u8, day: u8) -> Result<Self, ComponentRangeError> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(month in 1 => 12);
        ensure_value_in_range!(day in 1 => days_in_year_month(year, month), given year, month);

        Ok(internals::Date::from_ymd_unchecked(year, month, day))
    }

    /// Create a `Date` from the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::{Date, date};
    /// assert_eq!(Date::from_yo(2019, 1), date!(2019-01-01));
    /// assert_eq!(Date::from_yo(2019, 365), date!(2019-12-31));
    /// ```
    ///
    /// Panics if the date is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Date;
    /// Date::from_yo(2019, 366); // 2019 isn't a leap year.
    /// ```
    #[inline(always)]
    #[cfg(panicking_api)]
    #[cfg_attr(docs, doc(cfg(feature = "panicking-api")))]
    #[deprecated(
        since = "0.2.3",
        note = "For dates knowable at compile-time, use the `date!` macro. For situations where a \
                value isn't known, use `Date::try_from_yo`."
    )]
    pub fn from_yo(year: i32, ordinal: u16) -> Self {
        assert_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        assert_value_in_range!(ordinal in 1 => days_in_year(year), given year);
        Self { year, ordinal }
    }

    /// Attempt to create a `Date` from the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_yo(2019, 1).is_ok());
    /// assert!(Date::try_from_yo(2019, 365).is_ok());
    /// ```
    ///
    /// Returns `None` if the date is not valid.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_yo(2019, 366).is_err()); // 2019 isn't a leap year.
    /// ```
    #[inline(always)]
    pub fn try_from_yo(year: i32, ordinal: u16) -> Result<Self, ComponentRangeError> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(ordinal in 1 => days_in_year(year), given year);
        Ok(Self { year, ordinal })
    }

    /// Create a `Date` from the ISO year, week, and weekday.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*, date};
    /// assert_eq!(
    ///     Date::from_iso_ywd(2019, 1, Monday),
    ///     date!(2018-12-31)
    /// );
    /// assert_eq!(
    ///     Date::from_iso_ywd(2019, 1, Tuesday),
    ///     date!(2019-01-01)
    /// );
    /// assert_eq!(
    ///     Date::from_iso_ywd(2020, 53, Friday),
    ///     date!(2021-01-01)
    /// );
    /// ```
    ///
    /// Panics if the week is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::{Date, Weekday::*};
    /// Date::from_iso_ywd(2019, 53, Monday); // 2019 doesn't have 53 weeks.
    /// ```
    #[inline]
    #[cfg(panicking_api)]
    #[cfg_attr(docs, doc(cfg(feature = "panicking-api")))]
    #[deprecated(
        since = "0.2.3",
        note = "For dates knowable at compile-time, use the `date!` macro. For situations where a \
                value isn't known, use `Date::try_from_iso_ywd`."
    )]
    pub fn from_iso_ywd(year: i32, week: u8, weekday: Weekday) -> Self {
        assert_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        assert_value_in_range!(week in 1 => weeks_in_year(year), given year);
        internals::Date::from_iso_ywd_unchecked(year, week, weekday)
    }

    /// Attempt to create a `Date` from the ISO year, week, and weekday.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::try_from_iso_ywd(2019, 1, Monday).is_ok());
    /// assert!(Date::try_from_iso_ywd(2019, 1, Tuesday).is_ok());
    /// assert!(Date::try_from_iso_ywd(2020, 53, Friday).is_ok());
    /// ```
    ///
    /// Returns `None` if the week is not valid.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::try_from_iso_ywd(2019, 53, Monday).is_err()); // 2019 doesn't have 53 weeks.
    /// ```
    #[inline]
    pub fn try_from_iso_ywd(
        year: i32,
        week: u8,
        weekday: Weekday,
    ) -> Result<Self, ComponentRangeError> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(week in 1 => weeks_in_year(year), given year);
        Ok(internals::Date::from_iso_ywd_unchecked(year, week, weekday))
    }

    /// Create a `Date` representing the current date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::today().year() >= 2019);
    /// ```
    #[inline(always)]
    #[cfg(std)]
    #[cfg_attr(docs, doc(cfg(feature = "std")))]
    #[deprecated(
        since = "0.2.7",
        note = "This method returns a value that assumes an offset of UTC."
    )]
    #[allow(deprecated)]
    pub fn today() -> Self {
        PrimitiveDateTime::now().date()
    }

    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).year(), 2019);
    /// assert_eq!(date!(2019-12-31).year(), 2019);
    /// assert_eq!(date!(2020-01-01).year(), 2020);
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn year(self) -> i32 {
        self.year
    }

    /// Get the month. If fetching both the month and day, it is more efficient
    /// to use [`Date::month_day`].
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).month(), 1);
    /// assert_eq!(date!(2019-12-31).month(), 12);
    /// ```
    #[inline(always)]
    pub fn month(self) -> u8 {
        self.month_day().0
    }

    /// Get the day of the month. If fetching both the month and day, it is more
    /// efficient to use [`Date::month_day`].
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).day(), 1);
    /// assert_eq!(date!(2019-12-31).day(), 31);
    /// ```
    #[inline(always)]
    pub fn day(self) -> u8 {
        self.month_day().1
    }

    /// Get the month and day. This is more efficient than fetching the
    /// components individually.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).month_day(), (1, 1));
    /// assert_eq!(date!(2019-12-31).month_day(), (12, 31));
    /// ```
    // For whatever reason, rustc has difficulty optimizing this function. It's
    // significantly faster to write the statements out by hand.
    #[inline]
    pub fn month_day(self) -> (u8, u8) {
        /// The number of days up to and including the given month. Common years
        /// are first, followed by leap years.
        #[allow(clippy::items_after_statements)]
        const CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP: [[u16; 11]; 2] = [
            [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        let days = CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(self.year) as usize];
        let ordinal = self.ordinal;

        #[allow(clippy::cast_possible_truncation)]
        {
            if ordinal > days[10] {
                (12, (ordinal - days[10]) as u8)
            } else if ordinal > days[9] {
                (11, (ordinal - days[9]) as u8)
            } else if ordinal > days[8] {
                (10, (ordinal - days[8]) as u8)
            } else if ordinal > days[7] {
                (9, (ordinal - days[7]) as u8)
            } else if ordinal > days[6] {
                (8, (ordinal - days[6]) as u8)
            } else if ordinal > days[5] {
                (7, (ordinal - days[5]) as u8)
            } else if ordinal > days[4] {
                (6, (ordinal - days[4]) as u8)
            } else if ordinal > days[3] {
                (5, (ordinal - days[3]) as u8)
            } else if ordinal > days[2] {
                (4, (ordinal - days[2]) as u8)
            } else if ordinal > days[1] {
                (3, (ordinal - days[1]) as u8)
            } else if ordinal > days[0] {
                (2, (ordinal - days[0]) as u8)
            } else {
                (1, ordinal as u8)
            }
        }
    }

    /// Get the day of the year.
    ///
    /// The returned value will always be in the range `1..=366` (`1..=365` for
    /// common years).
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).ordinal(), 1);
    /// assert_eq!(date!(2019-12-31).ordinal(), 365);
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn ordinal(self) -> u16 {
        self.ordinal
    }

    /// Get the ISO 8601 year and week number.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).iso_year_week(), (2019, 1));
    /// assert_eq!(date!(2019-10-04).iso_year_week(), (2019, 40));
    /// assert_eq!(date!(2020-01-01).iso_year_week(), (2020, 1));
    /// assert_eq!(date!(2020-12-31).iso_year_week(), (2020, 53));
    /// assert_eq!(date!(2021-01-01).iso_year_week(), (2020, 53));
    /// ```
    #[inline]
    pub fn iso_year_week(self) -> (i32, u8) {
        let weekday = self.weekday();
        #[allow(clippy::cast_possible_truncation)]
        let week = ((self.ordinal + 10 - weekday.iso_weekday_number() as u16) / 7) as u8;

        match week {
            0 => (self.year - 1, weeks_in_year(self.year - 1)),
            53 if weeks_in_year(self.year) == 52 => (self.year + 1, 1),
            _ => (self.year, week),
        }
    }

    /// Get the ISO week number.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).week(), 1);
    /// assert_eq!(date!(2019-10-04).week(), 40);
    /// assert_eq!(date!(2020-01-01).week(), 1);
    /// assert_eq!(date!(2020-12-31).week(), 53);
    /// assert_eq!(date!(2021-01-01).week(), 53);
    /// ```
    #[inline(always)]
    pub fn week(self) -> u8 {
        self.iso_year_week().1
    }

    /// Get the week number where week 1 begins on the first Sunday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).sunday_based_week(), 0);
    /// assert_eq!(date!(2020-01-01).sunday_based_week(), 0);
    /// assert_eq!(date!(2020-12-31).sunday_based_week(), 52);
    /// assert_eq!(date!(2021-01-01).sunday_based_week(), 0);
    /// ```
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn sunday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.weekday().number_days_from_sunday() as i16 + 6) / 7) as u8
    }

    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).monday_based_week(), 0);
    /// assert_eq!(date!(2020-01-01).monday_based_week(), 0);
    /// assert_eq!(date!(2020-12-31).monday_based_week(), 52);
    /// assert_eq!(date!(2021-01-01).monday_based_week(), 0);
    /// ```
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn monday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.weekday().number_days_from_monday() as i16 + 6) / 7) as u8
    }

    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).as_ymd(), (2019, 1, 1));
    /// ```
    #[inline(always)]
    pub fn as_ymd(self) -> (i32, u8, u8) {
        let (month, day) = self.month_day();
        (self.year, month, day)
    }

    /// Get the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).as_yo(), (2019, 1));
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_yo(self) -> (i32, u16) {
        (self.year, self.ordinal)
    }

    /// Get the weekday.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
    ///
    /// ```rust
    /// # use time::{date, Weekday::*};
    /// assert_eq!(date!(2019-01-01).weekday(), Tuesday);
    /// assert_eq!(date!(2019-02-01).weekday(), Friday);
    /// assert_eq!(date!(2019-03-01).weekday(), Friday);
    /// assert_eq!(date!(2019-04-01).weekday(), Monday);
    /// assert_eq!(date!(2019-05-01).weekday(), Wednesday);
    /// assert_eq!(date!(2019-06-01).weekday(), Saturday);
    /// assert_eq!(date!(2019-07-01).weekday(), Monday);
    /// assert_eq!(date!(2019-08-01).weekday(), Thursday);
    /// assert_eq!(date!(2019-09-01).weekday(), Sunday);
    /// assert_eq!(date!(2019-10-01).weekday(), Tuesday);
    /// assert_eq!(date!(2019-11-01).weekday(), Friday);
    /// assert_eq!(date!(2019-12-01).weekday(), Sunday);
    /// ```
    #[inline]
    pub fn weekday(self) -> Weekday {
        let (month, day) = self.month_day();

        let (month, adjusted_year) = if month < 3 {
            (month + 12, self.year - 1)
        } else {
            (month, self.year)
        };

        match (day as i32 + (13 * (month as i32 + 1)) / 5 + adjusted_year + adjusted_year / 4
            - adjusted_year / 100
            + adjusted_year / 400)
            .rem_euclid(7)
        {
            0 => Saturday,
            1 => Sunday,
            2 => Monday,
            3 => Tuesday,
            4 => Wednesday,
            5 => Thursday,
            6 => Friday,
            // FIXME The compiler isn't able to optimize this away. See
            // rust-lang/rust#66993.
            _ => unreachable!("A value mod 7 is always in the range 0..7"),
        }
    }

    /// Get the next calendar date.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).next_day(), date!(2019-01-02));
    /// assert_eq!(date!(2019-01-31).next_day(), date!(2019-02-01));
    /// assert_eq!(date!(2019-12-31).next_day(), date!(2020-01-01));
    /// ```
    #[inline(always)]
    pub fn next_day(mut self) -> Self {
        self.ordinal += 1;

        if self.ordinal > days_in_year(self.year) {
            self.year += 1;
            self.ordinal = 1;
        }

        if self.year > MAX_YEAR {
            panic!("overflow when fetching next day");
        }

        self
    }

    /// Get the previous calendar date.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-02).previous_day(), date!(2019-01-01));
    /// assert_eq!(date!(2019-02-01).previous_day(), date!(2019-01-31));
    /// assert_eq!(date!(2020-01-01).previous_day(), date!(2019-12-31));
    /// ```
    #[inline(always)]
    pub fn previous_day(mut self) -> Self {
        self.ordinal -= 1;

        if self.ordinal == 0 {
            self.year -= 1;
            self.ordinal = days_in_year(self.year);
        }

        if self.year < MIN_YEAR {
            panic!("overflow when fetching previous day");
        }

        self
    }

    /// Get the Julian day for the date.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(-4713-11-24).julian_day(), 0);
    /// assert_eq!(date!(2000-01-01).julian_day(), 2_451_545);
    /// assert_eq!(date!(2019-01-01).julian_day(), 2_458_485);
    /// assert_eq!(date!(2019-12-31).julian_day(), 2_458_849);
    /// ```
    #[inline]
    pub fn julian_day(self) -> i64 {
        let year = self.year as i64;
        let (month, day) = self.month_day();
        let month = month as i64;
        let day = day as i64;
        (1_461 * (year + 4_800 + (month - 14) / 12)) / 4
            + (367 * (month - 2 - 12 * ((month - 14) / 12))) / 12
            - (3 * ((year + 4_900 + (month - 14) / 12) / 100)) / 4
            + day
            - 32_075
    }

    /// Create a `Date` from the Julian day.
    ///
    /// The algorithm to perform this conversion comes from E.G. Richards; it is
    /// available in [section 15.11.3](https://aa.usno.navy.mil/publications/docs/c15_usb_online.pdf),
    /// courtesy of the United States Naval Observatory.
    ///
    /// ```rust
    /// # use time::{Date, date};
    /// assert_eq!(
    ///     Date::from_julian_day(0),
    ///     date!(-4713-11-24)
    /// );
    /// assert_eq!(Date::from_julian_day(2_451_545), date!(2000-01-01));
    /// assert_eq!(Date::from_julian_day(2_458_485), date!(2019-01-01));
    /// assert_eq!(Date::from_julian_day(2_458_849), date!(2019-12-31));
    /// ```
    // TODO Return a `Result<Self, ComponentRangeError>` in 0.3
    #[inline]
    pub fn from_julian_day(julian_day: i64) -> Self {
        #![allow(clippy::missing_docs_in_private_items)]
        const Y: i64 = 4_716;
        const J: i64 = 1_401;
        const M: i64 = 2;
        const N: i64 = 12;
        const R: i64 = 4;
        const P: i64 = 1_461;
        const V: i64 = 3;
        const U: i64 = 5;
        const S: i64 = 153;
        const W: i64 = 2;
        const B: i64 = 274_277;
        const C: i64 = -38;

        let f = julian_day + J + (((4 * julian_day + B) / 146_097) * 3) / 4 + C;
        let e = R * f + V;
        let g = e.rem_euclid(P) / R;
        let h = U * g + W;
        let day = h.rem_euclid(S) / U + 1;
        let month = (h / S + M).rem_euclid(N) + 1;
        let year = (e / P) - Y + (N + M - month) / N;

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        match Date::try_from_ymd(year as i32, month as u8, day as u8) {
            Ok(date) => date,
            Err(err) => panic!("{}", err),
        }
    }
}

/// Methods to add a `Time` component, resulting in a `PrimitiveDateTime`.
impl Date {
    /// Create a `PrimitiveDateTime` using the existing date. The `Time` component will
    /// be set to midnight.
    ///
    /// ```rust
    /// # use time::{date, PrimitiveDateTime, Time};
    /// assert_eq!(
    ///     date!(1970-01-01).midnight(),
    ///     PrimitiveDateTime::unix_epoch()
    /// );
    /// ```
    #[inline(always)]
    pub const fn midnight(self) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::midnight())
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided `Time`.
    ///
    /// ```rust
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_time(time!(0:00)),
    ///     date!(1970-01-01).midnight(),
    /// );
    /// ```
    #[inline(always)]
    pub const fn with_time(self, time: Time) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, time)
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_hms(0, 0, 0),
    ///     date!(1970-01-01).with_time(time!(0:00)),
    /// );
    /// ```
    #[inline(always)]
    #[cfg(panicking_api)]
    #[cfg_attr(docs, doc(cfg(feature = "panicking-api")))]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.3",
        note = "For times knowable at compile-time, use the `time!` macro and `Date::with_time`. \
                For situations where a value isn't known, use `Date::try_with_hms`."
    )]
    pub fn with_hms(self, hour: u8, minute: u8, second: u8) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::from_hms(hour, minute, second))
    }

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the
    /// provided time.
    ///
    /// ```rust
    /// # use time::date;
    /// assert!(date!(1970-01-01).try_with_hms(0, 0, 0).is_ok());
    /// assert!(date!(1970-01-01).try_with_hms(24, 0, 0).is_err());
    /// ```
    #[inline(always)]
    pub fn try_with_hms(
        self,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<PrimitiveDateTime, ComponentRangeError> {
        Ok(PrimitiveDateTime::new(
            self,
            Time::try_from_hms(hour, minute, second)?,
        ))
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided
    /// time.
    ///
    /// ```rust
    /// # use time::{date, Time, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_hms_milli(0, 0, 0, 0),
    ///     date!(1970-01-01).with_time(time!(0:00)),
    /// );
    /// ```
    #[inline(always)]
    #[cfg(panicking_api)]
    #[cfg_attr(docs, doc(cfg(feature = "panicking-api")))]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.3",
        note = "For times knowable at compile-time, use the `time!` macro and `Date::with_time`. \
                For situations where a value isn't known, use `Date::try_with_hms_milli`."
    )]
    pub fn with_hms_milli(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> PrimitiveDateTime {
        PrimitiveDateTime::new(
            self,
            Time::from_hms_milli(hour, minute, second, millisecond),
        )
    }

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::date;
    /// assert!(date!(1970-01-01).try_with_hms_milli(0, 0, 0, 0).is_ok());
    /// assert!(date!(1970-01-01).try_with_hms_milli(24, 0, 0, 0).is_err());
    /// ```
    #[inline(always)]
    pub fn try_with_hms_milli(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> Result<PrimitiveDateTime, ComponentRangeError> {
        Ok(PrimitiveDateTime::new(
            self,
            Time::try_from_hms_milli(hour, minute, second, millisecond)?,
        ))
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{date, Time, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_hms_micro(0, 0, 0, 0),
    ///     date!(1970-01-01).with_time(time!(0:00)),
    /// );
    /// ```
    #[inline(always)]
    #[cfg(panicking_api)]
    #[cfg_attr(docs, doc(cfg(feature = "panicking-api")))]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.3",
        note = "For times knowable at compile-time, use the `time!` macro and `Date::with_time`. \
                For situations where a value isn't known, use `Date::try_with_hms_micro`."
    )]
    pub fn with_hms_micro(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> PrimitiveDateTime {
        PrimitiveDateTime::new(
            self,
            Time::from_hms_micro(hour, minute, second, microsecond),
        )
    }

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the
    /// provided time.
    ///
    /// ```rust
    /// # use time::date;
    /// assert!(date!(1970-01-01)
    ///     .try_with_hms_micro(0, 0, 0, 0)
    ///     .is_ok());
    /// assert!(date!(1970-01-01)
    ///     .try_with_hms_micro(24, 0, 0, 0)
    ///     .is_err());
    /// ```
    #[inline(always)]
    pub fn try_with_hms_micro(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> Result<PrimitiveDateTime, ComponentRangeError> {
        Ok(PrimitiveDateTime::new(
            self,
            Time::try_from_hms_micro(hour, minute, second, microsecond)?,
        ))
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_hms_nano(0, 0, 0, 0),
    ///     date!(1970-01-01).with_time(time!(0:00)),
    /// );
    /// ```
    #[inline(always)]
    #[cfg(panicking_api)]
    #[cfg_attr(docs, doc(cfg(feature = "panicking-api")))]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.3",
        note = "For times knowable at compile-time, use the `time!` macro and `Date::with_time`. \
                For situations where a value isn't known, use `Date::try_with_hms_nano`."
    )]
    pub fn with_hms_nano(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::from_hms_nano(hour, minute, second, nanosecond))
    }

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::date;
    /// assert!(date!(1970-01-01).try_with_hms_nano(0, 0, 0, 0).is_ok());
    /// assert!(date!(1970-01-01).try_with_hms_nano(24, 0, 0, 0).is_err());
    /// ```
    #[inline(always)]
    pub fn try_with_hms_nano(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Result<PrimitiveDateTime, ComponentRangeError> {
        Ok(PrimitiveDateTime::new(
            self,
            Time::try_from_hms_nano(hour, minute, second, nanosecond)?,
        ))
    }
}

/// Methods that allow formatting the `Date`.
impl Date {
    /// Format the `Date` using the provided string.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-02).format("%Y-%m-%d"), "2019-01-02");
    /// ```
    #[inline(always)]
    pub fn format(self, format: impl AsRef<str>) -> String {
        DeferredFormat::new(format.as_ref())
            .with_date(self)
            .to_string()
    }

    /// Attempt to parse a `Date` using the provided string.
    ///
    /// ```rust
    /// # use time::{Date, date};
    /// assert_eq!(
    ///     Date::parse("2019-01-02", "%F"),
    ///     Ok(date!(2019-01-02))
    /// );
    /// assert_eq!(
    ///     Date::parse("2019-002", "%Y-%j"),
    ///     Ok(date!(2019-002))
    /// );
    /// assert_eq!(
    ///     Date::parse("2019-W01-3", "%G-W%V-%u"),
    ///     Ok(date!(2019-W01-3))
    /// );
    /// ```
    #[inline(always)]
    pub fn parse(s: impl AsRef<str>, format: impl AsRef<str>) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s.as_ref(), format.as_ref())?)
    }

    /// Given the items already parsed, attempt to create a `Date`.
    #[inline]
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        macro_rules! items {
            ($($item:ident),* $(,)?) => {
                ParsedItems { $($item: Some($item)),*, .. }
            };
        }

        /// Get the value needed to adjust the ordinal day for Sunday and
        /// Monday-based week numbering.
        #[inline(always)]
        fn adjustment(year: i32) -> i16 {
            match internals::Date::from_yo_unchecked(year, 1).weekday() {
                Monday => 7,
                Tuesday => 1,
                Wednesday => 2,
                Thursday => 3,
                Friday => 4,
                Saturday => 5,
                Sunday => 6,
            }
        }

        match items {
            items!(year, month, day) => {
                Date::try_from_ymd(year, month.get(), day.get()).map_err(Into::into)
            }
            items!(year, ordinal_day) => {
                Date::try_from_yo(year, ordinal_day.get()).map_err(Into::into)
            }
            items!(week_based_year, iso_week, weekday) => {
                Date::try_from_iso_ywd(week_based_year, iso_week.get(), weekday).map_err(Into::into)
            }
            items!(year, sunday_week, weekday) => Date::try_from_yo(
                year,
                #[allow(clippy::cast_sign_loss)]
                {
                    (sunday_week as i16 * 7 + weekday.number_days_from_sunday() as i16
                        - adjustment(year)
                        + 1) as u16
                },
            )
            .map_err(Into::into),
            items!(year, monday_week, weekday) => Date::try_from_yo(
                year,
                #[allow(clippy::cast_sign_loss)]
                {
                    (monday_week as i16 * 7 + weekday.number_days_from_monday() as i16
                        - adjustment(year)
                        + 1) as u16
                },
            )
            .map_err(Into::into),
            _ => Err(ParseError::InsufficientInformation),
        }
    }
}

impl Display for Date {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::format::{date, Padding};

        date::fmt_Y(f, *self, Padding::Zero)?;
        f.write_str("-")?;
        date::fmt_m(f, *self, Padding::Zero)?;
        f.write_str("-")?;
        date::fmt_d(f, *self, Padding::Zero)?;

        Ok(())
    }
}

impl Add<Duration> for Date {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: Duration) -> Self::Output {
        Self::from_julian_day(self.julian_day() + duration.whole_days())
    }
}

impl Add<StdDuration> for Date {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(self.julian_day() + (duration.as_secs() / 86_400) as i64)
    }
}

impl AddAssign<Duration> for Date {
    #[inline(always)]
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for Date {
    #[inline(always)]
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Date {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl Sub<StdDuration> for Date {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(self.julian_day() - (duration.as_secs() / 86_400) as i64)
    }
}

impl SubAssign<Duration> for Date {
    #[inline(always)]
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for Date {
    #[inline(always)]
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<Date> for Date {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Duration::days(self.julian_day() - other.julian_day())
    }
}

impl PartialOrd for Date {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.ordinal.cmp(&other.ordinal),
        }
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(date)]
mod test {
    use super::*;

    macro_rules! julian {
        ($julian:literal) => {
            Date::from_julian_day($julian)
        };
    }

    #[test]
    fn weeks_in_year_exhaustive() {
        let years_with_53 = &[
            4, 9, 15, 20, 26, 32, 37, 43, 48, 54, 60, 65, 71, 76, 82, 88, 93, 99, 105, 111, 116,
            122, 128, 133, 139, 144, 150, 156, 161, 167, 172, 178, 184, 189, 195, 201, 207, 212,
            218, 224, 229, 235, 240, 246, 252, 257, 263, 268, 274, 280, 285, 291, 296, 303, 308,
            314, 320, 325, 331, 336, 342, 348, 353, 359, 364, 370, 376, 381, 387, 392, 398,
        ];

        for year in 0..400 {
            assert_eq!(
                super::weeks_in_year(year),
                if years_with_53.contains(&year) {
                    53
                } else {
                    52
                }
            );
        }
    }

    #[test]
    fn test_days_in_year_month() {
        // Common year
        assert_eq!(days_in_year_month(2019, 1), 31);
        assert_eq!(days_in_year_month(2019, 2), 28);
        assert_eq!(days_in_year_month(2019, 3), 31);
        assert_eq!(days_in_year_month(2019, 4), 30);
        assert_eq!(days_in_year_month(2019, 5), 31);
        assert_eq!(days_in_year_month(2019, 6), 30);
        assert_eq!(days_in_year_month(2019, 7), 31);
        assert_eq!(days_in_year_month(2019, 8), 31);
        assert_eq!(days_in_year_month(2019, 9), 30);
        assert_eq!(days_in_year_month(2019, 10), 31);
        assert_eq!(days_in_year_month(2019, 11), 30);
        assert_eq!(days_in_year_month(2019, 12), 31);

        // Leap year
        assert_eq!(days_in_year_month(2020, 1), 31);
        assert_eq!(days_in_year_month(2020, 2), 29);
        assert_eq!(days_in_year_month(2020, 3), 31);
        assert_eq!(days_in_year_month(2020, 4), 30);
        assert_eq!(days_in_year_month(2020, 5), 31);
        assert_eq!(days_in_year_month(2020, 6), 30);
        assert_eq!(days_in_year_month(2020, 7), 31);
        assert_eq!(days_in_year_month(2020, 8), 31);
        assert_eq!(days_in_year_month(2020, 9), 30);
        assert_eq!(days_in_year_month(2020, 10), 31);
        assert_eq!(days_in_year_month(2020, 11), 30);
        assert_eq!(days_in_year_month(2020, 12), 31);
    }

    // Test all dominical letters. For leap years, check the dates
    // immediately preceding and after the leap day.

    #[test]
    #[allow(clippy::zero_prefixed_literal)]
    fn test_monday_based_week() {
        // A
        assert_eq!(date!(2023-01-01).monday_based_week(), 0);
        assert_eq!(date!(2023-01-02).monday_based_week(), 1);
        assert_eq!(date!(2023-01-03).monday_based_week(), 1);
        assert_eq!(date!(2023-01-04).monday_based_week(), 1);
        assert_eq!(date!(2023-01-05).monday_based_week(), 1);
        assert_eq!(date!(2023-01-06).monday_based_week(), 1);
        assert_eq!(date!(2023-01-07).monday_based_week(), 1);

        // B
        assert_eq!(date!(2022-01-01).monday_based_week(), 0);
        assert_eq!(date!(2022-01-02).monday_based_week(), 0);
        assert_eq!(date!(2022-01-03).monday_based_week(), 1);
        assert_eq!(date!(2022-01-04).monday_based_week(), 1);
        assert_eq!(date!(2022-01-05).monday_based_week(), 1);
        assert_eq!(date!(2022-01-06).monday_based_week(), 1);
        assert_eq!(date!(2022-01-07).monday_based_week(), 1);

        // C
        assert_eq!(date!(2021-01-01).monday_based_week(), 0);
        assert_eq!(date!(2021-01-02).monday_based_week(), 0);
        assert_eq!(date!(2021-01-03).monday_based_week(), 0);
        assert_eq!(date!(2021-01-04).monday_based_week(), 1);
        assert_eq!(date!(2021-01-05).monday_based_week(), 1);
        assert_eq!(date!(2021-01-06).monday_based_week(), 1);
        assert_eq!(date!(2021-01-07).monday_based_week(), 1);

        // D
        assert_eq!(date!(2026-01-01).monday_based_week(), 0);
        assert_eq!(date!(2026-01-02).monday_based_week(), 0);
        assert_eq!(date!(2026-01-03).monday_based_week(), 0);
        assert_eq!(date!(2026-01-04).monday_based_week(), 0);
        assert_eq!(date!(2026-01-05).monday_based_week(), 1);
        assert_eq!(date!(2026-01-06).monday_based_week(), 1);
        assert_eq!(date!(2026-01-07).monday_based_week(), 1);

        // E
        assert_eq!(date!(2025-01-01).monday_based_week(), 0);
        assert_eq!(date!(2025-01-02).monday_based_week(), 0);
        assert_eq!(date!(2025-01-03).monday_based_week(), 0);
        assert_eq!(date!(2025-01-04).monday_based_week(), 0);
        assert_eq!(date!(2025-01-05).monday_based_week(), 0);
        assert_eq!(date!(2025-01-06).monday_based_week(), 1);
        assert_eq!(date!(2025-01-07).monday_based_week(), 1);

        // F
        assert_eq!(date!(2019-01-01).monday_based_week(), 0);
        assert_eq!(date!(2019-01-02).monday_based_week(), 0);
        assert_eq!(date!(2019-01-03).monday_based_week(), 0);
        assert_eq!(date!(2019-01-04).monday_based_week(), 0);
        assert_eq!(date!(2019-01-05).monday_based_week(), 0);
        assert_eq!(date!(2019-01-06).monday_based_week(), 0);
        assert_eq!(date!(2019-01-07).monday_based_week(), 1);

        // G
        assert_eq!(date!(2018-01-01).monday_based_week(), 1);
        assert_eq!(date!(2018-01-02).monday_based_week(), 1);
        assert_eq!(date!(2018-01-03).monday_based_week(), 1);
        assert_eq!(date!(2018-01-04).monday_based_week(), 1);
        assert_eq!(date!(2018-01-05).monday_based_week(), 1);
        assert_eq!(date!(2018-01-06).monday_based_week(), 1);
        assert_eq!(date!(2018-01-07).monday_based_week(), 1);

        // AG
        assert_eq!(date!(2012-01-01).monday_based_week(), 0);
        assert_eq!(date!(2012-01-02).monday_based_week(), 1);
        assert_eq!(date!(2012-01-03).monday_based_week(), 1);
        assert_eq!(date!(2012-01-04).monday_based_week(), 1);
        assert_eq!(date!(2012-01-05).monday_based_week(), 1);
        assert_eq!(date!(2012-01-06).monday_based_week(), 1);
        assert_eq!(date!(2012-01-07).monday_based_week(), 1);
        assert_eq!(date!(2012-02-28).monday_based_week(), 9);
        assert_eq!(date!(2012-02-29).monday_based_week(), 9);
        assert_eq!(date!(2012-03-01).monday_based_week(), 9);
        assert_eq!(date!(2012-03-02).monday_based_week(), 9);
        assert_eq!(date!(2012-03-03).monday_based_week(), 9);
        assert_eq!(date!(2012-03-04).monday_based_week(), 9);
        assert_eq!(date!(2012-03-05).monday_based_week(), 10);
        assert_eq!(date!(2012-03-06).monday_based_week(), 10);
        assert_eq!(date!(2012-03-07).monday_based_week(), 10);

        // BA
        assert_eq!(date!(2028-01-01).monday_based_week(), 0);
        assert_eq!(date!(2028-01-02).monday_based_week(), 0);
        assert_eq!(date!(2028-01-03).monday_based_week(), 1);
        assert_eq!(date!(2028-01-04).monday_based_week(), 1);
        assert_eq!(date!(2028-01-05).monday_based_week(), 1);
        assert_eq!(date!(2028-01-06).monday_based_week(), 1);
        assert_eq!(date!(2028-01-07).monday_based_week(), 1);
        assert_eq!(date!(2028-02-28).monday_based_week(), 9);
        assert_eq!(date!(2028-02-29).monday_based_week(), 9);
        assert_eq!(date!(2028-03-01).monday_based_week(), 9);
        assert_eq!(date!(2028-03-02).monday_based_week(), 9);
        assert_eq!(date!(2028-03-03).monday_based_week(), 9);
        assert_eq!(date!(2028-03-04).monday_based_week(), 9);
        assert_eq!(date!(2028-03-05).monday_based_week(), 9);
        assert_eq!(date!(2028-03-06).monday_based_week(), 10);
        assert_eq!(date!(2028-03-07).monday_based_week(), 10);

        // CB
        assert_eq!(date!(2016-01-01).monday_based_week(), 0);
        assert_eq!(date!(2016-01-02).monday_based_week(), 0);
        assert_eq!(date!(2016-01-03).monday_based_week(), 0);
        assert_eq!(date!(2016-01-04).monday_based_week(), 1);
        assert_eq!(date!(2016-01-05).monday_based_week(), 1);
        assert_eq!(date!(2016-01-06).monday_based_week(), 1);
        assert_eq!(date!(2016-01-07).monday_based_week(), 1);
        assert_eq!(date!(2016-02-28).monday_based_week(), 8);
        assert_eq!(date!(2016-02-29).monday_based_week(), 9);
        assert_eq!(date!(2016-03-01).monday_based_week(), 9);
        assert_eq!(date!(2016-03-02).monday_based_week(), 9);
        assert_eq!(date!(2016-03-03).monday_based_week(), 9);
        assert_eq!(date!(2016-03-04).monday_based_week(), 9);
        assert_eq!(date!(2016-03-05).monday_based_week(), 9);
        assert_eq!(date!(2016-03-06).monday_based_week(), 9);
        assert_eq!(date!(2016-03-07).monday_based_week(), 10);

        // DC
        assert_eq!(date!(2032-01-01).monday_based_week(), 0);
        assert_eq!(date!(2032-01-02).monday_based_week(), 0);
        assert_eq!(date!(2032-01-03).monday_based_week(), 0);
        assert_eq!(date!(2032-01-04).monday_based_week(), 0);
        assert_eq!(date!(2032-01-05).monday_based_week(), 1);
        assert_eq!(date!(2032-01-06).monday_based_week(), 1);
        assert_eq!(date!(2032-01-07).monday_based_week(), 1);
        assert_eq!(date!(2032-02-28).monday_based_week(), 8);
        assert_eq!(date!(2032-02-29).monday_based_week(), 8);
        assert_eq!(date!(2032-03-01).monday_based_week(), 9);
        assert_eq!(date!(2032-03-02).monday_based_week(), 9);
        assert_eq!(date!(2032-03-03).monday_based_week(), 9);
        assert_eq!(date!(2032-03-04).monday_based_week(), 9);
        assert_eq!(date!(2032-03-05).monday_based_week(), 9);
        assert_eq!(date!(2032-03-06).monday_based_week(), 9);
        assert_eq!(date!(2032-03-07).monday_based_week(), 9);

        // ED
        assert_eq!(date!(2020-01-01).monday_based_week(), 0);
        assert_eq!(date!(2020-01-02).monday_based_week(), 0);
        assert_eq!(date!(2020-01-03).monday_based_week(), 0);
        assert_eq!(date!(2020-01-04).monday_based_week(), 0);
        assert_eq!(date!(2020-01-05).monday_based_week(), 0);
        assert_eq!(date!(2020-01-06).monday_based_week(), 1);
        assert_eq!(date!(2020-01-07).monday_based_week(), 1);
        assert_eq!(date!(2020-02-28).monday_based_week(), 8);
        assert_eq!(date!(2020-02-29).monday_based_week(), 8);
        assert_eq!(date!(2020-03-01).monday_based_week(), 8);
        assert_eq!(date!(2020-03-02).monday_based_week(), 9);
        assert_eq!(date!(2020-03-03).monday_based_week(), 9);
        assert_eq!(date!(2020-03-04).monday_based_week(), 9);
        assert_eq!(date!(2020-03-05).monday_based_week(), 9);
        assert_eq!(date!(2020-03-06).monday_based_week(), 9);
        assert_eq!(date!(2020-03-07).monday_based_week(), 9);

        // FE
        assert_eq!(date!(2036-01-01).monday_based_week(), 0);
        assert_eq!(date!(2036-01-02).monday_based_week(), 0);
        assert_eq!(date!(2036-01-03).monday_based_week(), 0);
        assert_eq!(date!(2036-01-04).monday_based_week(), 0);
        assert_eq!(date!(2036-01-05).monday_based_week(), 0);
        assert_eq!(date!(2036-01-06).monday_based_week(), 0);
        assert_eq!(date!(2036-01-07).monday_based_week(), 1);
        assert_eq!(date!(2036-02-28).monday_based_week(), 8);
        assert_eq!(date!(2036-02-29).monday_based_week(), 8);
        assert_eq!(date!(2036-03-01).monday_based_week(), 8);
        assert_eq!(date!(2036-03-02).monday_based_week(), 8);
        assert_eq!(date!(2036-03-03).monday_based_week(), 9);
        assert_eq!(date!(2036-03-04).monday_based_week(), 9);
        assert_eq!(date!(2036-03-05).monday_based_week(), 9);
        assert_eq!(date!(2036-03-06).monday_based_week(), 9);
        assert_eq!(date!(2036-03-07).monday_based_week(), 9);

        // GF
        assert_eq!(date!(2024-01-01).monday_based_week(), 1);
        assert_eq!(date!(2024-01-02).monday_based_week(), 1);
        assert_eq!(date!(2024-01-03).monday_based_week(), 1);
        assert_eq!(date!(2024-01-04).monday_based_week(), 1);
        assert_eq!(date!(2024-01-05).monday_based_week(), 1);
        assert_eq!(date!(2024-01-06).monday_based_week(), 1);
        assert_eq!(date!(2024-01-07).monday_based_week(), 1);
        assert_eq!(date!(2024-02-28).monday_based_week(), 9);
        assert_eq!(date!(2024-02-29).monday_based_week(), 9);
        assert_eq!(date!(2024-03-01).monday_based_week(), 9);
        assert_eq!(date!(2024-03-02).monday_based_week(), 9);
        assert_eq!(date!(2024-03-03).monday_based_week(), 9);
        assert_eq!(date!(2024-03-04).monday_based_week(), 10);
        assert_eq!(date!(2024-03-05).monday_based_week(), 10);
        assert_eq!(date!(2024-03-06).monday_based_week(), 10);
        assert_eq!(date!(2024-03-07).monday_based_week(), 10);
    }

    #[test]
    #[allow(clippy::zero_prefixed_literal)]
    fn test_sunday_based_week() {
        // A
        assert_eq!(date!(2023-01-01).sunday_based_week(), 1);
        assert_eq!(date!(2023-01-02).sunday_based_week(), 1);
        assert_eq!(date!(2023-01-03).sunday_based_week(), 1);
        assert_eq!(date!(2023-01-04).sunday_based_week(), 1);
        assert_eq!(date!(2023-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2023-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2023-01-07).sunday_based_week(), 1);

        // B
        assert_eq!(date!(2022-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2022-01-02).sunday_based_week(), 1);
        assert_eq!(date!(2022-01-03).sunday_based_week(), 1);
        assert_eq!(date!(2022-01-04).sunday_based_week(), 1);
        assert_eq!(date!(2022-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2022-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2022-01-07).sunday_based_week(), 1);

        // C
        assert_eq!(date!(2021-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2021-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2021-01-03).sunday_based_week(), 1);
        assert_eq!(date!(2021-01-04).sunday_based_week(), 1);
        assert_eq!(date!(2021-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2021-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2021-01-07).sunday_based_week(), 1);

        // D
        assert_eq!(date!(2026-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2026-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2026-01-03).sunday_based_week(), 0);
        assert_eq!(date!(2026-01-04).sunday_based_week(), 1);
        assert_eq!(date!(2026-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2026-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2026-01-07).sunday_based_week(), 1);

        // E
        assert_eq!(date!(2025-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2025-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2025-01-03).sunday_based_week(), 0);
        assert_eq!(date!(2025-01-04).sunday_based_week(), 0);
        assert_eq!(date!(2025-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2025-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2025-01-07).sunday_based_week(), 1);

        // F
        assert_eq!(date!(2019-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2019-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2019-01-03).sunday_based_week(), 0);
        assert_eq!(date!(2019-01-04).sunday_based_week(), 0);
        assert_eq!(date!(2019-01-05).sunday_based_week(), 0);
        assert_eq!(date!(2019-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2019-01-07).sunday_based_week(), 1);

        // G
        assert_eq!(date!(2018-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2018-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2018-01-03).sunday_based_week(), 0);
        assert_eq!(date!(2018-01-04).sunday_based_week(), 0);
        assert_eq!(date!(2018-01-05).sunday_based_week(), 0);
        assert_eq!(date!(2018-01-06).sunday_based_week(), 0);
        assert_eq!(date!(2018-01-07).sunday_based_week(), 1);

        // AG
        assert_eq!(date!(2012-01-01).sunday_based_week(), 1);
        assert_eq!(date!(2012-01-02).sunday_based_week(), 1);
        assert_eq!(date!(2012-01-03).sunday_based_week(), 1);
        assert_eq!(date!(2012-01-04).sunday_based_week(), 1);
        assert_eq!(date!(2012-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2012-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2012-01-07).sunday_based_week(), 1);
        assert_eq!(date!(2012-02-28).sunday_based_week(), 9);
        assert_eq!(date!(2012-02-29).sunday_based_week(), 9);
        assert_eq!(date!(2012-03-01).sunday_based_week(), 9);
        assert_eq!(date!(2012-03-02).sunday_based_week(), 9);
        assert_eq!(date!(2012-03-03).sunday_based_week(), 9);
        assert_eq!(date!(2012-03-04).sunday_based_week(), 10);
        assert_eq!(date!(2012-03-05).sunday_based_week(), 10);
        assert_eq!(date!(2012-03-06).sunday_based_week(), 10);
        assert_eq!(date!(2012-03-07).sunday_based_week(), 10);

        // BA
        assert_eq!(date!(2028-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2028-01-02).sunday_based_week(), 1);
        assert_eq!(date!(2028-01-03).sunday_based_week(), 1);
        assert_eq!(date!(2028-01-04).sunday_based_week(), 1);
        assert_eq!(date!(2028-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2028-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2028-01-07).sunday_based_week(), 1);
        assert_eq!(date!(2028-02-28).sunday_based_week(), 9);
        assert_eq!(date!(2028-02-29).sunday_based_week(), 9);
        assert_eq!(date!(2028-03-01).sunday_based_week(), 9);
        assert_eq!(date!(2028-03-02).sunday_based_week(), 9);
        assert_eq!(date!(2028-03-03).sunday_based_week(), 9);
        assert_eq!(date!(2028-03-04).sunday_based_week(), 9);
        assert_eq!(date!(2028-03-05).sunday_based_week(), 10);
        assert_eq!(date!(2028-03-06).sunday_based_week(), 10);
        assert_eq!(date!(2028-03-07).sunday_based_week(), 10);

        // CB
        assert_eq!(date!(2016-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2016-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2016-01-03).sunday_based_week(), 1);
        assert_eq!(date!(2016-01-04).sunday_based_week(), 1);
        assert_eq!(date!(2016-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2016-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2016-01-07).sunday_based_week(), 1);
        assert_eq!(date!(2016-02-28).sunday_based_week(), 9);
        assert_eq!(date!(2016-02-29).sunday_based_week(), 9);
        assert_eq!(date!(2016-03-01).sunday_based_week(), 9);
        assert_eq!(date!(2016-03-02).sunday_based_week(), 9);
        assert_eq!(date!(2016-03-03).sunday_based_week(), 9);
        assert_eq!(date!(2016-03-04).sunday_based_week(), 9);
        assert_eq!(date!(2016-03-05).sunday_based_week(), 9);
        assert_eq!(date!(2016-03-06).sunday_based_week(), 10);
        assert_eq!(date!(2016-03-07).sunday_based_week(), 10);

        // DC
        assert_eq!(date!(2032-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2032-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2032-01-03).sunday_based_week(), 0);
        assert_eq!(date!(2032-01-04).sunday_based_week(), 1);
        assert_eq!(date!(2032-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2032-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2032-01-07).sunday_based_week(), 1);
        assert_eq!(date!(2032-02-28).sunday_based_week(), 8);
        assert_eq!(date!(2032-02-29).sunday_based_week(), 9);
        assert_eq!(date!(2032-03-01).sunday_based_week(), 9);
        assert_eq!(date!(2032-03-02).sunday_based_week(), 9);
        assert_eq!(date!(2032-03-03).sunday_based_week(), 9);
        assert_eq!(date!(2032-03-04).sunday_based_week(), 9);
        assert_eq!(date!(2032-03-05).sunday_based_week(), 9);
        assert_eq!(date!(2032-03-06).sunday_based_week(), 9);
        assert_eq!(date!(2032-03-07).sunday_based_week(), 10);

        // ED
        assert_eq!(date!(2020-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2020-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2020-01-03).sunday_based_week(), 0);
        assert_eq!(date!(2020-01-04).sunday_based_week(), 0);
        assert_eq!(date!(2020-01-05).sunday_based_week(), 1);
        assert_eq!(date!(2020-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2020-01-07).sunday_based_week(), 1);
        assert_eq!(date!(2020-02-28).sunday_based_week(), 8);
        assert_eq!(date!(2020-02-29).sunday_based_week(), 8);
        assert_eq!(date!(2020-03-01).sunday_based_week(), 9);
        assert_eq!(date!(2020-03-02).sunday_based_week(), 9);
        assert_eq!(date!(2020-03-03).sunday_based_week(), 9);
        assert_eq!(date!(2020-03-04).sunday_based_week(), 9);
        assert_eq!(date!(2020-03-05).sunday_based_week(), 9);
        assert_eq!(date!(2020-03-06).sunday_based_week(), 9);
        assert_eq!(date!(2020-03-07).sunday_based_week(), 9);

        // FE
        assert_eq!(date!(2036-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2036-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2036-01-03).sunday_based_week(), 0);
        assert_eq!(date!(2036-01-04).sunday_based_week(), 0);
        assert_eq!(date!(2036-01-05).sunday_based_week(), 0);
        assert_eq!(date!(2036-01-06).sunday_based_week(), 1);
        assert_eq!(date!(2036-01-07).sunday_based_week(), 1);
        assert_eq!(date!(2036-02-28).sunday_based_week(), 8);
        assert_eq!(date!(2036-02-29).sunday_based_week(), 8);
        assert_eq!(date!(2036-03-01).sunday_based_week(), 8);
        assert_eq!(date!(2036-03-02).sunday_based_week(), 9);
        assert_eq!(date!(2036-03-03).sunday_based_week(), 9);
        assert_eq!(date!(2036-03-04).sunday_based_week(), 9);
        assert_eq!(date!(2036-03-05).sunday_based_week(), 9);
        assert_eq!(date!(2036-03-06).sunday_based_week(), 9);
        assert_eq!(date!(2036-03-07).sunday_based_week(), 9);

        // GF
        assert_eq!(date!(2024-01-01).sunday_based_week(), 0);
        assert_eq!(date!(2024-01-02).sunday_based_week(), 0);
        assert_eq!(date!(2024-01-03).sunday_based_week(), 0);
        assert_eq!(date!(2024-01-04).sunday_based_week(), 0);
        assert_eq!(date!(2024-01-05).sunday_based_week(), 0);
        assert_eq!(date!(2024-01-06).sunday_based_week(), 0);
        assert_eq!(date!(2024-01-07).sunday_based_week(), 1);
        assert_eq!(date!(2024-02-28).sunday_based_week(), 8);
        assert_eq!(date!(2024-02-29).sunday_based_week(), 8);
        assert_eq!(date!(2024-03-01).sunday_based_week(), 8);
        assert_eq!(date!(2024-03-02).sunday_based_week(), 8);
        assert_eq!(date!(2024-03-03).sunday_based_week(), 9);
        assert_eq!(date!(2024-03-04).sunday_based_week(), 9);
        assert_eq!(date!(2024-03-05).sunday_based_week(), 9);
        assert_eq!(date!(2024-03-06).sunday_based_week(), 9);
        assert_eq!(date!(2024-03-07).sunday_based_week(), 9);
    }

    #[test]
    #[allow(clippy::zero_prefixed_literal)]
    fn test_parse_monday_based_week() -> crate::Result<()> {
        macro_rules! parse {
            ($s:literal) => {
                Date::parse($s, "%a %W %Y")?
            };
        }

        // A
        assert_eq!(parse!("Sun 00 2023"), date!(2023-001));
        assert_eq!(parse!("Mon 01 2023"), date!(2023-002));
        assert_eq!(parse!("Tue 01 2023"), date!(2023-003));
        assert_eq!(parse!("Wed 01 2023"), date!(2023-004));
        assert_eq!(parse!("Thu 01 2023"), date!(2023-005));
        assert_eq!(parse!("Fri 01 2023"), date!(2023-006));
        assert_eq!(parse!("Sat 01 2023"), date!(2023-007));

        // B
        assert_eq!(parse!("Sat 00 2022"), date!(2022-001));
        assert_eq!(parse!("Sun 00 2022"), date!(2022-002));
        assert_eq!(parse!("Mon 01 2022"), date!(2022-003));
        assert_eq!(parse!("Tue 01 2022"), date!(2022-004));
        assert_eq!(parse!("Wed 01 2022"), date!(2022-005));
        assert_eq!(parse!("Thu 01 2022"), date!(2022-006));
        assert_eq!(parse!("Fri 01 2022"), date!(2022-007));

        // C
        assert_eq!(parse!("Fri 00 2021"), date!(2021-001));
        assert_eq!(parse!("Sat 00 2021"), date!(2021-002));
        assert_eq!(parse!("Sun 00 2021"), date!(2021-003));
        assert_eq!(parse!("Mon 01 2021"), date!(2021-004));
        assert_eq!(parse!("Tue 01 2021"), date!(2021-005));
        assert_eq!(parse!("Wed 01 2021"), date!(2021-006));
        assert_eq!(parse!("Thu 01 2021"), date!(2021-007));

        // D
        assert_eq!(parse!("Thu 00 2026"), date!(2026-001));
        assert_eq!(parse!("Fri 00 2026"), date!(2026-002));
        assert_eq!(parse!("Sat 00 2026"), date!(2026-003));
        assert_eq!(parse!("Sun 00 2026"), date!(2026-004));
        assert_eq!(parse!("Mon 01 2026"), date!(2026-005));
        assert_eq!(parse!("Tue 01 2026"), date!(2026-006));
        assert_eq!(parse!("Wed 01 2026"), date!(2026-007));

        // E
        assert_eq!(parse!("Wed 00 2025"), date!(2025-001));
        assert_eq!(parse!("Thu 00 2025"), date!(2025-002));
        assert_eq!(parse!("Fri 00 2025"), date!(2025-003));
        assert_eq!(parse!("Sat 00 2025"), date!(2025-004));
        assert_eq!(parse!("Sun 00 2025"), date!(2025-005));
        assert_eq!(parse!("Mon 01 2025"), date!(2025-006));
        assert_eq!(parse!("Tue 01 2025"), date!(2025-007));

        // F
        assert_eq!(parse!("Tue 00 2019"), date!(2019-001));
        assert_eq!(parse!("Wed 00 2019"), date!(2019-002));
        assert_eq!(parse!("Thu 00 2019"), date!(2019-003));
        assert_eq!(parse!("Fri 00 2019"), date!(2019-004));
        assert_eq!(parse!("Sat 00 2019"), date!(2019-005));
        assert_eq!(parse!("Sun 00 2019"), date!(2019-006));
        assert_eq!(parse!("Mon 01 2019"), date!(2019-007));

        // G
        assert_eq!(parse!("Mon 01 2018"), date!(2018-001));
        assert_eq!(parse!("Tue 01 2018"), date!(2018-002));
        assert_eq!(parse!("Wed 01 2018"), date!(2018-003));
        assert_eq!(parse!("Thu 01 2018"), date!(2018-004));
        assert_eq!(parse!("Fri 01 2018"), date!(2018-005));
        assert_eq!(parse!("Sat 01 2018"), date!(2018-006));
        assert_eq!(parse!("Sun 01 2018"), date!(2018-007));

        // AG
        assert_eq!(parse!("Sun 00 2012"), date!(2012-001));
        assert_eq!(parse!("Mon 01 2012"), date!(2012-002));
        assert_eq!(parse!("Tue 01 2012"), date!(2012-003));
        assert_eq!(parse!("Wed 01 2012"), date!(2012-004));
        assert_eq!(parse!("Thu 01 2012"), date!(2012-005));
        assert_eq!(parse!("Fri 01 2012"), date!(2012-006));
        assert_eq!(parse!("Sat 01 2012"), date!(2012-007));
        assert_eq!(parse!("Tue 09 2012"), date!(2012-059));
        assert_eq!(parse!("Wed 09 2012"), date!(2012-060));
        assert_eq!(parse!("Thu 09 2012"), date!(2012-061));
        assert_eq!(parse!("Fri 09 2012"), date!(2012-062));
        assert_eq!(parse!("Sat 09 2012"), date!(2012-063));
        assert_eq!(parse!("Sun 09 2012"), date!(2012-064));
        assert_eq!(parse!("Mon 10 2012"), date!(2012-065));
        assert_eq!(parse!("Tue 10 2012"), date!(2012-066));
        assert_eq!(parse!("Wed 10 2012"), date!(2012-067));

        // BA
        assert_eq!(parse!("Sat 00 2028"), date!(2028-001));
        assert_eq!(parse!("Sun 00 2028"), date!(2028-002));
        assert_eq!(parse!("Mon 01 2028"), date!(2028-003));
        assert_eq!(parse!("Tue 01 2028"), date!(2028-004));
        assert_eq!(parse!("Wed 01 2028"), date!(2028-005));
        assert_eq!(parse!("Thu 01 2028"), date!(2028-006));
        assert_eq!(parse!("Fri 01 2028"), date!(2028-007));
        assert_eq!(parse!("Mon 09 2028"), date!(2028-059));
        assert_eq!(parse!("Tue 09 2028"), date!(2028-060));
        assert_eq!(parse!("Wed 09 2028"), date!(2028-061));
        assert_eq!(parse!("Thu 09 2028"), date!(2028-062));
        assert_eq!(parse!("Fri 09 2028"), date!(2028-063));
        assert_eq!(parse!("Sat 09 2028"), date!(2028-064));
        assert_eq!(parse!("Sun 09 2028"), date!(2028-065));
        assert_eq!(parse!("Mon 10 2028"), date!(2028-066));
        assert_eq!(parse!("Tue 10 2028"), date!(2028-067));

        // CB
        assert_eq!(parse!("Fri 00 2016"), date!(2016-001));
        assert_eq!(parse!("Sat 00 2016"), date!(2016-002));
        assert_eq!(parse!("Sun 00 2016"), date!(2016-003));
        assert_eq!(parse!("Mon 01 2016"), date!(2016-004));
        assert_eq!(parse!("Tue 01 2016"), date!(2016-005));
        assert_eq!(parse!("Wed 01 2016"), date!(2016-006));
        assert_eq!(parse!("Thu 01 2016"), date!(2016-007));
        assert_eq!(parse!("Sun 08 2016"), date!(2016-059));
        assert_eq!(parse!("Mon 09 2016"), date!(2016-060));
        assert_eq!(parse!("Tue 09 2016"), date!(2016-061));
        assert_eq!(parse!("Wed 09 2016"), date!(2016-062));
        assert_eq!(parse!("Thu 09 2016"), date!(2016-063));
        assert_eq!(parse!("Fri 09 2016"), date!(2016-064));
        assert_eq!(parse!("Sat 09 2016"), date!(2016-065));
        assert_eq!(parse!("Sun 09 2016"), date!(2016-066));
        assert_eq!(parse!("Mon 10 2016"), date!(2016-067));

        // DC
        assert_eq!(parse!("Thu 00 2032"), date!(2032-001));
        assert_eq!(parse!("Fri 00 2032"), date!(2032-002));
        assert_eq!(parse!("Sat 00 2032"), date!(2032-003));
        assert_eq!(parse!("Sun 00 2032"), date!(2032-004));
        assert_eq!(parse!("Mon 01 2032"), date!(2032-005));
        assert_eq!(parse!("Tue 01 2032"), date!(2032-006));
        assert_eq!(parse!("Wed 01 2032"), date!(2032-007));
        assert_eq!(parse!("Sat 08 2032"), date!(2032-059));
        assert_eq!(parse!("Sun 08 2032"), date!(2032-060));
        assert_eq!(parse!("Mon 09 2032"), date!(2032-061));
        assert_eq!(parse!("Tue 09 2032"), date!(2032-062));
        assert_eq!(parse!("Wed 09 2032"), date!(2032-063));
        assert_eq!(parse!("Thu 09 2032"), date!(2032-064));
        assert_eq!(parse!("Fri 09 2032"), date!(2032-065));
        assert_eq!(parse!("Sat 09 2032"), date!(2032-066));
        assert_eq!(parse!("Sun 09 2032"), date!(2032-067));

        // ED
        assert_eq!(parse!("Wed 00 2020"), date!(2020-001));
        assert_eq!(parse!("Thu 00 2020"), date!(2020-002));
        assert_eq!(parse!("Fri 00 2020"), date!(2020-003));
        assert_eq!(parse!("Sat 00 2020"), date!(2020-004));
        assert_eq!(parse!("Sun 00 2020"), date!(2020-005));
        assert_eq!(parse!("Mon 01 2020"), date!(2020-006));
        assert_eq!(parse!("Tue 01 2020"), date!(2020-007));
        assert_eq!(parse!("Fri 08 2020"), date!(2020-059));
        assert_eq!(parse!("Sat 08 2020"), date!(2020-060));
        assert_eq!(parse!("Sun 08 2020"), date!(2020-061));
        assert_eq!(parse!("Mon 09 2020"), date!(2020-062));
        assert_eq!(parse!("Tue 09 2020"), date!(2020-063));
        assert_eq!(parse!("Wed 09 2020"), date!(2020-064));
        assert_eq!(parse!("Thu 09 2020"), date!(2020-065));
        assert_eq!(parse!("Fri 09 2020"), date!(2020-066));
        assert_eq!(parse!("Sat 09 2020"), date!(2020-067));

        // FE
        assert_eq!(parse!("Tue 00 2036"), date!(2036-001));
        assert_eq!(parse!("Wed 00 2036"), date!(2036-002));
        assert_eq!(parse!("Thu 00 2036"), date!(2036-003));
        assert_eq!(parse!("Fri 00 2036"), date!(2036-004));
        assert_eq!(parse!("Sat 00 2036"), date!(2036-005));
        assert_eq!(parse!("Sun 00 2036"), date!(2036-006));
        assert_eq!(parse!("Mon 01 2036"), date!(2036-007));
        assert_eq!(parse!("Thu 08 2036"), date!(2036-059));
        assert_eq!(parse!("Fri 08 2036"), date!(2036-060));
        assert_eq!(parse!("Sat 08 2036"), date!(2036-061));
        assert_eq!(parse!("Sun 08 2036"), date!(2036-062));
        assert_eq!(parse!("Mon 09 2036"), date!(2036-063));
        assert_eq!(parse!("Tue 09 2036"), date!(2036-064));
        assert_eq!(parse!("Wed 09 2036"), date!(2036-065));
        assert_eq!(parse!("Thu 09 2036"), date!(2036-066));
        assert_eq!(parse!("Fri 09 2036"), date!(2036-067));

        // GF
        assert_eq!(parse!("Mon 01 2024"), date!(2024-001));
        assert_eq!(parse!("Tue 01 2024"), date!(2024-002));
        assert_eq!(parse!("Wed 01 2024"), date!(2024-003));
        assert_eq!(parse!("Thu 01 2024"), date!(2024-004));
        assert_eq!(parse!("Fri 01 2024"), date!(2024-005));
        assert_eq!(parse!("Sat 01 2024"), date!(2024-006));
        assert_eq!(parse!("Sun 01 2024"), date!(2024-007));
        assert_eq!(parse!("Wed 09 2024"), date!(2024-059));
        assert_eq!(parse!("Thu 09 2024"), date!(2024-060));
        assert_eq!(parse!("Fri 09 2024"), date!(2024-061));
        assert_eq!(parse!("Sat 09 2024"), date!(2024-062));
        assert_eq!(parse!("Sun 09 2024"), date!(2024-063));
        assert_eq!(parse!("Mon 10 2024"), date!(2024-064));
        assert_eq!(parse!("Tue 10 2024"), date!(2024-065));
        assert_eq!(parse!("Wed 10 2024"), date!(2024-066));
        assert_eq!(parse!("Thu 10 2024"), date!(2024-067));

        Ok(())
    }

    #[test]
    #[allow(clippy::zero_prefixed_literal)]
    fn test_parse_sunday_based_week() -> crate::Result<()> {
        macro_rules! parse {
            ($s:literal) => {
                Date::parse($s, "%a %U %Y")?
            };
        }

        // A
        assert_eq!(parse!("Sun 01 2018"), date!(2018-001));
        assert_eq!(parse!("Mon 01 2018"), date!(2018-002));
        assert_eq!(parse!("Tue 01 2018"), date!(2018-003));
        assert_eq!(parse!("Wed 01 2018"), date!(2018-004));
        assert_eq!(parse!("Thu 01 2018"), date!(2018-005));
        assert_eq!(parse!("Fri 01 2018"), date!(2018-006));
        assert_eq!(parse!("Sat 01 2018"), date!(2018-007));

        // B
        assert_eq!(parse!("Sat 00 2023"), date!(2023-001));
        assert_eq!(parse!("Sun 01 2023"), date!(2023-002));
        assert_eq!(parse!("Mon 01 2023"), date!(2023-003));
        assert_eq!(parse!("Tue 01 2023"), date!(2023-004));
        assert_eq!(parse!("Wed 01 2023"), date!(2023-005));
        assert_eq!(parse!("Thu 01 2023"), date!(2023-006));
        assert_eq!(parse!("Fri 01 2023"), date!(2023-007));

        // C
        assert_eq!(parse!("Fri 00 2022"), date!(2022-001));
        assert_eq!(parse!("Sat 00 2022"), date!(2022-002));
        assert_eq!(parse!("Sun 01 2022"), date!(2022-003));
        assert_eq!(parse!("Mon 01 2022"), date!(2022-004));
        assert_eq!(parse!("Tue 01 2022"), date!(2022-005));
        assert_eq!(parse!("Wed 01 2022"), date!(2022-006));
        assert_eq!(parse!("Thu 01 2022"), date!(2022-007));

        // D
        assert_eq!(parse!("Thu 00 2021"), date!(2021-001));
        assert_eq!(parse!("Fri 00 2021"), date!(2021-002));
        assert_eq!(parse!("Sat 00 2021"), date!(2021-003));
        assert_eq!(parse!("Sun 01 2021"), date!(2021-004));
        assert_eq!(parse!("Mon 01 2021"), date!(2021-005));
        assert_eq!(parse!("Tue 01 2021"), date!(2021-006));
        assert_eq!(parse!("Wed 01 2021"), date!(2021-007));

        // E
        assert_eq!(parse!("Wed 00 2026"), date!(2026-001));
        assert_eq!(parse!("Thu 00 2026"), date!(2026-002));
        assert_eq!(parse!("Fri 00 2026"), date!(2026-003));
        assert_eq!(parse!("Sat 00 2026"), date!(2026-004));
        assert_eq!(parse!("Sun 01 2026"), date!(2026-005));
        assert_eq!(parse!("Mon 01 2026"), date!(2026-006));
        assert_eq!(parse!("Tue 01 2026"), date!(2026-007));

        // F
        assert_eq!(parse!("Tue 00 2025"), date!(2025-001));
        assert_eq!(parse!("Wed 00 2025"), date!(2025-002));
        assert_eq!(parse!("Thu 00 2025"), date!(2025-003));
        assert_eq!(parse!("Fri 00 2025"), date!(2025-004));
        assert_eq!(parse!("Sat 00 2025"), date!(2025-005));
        assert_eq!(parse!("Sun 01 2025"), date!(2025-006));
        assert_eq!(parse!("Mon 01 2025"), date!(2025-007));

        // G
        assert_eq!(parse!("Mon 00 2019"), date!(2019-001));
        assert_eq!(parse!("Tue 00 2019"), date!(2019-002));
        assert_eq!(parse!("Wed 00 2019"), date!(2019-003));
        assert_eq!(parse!("Thu 00 2019"), date!(2019-004));
        assert_eq!(parse!("Fri 00 2019"), date!(2019-005));
        assert_eq!(parse!("Sat 00 2019"), date!(2019-006));
        assert_eq!(parse!("Sun 01 2019"), date!(2019-007));

        // AG
        assert_eq!(parse!("Sun 01 2024"), date!(2024-001));
        assert_eq!(parse!("Mon 01 2024"), date!(2024-002));
        assert_eq!(parse!("Tue 01 2024"), date!(2024-003));
        assert_eq!(parse!("Wed 01 2024"), date!(2024-004));
        assert_eq!(parse!("Thu 01 2024"), date!(2024-005));
        assert_eq!(parse!("Fri 01 2024"), date!(2024-006));
        assert_eq!(parse!("Sat 01 2024"), date!(2024-007));
        assert_eq!(parse!("Tue 09 2024"), date!(2024-059));
        assert_eq!(parse!("Wed 09 2024"), date!(2024-060));
        assert_eq!(parse!("Thu 09 2024"), date!(2024-061));
        assert_eq!(parse!("Fri 09 2024"), date!(2024-062));
        assert_eq!(parse!("Sat 09 2024"), date!(2024-063));
        assert_eq!(parse!("Sun 10 2024"), date!(2024-064));
        assert_eq!(parse!("Mon 10 2024"), date!(2024-065));
        assert_eq!(parse!("Tue 10 2024"), date!(2024-066));
        assert_eq!(parse!("Wed 10 2024"), date!(2024-067));

        // BA
        assert_eq!(parse!("Sat 00 2012"), date!(2012-001));
        assert_eq!(parse!("Sun 01 2012"), date!(2012-002));
        assert_eq!(parse!("Mon 01 2012"), date!(2012-003));
        assert_eq!(parse!("Tue 01 2012"), date!(2012-004));
        assert_eq!(parse!("Wed 01 2012"), date!(2012-005));
        assert_eq!(parse!("Thu 01 2012"), date!(2012-006));
        assert_eq!(parse!("Fri 01 2012"), date!(2012-007));
        assert_eq!(parse!("Mon 09 2012"), date!(2012-059));
        assert_eq!(parse!("Tue 09 2012"), date!(2012-060));
        assert_eq!(parse!("Wed 09 2012"), date!(2012-061));
        assert_eq!(parse!("Thu 09 2012"), date!(2012-062));
        assert_eq!(parse!("Fri 09 2012"), date!(2012-063));
        assert_eq!(parse!("Sat 09 2012"), date!(2012-064));
        assert_eq!(parse!("Sun 10 2012"), date!(2012-065));
        assert_eq!(parse!("Mon 10 2012"), date!(2012-066));
        assert_eq!(parse!("Tue 10 2012"), date!(2012-067));

        // CB
        assert_eq!(parse!("Fri 00 2028"), date!(2028-001));
        assert_eq!(parse!("Sat 00 2028"), date!(2028-002));
        assert_eq!(parse!("Sun 01 2028"), date!(2028-003));
        assert_eq!(parse!("Mon 01 2028"), date!(2028-004));
        assert_eq!(parse!("Tue 01 2028"), date!(2028-005));
        assert_eq!(parse!("Wed 01 2028"), date!(2028-006));
        assert_eq!(parse!("Thu 01 2028"), date!(2028-007));
        assert_eq!(parse!("Sun 09 2028"), date!(2028-059));
        assert_eq!(parse!("Mon 09 2028"), date!(2028-060));
        assert_eq!(parse!("Tue 09 2028"), date!(2028-061));
        assert_eq!(parse!("Wed 09 2028"), date!(2028-062));
        assert_eq!(parse!("Thu 09 2028"), date!(2028-063));
        assert_eq!(parse!("Fri 09 2028"), date!(2028-064));
        assert_eq!(parse!("Sat 09 2028"), date!(2028-065));
        assert_eq!(parse!("Sun 10 2028"), date!(2028-066));
        assert_eq!(parse!("Mon 10 2028"), date!(2028-067));

        // DC
        assert_eq!(parse!("Thu 00 2016"), date!(2016-001));
        assert_eq!(parse!("Fri 00 2016"), date!(2016-002));
        assert_eq!(parse!("Sat 00 2016"), date!(2016-003));
        assert_eq!(parse!("Sun 01 2016"), date!(2016-004));
        assert_eq!(parse!("Mon 01 2016"), date!(2016-005));
        assert_eq!(parse!("Tue 01 2016"), date!(2016-006));
        assert_eq!(parse!("Wed 01 2016"), date!(2016-007));
        assert_eq!(parse!("Sat 08 2016"), date!(2016-059));
        assert_eq!(parse!("Sun 09 2016"), date!(2016-060));
        assert_eq!(parse!("Mon 09 2016"), date!(2016-061));
        assert_eq!(parse!("Tue 09 2016"), date!(2016-062));
        assert_eq!(parse!("Wed 09 2016"), date!(2016-063));
        assert_eq!(parse!("Thu 09 2016"), date!(2016-064));
        assert_eq!(parse!("Fri 09 2016"), date!(2016-065));
        assert_eq!(parse!("Sat 09 2016"), date!(2016-066));
        assert_eq!(parse!("Sun 10 2016"), date!(2016-067));

        // ED
        assert_eq!(parse!("Wed 00 2032"), date!(2032-001));
        assert_eq!(parse!("Thu 00 2032"), date!(2032-002));
        assert_eq!(parse!("Fri 00 2032"), date!(2032-003));
        assert_eq!(parse!("Sat 00 2032"), date!(2032-004));
        assert_eq!(parse!("Sun 01 2032"), date!(2032-005));
        assert_eq!(parse!("Mon 01 2032"), date!(2032-006));
        assert_eq!(parse!("Tue 01 2032"), date!(2032-007));
        assert_eq!(parse!("Fri 08 2032"), date!(2032-059));
        assert_eq!(parse!("Sat 08 2032"), date!(2032-060));
        assert_eq!(parse!("Sun 09 2032"), date!(2032-061));
        assert_eq!(parse!("Mon 09 2032"), date!(2032-062));
        assert_eq!(parse!("Tue 09 2032"), date!(2032-063));
        assert_eq!(parse!("Wed 09 2032"), date!(2032-064));
        assert_eq!(parse!("Thu 09 2032"), date!(2032-065));
        assert_eq!(parse!("Fri 09 2032"), date!(2032-066));
        assert_eq!(parse!("Sat 09 2032"), date!(2032-067));

        // FE
        assert_eq!(parse!("Tue 00 2020"), date!(2020-001));
        assert_eq!(parse!("Wed 00 2020"), date!(2020-002));
        assert_eq!(parse!("Thu 00 2020"), date!(2020-003));
        assert_eq!(parse!("Fri 00 2020"), date!(2020-004));
        assert_eq!(parse!("Sat 00 2020"), date!(2020-005));
        assert_eq!(parse!("Sun 01 2020"), date!(2020-006));
        assert_eq!(parse!("Mon 01 2020"), date!(2020-007));
        assert_eq!(parse!("Thu 08 2020"), date!(2020-059));
        assert_eq!(parse!("Fri 08 2020"), date!(2020-060));
        assert_eq!(parse!("Sat 08 2020"), date!(2020-061));
        assert_eq!(parse!("Sun 09 2020"), date!(2020-062));
        assert_eq!(parse!("Mon 09 2020"), date!(2020-063));
        assert_eq!(parse!("Tue 09 2020"), date!(2020-064));
        assert_eq!(parse!("Wed 09 2020"), date!(2020-065));
        assert_eq!(parse!("Thu 09 2020"), date!(2020-066));
        assert_eq!(parse!("Fri 09 2020"), date!(2020-067));

        // GF
        assert_eq!(parse!("Mon 00 2036"), date!(2036-001));
        assert_eq!(parse!("Tue 00 2036"), date!(2036-002));
        assert_eq!(parse!("Wed 00 2036"), date!(2036-003));
        assert_eq!(parse!("Thu 00 2036"), date!(2036-004));
        assert_eq!(parse!("Fri 00 2036"), date!(2036-005));
        assert_eq!(parse!("Sat 00 2036"), date!(2036-006));
        assert_eq!(parse!("Sun 01 2036"), date!(2036-007));
        assert_eq!(parse!("Wed 08 2036"), date!(2036-059));
        assert_eq!(parse!("Thu 08 2036"), date!(2036-060));
        assert_eq!(parse!("Fri 08 2036"), date!(2036-061));
        assert_eq!(parse!("Sat 08 2036"), date!(2036-062));
        assert_eq!(parse!("Sun 09 2036"), date!(2036-063));
        assert_eq!(parse!("Mon 09 2036"), date!(2036-064));
        assert_eq!(parse!("Tue 09 2036"), date!(2036-065));
        assert_eq!(parse!("Wed 09 2036"), date!(2036-066));
        assert_eq!(parse!("Thu 09 2036"), date!(2036-067));

        Ok(())
    }

    #[test]
    fn is_leap_year() {
        use super::is_leap_year;
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2004));
        assert!(!is_leap_year(2005));
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn days_in_year() {
        use super::days_in_year;
        assert_eq!(days_in_year(1900), 365);
        assert_eq!(days_in_year(2000), 366);
        assert_eq!(days_in_year(2004), 366);
        assert_eq!(days_in_year(2005), 365);
        assert_eq!(days_in_year(2100), 365);
    }

    #[test]
    fn weeks_in_year() {
        use super::weeks_in_year;
        assert_eq!(weeks_in_year(2019), 52);
        assert_eq!(weeks_in_year(2020), 53);
    }

    #[test]
    fn year() {
        assert_eq!(date!(2019-002).year(), 2019);
        assert_eq!(date!(2020-002).year(), 2020);
    }

    #[test]
    fn month() {
        assert_eq!(date!(2019-002).month(), 1);
        assert_eq!(date!(2020-002).month(), 1);
        assert_eq!(date!(2019-060).month(), 3);
        assert_eq!(date!(2020-060).month(), 2);
    }

    #[test]
    fn day() {
        assert_eq!(date!(2019-002).day(), 2);
        assert_eq!(date!(2020-002).day(), 2);
        assert_eq!(date!(2019-060).day(), 1);
        assert_eq!(date!(2020-060).day(), 29);
    }

    #[test]
    fn iso_year_week() {
        assert_eq!(date!(2019-01-01).iso_year_week(), (2019, 1));
        assert_eq!(date!(2019-10-04).iso_year_week(), (2019, 40));
        assert_eq!(date!(2020-01-01).iso_year_week(), (2020, 1));
        assert_eq!(date!(2020-12-31).iso_year_week(), (2020, 53));
        assert_eq!(date!(2021-01-01).iso_year_week(), (2020, 53));
    }

    #[test]
    fn week() {
        assert_eq!(date!(2019-01-01).week(), 1);
        assert_eq!(date!(2019-10-04).week(), 40);
        assert_eq!(date!(2020-01-01).week(), 1);
        assert_eq!(date!(2020-12-31).week(), 53);
        assert_eq!(date!(2021-01-01).week(), 53);
    }

    #[test]
    fn as_ymd() {
        assert_eq!(date!(2019-01-02).as_ymd(), (2019, 1, 2));
    }

    #[test]
    fn as_yo() {
        assert_eq!(date!(2019-01-01).as_yo(), (2019, 1));
    }

    #[test]
    fn next_day() {
        assert_eq!(date!(2019-01-01).next_day(), date!(2019-01-02));
        assert_eq!(date!(2019-01-31).next_day(), date!(2019-02-01));
        assert_eq!(date!(2019-12-31).next_day(), date!(2020-01-01));
    }

    #[test]
    fn previous_day() {
        assert_eq!(date!(2019-01-02).previous_day(), date!(2019-01-01));
        assert_eq!(date!(2019-02-01).previous_day(), date!(2019-01-31));
        assert_eq!(date!(2020-01-01).previous_day(), date!(2019-12-31));
    }

    #[test]
    fn julian_day() {
        assert_eq!(date!(-4713-11-24).julian_day(), 0);
        assert_eq!(date!(2000-01-01).julian_day(), 2_451_545);
        assert_eq!(date!(2019-01-01).julian_day(), 2_458_485);
        assert_eq!(date!(2019-12-31).julian_day(), 2_458_849);
    }

    #[test]
    fn from_julian_day() {
        assert_eq!(julian!(0), date!(-4713-11-24));
        assert_eq!(julian!(2_451_545), date!(2000-01-01));
        assert_eq!(julian!(2_458_485), date!(2019-01-01));
        assert_eq!(julian!(2_458_849), date!(2019-12-31));
    }

    #[test]
    fn midnight() {
        assert_eq!(
            date!(1970-01-01).midnight(),
            date!(1970-01-01).with_time(time!(0:00)),
        );
    }

    #[test]
    fn with_time() {
        assert_eq!(
            date!(1970-01-01).with_time(time!(0:00)),
            date!(1970-01-01).midnight(),
        );
    }

    #[test]
    #[cfg(panicking_api)]
    #[allow(deprecated)]
    fn with_hms() {
        assert_eq!(
            date!(1970-01-01).with_hms(0, 0, 0),
            date!(1970-01-01).midnight(),
        );
    }

    #[test]
    fn try_with_hms() {
        assert_eq!(
            date!(1970-01-01).try_with_hms(0, 0, 0),
            Ok(date!(1970-01-01).midnight()),
        );
        assert!(date!(1970-01-01).try_with_hms(24, 0, 0).is_err());
    }

    #[test]
    #[cfg(panicking_api)]
    #[allow(deprecated)]
    fn with_hms_milli() {
        assert_eq!(
            date!(1970-01-01).with_hms_milli(0, 0, 0, 0),
            date!(1970-01-01).midnight(),
        );
    }

    #[test]
    fn try_with_hms_milli() {
        assert_eq!(
            date!(1970-01-01).try_with_hms_milli(0, 0, 0, 0),
            Ok(date!(1970-01-01).midnight()),
        );
        assert!(date!(1970-01-01).try_with_hms_milli(24, 0, 0, 0).is_err());
    }

    #[test]
    #[cfg(panicking_api)]
    #[allow(deprecated)]
    fn with_hms_micro() {
        assert_eq!(
            date!(1970-01-01).with_hms_micro(0, 0, 0, 0),
            date!(1970-01-01).midnight(),
        );
    }

    #[test]
    fn try_with_hms_micro() {
        assert_eq!(
            date!(1970-01-01).try_with_hms_micro(0, 0, 0, 0),
            Ok(date!(1970-01-01).midnight()),
        );
        assert!(date!(1970-01-01).try_with_hms_micro(24, 0, 0, 0).is_err());
    }

    #[test]
    #[cfg(panicking_api)]
    #[allow(deprecated)]
    fn with_hms_nano() {
        assert_eq!(
            date!(1970-01-01).with_hms_nano(0, 0, 0, 0),
            date!(1970-01-01).midnight(),
        );
    }

    #[test]
    fn try_with_hms_nano() {
        assert_eq!(
            date!(1970-01-01).try_with_hms_nano(0, 0, 0, 0),
            Ok(date!(1970-01-01).midnight()),
        );
        assert!(date!(1970-01-01).try_with_hms_nano(24, 0, 0, 0).is_err());
    }

    #[test]
    fn format() {
        assert_eq!(date!(2019-01-02).format("%Y-%m-%d"), "2019-01-02");
    }

    #[test]
    fn parse() {
        assert_eq!(Date::parse("2019-01-02", "%F"), Ok(date!(2019-01-02)));
        assert_eq!(Date::parse("2019-002", "%Y-%j"), Ok(date!(2019-002)));
        assert_eq!(
            Date::parse("2019-W01-3", "%G-W%V-%u"),
            Ok(date!(2019-W01-3))
        );
        assert_eq!(Date::parse("20200201", "%Y%m%d"), Ok(date!(2020-02-01)));
        assert_eq!(Date::parse("-1234-01-02", "%F"), Ok(date!(-1234-01-02)));
        assert_eq!(Date::parse("-12345-01-02", "%F"), Ok(date!(-12345-01-02)));
        assert!(Date::parse("-123456-01-02", "%F").is_err());
    }

    // See #221.
    #[test]
    fn parse_regression() {
        assert_eq!(Date::parse("0000-01-01", "%Y-%m-%d"), Ok(date!(0000-01-01)));
    }

    #[test]
    fn display() {
        assert_eq!(date!(2019-01-01).to_string(), "2019-01-01");
        assert_eq!(date!(2019-12-31).to_string(), "2019-12-31");
        assert_eq!(date!(-4713-11-24).to_string(), "-4713-11-24");
        assert_eq!(date!(10_000-01-01).to_string(), "+10000-01-01");
    }

    #[test]
    fn add() {
        assert_eq!(date!(2019-01-01) + 5.days(), date!(2019-01-06));
        assert_eq!(date!(2019-12-31) + 1.days(), date!(2020-01-01));
    }

    #[test]
    fn add_std() {
        assert_eq!(date!(2019-01-01) + 5.std_days(), date!(2019-01-06));
        assert_eq!(date!(2019-12-31) + 1.std_days(), date!(2020-01-01));
    }

    #[test]
    fn add_assign() {
        let mut date = date!(2019-12-31);
        date += 1.days();
        assert_eq!(date, date!(2020-01-01));
    }

    #[test]
    fn add_assign_std() {
        let mut date = date!(2019-12-31);
        date += 1.std_days();
        assert_eq!(date, date!(2020-01-01));
    }

    #[test]
    fn sub() {
        assert_eq!(date!(2019-01-06) - 5.days(), date!(2019-01-01));
        assert_eq!(date!(2020-01-01) - 1.days(), date!(2019-12-31));
    }

    #[test]
    fn sub_std() {
        assert_eq!(date!(2019-01-06) - 5.std_days(), date!(2019-01-01));
        assert_eq!(date!(2020-01-01) - 1.std_days(), date!(2019-12-31));
    }

    #[test]
    fn sub_assign() {
        let mut date = date!(2020-01-01);
        date -= 1.days();
        assert_eq!(date, date!(2019-12-31));
    }

    #[test]
    fn sub_assign_std() {
        let mut date = date!(2020-01-01);
        date -= 1.std_days();
        assert_eq!(date, date!(2019-12-31));
    }

    #[test]
    fn sub_self() {
        assert_eq!(date!(2019-01-06) - date!(2019-01-01), 5.days());
        assert_eq!(date!(2020-01-01) - date!(2019-12-31), 1.days());
    }

    #[test]
    fn partial_ord() {
        let first = date!(2019-01-01);
        let second = date!(2019-01-02);

        assert_eq!(first.partial_cmp(&first), Some(Ordering::Equal));
        assert_eq!(first.partial_cmp(&second), Some(Ordering::Less));
        assert_eq!(second.partial_cmp(&first), Some(Ordering::Greater));
    }

    #[test]
    fn ord() {
        let first = date!(2019-01-01);
        let second = date!(2019-01-02);

        assert_eq!(first.cmp(&first), Ordering::Equal);
        assert_eq!(first.cmp(&second), Ordering::Less);
        assert_eq!(second.cmp(&first), Ordering::Greater);
    }
}
