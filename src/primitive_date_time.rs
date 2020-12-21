use crate::{
    error, format_description::FormatDescription, hack, util, Date, Duration, OffsetDateTime, Time,
    UtcOffset, Weekday,
};
#[cfg(feature = "alloc")]
use alloc::string::String;
use core::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};

/// Combined date and time.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        into = "crate::serde::PrimitiveDateTime",
        try_from = "crate::serde::PrimitiveDateTime"
    )
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrimitiveDateTime {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) date: Date,
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) time: Time,
}

impl PrimitiveDateTime {
    /// Create a new `PrimitiveDateTime` from the provided [`Date`] and [`Time`].
    ///
    /// ```rust
    /// # use time::PrimitiveDateTime;
    /// # use time_macros::{date, datetime, time};
    /// assert_eq!(
    ///     PrimitiveDateTime::new(date!("2019-01-01"), time!("0:00")),
    ///     datetime!("2019-01-01 0:00"),
    /// );
    /// ```
    pub const fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    /// Get the [`Date`] component of the `PrimitiveDateTime`.
    ///
    /// ```rust
    /// # use time_macros::{date, datetime};
    /// assert_eq!(datetime!("2019-01-01 0:00").date(), date!("2019-01-01"));
    /// ```
    pub const fn date(self) -> Date {
        self.date
    }

    /// Get the [`Time`] component of the `PrimitiveDateTime`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, time};
    /// assert_eq!(datetime!("2019-01-01 0:00").time(), time!("0:00"));
    pub const fn time(self) -> Time {
        self.time
    }

    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").year(), 2019);
    /// assert_eq!(datetime!("2019-12-31 0:00").year(), 2019);
    /// assert_eq!(datetime!("2020-01-01 0:00").year(), 2020);
    /// ```
    pub const fn year(self) -> i32 {
        self.date.year()
    }

    /// Get the month of the date.
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").month(), 1);
    /// assert_eq!(datetime!("2019-12-31 0:00").month(), 12);
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn month(self) -> u8 {
        self.date.month()
    }

    /// Get the day of the date.
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").day(), 1);
    /// assert_eq!(datetime!("2019-12-31 0:00").day(), 31);
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn day(self) -> u8 {
        self.date.day()
    }

    /// Get the day of the year.
    ///
    /// The returned value will always be in the range `1..=366` (`1..=365` for common years).
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").ordinal(), 1);
    /// assert_eq!(datetime!("2019-12-31 0:00").ordinal(), 365);
    /// ```
    pub const fn ordinal(self) -> u16 {
        self.date.ordinal()
    }

    /// Get the ISO week number.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").iso_week(), 1);
    /// assert_eq!(datetime!("2019-10-04 0:00").iso_week(), 40);
    /// assert_eq!(datetime!("2020-01-01 0:00").iso_week(), 1);
    /// assert_eq!(datetime!("2020-12-31 0:00").iso_week(), 53);
    /// assert_eq!(datetime!("2021-01-01 0:00").iso_week(), 53);
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn iso_week(self) -> u8 {
        self.date.iso_week()
    }

    /// Get the week number where week 1 begins on the first Sunday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").sunday_based_week(), 0);
    /// assert_eq!(datetime!("2020-01-01 0:00").sunday_based_week(), 0);
    /// assert_eq!(datetime!("2020-12-31 0:00").sunday_based_week(), 52);
    /// assert_eq!(datetime!("2021-01-01 0:00").sunday_based_week(), 0);
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn sunday_based_week(self) -> u8 {
        self.date.sunday_based_week()
    }

    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").monday_based_week(), 0);
    /// assert_eq!(datetime!("2020-01-01 0:00").monday_based_week(), 0);
    /// assert_eq!(datetime!("2020-12-31 0:00").monday_based_week(), 52);
    /// assert_eq!(datetime!("2021-01-01 0:00").monday_based_week(), 0);
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn monday_based_week(self) -> u8 {
        self.date.monday_based_week()
    }

    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!("2019-01-01 0:00").to_calendar_date(),
    ///     (2019, 1, 1)
    /// );
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn to_calendar_date(self) -> (i32, u8, u8) {
        self.date.to_calendar_date()
    }

    /// Get the year and ordinal day number.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").to_ordinal_date(), (2019, 1));
    /// ```
    pub const fn to_ordinal_date(self) -> (i32, u16) {
        self.date.to_ordinal_date()
    }

    /// Get the ISO 8601 year, week number, and weekday.
    ///
    /// ```rust
    /// # use time::Weekday::*;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!("2019-01-01 0:00").to_iso_week_date(),
    ///     (2019, 1, Tuesday)
    /// );
    /// assert_eq!(
    ///     datetime!("2019-10-04 0:00").to_iso_week_date(),
    ///     (2019, 40, Friday)
    /// );
    /// assert_eq!(
    ///     datetime!("2020-01-01 0:00").to_iso_week_date(),
    ///     (2020, 1, Wednesday)
    /// );
    /// assert_eq!(
    ///     datetime!("2020-12-31 0:00").to_iso_week_date(),
    ///     (2020, 53, Thursday)
    /// );
    /// assert_eq!(
    ///     datetime!("2021-01-01 0:00").to_iso_week_date(),
    ///     (2020, 53, Friday)
    /// );
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn to_iso_week_date(self) -> (i32, u8, Weekday) {
        self.date.to_iso_week_date()
    }

    /// Get the weekday.
    ///
    /// ```rust
    /// # use time::Weekday::*;
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").weekday(), Tuesday);
    /// assert_eq!(datetime!("2019-02-01 0:00").weekday(), Friday);
    /// assert_eq!(datetime!("2019-03-01 0:00").weekday(), Friday);
    /// assert_eq!(datetime!("2019-04-01 0:00").weekday(), Monday);
    /// assert_eq!(datetime!("2019-05-01 0:00").weekday(), Wednesday);
    /// assert_eq!(datetime!("2019-06-01 0:00").weekday(), Saturday);
    /// assert_eq!(datetime!("2019-07-01 0:00").weekday(), Monday);
    /// assert_eq!(datetime!("2019-08-01 0:00").weekday(), Thursday);
    /// assert_eq!(datetime!("2019-09-01 0:00").weekday(), Sunday);
    /// assert_eq!(datetime!("2019-10-01 0:00").weekday(), Tuesday);
    /// assert_eq!(datetime!("2019-11-01 0:00").weekday(), Friday);
    /// assert_eq!(datetime!("2019-12-01 0:00").weekday(), Sunday);
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn weekday(self) -> Weekday {
        self.date.weekday()
    }

    /// Get the Julian day for the date. The time is not taken into account for this calculation.
    ///
    /// The algorithm to perform this conversion is derived from one provided by Peter Baum; it is
    /// freely available [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("-4713-11-24 0:00").to_julian_day(), 0);
    /// assert_eq!(datetime!("2000-01-01 0:00").to_julian_day(), 2_451_545);
    /// assert_eq!(datetime!("2019-01-01 0:00").to_julian_day(), 2_458_485);
    /// assert_eq!(datetime!("2019-12-31 0:00").to_julian_day(), 2_458_849);
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn to_julian_day(self) -> i32 {
        self.date.to_julian_day()
    }

    /// Get the clock hour, minute, and second.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2020-01-01 0:00:00").as_hms(), (0, 0, 0));
    /// assert_eq!(datetime!("2020-01-01 23:59:59").as_hms(), (23, 59, 59));
    /// ```
    pub const fn as_hms(self) -> (u8, u8, u8) {
        self.time.as_hms()
    }

    /// Get the clock hour, minute, second, and millisecond.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2020-01-01 0:00:00").as_hms_milli(), (0, 0, 0, 0));
    /// assert_eq!(
    ///     datetime!("2020-01-01 23:59:59.999").as_hms_milli(),
    ///     (23, 59, 59, 999)
    /// );
    /// ```
    pub const fn as_hms_milli(self) -> (u8, u8, u8, u16) {
        self.time.as_hms_milli()
    }

    /// Get the clock hour, minute, second, and microsecond.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2020-01-01 0:00:00").as_hms_micro(), (0, 0, 0, 0));
    /// assert_eq!(
    ///     datetime!("2020-01-01 23:59:59.999_999").as_hms_micro(),
    ///     (23, 59, 59, 999_999)
    /// );
    /// ```
    pub const fn as_hms_micro(self) -> (u8, u8, u8, u32) {
        self.time.as_hms_micro()
    }

    /// Get the clock hour, minute, second, and nanosecond.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2020-01-01 0:00:00").as_hms_nano(), (0, 0, 0, 0));
    /// assert_eq!(
    ///     datetime!("2020-01-01 23:59:59.999_999_999").as_hms_nano(),
    ///     (23, 59, 59, 999_999_999)
    /// );
    /// ```
    pub const fn as_hms_nano(self) -> (u8, u8, u8, u32) {
        self.time.as_hms_nano()
    }

    /// Get the clock hour.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").hour(), 0);
    /// assert_eq!(datetime!("2019-01-01 23:59:59").hour(), 23);
    /// ```
    pub const fn hour(self) -> u8 {
        self.time.hour
    }

    /// Get the minute within the hour.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").minute(), 0);
    /// assert_eq!(datetime!("2019-01-01 23:59:59").minute(), 59);
    /// ```
    pub const fn minute(self) -> u8 {
        self.time.minute
    }

    /// Get the second within the minute.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").second(), 0);
    /// assert_eq!(datetime!("2019-01-01 23:59:59").second(), 59);
    /// ```
    pub const fn second(self) -> u8 {
        self.time.second
    }

    /// Get the milliseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").millisecond(), 0);
    /// assert_eq!(datetime!("2019-01-01 23:59:59.999").millisecond(), 999);
    /// ```
    pub const fn millisecond(self) -> u16 {
        self.time.millisecond()
    }

    /// Get the microseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").microsecond(), 0);
    /// assert_eq!(
    ///     datetime!("2019-01-01 23:59:59.999_999").microsecond(),
    ///     999_999
    /// );
    /// ```
    pub const fn microsecond(self) -> u32 {
        self.time.microsecond()
    }

    /// Get the nanoseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").nanosecond(), 0);
    /// assert_eq!(
    ///     datetime!("2019-01-01 23:59:59.999_999_999").nanosecond(),
    ///     999_999_999,
    /// );
    /// ```
    pub const fn nanosecond(self) -> u32 {
        self.time.nanosecond
    }

    /// Assuming that the existing `PrimitiveDateTime` represents a moment in the provided
    /// [`UtcOffset`], return an [`OffsetDateTime`].
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(
    ///     datetime!("2019-01-01 0:00")
    ///         .assume_offset(offset!("UTC"))
    ///         .unix_timestamp(),
    ///     1_546_300_800,
    /// );
    /// assert_eq!(
    ///     datetime!("2019-01-01 0:00")
    ///         .assume_offset(offset!("-1"))
    ///         .unix_timestamp(),
    ///     1_546_304_400,
    /// );
    /// ```
    #[cfg_attr(
        feature = "const_fn",
        doc = "This feature is `const fn` when using rustc >= 1.46."
    )]
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub fn assume_offset(self, offset: UtcOffset) -> OffsetDateTime {
        OffsetDateTime {
            utc_datetime: self.offset_to_utc(offset),
            offset,
        }
    }

    /// Assuming that the existing `PrimitiveDateTime` represents a moment in the UTC, return an
    /// [`OffsetDateTime`].
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!("2019-01-01 0:00").assume_utc().unix_timestamp(),
    ///     1_546_300_800,
    /// );
    /// ```
    pub const fn assume_utc(self) -> OffsetDateTime {
        OffsetDateTime {
            utc_datetime: self,
            offset: UtcOffset::UTC,
        }
    }
}

/// Methods that replace part of the `PrimitiveDateTime`.
impl PrimitiveDateTime {
    /// Replace the time, preserving the date.
    ///
    /// ```rust
    /// # use time_macros::{datetime, time};
    /// assert_eq!(
    ///     datetime!("2020-01-01 17:00").replace_time(time!("5:00")),
    ///     datetime!("2020-01-01 5:00")
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `PrimitiveDateTime`."]
    pub const fn replace_time(self, time: Time) -> Self {
        self.date.with_time(time)
    }

    /// Replace the date, preserving the time.
    ///
    /// ```rust
    /// # use time_macros::{datetime, date};
    /// assert_eq!(
    ///     datetime!("2020-01-01 12:00").replace_date(date!("2020-01-30")),
    ///     datetime!("2020-01-30 12:00")
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `PrimitiveDateTime`."]
    pub const fn replace_date(self, date: Date) -> Self {
        date.with_time(self.time)
    }
}

/// Helper methods to adjust a [`PrimitiveDateTime`] to a given [`UtcOffset`].
impl PrimitiveDateTime {
    /// Assuming that the current [`PrimitiveDateTime`] is a value in the provided [`UtcOffset`],
    /// obtain the equivalent value in the UTC.
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub(crate) fn offset_to_utc(self, offset: UtcOffset) -> Self {
        let mut second = self.second() as i8 - offset.seconds;
        let mut minute = self.minute() as i8 - offset.minutes;
        let mut hour = self.hour() as i8 - offset.hours;
        let (mut year, mut ordinal) = self.date.to_ordinal_date();

        if second >= 60 {
            second -= 60;
            minute += 1;
        } else if second < 0 {
            second += 60;
            minute -= 1;
        }
        if minute >= 60 {
            minute -= 60;
            hour += 1;
        } else if minute < 0 {
            minute += 60;
            hour -= 1;
        }
        if hour >= 24 {
            hour -= 24;
            ordinal += 1;
        } else if hour < 0 {
            hour += 24;
            ordinal -= 1;
        }
        if ordinal > util::days_in_year(year) {
            year += 1;
            ordinal = 1;
        } else if ordinal == 0 {
            year -= 1;
            ordinal = util::days_in_year(year);
        }

        Self {
            date: Date::from_ordinal_date_unchecked(year, ordinal),
            time: Time {
                hour: hour as _,
                minute: minute as _,
                second: second as _,
                nanosecond: self.nanosecond(),
                padding: hack::Padding::Optimize,
            },
        }
    }

    /// Assuming that the current [`PrimitiveDateTime`] is a value in UTC, obtain the equivalent
    /// value in the provided [`UtcOffset`].
    #[cfg_attr(feature = "const_fn", const_fn::const_fn("1.46"))]
    pub(crate) fn utc_to_offset(self, offset: UtcOffset) -> Self {
        self.offset_to_utc(UtcOffset::from_hms_unchecked(
            -offset.hours,
            -offset.minutes,
            -offset.seconds,
        ))
    }
}

impl PrimitiveDateTime {
    /// Format the `PrimitiveDateTime` using the provided format description. The formatted value
    /// will be output to the provided writer. The format description will typically be parsed by
    /// using [`FormatDescription::parse`].
    pub fn format_into<'a>(
        self,
        output: &mut dyn fmt::Write,
        description: &FormatDescription<'a>,
    ) -> Result<(), error::Format> {
        description.format_into(output, Some(self.date), Some(self.time), None)
    }

    /// Format the `PrimitiveDateTime` using the provided format description. The format description
    /// will typically be parsed by using [`FormatDescription::parse`].
    ///
    /// ```rust
    /// # use time::format_description::FormatDescription;
    /// # use time_macros::datetime;
    /// let format =
    ///     FormatDescription::parse("[year]-[month repr:numerical]-[day] [hour]:[minute]:[second]")?;
    /// assert_eq!(
    ///     datetime!("2020-01-02 03:04:05").format(&format)?,
    ///     "2020-01-02 03:04:05"
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
    pub fn format<'a>(self, description: &FormatDescription<'a>) -> Result<String, error::Format> {
        let mut s = String::new();
        self.format_into(&mut s, description)?;
        Ok(s)
    }
}

impl fmt::Display for PrimitiveDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}

impl Add<Duration> for PrimitiveDateTime {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        let (date_adjustment, time) = self.time.adjusting_add(duration);
        let date = self.date + duration;

        Self {
            date: match date_adjustment {
                util::DateAdjustment::Previous => date.previous_day(),
                util::DateAdjustment::Next => date.next_day(),
                util::DateAdjustment::None => date,
            },
            time,
        }
    }
}

impl Add<StdDuration> for PrimitiveDateTime {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        let (is_next_day, time) = self.time.adjusting_add_std(duration);

        Self {
            date: if is_next_day {
                (self.date + duration).next_day()
            } else {
                self.date + duration
            },
            time,
        }
    }
}

impl AddAssign<Duration> for PrimitiveDateTime {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for PrimitiveDateTime {
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for PrimitiveDateTime {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl Sub<StdDuration> for PrimitiveDateTime {
    type Output = Self;

    fn sub(self, duration: StdDuration) -> Self::Output {
        let (is_previous_day, time) = self.time.adjusting_sub_std(duration);

        Self {
            date: if is_previous_day {
                (self.date - duration).previous_day()
            } else {
                self.date - duration
            },
            time,
        }
    }
}

impl SubAssign<Duration> for PrimitiveDateTime {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for PrimitiveDateTime {
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<PrimitiveDateTime> for PrimitiveDateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.date - rhs.date) + (self.time - rhs.time)
    }
}
