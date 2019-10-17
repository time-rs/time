#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::{Date, DateTime, DeferredFormat, Duration, Language, Time, UtcOffset, Weekday};
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration as StdDuration;

/// A [`DateTime`](DateTime) with a [`UtcOffset`](UtcOffset).
///
/// For equality, comparisons, and hashing, calculations are performed using the
/// [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
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
    pub fn from_unix_timestamp(timestamp: i64) -> Self {
        DateTime::from_unix_timestamp(timestamp).using_offset(UtcOffset::UTC)
    }

    /// Get the `UtcOffset` of the `OffsetDateTime`.
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
    pub const fn offset(self) -> UtcOffset {
        self.offset
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time)
    /// representing the `OffsetDateTime`.
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
    pub fn timestamp(self) -> i64 {
        self.datetime.timestamp() - self.offset.as_seconds() as i64
    }

    /// Get the `Date` in the stored offset of the `OffsetDateTime`.
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
    pub fn date(self) -> Date {
        (self.datetime + self.offset.as_duration()).date()
    }

    /// Get the `Time` in the stored offset of the `OffsetDateTime`.
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
    pub fn year(self) -> i32 {
        self.date().year()
    }

    /// Get the month of the date in the stored offset. If fetching both the
    /// month and day, use [`OffsetDateTime::month_day`] instead.
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
    pub fn month(self) -> u8 {
        self.date().month()
    }

    /// Get the day of the date in the stored offset. If fetching both the month
    /// and day, use [`OffsetDateTime::month_day`] instead.
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
    pub fn ordinal(self) -> u16 {
        self.date().ordinal()
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
    pub fn weekday(self) -> Weekday {
        self.date().weekday()
    }

    /// Returns the clock hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..=23`.
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
    pub fn hour(self) -> u8 {
        self.time().hour()
    }

    /// Returns the minute within the hour in the stored offset.
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
    pub fn minute(self) -> u8 {
        self.time().minute()
    }

    /// Returns the second within the minute in the stored offset.
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
    pub fn second(self) -> u8 {
        self.time().second()
    }

    /// Return the milliseconds within the second in the stored offset.
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
    pub fn millisecond(self) -> u16 {
        self.time().millisecond()
    }

    /// Return the microseconds within the second in the stored offset.
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
    pub fn microsecond(self) -> u32 {
        self.time().microsecond()
    }

    /// Return the nanoseconds within the second in the stored offset.
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
    ///     "Tue Jan  1 22:00:00 2019 -0200",
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 2)
    ///         .midnight()
    ///         .using_offset(UtcOffset::hours(2))
    ///         .format_language("%c %z", Language::es),
    ///     "Mi enero  2 02:00:00 2019 +0200",
    /// );
    /// ```
    pub fn format_language(self, format: &str, language: Language) -> String {
        DeferredFormat {
            date: Some(self.date()),
            time: Some(self.time()),
            offset: Some(self.offset()),
            format: crate::format::parse_with_language(format, language),
        }
        .to_string()
    }
}

impl PartialEq for OffsetDateTime {
    /// Check if two `OffsetDateTime`s represent the same moment.
    ///
    /// ```rust
    /// # use time::{Date, OffsetDateTime, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(1999, 12, 31)
    ///         .with_hms(23, 0, 0)
    ///         .using_offset(UtcOffset::hours(-1)),
    ///     Date::from_ymd(2000, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC),
    /// );
    /// ```
    fn eq(&self, rhs: &Self) -> bool {
        self.timestamp() == rhs.timestamp()
    }
}

impl PartialOrd for OffsetDateTime {
    /// Compare two `OffsetDateTime`s.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// # use core::cmp::{Ordering, PartialOrd};
    /// let t1 = Date::from_ymd(2019, 1, 1)
    ///     .midnight()
    ///     .using_offset(UtcOffset::UTC);
    /// let t2 = Date::from_ymd(2018, 12, 31)
    ///     .with_hms(23, 0, 0)
    ///     .using_offset(UtcOffset::hours(-1));
    /// assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Equal));
    /// ```
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for OffsetDateTime {
    /// Compare two `OffsetDateTime`s.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// let t1 = Date::from_ymd(2019, 1, 1)
    ///     .midnight()
    ///     .using_offset(UtcOffset::UTC);
    /// let t2 = Date::from_ymd(2018, 12, 31)
    ///     .with_hms(23, 0, 0)
    ///     .using_offset(UtcOffset::hours(-1));
    /// assert_eq!(t1, t2);
    /// ```
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.timestamp().cmp(&rhs.timestamp())
    }
}

impl Hash for OffsetDateTime {
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// # use std::collections::HashSet;
    /// let t1 = Date::from_ymd(2019, 1, 1)
    ///     .midnight()
    ///     .using_offset(UtcOffset::UTC);
    /// let t2 = Date::from_ymd(2018, 12, 31)
    ///     .with_hms(23, 0, 0)
    ///     .using_offset(UtcOffset::hours(-1));
    ///
    /// let mut hashset = HashSet::new();
    /// hashset.insert(t1);
    /// // This represents the same moment, so it is already in the set.
    /// hashset.insert(t2);
    /// assert_eq!(hashset.len(), 1);
    /// ```
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_i64(self.timestamp());
    }
}

impl Add<Duration> for OffsetDateTime {
    type Output = Self;

    /// Add the `Duration` to the `OffsetDateTime`.
    ///
    /// ```rust
    /// # use time::{Date, Duration, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC) + Duration::days(5),
    ///     Date::from_ymd(2019, 1, 6).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC) + Duration::day(),
    ///     Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC) + Duration::seconds(2),
    ///     Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC) + Duration::seconds(-2),
    ///     Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(1999, 12, 31).with_hms(23, 0, 0).using_offset(UtcOffset::UTC) + Duration::seconds(3_600),
    ///     Date::from_ymd(2000, 1, 1).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// ```
    fn add(self, duration: Duration) -> Self::Output {
        (self.datetime + duration).using_offset(self.offset)
    }
}

impl Add<StdDuration> for OffsetDateTime {
    type Output = Self;

    /// Add the `std::time::Duration` to the `OffsetDateTime`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// # use core::time::Duration;
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC) + Duration::from_secs(5 * 86_400),
    ///     Date::from_ymd(2019, 1, 6).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC) + Duration::from_secs(86_400),
    ///     Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC) + Duration::from_secs(2),
    ///     Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC),
    /// );
    /// ```
    fn add(self, duration: StdDuration) -> Self::Output {
        (self.datetime + duration).using_offset(self.offset)
    }
}

impl AddAssign<Duration> for OffsetDateTime {
    /// Add the `Duration` to the `OffsetDateTime`.
    ///
    /// ```rust
    /// # use time::{Date, Duration, UtcOffset};
    /// let mut ny19 = Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC);
    /// ny19 += Duration::days(5);
    /// assert_eq!(ny19, Date::from_ymd(2019, 1, 6).midnight().using_offset(UtcOffset::UTC));
    ///
    /// let mut nye20 = Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC);
    /// nye20 += Duration::day();
    /// assert_eq!(nye20, Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC));
    ///
    /// let mut nye20t = Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC);
    /// nye20t += Duration::seconds(2);
    /// assert_eq!(nye20t, Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC));
    ///
    /// let mut ny20t = Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC);
    /// ny20t += Duration::seconds(-2);
    /// assert_eq!(ny20t, Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC));
    /// ```
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for OffsetDateTime {
    /// Add the `std::time::Duration` to the `OffsetDateTime`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// # use core::time::Duration;
    /// let mut ny19 = Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC);
    /// ny19 += Duration::from_secs(5 * 86_400);
    /// assert_eq!(ny19, Date::from_ymd(2019, 1, 6).midnight().using_offset(UtcOffset::UTC));
    ///
    /// let mut nye20 = Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC);
    /// nye20 += Duration::from_secs(86_400);
    /// assert_eq!(nye20, Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC));
    ///
    /// let mut nye20t = Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC);
    /// nye20t += Duration::from_secs(2);
    /// assert_eq!(nye20t, Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC));
    /// ```
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for OffsetDateTime {
    type Output = Self;

    /// Subtract the `Duration` from the `OffsetDateTime`.
    ///
    /// ```rust
    /// # use time::{Date, Duration, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 6).midnight().using_offset(UtcOffset::UTC) - Duration::days(5),
    ///     Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC) - Duration::day(),
    ///     Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC) - Duration::seconds(2),
    ///     Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC) - Duration::seconds(-2),
    ///     Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(1999, 12, 31).with_hms(23, 0, 0).using_offset(UtcOffset::UTC) - Duration::seconds(-3_600),
    ///     Date::from_ymd(2000, 1, 1).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// ```
    fn sub(self, duration: Duration) -> Self::Output {
        (self.datetime - duration).using_offset(self.offset)
    }
}

impl Sub<StdDuration> for OffsetDateTime {
    type Output = Self;

    /// Subtract the `std::time::Duration` from the `OffsetDateTime`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// # use core::time::Duration;
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 6).midnight().using_offset(UtcOffset::UTC) - Duration::from_secs(5 * 86_400),
    ///     Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC) - Duration::from_secs(86_400),
    ///     Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC) - Duration::from_secs(2),
    ///     Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC),
    /// );
    /// ```
    fn sub(self, duration: StdDuration) -> Self::Output {
        (self.datetime - duration).using_offset(self.offset)
    }
}

impl SubAssign<Duration> for OffsetDateTime {
    /// Subtract the `Duration` from the `OffsetDateTime`.
    ///
    /// ```rust
    /// # use time::{Date, Duration, UtcOffset};
    /// let mut ny19 = Date::from_ymd(2019, 1, 6).midnight().using_offset(UtcOffset::UTC);
    /// ny19 -= Duration::days(5);
    /// assert_eq!(ny19, Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC));
    ///
    /// let mut ny20 = Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC);
    /// ny20 -= Duration::day();
    /// assert_eq!(ny20, Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC));
    ///
    /// let mut ny20t = Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC);
    /// ny20t -= Duration::seconds(2);
    /// assert_eq!(ny20t, Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC));
    ///
    /// let mut nye20t = Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC);
    /// nye20t -= Duration::seconds(-2);
    /// assert_eq!(nye20t, Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC));
    /// ```
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for OffsetDateTime {
    /// Subtract the `std::time::Duration` from the `OffsetDateTime`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// # use core::time::Duration;
    /// let mut ny19 = Date::from_ymd(2019, 1, 6).midnight().using_offset(UtcOffset::UTC);
    /// ny19 -= Duration::from_secs(5 * 86_400);
    /// assert_eq!(ny19, Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC));
    ///
    /// let mut ny20 = Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC);
    /// ny20 -= Duration::from_secs(86_400);
    /// assert_eq!(ny20, Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC));
    ///
    /// let mut ny20t = Date::from_ymd(2020, 1, 1).with_hms(0, 0, 1).using_offset(UtcOffset::UTC);
    /// ny20t -= Duration::from_secs(2);
    /// assert_eq!(ny20t, Date::from_ymd(2019, 12, 31).with_hms(23, 59, 59).using_offset(UtcOffset::UTC));
    /// ```
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<OffsetDateTime> for OffsetDateTime {
    type Output = Duration;

    /// Find the `Duration` between two `OffsetDateTime`s.
    ///
    /// ```rust
    /// # use time::{Date, Duration, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 2).midnight().using_offset(UtcOffset::UTC)
    ///         - Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC),
    ///     Duration::day(),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1).midnight().using_offset(UtcOffset::UTC)
    ///         - Date::from_ymd(2019, 1, 2).midnight().using_offset(UtcOffset::UTC),
    ///     -Duration::day(),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC)
    ///         - Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC),
    ///     Duration::day(),
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 12, 31).midnight().using_offset(UtcOffset::UTC)
    ///         - Date::from_ymd(2020, 1, 1).midnight().using_offset(UtcOffset::UTC),
    ///     -Duration::day(),
    /// );
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Duration::seconds(self.timestamp() - rhs.timestamp())
    }
}
