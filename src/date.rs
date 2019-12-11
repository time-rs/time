#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::{
    format::parse::{parse, ParseError, ParseResult, ParsedItems},
    DeferredFormat, Duration, PrimitiveDateTime, Time,
    Weekday::{self, Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday},
};
use core::{
    cmp::{Ord, Ordering, PartialOrd},
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
    let weekday = Date::from_yo(year, 1).weekday();

    if (weekday == Thursday) || (weekday == Wednesday && is_leap_year(year)) {
        53
    } else {
        52
    }
}

/// Calendar date.
///
/// Years between `-100_000` and `+100_000` inclusive are guaranteed to be
/// representable. Any values outside this range may have incidental support
/// that can change at any time without notice. If you need support outside this
/// range, please [file an issue](https://github.com/time-rs/time/issues/new)
/// with your use case.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1), Date::from_yo(2019, 1));
    /// assert_eq!(Date::from_ymd(2019, 12, 31), Date::from_yo(2019, 365));
    /// ```
    ///
    /// Panics if the date is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Date;
    /// Date::from_ymd(2019, 2, 29); // 2019 isn't a leap year.
    /// ```
    #[inline]
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Self {
        /// Cumulative days through the beginning of a month in both common and
        /// leap years.
        const DAYS_CUMULATIVE_COMMON_LEAP: [[u16; 12]; 2] = [
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        assert_value_in_range!(month in 1 => 12);
        assert_value_in_range!(day in 1 => days_in_year_month(year, month), given year, month);

        let ordinal = DAYS_CUMULATIVE_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1];

        Self {
            year,
            ordinal: ordinal + day as u16,
        }
    }

    /// Attempt to create a `Date` from the year, month, and day.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_ymd(2019, 1, 1).is_some());
    /// assert!(Date::try_from_ymd(2019, 12, 31).is_some());
    /// ```
    ///
    /// Returns `None` if the date is not valid.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_ymd(2019, 2, 29).is_none()); // 2019 isn't a leap year.
    /// ```
    #[inline]
    pub fn try_from_ymd(year: i32, month: u8, day: u8) -> Option<Self> {
        /// Cumulative days through the beginning of a month in both common and
        /// leap years.
        const DAYS_CUMULATIVE_COMMON_LEAP: [[u16; 12]; 2] = [
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        ensure_value_in_range!(month in 1 => 12);
        ensure_value_in_range!(day in 1 => days_in_year_month(year, month), given year, month);

        let ordinal = DAYS_CUMULATIVE_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1];

        Some(Self {
            year,
            ordinal: ordinal + day as u16,
        })
    }

    /// Create a `Date` from the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_yo(2019, 1), Date::from_ymd(2019, 1, 1));
    /// assert_eq!(Date::from_yo(2019, 365), Date::from_ymd(2019, 12, 31));
    /// ```
    ///
    /// Panics if the date is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Date;
    /// Date::from_yo(2019, 366); // 2019 isn't a leap year.
    /// ```
    #[inline(always)]
    pub fn from_yo(year: i32, ordinal: u16) -> Self {
        assert_value_in_range!(ordinal in 1 => days_in_year(year), given year);
        Self { year, ordinal }
    }

    /// Attempt to create a `Date` from the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_yo(2019, 1).is_some());
    /// assert!(Date::try_from_yo(2019, 365).is_some());
    /// ```
    ///
    /// Returns `None` if the date is not valid.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_yo(2019, 366).is_none()); // 2019 isn't a leap year.
    /// ```
    #[inline(always)]
    pub fn try_from_yo(year: i32, ordinal: u16) -> Option<Self> {
        ensure_value_in_range!(ordinal in 1 => days_in_year(year), given year);
        Some(Self { year, ordinal })
    }

    /// Create a `Date` from the ISO year, week, and weekday.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert_eq!(
    ///     Date::from_iso_ywd(2019, 1, Monday),
    ///     Date::from_ymd(2018, 12, 31)
    /// );
    /// assert_eq!(
    ///     Date::from_iso_ywd(2019, 1, Tuesday),
    ///     Date::from_ymd(2019, 1, 1)
    /// );
    /// assert_eq!(
    ///     Date::from_iso_ywd(2020, 53, Friday),
    ///     Date::from_ymd(2021, 1, 1)
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
    pub fn from_iso_ywd(year: i32, week: u8, weekday: Weekday) -> Self {
        assert_value_in_range!(week in 1 => weeks_in_year(year), given year);

        let ordinal = week as u16 * 7 + weekday.iso_weekday_number() as u16
            - (Self::from_yo(year, 4).weekday().iso_weekday_number() as u16 + 3);

        if ordinal < 1 {
            return Self::from_yo(year - 1, ordinal + days_in_year(year - 1));
        }

        let days_in_cur_year = days_in_year(year);
        if ordinal > days_in_cur_year {
            Self::from_yo(year + 1, ordinal - days_in_cur_year)
        } else {
            Self::from_yo(year, ordinal)
        }
    }

    /// Attempt to create a `Date` from the ISO year, week, and weekday.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::try_from_iso_ywd(2019, 1, Monday).is_some());
    /// assert!(Date::try_from_iso_ywd(2019, 1, Tuesday).is_some());
    /// assert!(Date::try_from_iso_ywd(2020, 53, Friday).is_some());
    /// ```
    ///
    /// Returns `None` if the week is not valid.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::try_from_iso_ywd(2019, 53, Monday).is_none()); // 2019 doesn't have 53 weeks.
    /// ```
    #[inline]
    pub fn try_from_iso_ywd(year: i32, week: u8, weekday: Weekday) -> Option<Self> {
        ensure_value_in_range!(week in 1 => weeks_in_year(year), given year);

        let ordinal = week as u16 * 7 + weekday.iso_weekday_number() as u16
            - (Self::from_yo(year, 4).weekday().iso_weekday_number() as u16 + 3);

        if ordinal < 1 {
            return Some(Self::from_yo(year - 1, ordinal + days_in_year(year - 1)));
        }

        let days_in_cur_year = days_in_year(year);
        if ordinal > days_in_cur_year {
            Some(Self::from_yo(year + 1, ordinal - days_in_cur_year))
        } else {
            Some(Self::from_yo(year, ordinal))
        }
    }

    /// Create a `Date` representing the current date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::today().year() >= 2019);
    /// ```
    #[inline(always)]
    #[cfg(feature = "std")]
    #[cfg_attr(doc, doc(cfg(feature = "std")))]
    pub fn today() -> Self {
        PrimitiveDateTime::now().date()
    }

    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).year(), 2019);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).year(), 2019);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).year(), 2020);
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
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).month(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).month(), 12);
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
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).day(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).day(), 31);
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
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).month_day(), (1, 1));
    /// assert_eq!(Date::from_ymd(2019, 12, 31).month_day(), (12, 31));
    /// ```
    #[inline]
    pub fn month_day(self) -> (u8, u8) {
        let mut ordinal = self.ordinal;
        let days = DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(self.year) as usize];
        let mut month = 0;
        let month = loop {
            if ordinal <= days[month] {
                break month;
            }
            ordinal -= days[month];
            month += 1;
        };

        #[allow(clippy::cast_possible_truncation)]
        (month as u8 + 1, ordinal as u8)
    }

    /// Get the day of the year.
    ///
    /// The returned value will always be in the range `1..=366` (`1..=365` for
    /// common years).
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).ordinal(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).ordinal(), 365);
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn ordinal(self) -> u16 {
        self.ordinal
    }

    /// Get the ISO 8601 year and week number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).iso_year_week(), (2019, 1));
    /// assert_eq!(Date::from_ymd(2019, 10, 4).iso_year_week(), (2019, 40));
    /// assert_eq!(Date::from_ymd(2020, 1, 1).iso_year_week(), (2020, 1));
    /// assert_eq!(Date::from_ymd(2020, 12, 31).iso_year_week(), (2020, 53));
    /// assert_eq!(Date::from_ymd(2021, 1, 1).iso_year_week(), (2020, 53));
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
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).week(), 1);
    /// assert_eq!(Date::from_ymd(2019, 10, 4).week(), 40);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).week(), 1);
    /// assert_eq!(Date::from_ymd(2020, 12, 31).week(), 53);
    /// assert_eq!(Date::from_ymd(2021, 1, 1).week(), 53);
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
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).sunday_based_week(), 0);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).sunday_based_week(), 0);
    /// assert_eq!(Date::from_ymd(2020, 12, 31).sunday_based_week(), 52);
    /// assert_eq!(Date::from_ymd(2021, 1, 1).sunday_based_week(), 0);
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
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).monday_based_week(), 0);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).monday_based_week(), 0);
    /// assert_eq!(Date::from_ymd(2020, 12, 31).monday_based_week(), 52);
    /// assert_eq!(Date::from_ymd(2021, 1, 1).monday_based_week(), 0);
    /// ```
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn monday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.weekday().number_days_from_monday() as i16 + 6) / 7) as u8
    }

    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).as_ymd(), (2019, 1, 1));
    /// ```
    #[inline(always)]
    pub fn as_ymd(self) -> (i32, u8, u8) {
        let (month, day) = self.month_day();
        (self.year, month, day)
    }

    /// Get the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).as_yo(), (2019, 1));
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
    /// # use time::{Date, Weekday::*};
    /// assert_eq!(Date::from_ymd(2019, 1, 1).weekday(), Tuesday);
    /// assert_eq!(Date::from_ymd(2019, 2, 1).weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 3, 1).weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 4, 1).weekday(), Monday);
    /// assert_eq!(Date::from_ymd(2019, 5, 1).weekday(), Wednesday);
    /// assert_eq!(Date::from_ymd(2019, 6, 1).weekday(), Saturday);
    /// assert_eq!(Date::from_ymd(2019, 7, 1).weekday(), Monday);
    /// assert_eq!(Date::from_ymd(2019, 8, 1).weekday(), Thursday);
    /// assert_eq!(Date::from_ymd(2019, 9, 1).weekday(), Sunday);
    /// assert_eq!(Date::from_ymd(2019, 10, 1).weekday(), Tuesday);
    /// assert_eq!(Date::from_ymd(2019, 11, 1).weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 12, 1).weekday(), Sunday);
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
            % 7
        {
            0 => Saturday,
            1 => Sunday,
            2 => Monday,
            3 => Tuesday,
            4 => Wednesday,
            5 => Thursday,
            6 => Friday,
            // FIXME The compiler isn't able to optimize this away. Convert to
            // `.rem_euclid(7)` once rust-lang/rust#66993 is resolved.
            _ => unreachable!("A value mod 7 is always in the range 0..7"),
        }
    }

    /// Get the next calendar date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1).next_day(),
    ///     Date::from_ymd(2019, 1, 2)
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 31).next_day(),
    ///     Date::from_ymd(2019, 2, 1)
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31).next_day(),
    ///     Date::from_ymd(2020, 1, 1)
    /// );
    /// ```
    #[inline(always)]
    pub fn next_day(mut self) -> Self {
        self.ordinal += 1;

        if self.ordinal > days_in_year(self.year) {
            self.year += 1;
            self.ordinal = 1;
        }

        self
    }

    /// Get the previous calendar date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 2).previous_day(),
    ///     Date::from_ymd(2019, 1, 1)
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 2, 1).previous_day(),
    ///     Date::from_ymd(2019, 1, 31)
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1).previous_day(),
    ///     Date::from_ymd(2019, 12, 31)
    /// );
    /// ```
    #[inline(always)]
    pub fn previous_day(mut self) -> Self {
        self.ordinal -= 1;

        if self.ordinal == 0 {
            self.year -= 1;
            self.ordinal = days_in_year(self.year);
        }

        self
    }

    /// Get the Julian day for the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(-4713, 11, 24).julian_day(), 0);
    /// assert_eq!(Date::from_ymd(2000, 1, 1).julian_day(), 2_451_545);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).julian_day(), 2_458_485);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).julian_day(), 2_458_849);
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
    /// # use time::Date;
    /// assert_eq!(Date::from_julian_day(0), Date::from_ymd(-4713, 11, 24));
    /// assert_eq!(Date::from_julian_day(2_451_545), Date::from_ymd(2000, 1, 1));
    /// assert_eq!(Date::from_julian_day(2_458_485), Date::from_ymd(2019, 1, 1));
    /// assert_eq!(
    ///     Date::from_julian_day(2_458_849),
    ///     Date::from_ymd(2019, 12, 31)
    /// );
    /// ```
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
        Self::from_ymd(year as i32, month as u8, day as u8)
    }
}

/// Methods to add a `Time` component, resulting in a `PrimitiveDateTime`.
impl Date {
    /// Create a `PrimitiveDateTime` using the existing date. The `Time` component will
    /// be set to midnight.
    ///
    /// ```rust
    /// # use time::{Date, PrimitiveDateTime, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).midnight(),
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
    /// # use time::{Date, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms(0, 0, 0)),
    ///     Date::from_ymd(1970, 1, 1).midnight(),
    /// );
    /// ```
    #[inline(always)]
    pub const fn with_time(self, time: Time) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, time)
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{Date, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_hms(0, 0, 0),
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms(0, 0, 0)),
    /// );
    /// ```
    #[inline(always)]
    pub fn with_hms(self, hour: u8, minute: u8, second: u8) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::from_hms(hour, minute, second))
    }

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::from_ymd(1970, 1, 1).try_with_hms(0, 0, 0).is_some());
    /// assert!(Date::from_ymd(1970, 1, 1).try_with_hms(24, 0, 0).is_none());
    /// ```
    #[inline(always)]
    pub fn try_with_hms(self, hour: u8, minute: u8, second: u8) -> Option<PrimitiveDateTime> {
        Some(PrimitiveDateTime::new(
            self,
            Time::try_from_hms(hour, minute, second)?,
        ))
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{Date, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_hms_milli(0, 0, 0, 0),
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms_milli(0, 0, 0, 0)),
    /// );
    /// ```
    #[inline(always)]
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
    /// # use time::Date;
    /// assert!(Date::from_ymd(1970, 1, 1)
    ///     .try_with_hms_milli(0, 0, 0, 0)
    ///     .is_some());
    /// assert!(Date::from_ymd(1970, 1, 1)
    ///     .try_with_hms_milli(24, 0, 0, 0)
    ///     .is_none());
    /// ```
    #[inline(always)]
    pub fn try_with_hms_milli(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> Option<PrimitiveDateTime> {
        Some(PrimitiveDateTime::new(
            self,
            Time::try_from_hms_milli(hour, minute, second, millisecond)?,
        ))
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{Date, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_hms_micro(0, 0, 0, 0),
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms_micro(0, 0, 0, 0)),
    /// );
    /// ```
    #[inline(always)]
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

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::from_ymd(1970, 1, 1)
    ///     .try_with_hms_micro(0, 0, 0, 0)
    ///     .is_some());
    /// assert!(Date::from_ymd(1970, 1, 1)
    ///     .try_with_hms_micro(24, 0, 0, 0)
    ///     .is_none());
    /// ```
    #[inline(always)]
    pub fn try_with_hms_micro(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> Option<PrimitiveDateTime> {
        Some(PrimitiveDateTime::new(
            self,
            Time::try_from_hms_micro(hour, minute, second, microsecond)?,
        ))
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{Date, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_hms_nano(0, 0, 0, 0),
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms_nano(0, 0, 0, 0)),
    /// );
    /// ```
    #[inline(always)]
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
    /// # use time::Date;
    /// assert!(Date::from_ymd(1970, 1, 1)
    ///     .try_with_hms_nano(0, 0, 0, 0)
    ///     .is_some());
    /// assert!(Date::from_ymd(1970, 1, 1)
    ///     .try_with_hms_nano(24, 0, 0, 0)
    ///     .is_none());
    /// ```
    #[inline(always)]
    pub fn try_with_hms_nano(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Option<PrimitiveDateTime> {
        Some(PrimitiveDateTime::new(
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
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 2).format("%Y-%m-%d"), "2019-01-02");
    /// ```
    #[inline(always)]
    pub fn format(self, format: &str) -> String {
        DeferredFormat {
            date: Some(self),
            time: None,
            offset: None,
            format: crate::format::parse_fmt_string(format),
        }
        .to_string()
    }

    /// Attempt to parse a `Date` using the provided string.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::Wednesday};
    /// assert_eq!(
    ///     Date::parse("2019-01-02", "%F"),
    ///     Ok(Date::from_ymd(2019, 1, 2))
    /// );
    /// assert_eq!(Date::parse("2019-002", "%Y-%j"), Ok(Date::from_yo(2019, 2)));
    /// assert_eq!(
    ///     Date::parse("2019-W01-3", "%G-W%V-%u"),
    ///     Ok(Date::from_iso_ywd(2019, 1, Wednesday))
    /// );
    /// ```
    #[inline(always)]
    pub fn parse(s: &str, format: &str) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s, format)?)
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
            match Date::from_yo(year, 1).weekday() {
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
            items!(year, month, day) => Ok(Self::from_ymd(year, month.get(), day.get())),
            items!(year, ordinal_day) => Ok(Self::from_yo(year, ordinal_day.get())),
            items!(week_based_year, iso_week, weekday) => {
                Ok(Self::from_iso_ywd(week_based_year, iso_week.get(), weekday))
            }
            items!(year, sunday_week, weekday) => Ok(Self::from_yo(
                year,
                #[allow(clippy::cast_sign_loss)]
                {
                    (sunday_week as i16 * 7 + weekday.number_days_from_sunday() as i16
                        - adjustment(year)
                        + 1) as u16
                },
            )),
            items!(year, monday_week, weekday) => Ok(Self::from_yo(
                year,
                #[allow(clippy::cast_sign_loss)]
                {
                    (monday_week as i16 * 7 + weekday.number_days_from_monday() as i16
                        - adjustment(year)
                        + 1) as u16
                },
            )),
            _ => Err(ParseError::InsufficientInformation),
        }
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
mod test {
    use super::*;
    use crate::prelude::*;

    macro_rules! yo {
        ($year:literal, $ordinal:literal) => {
            Date::from_yo($year, $ordinal)
        };
    }

    macro_rules! ymd {
        ($year:literal, $month:literal, $day:literal) => {
            Date::from_ymd($year, $month, $day)
        };
    }

    macro_rules! ywd {
        ($year:literal, $week:literal, $day:ident) => {
            Date::from_iso_ywd($year, $week, $day)
        };
    }

    macro_rules! julian {
        ($julian:literal) => {
            Date::from_julian_day($julian)
        };
    }

    #[test]
    fn weeks_in_year_exhaustive() {
        extern crate alloc;
        let mut years_with_53 = alloc::collections::btree_set::BTreeSet::new();
        for year in [
            4, 9, 15, 20, 26, 32, 37, 43, 48, 54, 60, 65, 71, 76, 82, 88, 93, 99, 105, 111, 116,
            122, 128, 133, 139, 144, 150, 156, 161, 167, 172, 178, 184, 189, 195, 201, 207, 212,
            218, 224, 229, 235, 240, 246, 252, 257, 263, 268, 274, 280, 285, 291, 296, 303, 308,
            314, 320, 325, 331, 336, 342, 348, 353, 359, 364, 370, 376, 381, 387, 392, 398,
        ]
        .iter()
        {
            years_with_53.insert(year);
        }

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
        macro_rules! assert_monday_week {
            ($y:literal - $m:literal - $d:literal => $week:literal) => {
                assert_eq!(Date::from_ymd($y, $m, $d).monday_based_week(), $week);
            };
        }

        // A
        assert_monday_week!(2023-01-01 => 0);
        assert_monday_week!(2023-01-02 => 1);
        assert_monday_week!(2023-01-03 => 1);
        assert_monday_week!(2023-01-04 => 1);
        assert_monday_week!(2023-01-05 => 1);
        assert_monday_week!(2023-01-06 => 1);
        assert_monday_week!(2023-01-07 => 1);

        // B
        assert_monday_week!(2022-01-01 => 0);
        assert_monday_week!(2022-01-02 => 0);
        assert_monday_week!(2022-01-03 => 1);
        assert_monday_week!(2022-01-04 => 1);
        assert_monday_week!(2022-01-05 => 1);
        assert_monday_week!(2022-01-06 => 1);
        assert_monday_week!(2022-01-07 => 1);

        // C
        assert_monday_week!(2021-01-01 => 0);
        assert_monday_week!(2021-01-02 => 0);
        assert_monday_week!(2021-01-03 => 0);
        assert_monday_week!(2021-01-04 => 1);
        assert_monday_week!(2021-01-05 => 1);
        assert_monday_week!(2021-01-06 => 1);
        assert_monday_week!(2021-01-07 => 1);

        // D
        assert_monday_week!(2026-01-01 => 0);
        assert_monday_week!(2026-01-02 => 0);
        assert_monday_week!(2026-01-03 => 0);
        assert_monday_week!(2026-01-04 => 0);
        assert_monday_week!(2026-01-05 => 1);
        assert_monday_week!(2026-01-06 => 1);
        assert_monday_week!(2026-01-07 => 1);

        // E
        assert_monday_week!(2025-01-01 => 0);
        assert_monday_week!(2025-01-02 => 0);
        assert_monday_week!(2025-01-03 => 0);
        assert_monday_week!(2025-01-04 => 0);
        assert_monday_week!(2025-01-05 => 0);
        assert_monday_week!(2025-01-06 => 1);
        assert_monday_week!(2025-01-07 => 1);

        // F
        assert_monday_week!(2019-01-01 => 0);
        assert_monday_week!(2019-01-02 => 0);
        assert_monday_week!(2019-01-03 => 0);
        assert_monday_week!(2019-01-04 => 0);
        assert_monday_week!(2019-01-05 => 0);
        assert_monday_week!(2019-01-06 => 0);
        assert_monday_week!(2019-01-07 => 1);

        // G
        assert_monday_week!(2018-01-01 => 1);
        assert_monday_week!(2018-01-02 => 1);
        assert_monday_week!(2018-01-03 => 1);
        assert_monday_week!(2018-01-04 => 1);
        assert_monday_week!(2018-01-05 => 1);
        assert_monday_week!(2018-01-06 => 1);
        assert_monday_week!(2018-01-07 => 1);

        // AG
        assert_monday_week!(2012-01-01 => 0);
        assert_monday_week!(2012-01-02 => 1);
        assert_monday_week!(2012-01-03 => 1);
        assert_monday_week!(2012-01-04 => 1);
        assert_monday_week!(2012-01-05 => 1);
        assert_monday_week!(2012-01-06 => 1);
        assert_monday_week!(2012-01-07 => 1);
        assert_monday_week!(2012-02-28 => 9);
        assert_monday_week!(2012-02-29 => 9);
        assert_monday_week!(2012-03-01 => 9);
        assert_monday_week!(2012-03-02 => 9);
        assert_monday_week!(2012-03-03 => 9);
        assert_monday_week!(2012-03-04 => 9);
        assert_monday_week!(2012-03-05 => 10);
        assert_monday_week!(2012-03-06 => 10);
        assert_monday_week!(2012-03-07 => 10);

        // BA
        assert_monday_week!(2028-01-01 => 0);
        assert_monday_week!(2028-01-02 => 0);
        assert_monday_week!(2028-01-03 => 1);
        assert_monday_week!(2028-01-04 => 1);
        assert_monday_week!(2028-01-05 => 1);
        assert_monday_week!(2028-01-06 => 1);
        assert_monday_week!(2028-01-07 => 1);
        assert_monday_week!(2028-02-28 => 9);
        assert_monday_week!(2028-02-29 => 9);
        assert_monday_week!(2028-03-01 => 9);
        assert_monday_week!(2028-03-02 => 9);
        assert_monday_week!(2028-03-03 => 9);
        assert_monday_week!(2028-03-04 => 9);
        assert_monday_week!(2028-03-05 => 9);
        assert_monday_week!(2028-03-06 => 10);
        assert_monday_week!(2028-03-07 => 10);

        // CB
        assert_monday_week!(2016-01-01 => 0);
        assert_monday_week!(2016-01-02 => 0);
        assert_monday_week!(2016-01-03 => 0);
        assert_monday_week!(2016-01-04 => 1);
        assert_monday_week!(2016-01-05 => 1);
        assert_monday_week!(2016-01-06 => 1);
        assert_monday_week!(2016-01-07 => 1);
        assert_monday_week!(2016-02-28 => 8);
        assert_monday_week!(2016-02-29 => 9);
        assert_monday_week!(2016-03-01 => 9);
        assert_monday_week!(2016-03-02 => 9);
        assert_monday_week!(2016-03-03 => 9);
        assert_monday_week!(2016-03-04 => 9);
        assert_monday_week!(2016-03-05 => 9);
        assert_monday_week!(2016-03-06 => 9);
        assert_monday_week!(2016-03-07 => 10);

        // DC
        assert_monday_week!(2032-01-01 => 0);
        assert_monday_week!(2032-01-02 => 0);
        assert_monday_week!(2032-01-03 => 0);
        assert_monday_week!(2032-01-04 => 0);
        assert_monday_week!(2032-01-05 => 1);
        assert_monday_week!(2032-01-06 => 1);
        assert_monday_week!(2032-01-07 => 1);
        assert_monday_week!(2032-02-28 => 8);
        assert_monday_week!(2032-02-29 => 8);
        assert_monday_week!(2032-03-01 => 9);
        assert_monday_week!(2032-03-02 => 9);
        assert_monday_week!(2032-03-03 => 9);
        assert_monday_week!(2032-03-04 => 9);
        assert_monday_week!(2032-03-05 => 9);
        assert_monday_week!(2032-03-06 => 9);
        assert_monday_week!(2032-03-07 => 9);

        // ED
        assert_monday_week!(2020-01-01 => 0);
        assert_monday_week!(2020-01-02 => 0);
        assert_monday_week!(2020-01-03 => 0);
        assert_monday_week!(2020-01-04 => 0);
        assert_monday_week!(2020-01-05 => 0);
        assert_monday_week!(2020-01-06 => 1);
        assert_monday_week!(2020-01-07 => 1);
        assert_monday_week!(2020-02-28 => 8);
        assert_monday_week!(2020-02-29 => 8);
        assert_monday_week!(2020-03-01 => 8);
        assert_monday_week!(2020-03-02 => 9);
        assert_monday_week!(2020-03-03 => 9);
        assert_monday_week!(2020-03-04 => 9);
        assert_monday_week!(2020-03-05 => 9);
        assert_monday_week!(2020-03-06 => 9);
        assert_monday_week!(2020-03-07 => 9);

        // FE
        assert_monday_week!(2036-01-01 => 0);
        assert_monday_week!(2036-01-02 => 0);
        assert_monday_week!(2036-01-03 => 0);
        assert_monday_week!(2036-01-04 => 0);
        assert_monday_week!(2036-01-05 => 0);
        assert_monday_week!(2036-01-06 => 0);
        assert_monday_week!(2036-01-07 => 1);
        assert_monday_week!(2036-02-28 => 8);
        assert_monday_week!(2036-02-29 => 8);
        assert_monday_week!(2036-03-01 => 8);
        assert_monday_week!(2036-03-02 => 8);
        assert_monday_week!(2036-03-03 => 9);
        assert_monday_week!(2036-03-04 => 9);
        assert_monday_week!(2036-03-05 => 9);
        assert_monday_week!(2036-03-06 => 9);
        assert_monday_week!(2036-03-07 => 9);

        // GF
        assert_monday_week!(2024-01-01 => 1);
        assert_monday_week!(2024-01-02 => 1);
        assert_monday_week!(2024-01-03 => 1);
        assert_monday_week!(2024-01-04 => 1);
        assert_monday_week!(2024-01-05 => 1);
        assert_monday_week!(2024-01-06 => 1);
        assert_monday_week!(2024-01-07 => 1);
        assert_monday_week!(2024-02-28 => 9);
        assert_monday_week!(2024-02-29 => 9);
        assert_monday_week!(2024-03-01 => 9);
        assert_monday_week!(2024-03-02 => 9);
        assert_monday_week!(2024-03-03 => 9);
        assert_monday_week!(2024-03-04 => 10);
        assert_monday_week!(2024-03-05 => 10);
        assert_monday_week!(2024-03-06 => 10);
        assert_monday_week!(2024-03-07 => 10);
    }

    #[test]
    #[allow(clippy::zero_prefixed_literal)]
    fn test_sunday_based_week() {
        macro_rules! assert_sunday_week {
            ($y:literal - $m:literal - $d:literal => $week:literal) => {
                assert_eq!(Date::from_ymd($y, $m, $d).sunday_based_week(), $week);
            };
        }

        // A
        assert_sunday_week!(2023-01-01 => 1);
        assert_sunday_week!(2023-01-02 => 1);
        assert_sunday_week!(2023-01-03 => 1);
        assert_sunday_week!(2023-01-04 => 1);
        assert_sunday_week!(2023-01-05 => 1);
        assert_sunday_week!(2023-01-06 => 1);
        assert_sunday_week!(2023-01-07 => 1);

        // B
        assert_sunday_week!(2022-01-01 => 0);
        assert_sunday_week!(2022-01-02 => 1);
        assert_sunday_week!(2022-01-03 => 1);
        assert_sunday_week!(2022-01-04 => 1);
        assert_sunday_week!(2022-01-05 => 1);
        assert_sunday_week!(2022-01-06 => 1);
        assert_sunday_week!(2022-01-07 => 1);

        // C
        assert_sunday_week!(2021-01-01 => 0);
        assert_sunday_week!(2021-01-02 => 0);
        assert_sunday_week!(2021-01-03 => 1);
        assert_sunday_week!(2021-01-04 => 1);
        assert_sunday_week!(2021-01-05 => 1);
        assert_sunday_week!(2021-01-06 => 1);
        assert_sunday_week!(2021-01-07 => 1);

        // D
        assert_sunday_week!(2026-01-01 => 0);
        assert_sunday_week!(2026-01-02 => 0);
        assert_sunday_week!(2026-01-03 => 0);
        assert_sunday_week!(2026-01-04 => 1);
        assert_sunday_week!(2026-01-05 => 1);
        assert_sunday_week!(2026-01-06 => 1);
        assert_sunday_week!(2026-01-07 => 1);

        // E
        assert_sunday_week!(2025-01-01 => 0);
        assert_sunday_week!(2025-01-02 => 0);
        assert_sunday_week!(2025-01-03 => 0);
        assert_sunday_week!(2025-01-04 => 0);
        assert_sunday_week!(2025-01-05 => 1);
        assert_sunday_week!(2025-01-06 => 1);
        assert_sunday_week!(2025-01-07 => 1);

        // F
        assert_sunday_week!(2019-01-01 => 0);
        assert_sunday_week!(2019-01-02 => 0);
        assert_sunday_week!(2019-01-03 => 0);
        assert_sunday_week!(2019-01-04 => 0);
        assert_sunday_week!(2019-01-05 => 0);
        assert_sunday_week!(2019-01-06 => 1);
        assert_sunday_week!(2019-01-07 => 1);

        // G
        assert_sunday_week!(2018-01-01 => 0);
        assert_sunday_week!(2018-01-02 => 0);
        assert_sunday_week!(2018-01-03 => 0);
        assert_sunday_week!(2018-01-04 => 0);
        assert_sunday_week!(2018-01-05 => 0);
        assert_sunday_week!(2018-01-06 => 0);
        assert_sunday_week!(2018-01-07 => 1);

        // AG
        assert_sunday_week!(2012-01-01 => 1);
        assert_sunday_week!(2012-01-02 => 1);
        assert_sunday_week!(2012-01-03 => 1);
        assert_sunday_week!(2012-01-04 => 1);
        assert_sunday_week!(2012-01-05 => 1);
        assert_sunday_week!(2012-01-06 => 1);
        assert_sunday_week!(2012-01-07 => 1);
        assert_sunday_week!(2012-02-28 => 9);
        assert_sunday_week!(2012-02-29 => 9);
        assert_sunday_week!(2012-03-01 => 9);
        assert_sunday_week!(2012-03-02 => 9);
        assert_sunday_week!(2012-03-03 => 9);
        assert_sunday_week!(2012-03-04 => 10);
        assert_sunday_week!(2012-03-05 => 10);
        assert_sunday_week!(2012-03-06 => 10);
        assert_sunday_week!(2012-03-07 => 10);

        // BA
        assert_sunday_week!(2028-01-01 => 0);
        assert_sunday_week!(2028-01-02 => 1);
        assert_sunday_week!(2028-01-03 => 1);
        assert_sunday_week!(2028-01-04 => 1);
        assert_sunday_week!(2028-01-05 => 1);
        assert_sunday_week!(2028-01-06 => 1);
        assert_sunday_week!(2028-01-07 => 1);
        assert_sunday_week!(2028-02-28 => 9);
        assert_sunday_week!(2028-02-29 => 9);
        assert_sunday_week!(2028-03-01 => 9);
        assert_sunday_week!(2028-03-02 => 9);
        assert_sunday_week!(2028-03-03 => 9);
        assert_sunday_week!(2028-03-04 => 9);
        assert_sunday_week!(2028-03-05 => 10);
        assert_sunday_week!(2028-03-06 => 10);
        assert_sunday_week!(2028-03-07 => 10);

        // CB
        assert_sunday_week!(2016-01-01 => 0);
        assert_sunday_week!(2016-01-02 => 0);
        assert_sunday_week!(2016-01-03 => 1);
        assert_sunday_week!(2016-01-04 => 1);
        assert_sunday_week!(2016-01-05 => 1);
        assert_sunday_week!(2016-01-06 => 1);
        assert_sunday_week!(2016-01-07 => 1);
        assert_sunday_week!(2016-02-28 => 9);
        assert_sunday_week!(2016-02-29 => 9);
        assert_sunday_week!(2016-03-01 => 9);
        assert_sunday_week!(2016-03-02 => 9);
        assert_sunday_week!(2016-03-03 => 9);
        assert_sunday_week!(2016-03-04 => 9);
        assert_sunday_week!(2016-03-05 => 9);
        assert_sunday_week!(2016-03-06 => 10);
        assert_sunday_week!(2016-03-07 => 10);

        // DC
        assert_sunday_week!(2032-01-01 => 0);
        assert_sunday_week!(2032-01-02 => 0);
        assert_sunday_week!(2032-01-03 => 0);
        assert_sunday_week!(2032-01-04 => 1);
        assert_sunday_week!(2032-01-05 => 1);
        assert_sunday_week!(2032-01-06 => 1);
        assert_sunday_week!(2032-01-07 => 1);
        assert_sunday_week!(2032-02-28 => 8);
        assert_sunday_week!(2032-02-29 => 9);
        assert_sunday_week!(2032-03-01 => 9);
        assert_sunday_week!(2032-03-02 => 9);
        assert_sunday_week!(2032-03-03 => 9);
        assert_sunday_week!(2032-03-04 => 9);
        assert_sunday_week!(2032-03-05 => 9);
        assert_sunday_week!(2032-03-06 => 9);
        assert_sunday_week!(2032-03-07 => 10);

        // ED
        assert_sunday_week!(2020-01-01 => 0);
        assert_sunday_week!(2020-01-02 => 0);
        assert_sunday_week!(2020-01-03 => 0);
        assert_sunday_week!(2020-01-04 => 0);
        assert_sunday_week!(2020-01-05 => 1);
        assert_sunday_week!(2020-01-06 => 1);
        assert_sunday_week!(2020-01-07 => 1);
        assert_sunday_week!(2020-02-28 => 8);
        assert_sunday_week!(2020-02-29 => 8);
        assert_sunday_week!(2020-03-01 => 9);
        assert_sunday_week!(2020-03-02 => 9);
        assert_sunday_week!(2020-03-03 => 9);
        assert_sunday_week!(2020-03-04 => 9);
        assert_sunday_week!(2020-03-05 => 9);
        assert_sunday_week!(2020-03-06 => 9);
        assert_sunday_week!(2020-03-07 => 9);

        // FE
        assert_sunday_week!(2036-01-01 => 0);
        assert_sunday_week!(2036-01-02 => 0);
        assert_sunday_week!(2036-01-03 => 0);
        assert_sunday_week!(2036-01-04 => 0);
        assert_sunday_week!(2036-01-05 => 0);
        assert_sunday_week!(2036-01-06 => 1);
        assert_sunday_week!(2036-01-07 => 1);
        assert_sunday_week!(2036-02-28 => 8);
        assert_sunday_week!(2036-02-29 => 8);
        assert_sunday_week!(2036-03-01 => 8);
        assert_sunday_week!(2036-03-02 => 9);
        assert_sunday_week!(2036-03-03 => 9);
        assert_sunday_week!(2036-03-04 => 9);
        assert_sunday_week!(2036-03-05 => 9);
        assert_sunday_week!(2036-03-06 => 9);
        assert_sunday_week!(2036-03-07 => 9);

        // GF
        assert_sunday_week!(2024-01-01 => 0);
        assert_sunday_week!(2024-01-02 => 0);
        assert_sunday_week!(2024-01-03 => 0);
        assert_sunday_week!(2024-01-04 => 0);
        assert_sunday_week!(2024-01-05 => 0);
        assert_sunday_week!(2024-01-06 => 0);
        assert_sunday_week!(2024-01-07 => 1);
        assert_sunday_week!(2024-02-28 => 8);
        assert_sunday_week!(2024-02-29 => 8);
        assert_sunday_week!(2024-03-01 => 8);
        assert_sunday_week!(2024-03-02 => 8);
        assert_sunday_week!(2024-03-03 => 9);
        assert_sunday_week!(2024-03-04 => 9);
        assert_sunday_week!(2024-03-05 => 9);
        assert_sunday_week!(2024-03-06 => 9);
        assert_sunday_week!(2024-03-07 => 9);
    }

    #[test]
    #[allow(clippy::zero_prefixed_literal)]
    fn test_parse_monday_based_week() {
        macro_rules! assert_dwy {
            ($weekday:ident $week:literal $year:literal => $ordinal:literal) => {
                assert_eq!(
                    Date::parse(
                        concat!(
                            stringify!($weekday),
                            " ",
                            stringify!($week),
                            " ",
                            stringify!($year)
                        ),
                        "%a %W %Y"
                    ),
                    Ok(Date::from_yo($year, $ordinal))
                );
            };
        }

        // A
        assert_dwy!(Sun 00 2023 => 001);
        assert_dwy!(Mon 01 2023 => 002);
        assert_dwy!(Tue 01 2023 => 003);
        assert_dwy!(Wed 01 2023 => 004);
        assert_dwy!(Thu 01 2023 => 005);
        assert_dwy!(Fri 01 2023 => 006);
        assert_dwy!(Sat 01 2023 => 007);

        // B
        assert_dwy!(Sat 00 2022 => 001);
        assert_dwy!(Sun 00 2022 => 002);
        assert_dwy!(Mon 01 2022 => 003);
        assert_dwy!(Tue 01 2022 => 004);
        assert_dwy!(Wed 01 2022 => 005);
        assert_dwy!(Thu 01 2022 => 006);
        assert_dwy!(Fri 01 2022 => 007);

        // C
        assert_dwy!(Fri 00 2021 => 001);
        assert_dwy!(Sat 00 2021 => 002);
        assert_dwy!(Sun 00 2021 => 003);
        assert_dwy!(Mon 01 2021 => 004);
        assert_dwy!(Tue 01 2021 => 005);
        assert_dwy!(Wed 01 2021 => 006);
        assert_dwy!(Thu 01 2021 => 007);

        // D
        assert_dwy!(Thu 00 2026 => 001);
        assert_dwy!(Fri 00 2026 => 002);
        assert_dwy!(Sat 00 2026 => 003);
        assert_dwy!(Sun 00 2026 => 004);
        assert_dwy!(Mon 01 2026 => 005);
        assert_dwy!(Tue 01 2026 => 006);
        assert_dwy!(Wed 01 2026 => 007);

        // E
        assert_dwy!(Wed 00 2025 => 001);
        assert_dwy!(Thu 00 2025 => 002);
        assert_dwy!(Fri 00 2025 => 003);
        assert_dwy!(Sat 00 2025 => 004);
        assert_dwy!(Sun 00 2025 => 005);
        assert_dwy!(Mon 01 2025 => 006);
        assert_dwy!(Tue 01 2025 => 007);

        // F
        assert_dwy!(Tue 00 2019 => 001);
        assert_dwy!(Wed 00 2019 => 002);
        assert_dwy!(Thu 00 2019 => 003);
        assert_dwy!(Fri 00 2019 => 004);
        assert_dwy!(Sat 00 2019 => 005);
        assert_dwy!(Sun 00 2019 => 006);
        assert_dwy!(Mon 01 2019 => 007);

        // G
        assert_dwy!(Mon 01 2018 => 001);
        assert_dwy!(Tue 01 2018 => 002);
        assert_dwy!(Wed 01 2018 => 003);
        assert_dwy!(Thu 01 2018 => 004);
        assert_dwy!(Fri 01 2018 => 005);
        assert_dwy!(Sat 01 2018 => 006);
        assert_dwy!(Sun 01 2018 => 007);

        // AG
        assert_dwy!(Sun 00 2012 => 001);
        assert_dwy!(Mon 01 2012 => 002);
        assert_dwy!(Tue 01 2012 => 003);
        assert_dwy!(Wed 01 2012 => 004);
        assert_dwy!(Thu 01 2012 => 005);
        assert_dwy!(Fri 01 2012 => 006);
        assert_dwy!(Sat 01 2012 => 007);
        assert_dwy!(Tue 09 2012 => 059);
        assert_dwy!(Wed 09 2012 => 060);
        assert_dwy!(Thu 09 2012 => 061);
        assert_dwy!(Fri 09 2012 => 062);
        assert_dwy!(Sat 09 2012 => 063);
        assert_dwy!(Sun 09 2012 => 064);
        assert_dwy!(Mon 10 2012 => 065);
        assert_dwy!(Tue 10 2012 => 066);
        assert_dwy!(Wed 10 2012 => 067);

        // BA
        assert_dwy!(Sat 00 2028 => 001);
        assert_dwy!(Sun 00 2028 => 002);
        assert_dwy!(Mon 01 2028 => 003);
        assert_dwy!(Tue 01 2028 => 004);
        assert_dwy!(Wed 01 2028 => 005);
        assert_dwy!(Thu 01 2028 => 006);
        assert_dwy!(Fri 01 2028 => 007);
        assert_dwy!(Mon 09 2028 => 059);
        assert_dwy!(Tue 09 2028 => 060);
        assert_dwy!(Wed 09 2028 => 061);
        assert_dwy!(Thu 09 2028 => 062);
        assert_dwy!(Fri 09 2028 => 063);
        assert_dwy!(Sat 09 2028 => 064);
        assert_dwy!(Sun 09 2028 => 065);
        assert_dwy!(Mon 10 2028 => 066);
        assert_dwy!(Tue 10 2028 => 067);

        // CB
        assert_dwy!(Fri 00 2016 => 001);
        assert_dwy!(Sat 00 2016 => 002);
        assert_dwy!(Sun 00 2016 => 003);
        assert_dwy!(Mon 01 2016 => 004);
        assert_dwy!(Tue 01 2016 => 005);
        assert_dwy!(Wed 01 2016 => 006);
        assert_dwy!(Thu 01 2016 => 007);
        assert_dwy!(Sun 08 2016 => 059);
        assert_dwy!(Mon 09 2016 => 060);
        assert_dwy!(Tue 09 2016 => 061);
        assert_dwy!(Wed 09 2016 => 062);
        assert_dwy!(Thu 09 2016 => 063);
        assert_dwy!(Fri 09 2016 => 064);
        assert_dwy!(Sat 09 2016 => 065);
        assert_dwy!(Sun 09 2016 => 066);
        assert_dwy!(Mon 10 2016 => 067);

        // DC
        assert_dwy!(Thu 00 2032 => 001);
        assert_dwy!(Fri 00 2032 => 002);
        assert_dwy!(Sat 00 2032 => 003);
        assert_dwy!(Sun 00 2032 => 004);
        assert_dwy!(Mon 01 2032 => 005);
        assert_dwy!(Tue 01 2032 => 006);
        assert_dwy!(Wed 01 2032 => 007);
        assert_dwy!(Sat 08 2032 => 059);
        assert_dwy!(Sun 08 2032 => 060);
        assert_dwy!(Mon 09 2032 => 061);
        assert_dwy!(Tue 09 2032 => 062);
        assert_dwy!(Wed 09 2032 => 063);
        assert_dwy!(Thu 09 2032 => 064);
        assert_dwy!(Fri 09 2032 => 065);
        assert_dwy!(Sat 09 2032 => 066);
        assert_dwy!(Sun 09 2032 => 067);

        // ED
        assert_dwy!(Wed 00 2020 => 001);
        assert_dwy!(Thu 00 2020 => 002);
        assert_dwy!(Fri 00 2020 => 003);
        assert_dwy!(Sat 00 2020 => 004);
        assert_dwy!(Sun 00 2020 => 005);
        assert_dwy!(Mon 01 2020 => 006);
        assert_dwy!(Tue 01 2020 => 007);
        assert_dwy!(Fri 08 2020 => 059);
        assert_dwy!(Sat 08 2020 => 060);
        assert_dwy!(Sun 08 2020 => 061);
        assert_dwy!(Mon 09 2020 => 062);
        assert_dwy!(Tue 09 2020 => 063);
        assert_dwy!(Wed 09 2020 => 064);
        assert_dwy!(Thu 09 2020 => 065);
        assert_dwy!(Fri 09 2020 => 066);
        assert_dwy!(Sat 09 2020 => 067);

        // FE
        assert_dwy!(Tue 00 2036 => 001);
        assert_dwy!(Wed 00 2036 => 002);
        assert_dwy!(Thu 00 2036 => 003);
        assert_dwy!(Fri 00 2036 => 004);
        assert_dwy!(Sat 00 2036 => 005);
        assert_dwy!(Sun 00 2036 => 006);
        assert_dwy!(Mon 01 2036 => 007);
        assert_dwy!(Thu 08 2036 => 059);
        assert_dwy!(Fri 08 2036 => 060);
        assert_dwy!(Sat 08 2036 => 061);
        assert_dwy!(Sun 08 2036 => 062);
        assert_dwy!(Mon 09 2036 => 063);
        assert_dwy!(Tue 09 2036 => 064);
        assert_dwy!(Wed 09 2036 => 065);
        assert_dwy!(Thu 09 2036 => 066);
        assert_dwy!(Fri 09 2036 => 067);

        // GF
        assert_dwy!(Mon 01 2024 => 001);
        assert_dwy!(Tue 01 2024 => 002);
        assert_dwy!(Wed 01 2024 => 003);
        assert_dwy!(Thu 01 2024 => 004);
        assert_dwy!(Fri 01 2024 => 005);
        assert_dwy!(Sat 01 2024 => 006);
        assert_dwy!(Sun 01 2024 => 007);
        assert_dwy!(Wed 09 2024 => 059);
        assert_dwy!(Thu 09 2024 => 060);
        assert_dwy!(Fri 09 2024 => 061);
        assert_dwy!(Sat 09 2024 => 062);
        assert_dwy!(Sun 09 2024 => 063);
        assert_dwy!(Mon 10 2024 => 064);
        assert_dwy!(Tue 10 2024 => 065);
        assert_dwy!(Wed 10 2024 => 066);
        assert_dwy!(Thu 10 2024 => 067);
    }

    #[test]
    #[allow(clippy::zero_prefixed_literal)]
    fn test_parse_sunday_based_week() {
        macro_rules! assert_dwy {
            ($weekday:ident $week:literal $year:literal => $ordinal:literal) => {
                assert_eq!(
                    Date::parse(
                        concat!(
                            stringify!($weekday),
                            " ",
                            stringify!($week),
                            " ",
                            stringify!($year)
                        ),
                        "%a %U %Y"
                    ),
                    Ok(Date::from_yo($year, $ordinal))
                );
            };
        }

        // A
        assert_dwy!(Sun 01 2018 => 001);
        assert_dwy!(Mon 01 2018 => 002);
        assert_dwy!(Tue 01 2018 => 003);
        assert_dwy!(Wed 01 2018 => 004);
        assert_dwy!(Thu 01 2018 => 005);
        assert_dwy!(Fri 01 2018 => 006);
        assert_dwy!(Sat 01 2018 => 007);

        // B
        assert_dwy!(Sat 00 2023 => 001);
        assert_dwy!(Sun 01 2023 => 002);
        assert_dwy!(Mon 01 2023 => 003);
        assert_dwy!(Tue 01 2023 => 004);
        assert_dwy!(Wed 01 2023 => 005);
        assert_dwy!(Thu 01 2023 => 006);
        assert_dwy!(Fri 01 2023 => 007);

        // C
        assert_dwy!(Fri 00 2022 => 001);
        assert_dwy!(Sat 00 2022 => 002);
        assert_dwy!(Sun 01 2022 => 003);
        assert_dwy!(Mon 01 2022 => 004);
        assert_dwy!(Tue 01 2022 => 005);
        assert_dwy!(Wed 01 2022 => 006);
        assert_dwy!(Thu 01 2022 => 007);

        // D
        assert_dwy!(Thu 00 2021 => 001);
        assert_dwy!(Fri 00 2021 => 002);
        assert_dwy!(Sat 00 2021 => 003);
        assert_dwy!(Sun 01 2021 => 004);
        assert_dwy!(Mon 01 2021 => 005);
        assert_dwy!(Tue 01 2021 => 006);
        assert_dwy!(Wed 01 2021 => 007);

        // E
        assert_dwy!(Wed 00 2026 => 001);
        assert_dwy!(Thu 00 2026 => 002);
        assert_dwy!(Fri 00 2026 => 003);
        assert_dwy!(Sat 00 2026 => 004);
        assert_dwy!(Sun 01 2026 => 005);
        assert_dwy!(Mon 01 2026 => 006);
        assert_dwy!(Tue 01 2026 => 007);

        // F
        assert_dwy!(Tue 00 2025 => 001);
        assert_dwy!(Wed 00 2025 => 002);
        assert_dwy!(Thu 00 2025 => 003);
        assert_dwy!(Fri 00 2025 => 004);
        assert_dwy!(Sat 00 2025 => 005);
        assert_dwy!(Sun 01 2025 => 006);
        assert_dwy!(Mon 01 2025 => 007);

        // G
        assert_dwy!(Mon 00 2019 => 001);
        assert_dwy!(Tue 00 2019 => 002);
        assert_dwy!(Wed 00 2019 => 003);
        assert_dwy!(Thu 00 2019 => 004);
        assert_dwy!(Fri 00 2019 => 005);
        assert_dwy!(Sat 00 2019 => 006);
        assert_dwy!(Sun 01 2019 => 007);

        // AG
        assert_dwy!(Sun 01 2024 => 001);
        assert_dwy!(Mon 01 2024 => 002);
        assert_dwy!(Tue 01 2024 => 003);
        assert_dwy!(Wed 01 2024 => 004);
        assert_dwy!(Thu 01 2024 => 005);
        assert_dwy!(Fri 01 2024 => 006);
        assert_dwy!(Sat 01 2024 => 007);
        assert_dwy!(Tue 09 2024 => 059);
        assert_dwy!(Wed 09 2024 => 060);
        assert_dwy!(Thu 09 2024 => 061);
        assert_dwy!(Fri 09 2024 => 062);
        assert_dwy!(Sat 09 2024 => 063);
        assert_dwy!(Sun 10 2024 => 064);
        assert_dwy!(Mon 10 2024 => 065);
        assert_dwy!(Tue 10 2024 => 066);
        assert_dwy!(Wed 10 2024 => 067);

        // BA
        assert_dwy!(Sat 00 2012 => 001);
        assert_dwy!(Sun 01 2012 => 002);
        assert_dwy!(Mon 01 2012 => 003);
        assert_dwy!(Tue 01 2012 => 004);
        assert_dwy!(Wed 01 2012 => 005);
        assert_dwy!(Thu 01 2012 => 006);
        assert_dwy!(Fri 01 2012 => 007);
        assert_dwy!(Mon 09 2012 => 059);
        assert_dwy!(Tue 09 2012 => 060);
        assert_dwy!(Wed 09 2012 => 061);
        assert_dwy!(Thu 09 2012 => 062);
        assert_dwy!(Fri 09 2012 => 063);
        assert_dwy!(Sat 09 2012 => 064);
        assert_dwy!(Sun 10 2012 => 065);
        assert_dwy!(Mon 10 2012 => 066);
        assert_dwy!(Tue 10 2012 => 067);

        // CB
        assert_dwy!(Fri 00 2028 => 001);
        assert_dwy!(Sat 00 2028 => 002);
        assert_dwy!(Sun 01 2028 => 003);
        assert_dwy!(Mon 01 2028 => 004);
        assert_dwy!(Tue 01 2028 => 005);
        assert_dwy!(Wed 01 2028 => 006);
        assert_dwy!(Thu 01 2028 => 007);
        assert_dwy!(Sun 09 2028 => 059);
        assert_dwy!(Mon 09 2028 => 060);
        assert_dwy!(Tue 09 2028 => 061);
        assert_dwy!(Wed 09 2028 => 062);
        assert_dwy!(Thu 09 2028 => 063);
        assert_dwy!(Fri 09 2028 => 064);
        assert_dwy!(Sat 09 2028 => 065);
        assert_dwy!(Sun 10 2028 => 066);
        assert_dwy!(Mon 10 2028 => 067);

        // DC
        assert_dwy!(Thu 00 2016 => 001);
        assert_dwy!(Fri 00 2016 => 002);
        assert_dwy!(Sat 00 2016 => 003);
        assert_dwy!(Sun 01 2016 => 004);
        assert_dwy!(Mon 01 2016 => 005);
        assert_dwy!(Tue 01 2016 => 006);
        assert_dwy!(Wed 01 2016 => 007);
        assert_dwy!(Sat 08 2016 => 059);
        assert_dwy!(Sun 09 2016 => 060);
        assert_dwy!(Mon 09 2016 => 061);
        assert_dwy!(Tue 09 2016 => 062);
        assert_dwy!(Wed 09 2016 => 063);
        assert_dwy!(Thu 09 2016 => 064);
        assert_dwy!(Fri 09 2016 => 065);
        assert_dwy!(Sat 09 2016 => 066);
        assert_dwy!(Sun 10 2016 => 067);

        // ED
        assert_dwy!(Wed 00 2032 => 001);
        assert_dwy!(Thu 00 2032 => 002);
        assert_dwy!(Fri 00 2032 => 003);
        assert_dwy!(Sat 00 2032 => 004);
        assert_dwy!(Sun 01 2032 => 005);
        assert_dwy!(Mon 01 2032 => 006);
        assert_dwy!(Tue 01 2032 => 007);
        assert_dwy!(Fri 08 2032 => 059);
        assert_dwy!(Sat 08 2032 => 060);
        assert_dwy!(Sun 09 2032 => 061);
        assert_dwy!(Mon 09 2032 => 062);
        assert_dwy!(Tue 09 2032 => 063);
        assert_dwy!(Wed 09 2032 => 064);
        assert_dwy!(Thu 09 2032 => 065);
        assert_dwy!(Fri 09 2032 => 066);
        assert_dwy!(Sat 09 2032 => 067);

        // FE
        assert_dwy!(Tue 00 2020 => 001);
        assert_dwy!(Wed 00 2020 => 002);
        assert_dwy!(Thu 00 2020 => 003);
        assert_dwy!(Fri 00 2020 => 004);
        assert_dwy!(Sat 00 2020 => 005);
        assert_dwy!(Sun 01 2020 => 006);
        assert_dwy!(Mon 01 2020 => 007);
        assert_dwy!(Thu 08 2020 => 059);
        assert_dwy!(Fri 08 2020 => 060);
        assert_dwy!(Sat 08 2020 => 061);
        assert_dwy!(Sun 09 2020 => 062);
        assert_dwy!(Mon 09 2020 => 063);
        assert_dwy!(Tue 09 2020 => 064);
        assert_dwy!(Wed 09 2020 => 065);
        assert_dwy!(Thu 09 2020 => 066);
        assert_dwy!(Fri 09 2020 => 067);

        // GF
        assert_dwy!(Mon 00 2036 => 001);
        assert_dwy!(Tue 00 2036 => 002);
        assert_dwy!(Wed 00 2036 => 003);
        assert_dwy!(Thu 00 2036 => 004);
        assert_dwy!(Fri 00 2036 => 005);
        assert_dwy!(Sat 00 2036 => 006);
        assert_dwy!(Sun 01 2036 => 007);
        assert_dwy!(Wed 08 2036 => 059);
        assert_dwy!(Thu 08 2036 => 060);
        assert_dwy!(Fri 08 2036 => 061);
        assert_dwy!(Sat 08 2036 => 062);
        assert_dwy!(Sun 09 2036 => 063);
        assert_dwy!(Mon 09 2036 => 064);
        assert_dwy!(Tue 09 2036 => 065);
        assert_dwy!(Wed 09 2036 => 066);
        assert_dwy!(Thu 09 2036 => 067);
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
        assert_eq!(yo!(2019, 2).year(), 2019);
        assert_eq!(yo!(2020, 2).year(), 2020);
    }

    #[test]
    fn month() {
        assert_eq!(yo!(2019, 2).month(), 1);
        assert_eq!(yo!(2020, 2).month(), 1);
        assert_eq!(yo!(2019, 60).month(), 3);
        assert_eq!(yo!(2020, 60).month(), 2);
    }

    #[test]
    fn day() {
        assert_eq!(yo!(2019, 2).day(), 2);
        assert_eq!(yo!(2020, 2).day(), 2);
        assert_eq!(yo!(2019, 60).day(), 1);
        assert_eq!(yo!(2020, 60).day(), 29);
    }

    #[test]
    fn iso_year_week() {
        assert_eq!(ymd!(2019, 1, 1).iso_year_week(), (2019, 1));
        assert_eq!(ymd!(2019, 10, 4).iso_year_week(), (2019, 40));
        assert_eq!(ymd!(2020, 1, 1).iso_year_week(), (2020, 1));
        assert_eq!(ymd!(2020, 12, 31).iso_year_week(), (2020, 53));
        assert_eq!(ymd!(2021, 1, 1).iso_year_week(), (2020, 53));
    }

    #[test]
    fn week() {
        assert_eq!(ymd!(2019, 1, 1).week(), 1);
        assert_eq!(ymd!(2019, 10, 4).week(), 40);
        assert_eq!(ymd!(2020, 1, 1).week(), 1);
        assert_eq!(ymd!(2020, 12, 31).week(), 53);
        assert_eq!(ymd!(2021, 1, 1).week(), 53);
    }

    #[test]
    fn as_ymd() {
        assert_eq!(ymd!(2019, 1, 2).as_ymd(), (2019, 1, 2));
    }

    #[test]
    fn as_wo() {
        assert_eq!(ymd!(2019, 1, 1).as_yo(), (2019, 1));
    }

    #[test]
    fn next_day() {
        assert_eq!(ymd!(2019, 1, 1).next_day(), ymd!(2019, 1, 2));
        assert_eq!(ymd!(2019, 1, 31).next_day(), ymd!(2019, 2, 1));
        assert_eq!(ymd!(2019, 12, 31).next_day(), ymd!(2020, 1, 1));
    }

    #[test]
    fn previous_day() {
        assert_eq!(ymd!(2019, 1, 2).previous_day(), ymd!(2019, 1, 1));
        assert_eq!(ymd!(2019, 2, 1).previous_day(), ymd!(2019, 1, 31));
        assert_eq!(ymd!(2020, 1, 1).previous_day(), ymd!(2019, 12, 31));
    }

    #[test]
    fn julian_day() {
        assert_eq!(ymd!(-4713, 11, 24).julian_day(), 0);
        assert_eq!(ymd!(2000, 1, 1).julian_day(), 2_451_545);
        assert_eq!(ymd!(2019, 1, 1).julian_day(), 2_458_485);
        assert_eq!(ymd!(2019, 12, 31).julian_day(), 2_458_849);
    }

    #[test]
    fn from_julian_day() {
        assert_eq!(julian!(0), ymd!(-4713, 11, 24));
        assert_eq!(julian!(2_451_545), ymd!(2000, 1, 1));
        assert_eq!(julian!(2_458_485), ymd!(2019, 1, 1));
        assert_eq!(julian!(2_458_849), ymd!(2019, 12, 31));
    }

    #[test]
    fn midnight() {
        assert_eq!(ymd!(1970, 1, 1).midnight(), PrimitiveDateTime::unix_epoch());
    }

    #[test]
    fn with_time() {
        assert_eq!(
            ymd!(1970, 1, 1).with_time(Time::from_hms(0, 0, 0)),
            ymd!(1970, 1, 1).with_hms(0, 0, 0),
        );
    }

    #[test]
    fn with_hms() {
        assert_eq!(
            ymd!(1970, 1, 1).with_hms(0, 0, 0),
            ymd!(1970, 1, 1).with_time(Time::from_hms(0, 0, 0)),
        );
    }

    #[test]
    fn try_with_hms() {
        assert_eq!(
            ymd!(1970, 1, 1).try_with_hms(0, 0, 0),
            Some(ymd!(1970, 1, 1).with_time(Time::from_hms(0, 0, 0))),
        );
        assert_eq!(ymd!(1970, 1, 1).try_with_hms(24, 0, 0), None);
    }

    #[test]
    fn with_hms_milli() {
        assert_eq!(
            ymd!(1970, 1, 1).with_hms_milli(0, 0, 0, 0),
            ymd!(1970, 1, 1).with_time(Time::from_hms_milli(0, 0, 0, 0)),
        );
    }

    #[test]
    fn try_with_hms_milli() {
        assert_eq!(
            ymd!(1970, 1, 1).try_with_hms_milli(0, 0, 0, 0),
            Some(ymd!(1970, 1, 1).with_time(Time::from_hms_milli(0, 0, 0, 0))),
        );
        assert_eq!(ymd!(1970, 1, 1).try_with_hms_milli(24, 0, 0, 0), None);
    }

    #[test]
    fn with_hms_micro() {
        assert_eq!(
            ymd!(1970, 1, 1).with_hms_micro(0, 0, 0, 0),
            ymd!(1970, 1, 1).with_time(Time::from_hms_micro(0, 0, 0, 0)),
        );
    }

    #[test]
    fn try_with_hms_micro() {
        assert_eq!(
            ymd!(1970, 1, 1).try_with_hms_micro(0, 0, 0, 0),
            Some(ymd!(1970, 1, 1).with_time(Time::from_hms_micro(0, 0, 0, 0))),
        );
        assert_eq!(ymd!(1970, 1, 1).try_with_hms_micro(24, 0, 0, 0), None);
    }

    #[test]
    fn with_hms_nano() {
        assert_eq!(
            ymd!(1970, 1, 1).with_hms_nano(0, 0, 0, 0),
            ymd!(1970, 1, 1).with_time(Time::from_hms_nano(0, 0, 0, 0)),
        );
    }

    #[test]
    fn try_with_hms_nano() {
        assert_eq!(
            ymd!(1970, 1, 1).try_with_hms_nano(0, 0, 0, 0),
            Some(ymd!(1970, 1, 1).with_time(Time::from_hms_nano(0, 0, 0, 0))),
        );
        assert_eq!(ymd!(1970, 1, 1).try_with_hms_nano(24, 0, 0, 0), None);
    }

    #[test]
    fn format() {
        assert_eq!(ymd!(2019, 1, 2).format("%Y-%m-%d"), "2019-01-02");
    }

    #[test]
    fn parse() {
        assert_eq!(Date::parse("2019-01-02", "%F"), Ok(ymd!(2019, 1, 2)));
        assert_eq!(Date::parse("2019-002", "%Y-%j"), Ok(yo!(2019, 2)));
        assert_eq!(
            Date::parse("2019-W01-3", "%G-W%V-%u"),
            Ok(ywd!(2019, 1, Wednesday))
        );
    }

    #[test]
    fn add() {
        assert_eq!(ymd!(2019, 1, 1) + 5.days(), ymd!(2019, 1, 6));
        assert_eq!(ymd!(2019, 12, 31) + 1.days(), ymd!(2020, 1, 1));
    }

    #[test]
    fn add_std() {
        assert_eq!(ymd!(2019, 1, 1) + 5.std_days(), ymd!(2019, 1, 6));
        assert_eq!(ymd!(2019, 12, 31) + 1.std_days(), ymd!(2020, 1, 1));
    }

    #[test]
    fn add_assign() {
        let mut date = ymd!(2019, 12, 31);
        date += 1.days();
        assert_eq!(date, ymd!(2020, 1, 1));
    }

    #[test]
    fn add_assign_std() {
        let mut date = ymd!(2019, 12, 31);
        date += 1.std_days();
        assert_eq!(date, ymd!(2020, 1, 1));
    }

    #[test]
    fn sub() {
        assert_eq!(ymd!(2019, 1, 6) - 5.days(), ymd!(2019, 1, 1));
        assert_eq!(ymd!(2020, 1, 1) - 1.days(), ymd!(2019, 12, 31));
    }

    #[test]
    fn sub_std() {
        assert_eq!(ymd!(2019, 1, 6) - 5.std_days(), ymd!(2019, 1, 1));
        assert_eq!(ymd!(2020, 1, 1) - 1.std_days(), ymd!(2019, 12, 31));
    }

    #[test]
    fn sub_assign() {
        let mut date = ymd!(2020, 1, 1);
        date -= 1.days();
        assert_eq!(date, ymd!(2019, 12, 31));
    }

    #[test]
    fn sub_assign_std() {
        let mut date = ymd!(2020, 1, 1);
        date -= 1.std_days();
        assert_eq!(date, ymd!(2019, 12, 31));
    }

    #[test]
    fn sub_self() {
        assert_eq!(ymd!(2019, 1, 6) - ymd!(2019, 1, 1), 5.days());
        assert_eq!(ymd!(2020, 1, 1) - ymd!(2019, 12, 31), 1.days());
    }

    #[test]
    fn partial_ord() {
        let first = ymd!(2019, 1, 1);
        let second = ymd!(2019, 1, 2);

        assert_eq!(first.partial_cmp(&first), Some(Ordering::Equal));
        assert_eq!(first.partial_cmp(&second), Some(Ordering::Less));
        assert_eq!(second.partial_cmp(&first), Some(Ordering::Greater));
    }

    #[test]
    fn ord() {
        let first = ymd!(2019, 1, 1);
        let second = ymd!(2019, 1, 2);

        assert_eq!(first.cmp(&first), Ordering::Equal);
        assert_eq!(first.cmp(&second), Ordering::Less);
        assert_eq!(second.cmp(&first), Ordering::Greater);
    }
}
