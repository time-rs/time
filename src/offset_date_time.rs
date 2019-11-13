#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::{
    format::parse::{parse, ParseResult, ParsedItems},
    Date, DateTime, DeferredFormat, Duration, Language, Time, UtcOffset, Weekday,
};
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};

/// A [`DateTime`] with a [`UtcOffset`].
///
/// For equality, comparisons, and hashing, calculations are performed using the
/// [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq)]
pub struct OffsetDateTime {
    /// The `DateTime`, which is _always_ UTC.
    pub(crate) datetime: DateTime,
    /// The `UtcOffset`, which will be added to the `DateTime` as necessary.
    pub(crate) offset: UtcOffset,
}

impl OffsetDateTime {
    /// Create a new `OffsetDateTime` with the current date and time (UTC).
    ///
    /// ```rust
    /// # use time::{OffsetDateTime, UtcOffset};
    /// assert!(OffsetDateTime::now().year() >= 2019);
    /// assert_eq!(OffsetDateTime::now().offset(), UtcOffset::UTC);
    /// ```
    ///
    /// This method is not available with `#![no_std]`.
    #[inline(always)]
    #[cfg(feature = "std")]
    pub fn now() -> Self {
        DateTime::now().using_offset(UtcOffset::UTC)
    }

    /// Convert the `OffsetDateTime` from the current `UtcOffset` to the
    /// provided `UtcOffset`.
    ///
    /// ```rust
    /// # use time::{Date, OffsetDateTime, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2000, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .to_offset(UtcOffset::hours(-1))
    ///         .year(),
    ///     1999,
    /// );
    /// ```
    #[inline(always)]
    pub const fn to_offset(self, offset: UtcOffset) -> Self {
        self.datetime.using_offset(offset)
    }

    /// Midnight, 1 January, 1970 (UTC).
    ///
    /// ```rust
    /// # use time::{Date, OffsetDateTime, UtcOffset};
    /// assert_eq!(
    ///     OffsetDateTime::unix_epoch(),
    ///     Date::from_ymd(1970, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC),
    /// );
    /// ```
    #[inline(always)]
    pub const fn unix_epoch() -> Self {
        DateTime::unix_epoch().using_offset(UtcOffset::UTC)
    }

    /// Create an `OffsetDateTime` from the provided [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::{Date, OffsetDateTime, UtcOffset};
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(0),
    ///     OffsetDateTime::unix_epoch(),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(1_546_300_800),
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC),
    /// );
    /// ```
    #[inline(always)]
    pub fn from_unix_timestamp(timestamp: i64) -> Self {
        DateTime::from_unix_timestamp(timestamp).using_offset(UtcOffset::UTC)
    }

    /// Get the `UtcOffset`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms(0, 0, 0)
    ///         .using_offset(UtcOffset::UTC)
    ///         .offset(),
    ///     UtcOffset::UTC,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms(0, 0, 0)
    ///         .using_offset(UtcOffset::hours(1))
    ///         .offset(),
    ///     UtcOffset::hours(1),
    /// );
    /// ```
    #[inline(always)]
    pub const fn offset(self) -> UtcOffset {
        self.offset
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::{DateTime, UtcOffset};
    /// assert_eq!(
    ///     DateTime::unix_epoch()
    ///         .using_offset(UtcOffset::UTC)
    ///         .timestamp(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     DateTime::unix_epoch()
    ///         .using_offset(UtcOffset::hours(-1))
    ///         .timestamp(),
    ///     3_600,
    /// );
    /// ```
    #[inline(always)]
    pub fn timestamp(self) -> i64 {
        self.datetime.timestamp() - self.offset.as_seconds() as i64
    }

    /// Get the `Date` in the stored offset.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .date(),
    ///     Date::from_ymd(2019, 1, 1),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::hours(-1))
    ///         .date(),
    ///     Date::from_ymd(2018, 12, 31),
    /// );
    /// ```
    #[inline(always)]
    pub fn date(self) -> Date {
        (self.datetime + self.offset.as_duration()).date()
    }

    /// Get the `Time` in the stored offset.
    ///
    /// ```rust
    /// # use time::{Date, Time, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .time(),
    ///     Time::from_hms(0, 0, 0),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::hours(-1))
    ///         .time(),
    ///     Time::from_hms(23, 0, 0),
    /// );
    /// ```
    #[inline(always)]
    pub fn time(self) -> Time {
        (self.datetime + self.offset.as_duration()).time()
    }

    /// Get the year of the date in the stored offset.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .year(),
    ///     2019,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31)
    ///         .with_hms(23, 0, 0)
    ///         .using_offset(UtcOffset::UTC)
    ///         .to_offset(UtcOffset::hours(1))
    ///         .year(),
    ///     2020,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .year(),
    ///     2020,
    /// );
    /// ```
    #[inline(always)]
    pub fn year(self) -> i32 {
        self.date().year()
    }

    /// Get the month of the date in the stored offset. If fetching both the
    /// month and day, it is more efficient to use
    /// [`OffsetDateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .month(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31)
    ///         .with_hms(23, 0, 0)
    ///         .using_offset(UtcOffset::hours(1))
    ///         .month(),
    ///     1,
    /// );
    /// ```
    #[inline(always)]
    pub fn month(self) -> u8 {
        self.date().month()
    }

    /// Get the day of the date in the stored offset. If fetching both the month
    /// and day, it is more efficient to use [`OffsetDateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .day(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31)
    ///         .with_hms(23, 0, 0)
    ///         .using_offset(UtcOffset::hours(1))
    ///         .day(),
    ///     1,
    /// );
    /// ```
    #[inline(always)]
    pub fn day(self) -> u8 {
        self.date().day()
    }

    /// Get the month and day of the date in the stored offset.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .month_day(),
    ///     (1, 1),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31)
    ///         .with_hms(23, 0, 0)
    ///         .using_offset(UtcOffset::hours(1))
    ///         .month_day(),
    ///     (1, 1),
    /// );
    /// ```
    #[inline(always)]
    pub fn month_day(self) -> (u8, u8) {
        self.date().month_day()
    }

    /// Get the day of the year of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=366`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .ordinal(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31)
    ///         .with_hms(23, 0, 0)
    ///         .using_offset(UtcOffset::hours(1))
    ///         .ordinal(),
    ///     1,
    /// );
    /// ```
    #[inline(always)]
    pub fn ordinal(self) -> u16 {
        self.date().ordinal()
    }

    /// Get the ISO 8601 year and week number in the stored offset.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .iso_year_week(),
    ///     (2019, 1),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 10, 4)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .iso_year_week(),
    ///     (2019, 40),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .iso_year_week(),
    ///     (2020, 1),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 12, 31)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .iso_year_week(),
    ///     (2020, 53),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2021, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .iso_year_week(),
    ///     (2020, 53),
    /// );
    /// ```
    #[inline]
    pub fn iso_year_week(self) -> (i32, u8) {
        self.date().iso_year_week()
    }

    /// Get the ISO week number of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .week(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .week(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 12, 31)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .week(),
    ///     53,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2021, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .week(),
    ///     53,
    /// );
    /// ```
    #[inline(always)]
    pub fn week(self) -> u8 {
        self.date().week()
    }

    /// Get the weekday of the date in the stored offset.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset, Weekday::*};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .weekday(),
    ///     Tuesday,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 2, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .weekday(),
    ///     Friday,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 3, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .weekday(),
    ///     Friday,
    /// );
    /// ```
    #[inline(always)]
    pub fn weekday(self) -> Weekday {
        self.date().weekday()
    }

    /// Get the clock hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms(0, 0, 0)
    ///         .using_offset(UtcOffset::UTC)
    ///         .hour(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms(23, 59, 59)
    ///         .using_offset(UtcOffset::hours(-2))
    ///         .hour(),
    ///     21,
    /// );
    /// ```
    #[inline(always)]
    pub fn hour(self) -> u8 {
        self.time().hour()
    }

    /// Get the minute within the hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms(0, 0, 0)
    ///         .using_offset(UtcOffset::UTC)
    ///         .minute(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms(23, 59, 59)
    ///         .using_offset(UtcOffset::minutes(30))
    ///         .minute(),
    ///     29,
    /// );
    /// ```
    #[inline(always)]
    pub fn minute(self) -> u8 {
        self.time().minute()
    }

    /// Get the second within the minute in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms(0, 0, 0)
    ///         .using_offset(UtcOffset::UTC)
    ///         .second(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms(23, 59, 59)
    ///         .using_offset(UtcOffset::seconds(30))
    ///         .second(),
    ///     29,
    /// );
    /// ```
    #[inline(always)]
    pub fn second(self) -> u8 {
        self.time().second()
    }

    /// Get the milliseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms_milli(0, 0, 0, 0)
    ///         .using_offset(UtcOffset::UTC)
    ///         .millisecond(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms_milli(23, 59, 59, 999)
    ///         .using_offset(UtcOffset::UTC)
    ///         .millisecond(),
    ///     999,
    /// );
    /// ```
    #[inline(always)]
    pub fn millisecond(self) -> u16 {
        self.time().millisecond()
    }

    /// Get the microseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms_micro(0, 0, 0, 0)
    ///         .using_offset(UtcOffset::UTC)
    ///         .microsecond(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms_micro(23, 59, 59, 999_999)
    ///         .using_offset(UtcOffset::UTC)
    ///         .microsecond(),
    ///     999_999,
    /// );
    /// ```
    #[inline(always)]
    pub fn microsecond(self) -> u32 {
        self.time().microsecond()
    }

    /// Get the nanoseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms_nano(0, 0, 0, 0)
    ///         .using_offset(UtcOffset::UTC)
    ///         .nanosecond(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .with_hms_nano(23, 59, 59, 999_999_999)
    ///         .using_offset(UtcOffset::UTC)
    ///         .nanosecond(),
    ///     999_999_999,
    /// );
    /// ```
    #[inline(always)]
    pub fn nanosecond(self) -> u32 {
        self.time().nanosecond()
    }
}

/// Methods that allow formatting the `OffsetDateTime`.
impl OffsetDateTime {
    /// Format the `OffsetDateTime` using the provided string. As no language is
    /// specified, English is used.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 2)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .format("%F %r %z"),
    ///     "2019-01-02 12:00:00 am +0000",
    /// );
    /// ```
    #[inline(always)]
    pub fn format(self, format: &str) -> String {
        DeferredFormat {
            date: Some(self.date()),
            time: Some(self.time()),
            offset: Some(self.offset()),
            format: crate::format::parse_with_language(format, Language::en),
        }
        .to_string()
    }

    /// Format the `OffsetDateTime` using the provided string and language.
    ///
    /// ```rust
    /// # use time::{Date, Language, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 2)
    ///         .midnight()
    ///         .using_offset(UtcOffset::hours(-2))
    ///         .format_language("%c %z", Language::en),
    ///     "Tue Jan 1 22:00:00 2019 -0200",
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 2)
    ///         .midnight()
    ///         .using_offset(UtcOffset::hours(2))
    ///         .format_language("%c %z", Language::es),
    ///     "Mi enero 2 2:00:00 2019 +0200",
    /// );
    /// ```
    #[inline(always)]
    pub fn format_language(self, format: &str, language: Language) -> String {
        DeferredFormat {
            date: Some(self.date()),
            time: Some(self.time()),
            offset: Some(self.offset()),
            format: crate::format::parse_with_language(format, language),
        }
        .to_string()
    }

    /// Attempt to parse an `OffsetDateTime` using the provided string. As no
    /// language is specified, English is used.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Weekday::Wednesday};
    /// assert_eq!(
    ///     DateTime::parse("2019-01-02 00:00:00", "%F %T"),
    ///     Ok(Date::from_ymd(2019, 1, 2).midnight()),
    /// );
    /// assert_eq!(
    ///     DateTime::parse("2019-002 23:59:59", "%Y-%j %T"),
    ///     Ok(Date::from_yo(2019, 2).with_hms(23, 59, 59))
    /// );
    /// assert_eq!(
    ///     DateTime::parse("2019-W01-3 12:00:00 pm", "%G-W%V-%u %r"),
    ///     Ok(Date::from_iso_ywd(2019, 1, Wednesday).with_hms(12, 0, 0)),
    /// );
    /// ```
    #[inline(always)]
    pub fn parse(s: &str, format: &str) -> ParseResult<Self> {
        Self::parse_language(s, format, Language::en)
    }

    /// Attempt to parse an `OffsetDateTime` using the provided string and language.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Language::{en, es}};
    /// assert_eq!(
    ///     DateTime::parse_language("January 02 2019 12:00:00 am", "%B %d %Y %r", en),
    ///     Ok(Date::from_ymd(2019, 1, 2).midnight()),
    /// );
    /// assert_eq!(
    ///     DateTime::parse_language("02 enero 2019 00:00:00", "%d %B %Y %T", es),
    ///     Ok(Date::from_ymd(2019, 1, 2).midnight()),
    /// );
    /// ```
    #[inline(always)]
    pub fn parse_language(s: &str, format: &str, language: Language) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s, format, language)?)
    }

    /// Given the items already parsed, attempt to create a `DateTime`.
    #[inline(always)]
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        Ok(Self {
            datetime: DateTime::try_from_parsed_items(items)?,
            offset: UtcOffset::try_from_parsed_items(items)?,
        })
    }
}

impl PartialEq for OffsetDateTime {
    #[inline(always)]
    fn eq(&self, rhs: &Self) -> bool {
        self.timestamp() == rhs.timestamp()
    }
}

impl PartialOrd for OffsetDateTime {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for OffsetDateTime {
    #[inline(always)]
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.timestamp().cmp(&rhs.timestamp())
    }
}

impl Hash for OffsetDateTime {
    #[inline(always)]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_i64(self.timestamp());
    }
}

impl Add<Duration> for OffsetDateTime {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: Duration) -> Self::Output {
        (self.datetime + duration).using_offset(self.offset)
    }
}

impl Add<StdDuration> for OffsetDateTime {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: StdDuration) -> Self::Output {
        (self.datetime + duration).using_offset(self.offset)
    }
}

impl AddAssign<Duration> for OffsetDateTime {
    #[inline(always)]
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for OffsetDateTime {
    #[inline(always)]
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for OffsetDateTime {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: Duration) -> Self::Output {
        (self.datetime - duration).using_offset(self.offset)
    }
}

impl Sub<StdDuration> for OffsetDateTime {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: StdDuration) -> Self::Output {
        (self.datetime - duration).using_offset(self.offset)
    }
}

impl SubAssign<Duration> for OffsetDateTime {
    #[inline(always)]
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for OffsetDateTime {
    #[inline(always)]
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<OffsetDateTime> for OffsetDateTime {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Duration::seconds(self.timestamp() - rhs.timestamp())
    }
}

#[cfg(test)]
#[allow(clippy::zero_prefixed_literal)]
mod test {
    use super::*;
    use crate::prelude::*;

    macro_rules! ymd {
        ($year:literal, $month:literal, $date:literal) => {
            Date::from_ymd($year, $month, $date)
        };
    }

    macro_rules! time {
        ($hour:literal : $minute:literal : $second:literal) => {
            Time::from_hms($hour, $minute, $second)
        };
    }

    #[test]
    #[cfg(feature = "std")]
    fn now() {
        assert!(OffsetDateTime::now().year() >= 2019);
        assert_eq!(OffsetDateTime::now().offset(), UtcOffset::UTC);
    }

    #[test]
    fn to_offset() {
        assert_eq!(
            ymd!(2000, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .to_offset(UtcOffset::hours(-1))
                .year(),
            1999,
        );
    }

    #[test]
    fn unix_epoch() {
        assert_eq!(
            OffsetDateTime::unix_epoch(),
            ymd!(1970, 1, 1).midnight().using_offset(UtcOffset::UTC),
        );
    }

    #[test]
    fn from_unix_timestamp() {
        assert_eq!(
            OffsetDateTime::from_unix_timestamp(0),
            OffsetDateTime::unix_epoch(),
        );
        assert_eq!(
            OffsetDateTime::from_unix_timestamp(1_546_300_800),
            ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC),
        );
    }

    #[test]
    fn offset() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(0, 0, 0)
                .using_offset(UtcOffset::UTC)
                .offset(),
            UtcOffset::UTC,
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(0, 0, 0)
                .using_offset(UtcOffset::hours(1))
                .offset(),
            UtcOffset::hours(1),
        );
    }

    #[test]
    fn timestamp() {
        assert_eq!(OffsetDateTime::unix_epoch().timestamp(), 0);
        assert_eq!(
            DateTime::unix_epoch()
                .using_offset(UtcOffset::hours(-1))
                .timestamp(),
            3_600,
        );
    }

    #[test]
    fn date() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .date(),
            ymd!(2019, 1, 1),
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::hours(-1))
                .date(),
            ymd!(2018, 12, 31),
        );
    }

    #[test]
    fn time() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .time(),
            time!(0:00:00),
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::hours(-1))
                .time(),
            time!(23:00:00),
        );
    }

    #[test]
    fn year() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .year(),
            2019,
        );
        assert_eq!(
            ymd!(2019, 12, 31)
                .with_hms(23, 0, 0)
                .using_offset(UtcOffset::UTC)
                .to_offset(UtcOffset::hours(1))
                .year(),
            2020,
        );
        assert_eq!(
            ymd!(2020, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .year(),
            2020,
        );
    }

    #[test]
    fn month() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .month(),
            1,
        );
        assert_eq!(
            ymd!(2019, 12, 31)
                .with_hms(23, 0, 0)
                .using_offset(UtcOffset::hours(1))
                .month(),
            1,
        );
    }

    #[test]
    fn day() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .day(),
            1,
        );
        assert_eq!(
            ymd!(2019, 12, 31)
                .with_hms(23, 0, 0)
                .using_offset(UtcOffset::hours(1))
                .day(),
            1,
        );
    }

    #[test]
    fn month_day() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .month_day(),
            (1, 1),
        );
        assert_eq!(
            ymd!(2019, 12, 31)
                .with_hms(23, 0, 0)
                .using_offset(UtcOffset::hours(1))
                .month_day(),
            (1, 1),
        );
    }

    #[test]
    fn ordinal() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .ordinal(),
            1,
        );
        assert_eq!(
            ymd!(2019, 12, 31)
                .with_hms(23, 0, 0)
                .using_offset(UtcOffset::hours(1))
                .ordinal(),
            1,
        );
    }

    #[test]
    fn week() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .week(),
            1,
        );
        assert_eq!(
            ymd!(2020, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .week(),
            1,
        );
        assert_eq!(
            ymd!(2020, 12, 31)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .week(),
            53,
        );
        assert_eq!(
            ymd!(2021, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .week(),
            53,
        );
    }

    #[test]
    fn weekday() {
        use Weekday::*;
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .weekday(),
            Tuesday,
        );
        assert_eq!(
            ymd!(2019, 2, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .weekday(),
            Friday,
        );
        assert_eq!(
            ymd!(2019, 3, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .weekday(),
            Friday,
        );
    }

    #[test]
    fn hour() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(0, 0, 0)
                .using_offset(UtcOffset::UTC)
                .hour(),
            0,
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::hours(-2))
                .hour(),
            21,
        );
    }

    #[test]
    fn minute() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(0, 0, 0)
                .using_offset(UtcOffset::UTC)
                .minute(),
            0,
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::minutes(30))
                .minute(),
            29,
        );
    }

    #[test]
    fn second() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(0, 0, 0)
                .using_offset(UtcOffset::UTC)
                .second(),
            0,
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::seconds(30))
                .second(),
            29,
        );
    }

    #[test]
    fn millisecond() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_milli(0, 0, 0, 0)
                .using_offset(UtcOffset::UTC)
                .millisecond(),
            0,
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_milli(23, 59, 59, 999)
                .using_offset(UtcOffset::UTC)
                .millisecond(),
            999,
        );
    }

    #[test]
    fn microsecond() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_micro(0, 0, 0, 0)
                .using_offset(UtcOffset::UTC)
                .microsecond(),
            0,
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_micro(23, 59, 59, 999_999)
                .using_offset(UtcOffset::UTC)
                .microsecond(),
            999_999,
        );
    }

    #[test]
    fn nanosecond() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_nano(0, 0, 0, 0)
                .using_offset(UtcOffset::UTC)
                .nanosecond(),
            0,
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_nano(23, 59, 59, 999_999_999)
                .using_offset(UtcOffset::UTC)
                .nanosecond(),
            999_999_999,
        );
    }

    #[test]
    fn format() {
        assert_eq!(
            ymd!(2019, 1, 2)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .format("%F %r %z"),
            "2019-01-02 12:00:00 am +0000",
        );
    }

    #[test]
    fn format_language() {
        assert_eq!(
            ymd!(2019, 1, 2)
                .midnight()
                .using_offset(UtcOffset::hours(-2))
                .format_language("%c %z", Language::en),
            "Tue Jan 1 22:00:00 2019 -0200",
        );
        assert_eq!(
            ymd!(2019, 1, 2)
                .midnight()
                .using_offset(UtcOffset::hours(2))
                .format_language("%c %z", Language::es),
            "Mi enero 2 2:00:00 2019 +0200",
        );
    }

    #[test]
    fn parse() {
        use Weekday::*;
        assert_eq!(
            DateTime::parse("2019-01-02 00:00:00", "%F %T"),
            Ok(ymd!(2019, 1, 2).midnight()),
        );
        assert_eq!(
            DateTime::parse("2019-002 23:59:59", "%Y-%j %T"),
            Ok(Date::from_yo(2019, 2).with_hms(23, 59, 59))
        );
        assert_eq!(
            DateTime::parse("2019-W01-3 12:00:00 pm", "%G-W%V-%u %r"),
            Ok(Date::from_iso_ywd(2019, 1, Wednesday).with_hms(12, 0, 0)),
        );
    }

    #[test]
    fn parse_language() {
        use Language::*;
        assert_eq!(
            DateTime::parse_language("January 02 2019 12:00:00 am", "%B %d %Y %r", en),
            Ok(ymd!(2019, 1, 2).midnight()),
        );
        assert_eq!(
            DateTime::parse_language("02 enero 2019 00:00:00", "%d %B %Y %T", es),
            Ok(ymd!(2019, 1, 2).midnight()),
        );
    }

    #[test]
    fn partial_eq() {
        assert_eq!(
            ymd!(1999, 12, 31)
                .with_hms(23, 0, 0)
                .using_offset(UtcOffset::hours(-1)),
            ymd!(2000, 1, 1).midnight().using_offset(UtcOffset::UTC),
        );
    }

    #[test]
    fn partial_ord() {
        let t1 = ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC);
        let t2 = ymd!(2018, 12, 31)
            .with_hms(23, 0, 0)
            .using_offset(UtcOffset::hours(-1));
        assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Equal));
    }

    #[test]
    fn ord() {
        let t1 = ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC);
        let t2 = ymd!(2018, 12, 31)
            .with_hms(23, 0, 0)
            .using_offset(UtcOffset::hours(-1));
        assert_eq!(t1, t2);
    }

    #[test]
    #[cfg(feature = "std")]
    fn hash() {
        use std::{collections::hash_map::DefaultHasher, hash::Hash};

        assert_eq!(
            {
                let mut hasher = DefaultHasher::new();
                ymd!(2019, 1, 1)
                    .midnight()
                    .using_offset(UtcOffset::UTC)
                    .hash(&mut hasher);
                hasher.finish()
            },
            {
                let mut hasher = DefaultHasher::new();
                ymd!(2018, 12, 31)
                    .with_hms(23, 0, 0)
                    .using_offset(UtcOffset::hours(-1))
                    .hash(&mut hasher);
                hasher.finish()
            }
        );
    }

    #[test]
    fn add_duration() {
        assert_eq!(
            ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC) + 5.days(),
            ymd!(2019, 1, 6).midnight().using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC) + 1.days(),
            ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2019, 12, 31)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::UTC)
                + 2.seconds(),
            ymd!(2020, 1, 1)
                .with_hms(0, 0, 1)
                .using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2020, 1, 1)
                .with_hms(0, 0, 1)
                .using_offset(UtcOffset::UTC)
                + (-2).seconds(),
            ymd!(2019, 12, 31)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(1999, 12, 31)
                .with_hms(23, 0, 0)
                .using_offset(UtcOffset::UTC)
                + 1.hours(),
            ymd!(2000, 1, 1).midnight().using_offset(UtcOffset::UTC),
        );
    }

    #[test]
    fn add_std_duration() {
        assert_eq!(
            ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC) + 5.std_days(),
            ymd!(2019, 1, 6).midnight().using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC) + 1.std_days(),
            ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2019, 12, 31)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::UTC)
                + 2.std_seconds(),
            ymd!(2020, 1, 1)
                .with_hms(0, 0, 1)
                .using_offset(UtcOffset::UTC),
        );
    }

    #[test]
    fn add_assign_duration() {
        let mut ny19 = ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC);
        ny19 += 5.days();
        assert_eq!(
            ny19,
            ymd!(2019, 1, 6).midnight().using_offset(UtcOffset::UTC)
        );

        let mut nye20 = ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC);
        nye20 += 1.days();
        assert_eq!(
            nye20,
            ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC)
        );

        let mut nye20t = ymd!(2019, 12, 31)
            .with_hms(23, 59, 59)
            .using_offset(UtcOffset::UTC);
        nye20t += 2.seconds();
        assert_eq!(
            nye20t,
            ymd!(2020, 1, 1)
                .with_hms(0, 0, 1)
                .using_offset(UtcOffset::UTC)
        );

        let mut ny20t = ymd!(2020, 1, 1)
            .with_hms(0, 0, 1)
            .using_offset(UtcOffset::UTC);
        ny20t += (-2).seconds();
        assert_eq!(
            ny20t,
            ymd!(2019, 12, 31)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::UTC)
        );
    }

    #[test]
    fn add_assign_std_duration() {
        let mut ny19 = ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC);
        ny19 += 5.std_days();
        assert_eq!(
            ny19,
            ymd!(2019, 1, 6).midnight().using_offset(UtcOffset::UTC)
        );

        let mut nye20 = ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC);
        nye20 += 1.std_days();
        assert_eq!(
            nye20,
            ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC)
        );

        let mut nye20t = ymd!(2019, 12, 31)
            .with_hms(23, 59, 59)
            .using_offset(UtcOffset::UTC);
        nye20t += 2.std_seconds();
        assert_eq!(
            nye20t,
            ymd!(2020, 1, 1)
                .with_hms(0, 0, 1)
                .using_offset(UtcOffset::UTC)
        );
    }

    #[test]
    fn sub_duration() {
        assert_eq!(
            ymd!(2019, 1, 6).midnight().using_offset(UtcOffset::UTC) - 5.days(),
            ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC) - 1.days(),
            ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2020, 1, 1)
                .with_hms(0, 0, 1)
                .using_offset(UtcOffset::UTC)
                - 2.seconds(),
            ymd!(2019, 12, 31)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2019, 12, 31)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::UTC)
                - (-2).seconds(),
            ymd!(2020, 1, 1)
                .with_hms(0, 0, 1)
                .using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(1999, 12, 31)
                .with_hms(23, 0, 0)
                .using_offset(UtcOffset::UTC)
                - (-1).hours(),
            ymd!(2000, 1, 1).midnight().using_offset(UtcOffset::UTC),
        );
    }

    #[test]
    fn sub_std_duration() {
        assert_eq!(
            ymd!(2019, 1, 6).midnight().using_offset(UtcOffset::UTC) - 5.std_days(),
            ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC) - 1.std_days(),
            ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC),
        );
        assert_eq!(
            ymd!(2020, 1, 1)
                .with_hms(0, 0, 1)
                .using_offset(UtcOffset::UTC)
                - 2.std_seconds(),
            ymd!(2019, 12, 31)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::UTC),
        );
    }

    #[test]
    fn sub_assign_duration() {
        let mut ny19 = ymd!(2019, 1, 6).midnight().using_offset(UtcOffset::UTC);
        ny19 -= 5.days();
        assert_eq!(
            ny19,
            ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC)
        );

        let mut ny20 = ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC);
        ny20 -= 1.days();
        assert_eq!(
            ny20,
            ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC)
        );

        let mut ny20t = ymd!(2020, 1, 1)
            .with_hms(0, 0, 1)
            .using_offset(UtcOffset::UTC);
        ny20t -= 2.seconds();
        assert_eq!(
            ny20t,
            ymd!(2019, 12, 31)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::UTC)
        );

        let mut nye20t = ymd!(2019, 12, 31)
            .with_hms(23, 59, 59)
            .using_offset(UtcOffset::UTC);
        nye20t -= (-2).seconds();
        assert_eq!(
            nye20t,
            ymd!(2020, 1, 1)
                .with_hms(0, 0, 1)
                .using_offset(UtcOffset::UTC)
        );
    }

    #[test]
    fn sub_assign_std_duration() {
        let mut ny19 = ymd!(2019, 1, 6).midnight().using_offset(UtcOffset::UTC);
        ny19 -= 5.std_days();
        assert_eq!(
            ny19,
            ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC)
        );

        let mut ny20 = ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC);
        ny20 -= 1.std_days();
        assert_eq!(
            ny20,
            ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC)
        );

        let mut ny20t = ymd!(2020, 1, 1)
            .with_hms(0, 0, 1)
            .using_offset(UtcOffset::UTC);
        ny20t -= 2.std_seconds();
        assert_eq!(
            ny20t,
            ymd!(2019, 12, 31)
                .with_hms(23, 59, 59)
                .using_offset(UtcOffset::UTC)
        );
    }

    #[test]
    fn sub_self() {
        assert_eq!(
            ymd!(2019, 1, 2).midnight().using_offset(UtcOffset::UTC)
                - ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC),
            1.days(),
        );
        assert_eq!(
            ymd!(2019, 1, 1).midnight().using_offset(UtcOffset::UTC)
                - ymd!(2019, 1, 2).midnight().using_offset(UtcOffset::UTC),
            (-1).days(),
        );
        assert_eq!(
            ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC)
                - ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC),
            1.days(),
        );
        assert_eq!(
            ymd!(2019, 12, 31).midnight().using_offset(UtcOffset::UTC)
                - ymd!(2020, 1, 1).midnight().using_offset(UtcOffset::UTC),
            (-1).days(),
        );
    }
}
