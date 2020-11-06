#[cfg(feature = "alloc")]
use crate::{
    format::parse::{parse, ParsedItems},
    DeferredFormat, Format, ParseResult,
};
use crate::{util, Date, Duration, OffsetDateTime, Time, UtcOffset, Weekday};
#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
use const_fn::const_fn;
#[cfg(feature = "alloc")]
use core::fmt::{self, Display};
use core::{
    cmp::Ordering,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimitiveDateTime {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) date: Date,
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) time: Time,
}

impl PrimitiveDateTime {
    /// Create a new `PrimitiveDateTime` from the provided [`Date`] and
    /// [`Time`].
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
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn year(self) -> i32 {
        self.date().year()
    }

    /// Get the month of the date. If fetching both the month and day, it is
    /// more efficient to use [`PrimitiveDateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").month(), 1);
    /// assert_eq!(datetime!("2019-12-31 0:00").month(), 12);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn month(self) -> u8 {
        self.date().month()
    }

    /// Get the day of the date.  If fetching both the month and day, it is
    /// more efficient to use [`PrimitiveDateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").day(), 1);
    /// assert_eq!(datetime!("2019-12-31 0:00").day(), 31);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn day(self) -> u8 {
        self.date().day()
    }

    /// Get the month and day of the date. This is more efficient than fetching
    /// the components individually.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").month_day(), (1, 1));
    /// assert_eq!(datetime!("2019-12-31 0:00").month_day(), (12, 31));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn month_day(self) -> (u8, u8) {
        self.date().month_day()
    }

    /// Get the day of the year.
    ///
    /// The returned value will always be in the range `1..=366` (`1..=365` for
    /// common years).
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").ordinal(), 1);
    /// assert_eq!(datetime!("2019-12-31 0:00").ordinal(), 365);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn ordinal(self) -> u16 {
        self.date().ordinal()
    }

    /// Get the ISO 8601 year and week number.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").iso_year_week(), (2019, 1));
    /// assert_eq!(datetime!("2019-10-04 0:00").iso_year_week(), (2019, 40));
    /// assert_eq!(datetime!("2020-01-01 0:00").iso_year_week(), (2020, 1));
    /// assert_eq!(datetime!("2020-12-31 0:00").iso_year_week(), (2020, 53));
    /// assert_eq!(datetime!("2021-01-01 0:00").iso_year_week(), (2020, 53));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn iso_year_week(self) -> (i32, u8) {
        self.date().iso_year_week()
    }

    /// Get the ISO week number.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00").week(), 1);
    /// assert_eq!(datetime!("2019-10-04 0:00").week(), 40);
    /// assert_eq!(datetime!("2020-01-01 0:00").week(), 1);
    /// assert_eq!(datetime!("2020-12-31 0:00").week(), 53);
    /// assert_eq!(datetime!("2021-01-01 0:00").week(), 53);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn week(self) -> u8 {
        self.date().week()
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
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn sunday_based_week(self) -> u8 {
        self.date().sunday_based_week()
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
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn monday_based_week(self) -> u8 {
        self.date().monday_based_week()
    }

    /// Get the weekday.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
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
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn weekday(self) -> Weekday {
        self.date().weekday()
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
        self.time().hour()
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
        self.time().minute()
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
        self.time().second()
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
        self.time().millisecond()
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
        self.time().microsecond()
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
        self.time().nanosecond()
    }

    /// Assuming that the existing `PrimitiveDateTime` represents a moment in
    /// the provided [`UtcOffset`], return an [`OffsetDateTime`].
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
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn assume_offset(self, offset: UtcOffset) -> OffsetDateTime {
        OffsetDateTime::new(self.offset_to_utc(offset), offset)
    }

    /// Assuming that the existing `PrimitiveDateTime` represents a moment in
    /// the UTC, return an [`OffsetDateTime`].
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!("2019-01-01 0:00").assume_utc().unix_timestamp(),
    ///     1_546_300_800,
    /// );
    /// ```
    pub const fn assume_utc(self) -> OffsetDateTime {
        OffsetDateTime::new(self, UtcOffset::UTC)
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
    /// Assuming that the current [`PrimitiveDateTime`] is a value in the
    /// provided [`UtcOffset`], obtain the equivalent value in the UTC.
    #[const_fn("1.46")]
    pub(crate) const fn offset_to_utc(self, offset: UtcOffset) -> Self {
        let mut second = self.time.second() as i8 - (offset.as_seconds() % 60) as i8;
        let mut minute = self.time.minute() as i8 - (offset.as_seconds() / 60 % 60) as i8;
        let mut hour = self.time.hour() as i8 - (offset.as_seconds() / 3_600) as i8;

        let mut ordinal = self.date.ordinal();
        let mut year = self.date.year();

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

        let date = Date::from_yo_unchecked(year, ordinal);
        let time = Time::from_hms_nanos_unchecked(
            hour as u8,
            minute as u8,
            second as u8,
            self.time.nanosecond(),
        );

        date.with_time(time)
    }

    /// Assuming that the current [`PrimitiveDateTime`] is a value in UTC,
    /// obtain the equivalent value in the provided [`UtcOffset`].
    #[const_fn("1.46")]
    pub(crate) const fn utc_to_offset(self, offset: UtcOffset) -> Self {
        self.offset_to_utc(UtcOffset::seconds_unchecked(-offset.as_seconds()))
    }
}

/// Methods that allow formatting the `PrimitiveDateTime`.
#[cfg(feature = "alloc")]
impl PrimitiveDateTime {
    /// Format the `PrimitiveDateTime` using the provided string.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!("2019-01-02 0:00").format("%F %r"),
    ///     "2019-01-02 12:00:00 am"
    /// );
    /// ```
    pub fn format<'a>(self, format: impl Into<Format<'a>>) -> String {
        DeferredFormat::new(format.into())
            .with_date(self.date())
            .with_time(self.time())
            .to_string()
    }

    /// Attempt to parse a `PrimitiveDateTime` using the provided string.
    ///
    /// ```rust
    /// # use time::PrimitiveDateTime;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     PrimitiveDateTime::parse("2019-01-02 00:00:00", "%F %T"),
    ///     Ok(datetime!("2019-01-02 00:00:00")),
    /// );
    /// assert_eq!(
    ///     PrimitiveDateTime::parse("2019-002 23:59:59", "%Y-%j %T"),
    ///     Ok(datetime!("2019-002 23:59:59"))
    /// );
    /// assert_eq!(
    ///     PrimitiveDateTime::parse("2019-W01-3 12:00:00 pm", "%G-W%V-%u %r"),
    ///     Ok(datetime!("2019-W01-3 12:00:00 pm")),
    /// );
    /// ```
    pub fn parse<'a>(s: impl AsRef<str>, format: impl Into<Format<'a>>) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s.as_ref(), &format.into())?)
    }

    /// Given the items already parsed, attempt to create a `PrimitiveDateTime`.
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        Ok(Self {
            date: Date::try_from_parsed_items(items)?,
            time: Time::try_from_parsed_items(items)?,
        })
    }
}

#[cfg(feature = "alloc")]
impl Display for PrimitiveDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.date(), self.time())
    }
}

impl Add<Duration> for PrimitiveDateTime {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        let (date_modifier, time) = self.time.adjusting_add(duration);
        Self::new(self.date + duration + date_modifier, time)
    }
}

impl Add<StdDuration> for PrimitiveDateTime {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        let nanos = self.time.nanoseconds_since_midnight()
            + (duration.as_nanos() % 86_400_000_000_000) as u64;

        let date_modifier = if nanos >= 86_400_000_000_000 {
            Duration::day()
        } else {
            Duration::zero()
        };

        Self::new(self.date + duration + date_modifier, self.time + duration)
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
        let nanos = self.time.nanoseconds_since_midnight() as i64
            - (duration.as_nanos() % 86_400_000_000_000) as i64;

        let date_modifier = if nanos < 0 {
            Duration::days(-1)
        } else {
            Duration::zero()
        };

        Self::new(self.date - duration + date_modifier, self.time - duration)
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

impl PartialOrd for PrimitiveDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrimitiveDateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date
            .cmp(&other.date)
            .then_with(|| self.time.cmp(&other.time))
    }
}
