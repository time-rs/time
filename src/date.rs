use crate::{
    error,
    util::{days_in_year, days_in_year_month, is_leap_year, weeks_in_year},
    Duration, PrimitiveDateTime, Time, Weekday,
};
#[cfg(feature = "alloc")]
use crate::{
    format::parse::{parse, ParsedItems},
    DeferredFormat, Format, ParseResult,
};
#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
use const_fn::const_fn;
#[cfg(feature = "alloc")]
use core::fmt::Display;
use core::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};

/// The minimum valid year.
pub(crate) const MIN_YEAR: i32 = -999_999;
/// The maximum valid year.
pub(crate) const MAX_YEAR: i32 = 999_999;

/// Floored division for integers. This differs from the default behavior, which
/// is truncation.
#[const_fn("1.46")]
pub(crate) const fn div_floor(a: i64, b: i64) -> i64 {
    let (quotient, remainder) = (a / b, a % b);

    if (remainder > 0 && b < 0) || (remainder < 0 && b > 0) {
        quotient - 1
    } else {
        quotient
    }
}

/// Calendar date.
///
/// Years between `-999_999` and `+999_999` inclusive are guaranteed to be
/// representable. Any values outside this range may have incidental support
/// that can change at any time without notice. If you need support outside this
/// range, please [file an issue](https://github.com/time-rs/time/issues/new)
/// with your use case.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(into = "crate::serde::Date", try_from = "crate::serde::Date")
)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    /// Bitpacked field containing both the year and ordinal.
    // |     xx     | xxxxxxxxxxxxxxxxxxxxx | xxxxxxxxx |
    // |   2 bits   |        21 bits        |  9 bits   |
    // | unassigned |         year          |  ordinal  |
    pub(crate) value: i32,
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Date")
            .field("year", &self.year())
            .field("ordinal", &self.ordinal())
            .finish()
    }
}

impl Date {
    /// Construct a `Date` from the year and ordinal values, the validity of
    /// which must be guaranteed by the caller.
    #[doc(hidden)]
    pub const fn from_yo_unchecked(year: i32, ordinal: u16) -> Self {
        Self {
            value: (year << 9) | ordinal as i32,
        }
    }

    /// Attempt to create a `Date` from the year, month, and day.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::from_ymd(2019, 1, 1).is_ok());
    /// assert!(Date::from_ymd(2019, 12, 31).is_ok());
    /// ```
    ///
    /// Returns `None` if the date is not valid.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::from_ymd(2019, 2, 29).is_err()); // 2019 isn't a leap year.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_ymd(year: i32, month: u8, day: u8) -> Result<Self, error::ComponentRange> {
        /// Cumulative days through the beginning of a month in both common and
        /// leap years.
        const DAYS_CUMULATIVE_COMMON_LEAP: [[u16; 12]; 2] = [
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(month in 1 => 12);
        ensure_value_in_range!(day conditionally in 1 => days_in_year_month(year, month));

        Ok(Self::from_yo_unchecked(
            year,
            DAYS_CUMULATIVE_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1]
                + day as u16,
        ))
    }

    /// Attempt to create a `Date` from the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::from_yo(2019, 1).is_ok());
    /// assert!(Date::from_yo(2019, 365).is_ok());
    /// ```
    ///
    /// Returns `None` if the date is not valid.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::from_yo(2019, 366).is_err()); // 2019 isn't a leap year.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_yo(year: i32, ordinal: u16) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(ordinal conditionally in 1 => days_in_year(year));
        Ok(Self::from_yo_unchecked(year, ordinal))
    }

    /// Attempt to create a `Date` from the ISO year, week, and weekday.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::from_iso_ywd(2019, 1, Monday).is_ok());
    /// assert!(Date::from_iso_ywd(2019, 1, Tuesday).is_ok());
    /// assert!(Date::from_iso_ywd(2020, 53, Friday).is_ok());
    /// ```
    ///
    /// Returns `None` if the week is not valid.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::from_iso_ywd(2019, 53, Monday).is_err()); // 2019 doesn't have 53 weeks.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_iso_ywd(
        year: i32,
        week: u8,
        weekday: Weekday,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(week conditionally in 1 => weeks_in_year(year));

        let (ordinal, overflow) = (week as u16 * 7 + weekday.iso_weekday_number() as u16)
            .overflowing_sub({
                let adj_year = year - 1;
                let rem = (adj_year + adj_year / 4 - adj_year / 100 + adj_year / 400 + 3) % 7;
                if rem < 0 {
                    (rem + 11) as u16
                } else {
                    (rem + 4) as u16
                }
            });

        if overflow || ordinal == 0 {
            return Ok(Self::from_yo_unchecked(
                year - 1,
                ordinal.wrapping_add(days_in_year(year - 1)),
            ));
        }

        let days_in_cur_year = days_in_year(year);
        if ordinal > days_in_cur_year {
            Ok(Self::from_yo_unchecked(
                year + 1,
                ordinal - days_in_cur_year,
            ))
        } else {
            Ok(Self::from_yo_unchecked(year, ordinal))
        }
    }

    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").year(), 2019);
    /// assert_eq!(date!("2019-12-31").year(), 2019);
    /// assert_eq!(date!("2020-01-01").year(), 2020);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[allow(clippy::missing_const_for_fn)]
    #[const_fn("1.46")]
    pub const fn year(self) -> i32 {
        self.value >> 9
    }

    /// Get the month. If fetching both the month and day, it is more efficient
    /// to use [`Date::month_day`].
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").month(), 1);
    /// assert_eq!(date!("2019-12-31").month(), 12);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn month(self) -> u8 {
        self.month_day().0
    }

    /// Get the day of the month. If fetching both the month and day, it is more
    /// efficient to use [`Date::month_day`].
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").day(), 1);
    /// assert_eq!(date!("2019-12-31").day(), 31);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn day(self) -> u8 {
        self.month_day().1
    }

    /// Get the month and day. This is more efficient than fetching the
    /// components individually.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").month_day(), (1, 1));
    /// assert_eq!(date!("2019-12-31").month_day(), (12, 31));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    // For whatever reason, rustc has difficulty optimizing this function. It's
    // significantly faster to write the statements out by hand.
    #[const_fn("1.46")]
    pub const fn month_day(self) -> (u8, u8) {
        /// The number of days up to and including the given month. Common years
        /// are first, followed by leap years.
        #[allow(clippy::items_after_statements)]
        const CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP: [[u16; 11]; 2] = [
            [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        let days = CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(self.year()) as usize];
        let ordinal = self.ordinal();

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

    /// Get the day of the year.
    ///
    /// The returned value will always be in the range `1..=366` (`1..=365` for
    /// common years).
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").ordinal(), 1);
    /// assert_eq!(date!("2019-12-31").ordinal(), 365);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[allow(clippy::missing_const_for_fn)]
    #[const_fn("1.46")]
    pub const fn ordinal(self) -> u16 {
        (self.value & 0x1FF) as u16
    }

    /// Get the ISO 8601 year and week number.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").iso_year_week(), (2019, 1));
    /// assert_eq!(date!("2019-10-04").iso_year_week(), (2019, 40));
    /// assert_eq!(date!("2020-01-01").iso_year_week(), (2020, 1));
    /// assert_eq!(date!("2020-12-31").iso_year_week(), (2020, 53));
    /// assert_eq!(date!("2021-01-01").iso_year_week(), (2020, 53));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn iso_year_week(self) -> (i32, u8) {
        let (year, ordinal) = self.as_yo();

        match ((ordinal + 10 - self.weekday().iso_weekday_number() as u16) / 7) as u8 {
            0 => (year - 1, weeks_in_year(year - 1)),
            53 if weeks_in_year(year) == 52 => (year + 1, 1),
            _ => (
                year,
                ((ordinal + 10 - self.weekday().iso_weekday_number() as u16) / 7) as u8,
            ),
        }
    }

    /// Get the ISO week number.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").week(), 1);
    /// assert_eq!(date!("2019-10-04").week(), 40);
    /// assert_eq!(date!("2020-01-01").week(), 1);
    /// assert_eq!(date!("2020-12-31").week(), 53);
    /// assert_eq!(date!("2021-01-01").week(), 53);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn week(self) -> u8 {
        self.iso_year_week().1
    }

    /// Get the week number where week 1 begins on the first Sunday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").sunday_based_week(), 0);
    /// assert_eq!(date!("2020-01-01").sunday_based_week(), 0);
    /// assert_eq!(date!("2020-12-31").sunday_based_week(), 52);
    /// assert_eq!(date!("2021-01-01").sunday_based_week(), 0);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn sunday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.weekday().number_days_from_sunday() as i16 + 6) / 7) as u8
    }

    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").monday_based_week(), 0);
    /// assert_eq!(date!("2020-01-01").monday_based_week(), 0);
    /// assert_eq!(date!("2020-12-31").monday_based_week(), 52);
    /// assert_eq!(date!("2021-01-01").monday_based_week(), 0);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn monday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.weekday().number_days_from_monday() as i16 + 6) / 7) as u8
    }

    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").as_ymd(), (2019, 1, 1));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn as_ymd(self) -> (i32, u8, u8) {
        let (month, day) = self.month_day();
        (self.year(), month, day)
    }

    /// Get the year and ordinal day number.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").as_yo(), (2019, 1));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[allow(clippy::missing_const_for_fn)]
    #[const_fn("1.46")]
    pub const fn as_yo(self) -> (i32, u16) {
        (self.year(), self.ordinal())
    }

    /// Get the weekday.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
    ///
    /// ```rust
    /// # use time::Weekday::*;
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").weekday(), Tuesday);
    /// assert_eq!(date!("2019-02-01").weekday(), Friday);
    /// assert_eq!(date!("2019-03-01").weekday(), Friday);
    /// assert_eq!(date!("2019-04-01").weekday(), Monday);
    /// assert_eq!(date!("2019-05-01").weekday(), Wednesday);
    /// assert_eq!(date!("2019-06-01").weekday(), Saturday);
    /// assert_eq!(date!("2019-07-01").weekday(), Monday);
    /// assert_eq!(date!("2019-08-01").weekday(), Thursday);
    /// assert_eq!(date!("2019-09-01").weekday(), Sunday);
    /// assert_eq!(date!("2019-10-01").weekday(), Tuesday);
    /// assert_eq!(date!("2019-11-01").weekday(), Friday);
    /// assert_eq!(date!("2019-12-01").weekday(), Sunday);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn weekday(self) -> Weekday {
        let (year, month, day) = self.as_ymd();

        let (month, adjusted_year) = if month < 3 {
            (month + 12, year - 1)
        } else {
            (month, year)
        };

        let raw_weekday =
            (day as i32 + (13 * (month as i32 + 1)) / 5 + adjusted_year + adjusted_year / 4
                - adjusted_year / 100
                + adjusted_year / 400)
                % 7;

        match raw_weekday {
            -6 | 1 => Weekday::Sunday,
            -5 | 2 => Weekday::Monday,
            -4 | 3 => Weekday::Tuesday,
            -3 | 4 => Weekday::Wednesday,
            -2 | 5 => Weekday::Thursday,
            -1 | 6 => Weekday::Friday,
            _ => Weekday::Saturday,
        }
    }

    /// Get the next calendar date.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-01").next_day(), date!("2019-01-02"));
    /// assert_eq!(date!("2019-01-31").next_day(), date!("2019-02-01"));
    /// assert_eq!(date!("2019-12-31").next_day(), date!("2020-01-01"));
    /// ```
    pub fn next_day(self) -> Self {
        let (mut year, mut ordinal) = self.as_yo();

        ordinal += 1;

        if ordinal > days_in_year(year) {
            year += 1;
            ordinal = 1;
        }

        if year > MAX_YEAR {
            panic!("overflow when fetching next day");
        }

        Self::from_yo_unchecked(year, ordinal)
    }

    /// Get the previous calendar date.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-02").previous_day(), date!("2019-01-01"));
    /// assert_eq!(date!("2019-02-01").previous_day(), date!("2019-01-31"));
    /// assert_eq!(date!("2020-01-01").previous_day(), date!("2019-12-31"));
    /// ```
    pub fn previous_day(self) -> Self {
        let (mut year, mut ordinal) = self.as_yo();

        ordinal -= 1;

        if ordinal == 0 {
            year -= 1;
            ordinal = days_in_year(year);
        }

        if year < MIN_YEAR {
            panic!("overflow when fetching previous day");
        }

        Self::from_yo_unchecked(year, ordinal)
    }

    /// Get the Julian day for the date.
    ///
    /// The algorithm to perform this conversion is derived from one provided by
    /// Peter Baum; it is freely available
    /// [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("-4713-11-24").julian_day(), 0);
    /// assert_eq!(date!("2000-01-01").julian_day(), 2_451_545);
    /// assert_eq!(date!("2019-01-01").julian_day(), 2_458_485);
    /// assert_eq!(date!("2019-12-31").julian_day(), 2_458_849);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn julian_day(self) -> i64 {
        let year = self.year() as i64 - 1;
        let ordinal = self.ordinal() as i64;

        ordinal + 365 * year + div_floor(year, 4) - div_floor(year, 100)
            + div_floor(year, 400)
            + 1_721_425
    }

    /// Create a `Date` from the Julian day.
    ///
    /// The algorithm to perform this conversion is derived from one provided by
    /// Peter Baum; it is freely available
    /// [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time::Date;
    /// # use time_macros::date;
    /// assert_eq!(Date::from_julian_day(0), Ok(date!("-4713-11-24")));
    /// assert_eq!(Date::from_julian_day(2_451_545), Ok(date!("2000-01-01")));
    /// assert_eq!(Date::from_julian_day(2_458_485), Ok(date!("2019-01-01")));
    /// assert_eq!(Date::from_julian_day(2_458_849), Ok(date!("2019-12-31")));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_julian_day(julian_day: i64) -> Result<Self, error::ComponentRange> {
        let min_julian_day = Date::from_yo_unchecked(MIN_YEAR, 1).julian_day();
        let max_julian_day = Date::from_yo_unchecked(MAX_YEAR, days_in_year(MAX_YEAR)).julian_day();
        ensure_value_in_range!(julian_day in min_julian_day => max_julian_day);

        let z = julian_day - 1_721_119;
        let g = 100 * z - 25;
        let a = g / 3_652_425;
        let b = a - a / 4;
        let mut year = div_floor(100 * b + g, 36525) as i32;
        let mut ordinal = (b + z - div_floor(36525 * year as i64, 100)) as u16;

        if year % 4 != 0 {
            ordinal += 59;
            if ordinal > 365 {
                ordinal -= 365;
                year += 1;
            }
        } else if year % 100 != 0 || year % 400 == 0 {
            ordinal += 60;
            if ordinal > 366 {
                ordinal -= 366;
                year += 1;
            }
        } else {
            ordinal += 59;
            if ordinal > 365 {
                ordinal -= 365;
                year += 1;
            }
        }

        Ok(Date::from_yo_unchecked(year, ordinal))
    }
}

/// Methods to add a [`Time`] component, resulting in a [`PrimitiveDateTime`].
impl Date {
    /// Create a [`PrimitiveDateTime`] using the existing date. The [`Time`]
    /// component will be set to midnight.
    ///
    /// ```rust
    /// # use time_macros::{date, datetime};
    /// assert_eq!(date!("1970-01-01").midnight(), datetime!("1970-01-01 0:00"));
    /// ```
    pub const fn midnight(self) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::midnight())
    }

    /// Create a [`PrimitiveDateTime`] using the existing date and the provided
    /// [`Time`].
    ///
    /// ```rust
    /// # use time_macros::{date, datetime, time};
    /// assert_eq!(
    ///     date!("1970-01-01").with_time(time!("0:00")),
    ///     datetime!("1970-01-01 0:00"),
    /// );
    /// ```
    pub const fn with_time(self, time: Time) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, time)
    }

    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and
    /// the provided time.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert!(date!("1970-01-01").with_hms(0, 0, 0).is_ok());
    /// assert!(date!("1970-01-01").with_hms(24, 0, 0).is_err());
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn with_hms(
        self,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::from_hms(hour, minute, second)),
        ))
    }

    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and
    /// the provided time.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert!(date!("1970-01-01").with_hms_milli(0, 0, 0, 0).is_ok());
    /// assert!(date!("1970-01-01").with_hms_milli(24, 0, 0, 0).is_err());
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn with_hms_milli(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::from_hms_milli(hour, minute, second, millisecond)),
        ))
    }

    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and
    /// the provided time.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert!(date!("1970-01-01").with_hms_micro(0, 0, 0, 0).is_ok());
    /// assert!(date!("1970-01-01").with_hms_micro(24, 0, 0, 0).is_err());
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn with_hms_micro(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::from_hms_micro(hour, minute, second, microsecond)),
        ))
    }

    /// Attempt to create a [`PrimitiveDateTime`] using the existing date and
    /// the provided time.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert!(date!("1970-01-01").with_hms_nano(0, 0, 0, 0).is_ok());
    /// assert!(date!("1970-01-01").with_hms_nano(24, 0, 0, 0).is_err());
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn with_hms_nano(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::from_hms_nano(hour, minute, second, nanosecond)),
        ))
    }
}

/// Methods that allow formatting the `Date`.
#[cfg(feature = "alloc")]
impl Date {
    /// Format the `Date` using the provided string.
    ///
    /// ```rust
    /// # use time_macros::date;
    /// assert_eq!(date!("2019-01-02").format("%Y-%m-%d"), "2019-01-02");
    /// ```
    pub fn format<'a>(self, format: impl Into<Format<'a>>) -> String {
        DeferredFormat::new(format.into())
            .with_date(self)
            .to_string()
    }

    /// Attempt to parse a `Date` using the provided string.
    ///
    /// ```rust
    /// # use time::Date;
    /// # use time_macros::date;
    /// assert_eq!(Date::parse("2019-01-02", "%F"), Ok(date!("2019-01-02")));
    /// assert_eq!(Date::parse("2019-002", "%Y-%j"), Ok(date!("2019-002")));
    /// assert_eq!(
    ///     Date::parse("2019-W01-3", "%G-W%V-%u"),
    ///     Ok(date!("2019-W01-3"))
    /// );
    /// ```
    pub fn parse<'a>(s: impl AsRef<str>, format: impl Into<Format<'a>>) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s.as_ref(), &format.into())?)
    }

    /// Given the items already parsed, attempt to create a `Date`.
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        macro_rules! items {
            ($($item:ident),* $(,)?) => {
                ParsedItems { $($item: Some($item)),*, .. }
            };
        }

        /// Get the value needed to adjust the ordinal day for Sunday and
        /// Monday-based week numbering.
        #[allow(clippy::missing_const_for_fn)] // inside non-const outer fn
        fn adjustment(year: i32) -> i16 {
            match Date::from_yo_unchecked(year, 1).weekday() {
                Weekday::Monday => 7,
                Weekday::Tuesday => 1,
                Weekday::Wednesday => 2,
                Weekday::Thursday => 3,
                Weekday::Friday => 4,
                Weekday::Saturday => 5,
                Weekday::Sunday => 6,
            }
        }

        match items {
            items!(year, month, day) => {
                Date::from_ymd(year, month.get(), day.get()).map_err(Into::into)
            }
            items!(year, ordinal_day) => Date::from_yo(year, ordinal_day.get()).map_err(Into::into),
            items!(week_based_year, iso_week, weekday) => {
                Date::from_iso_ywd(week_based_year, iso_week.get(), weekday).map_err(Into::into)
            }
            items!(year, sunday_week, weekday) => Date::from_yo(
                year,
                (sunday_week as i16 * 7 + weekday.number_days_from_sunday() as i16
                    - adjustment(year)
                    + 1) as u16,
            )
            .map_err(Into::into),
            items!(year, monday_week, weekday) => Date::from_yo(
                year,
                (monday_week as i16 * 7 + weekday.number_days_from_monday() as i16
                    - adjustment(year)
                    + 1) as u16,
            )
            .map_err(Into::into),
            _ => Err(error::Parse::InsufficientInformation),
        }
    }
}

#[cfg(feature = "alloc")]
impl Display for Date {
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

    fn add(self, duration: Duration) -> Self::Output {
        Self::from_julian_day(self.julian_day() + duration.whole_days())
            .expect("overflow adding duration to date")
    }
}

impl Add<StdDuration> for Date {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(self.julian_day() + (duration.as_secs() / 86_400) as i64)
            .expect("overflow adding duration to date")
    }
}

impl AddAssign<Duration> for Date {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for Date {
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Date {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl Sub<StdDuration> for Date {
    type Output = Self;

    fn sub(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(self.julian_day() - (duration.as_secs() / 86_400) as i64)
            .expect("overflow subtracting duration from date")
    }
}

impl SubAssign<Duration> for Date {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for Date {
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<Date> for Date {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        Duration::days(self.julian_day() - other.julian_day())
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
