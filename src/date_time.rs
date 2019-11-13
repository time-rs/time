use crate::format::parse::{parse, ParseResult, ParsedItems};
#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
#[cfg(feature = "std")]
use crate::Sign;
use crate::{Date, DeferredFormat, Duration, Language, OffsetDateTime, Time, UtcOffset, Weekday};
use core::cmp::Ordering;
#[cfg(feature = "std")]
use core::convert::{From, TryFrom};
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration as StdDuration;
#[cfg(feature = "std")]
use std::time::SystemTime;

/// Combined date and time.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTime {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) date: Date,
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) time: Time,
}

impl DateTime {
    /// Create a new `DateTime` from the provided `Date` and `Time`.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(
    ///     DateTime::new(Date::from_ymd(2019, 1, 1), Time::midnight()),
    ///     Date::from_ymd(2019, 1, 1).midnight(),
    /// );
    /// ```
    #[inline(always)]
    pub const fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    /// Create a new `DateTime` with the current date and time (UTC).
    ///
    /// ```rust
    /// # use time::DateTime;
    /// assert!(DateTime::now().year() >= 2019);
    /// ```
    ///
    /// This method is not available with `#![no_std]`.
    #[inline(always)]
    #[cfg(feature = "std")]
    pub fn now() -> Self {
        SystemTime::now().into()
    }

    /// Midnight, 1 January, 1970 (UTC).
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(DateTime::unix_epoch(), Date::from_ymd(1970, 1, 1).midnight());
    /// ```
    #[inline(always)]
    pub const fn unix_epoch() -> Self {
        Self {
            date: Date {
                year: 1970,
                ordinal: 1,
            },
            time: Time {
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: 0,
            },
        }
    }

    /// Create a `DateTime` from the provided [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::{Date, DateTime};
    /// assert_eq!(DateTime::from_unix_timestamp(0), DateTime::unix_epoch());
    /// assert_eq!(
    ///     DateTime::from_unix_timestamp(1_546_300_800),
    ///     Date::from_ymd(2019, 1, 1).midnight(),
    /// );
    /// ```
    #[inline(always)]
    pub fn from_unix_timestamp(timestamp: i64) -> Self {
        Self::unix_epoch() + Duration::seconds(timestamp)
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time)
    /// representing the `DateTime`.
    ///
    /// ```rust
    /// # use time::{Date, DateTime};
    /// assert_eq!(DateTime::unix_epoch().timestamp(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().timestamp(), 1_546_300_800);
    /// ```
    #[inline(always)]
    pub fn timestamp(self) -> i64 {
        (self - Self::unix_epoch()).whole_seconds()
    }

    /// Get the `Date` component of the `DateTime`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().date(), Date::from_ymd(2019, 1, 1));
    /// ```
    #[inline(always)]
    pub const fn date(self) -> Date {
        self.date
    }

    /// Get the `Time` component of the `DateTime`.
    ///
    /// ```rust
    /// # use time::{Date, Time};
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().time(), Time::midnight());
    #[inline(always)]
    pub const fn time(self) -> Time {
        self.time
    }

    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().year(), 2019);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().year(), 2019);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).midnight().year(), 2020);
    /// ```
    #[inline(always)]
    pub fn year(self) -> i32 {
        self.date().year()
    }

    /// Get the month of the date. If fetching both the month and day, it is
    /// more efficient to use [`DateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().month(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().month(), 12);
    /// ```
    #[inline(always)]
    pub fn month(self) -> u8 {
        self.date().month()
    }

    /// Get the day of the date.  If fetching both the month and day, it is
    /// more efficient to use [`DateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().day(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().day(), 31);
    /// ```
    #[inline(always)]
    pub fn day(self) -> u8 {
        self.date().day()
    }

    /// Get the month and day of the date. This is more efficient than fetching
    /// the components individually.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().month_day(), (1, 1));
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().month_day(), (12, 31));
    /// ```
    #[inline(always)]
    pub fn month_day(self) -> (u8, u8) {
        self.date().month_day()
    }

    /// Get the day of the year.
    ///
    /// The returned value will always be in the range `1..=366` (`1..=365` for
    /// common years).
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().ordinal(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).midnight().ordinal(), 365);
    /// ```
    #[inline(always)]
    pub fn ordinal(self) -> u16 {
        self.date().ordinal()
    }

    /// Get the ISO 8601 year and week number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().iso_year_week(), (2019, 1));
    /// assert_eq!(Date::from_ymd(2019, 10, 4).midnight().iso_year_week(), (2019, 40));
    /// assert_eq!(Date::from_ymd(2020, 1, 1).midnight().iso_year_week(), (2020, 1));
    /// assert_eq!(Date::from_ymd(2020, 12, 31).midnight().iso_year_week(), (2020, 53));
    /// assert_eq!(Date::from_ymd(2021, 1, 1).midnight().iso_year_week(), (2020, 53));
    /// ```
    #[inline]
    pub fn iso_year_week(self) -> (i32, u8) {
        self.date().iso_year_week()
    }

    /// Get the ISO week number.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().week(), 1);
    /// assert_eq!(Date::from_ymd(2019, 10, 4).midnight().week(), 40);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).midnight().week(), 1);
    /// assert_eq!(Date::from_ymd(2020, 12, 31).midnight().week(), 53);
    /// assert_eq!(Date::from_ymd(2021, 1, 1).midnight().week(), 53);
    /// ```
    #[inline(always)]
    pub fn week(self) -> u8 {
        self.date().week()
    }

    /// Get the week number where week 1 begins on the first Sunday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().sunday_based_week(), 0);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).midnight().sunday_based_week(), 0);
    /// assert_eq!(Date::from_ymd(2020, 12, 31).midnight().sunday_based_week(), 52);
    /// assert_eq!(Date::from_ymd(2021, 1, 1).midnight().sunday_based_week(), 0);
    /// ```
    #[inline(always)]
    pub fn sunday_based_week(self) -> u8 {
        self.date().sunday_based_week()
    }

    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().monday_based_week(), 0);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).midnight().monday_based_week(), 0);
    /// assert_eq!(Date::from_ymd(2020, 12, 31).midnight().monday_based_week(), 52);
    /// assert_eq!(Date::from_ymd(2021, 1, 1).midnight().monday_based_week(), 0);
    /// ```
    #[inline(always)]
    pub fn monday_based_week(self) -> u8 {
        self.date().monday_based_week()
    }

    /// Get the weekday.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert_eq!(Date::from_ymd(2019, 1, 1).midnight().weekday(), Tuesday);
    /// assert_eq!(Date::from_ymd(2019, 2, 1).midnight().weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 3, 1).midnight().weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 4, 1).midnight().weekday(), Monday);
    /// assert_eq!(Date::from_ymd(2019, 5, 1).midnight().weekday(), Wednesday);
    /// assert_eq!(Date::from_ymd(2019, 6, 1).midnight().weekday(), Saturday);
    /// assert_eq!(Date::from_ymd(2019, 7, 1).midnight().weekday(), Monday);
    /// assert_eq!(Date::from_ymd(2019, 8, 1).midnight().weekday(), Thursday);
    /// assert_eq!(Date::from_ymd(2019, 9, 1).midnight().weekday(), Sunday);
    /// assert_eq!(Date::from_ymd(2019, 10, 1).midnight().weekday(), Tuesday);
    /// assert_eq!(Date::from_ymd(2019, 11, 1).midnight().weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 12, 1).midnight().weekday(), Sunday);
    /// ```
    #[inline(always)]
    pub fn weekday(self) -> Weekday {
        self.date().weekday()
    }

    /// Get the clock hour.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(0, 0, 0).hour(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(23, 59, 59).hour(), 23);
    /// ```
    #[inline(always)]
    pub const fn hour(self) -> u8 {
        self.time().hour()
    }

    /// Get the minute within the hour.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(0, 0, 0).minute(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(23, 59, 59).minute(), 59);
    /// ```
    #[inline(always)]
    pub const fn minute(self) -> u8 {
        self.time().minute()
    }

    /// Get the second within the minute.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(0, 0, 0).second(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms(23, 59, 59).second(), 59);
    /// ```
    #[inline(always)]
    pub const fn second(self) -> u8 {
        self.time().second()
    }

    /// Get the milliseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_milli(0, 0, 0, 0).millisecond(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_milli(23, 59, 59, 999).millisecond(), 999);
    /// ```
    #[inline(always)]
    pub const fn millisecond(self) -> u16 {
        self.time().millisecond()
    }

    /// Get the microseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_micro(0, 0, 0, 0).microsecond(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_micro(23, 59, 59, 999_999).microsecond(), 999_999);
    /// ```
    #[inline(always)]
    pub const fn microsecond(self) -> u32 {
        self.time().microsecond()
    }

    /// Get the nanoseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_nano(0, 0, 0, 0).nanosecond(), 0);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).with_hms_nano(23, 59, 59, 999_999_999).nanosecond(), 999_999_999);
    /// ```
    #[inline(always)]
    pub const fn nanosecond(self) -> u32 {
        self.time().nanosecond()
    }

    /// Create an `OffsetDateTime` from the existing `DateTime` and provided
    /// `UtcOffset`.
    ///
    /// ```rust
    /// # use time::{Date, UtcOffset};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 1)
    ///         .midnight()
    ///         .using_offset(UtcOffset::UTC)
    ///         .timestamp(),
    ///     1_546_300_800,
    /// );
    /// ```
    #[inline(always)]
    pub const fn using_offset(self, offset: UtcOffset) -> OffsetDateTime {
        OffsetDateTime {
            datetime: self,
            offset,
        }
    }
}

/// Methods that allow formatting the `DateTime`.
impl DateTime {
    /// Format the `DateTime` using the provided string. As no language is
    /// specified, English is used.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 2).midnight().format("%F %r"), "2019-01-02 12:00:00 am");
    /// ```
    #[inline(always)]
    pub fn format(self, format: &str) -> String {
        DeferredFormat {
            date: Some(self.date()),
            time: Some(self.time()),
            offset: None,
            format: crate::format::parse_with_language(format, Language::en),
        }
        .to_string()
    }

    /// Format the `DateTime` using the provided string and language.
    ///
    /// ```rust
    /// # use time::{Date, Language};
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 2).midnight().format_language("%c", Language::en),
    ///     "Wed Jan 2 0:00:00 2019",
    /// );
    /// assert_eq!(
    ///     Date::from_ymd(2019, 1, 2).midnight().format_language("%c", Language::es),
    ///     "Mi enero 2 0:00:00 2019",
    /// );
    /// ```
    #[inline(always)]
    pub fn format_language(self, format: &str, language: Language) -> String {
        DeferredFormat {
            date: Some(self.date()),
            time: Some(self.time()),
            offset: None,
            format: crate::format::parse_with_language(format, language),
        }
        .to_string()
    }

    /// Attempt to parse a `DateTime` using the provided string. As no language
    /// is specified, English is used.
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

    /// Attempt to parse a `DateTime` using the provided string and language.
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
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        Ok(Self {
            date: Date::try_from_parsed_items(items)?,
            time: Time::try_from_parsed_items(items)?,
        })
    }
}

impl Add<Duration> for DateTime {
    type Output = Self;

    #[inline]
    fn add(self, duration: Duration) -> Self::Output {
        #[allow(clippy::cast_possible_truncation)]
        let nanos = self.time.nanoseconds_since_midnight() as i64
            + (duration.whole_nanoseconds() % 86_400_000_000_000) as i64;

        let date_modifier = if nanos < 0 {
            -Duration::day()
        } else if nanos >= 86_400_000_000_000 {
            Duration::day()
        } else {
            Duration::zero()
        };
        Self::new(self.date + duration + date_modifier, self.time + duration)
    }
}

#[cfg(feature = "std")]
impl Add<Duration> for SystemTime {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: Duration) -> Self::Output {
        (DateTime::from(self) + duration).into()
    }
}

impl Add<StdDuration> for DateTime {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: StdDuration) -> Self::Output {
        self + Duration::from(duration)
    }
}

impl AddAssign<Duration> for DateTime {
    #[inline(always)]
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for DateTime {
    #[inline(always)]
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

#[cfg(feature = "std")]
impl AddAssign<Duration> for SystemTime {
    #[inline(always)]
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for DateTime {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl Sub<StdDuration> for DateTime {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: StdDuration) -> Self::Output {
        self - Duration::from(duration)
    }
}

#[cfg(feature = "std")]
impl Sub<Duration> for SystemTime {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: Duration) -> Self::Output {
        (DateTime::from(self) - duration).into()
    }
}

impl SubAssign<Duration> for DateTime {
    #[inline(always)]
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for DateTime {
    #[inline(always)]
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

#[cfg(feature = "std")]
impl SubAssign<Duration> for SystemTime {
    #[inline(always)]
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl Sub<DateTime> for DateTime {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        (self.date - rhs.date) + (self.time - rhs.time)
    }
}

#[cfg(feature = "std")]
impl Sub<SystemTime> for DateTime {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: SystemTime) -> Self::Output {
        self - Self::from(rhs)
    }
}

#[cfg(feature = "std")]
impl Sub<DateTime> for SystemTime {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: DateTime) -> Self::Output {
        DateTime::from(self) - rhs
    }
}

impl PartialOrd for DateTime {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "std")]
impl PartialEq<SystemTime> for DateTime {
    #[inline(always)]
    fn eq(&self, rhs: &SystemTime) -> bool {
        self == &Self::from(*rhs)
    }
}

#[cfg(feature = "std")]
impl PartialEq<DateTime> for SystemTime {
    #[inline(always)]
    fn eq(&self, rhs: &DateTime) -> bool {
        &DateTime::from(*self) == rhs
    }
}

#[cfg(feature = "std")]
impl PartialOrd<SystemTime> for DateTime {
    #[inline(always)]
    fn partial_cmp(&self, other: &SystemTime) -> Option<Ordering> {
        self.partial_cmp(&Self::from(*other))
    }
}

#[cfg(feature = "std")]
impl PartialOrd<DateTime> for SystemTime {
    #[inline(always)]
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
        DateTime::from(*self).partial_cmp(other)
    }
}

impl Ord for DateTime {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.date.cmp(&other.date) {
            Ordering::Equal => match self.time.hour.cmp(&other.time.hour) {
                Ordering::Equal => match self.time.minute.cmp(&other.time.minute) {
                    Ordering::Equal => match self.time.second.cmp(&other.time.second) {
                        Ordering::Equal => self.time.nanosecond.cmp(&other.time.nanosecond),
                        other => other,
                    },
                    other => other,
                },
                other => other,
            },
            other => other,
        }
    }
}

#[cfg(feature = "std")]
impl From<SystemTime> for DateTime {
    #[inline(always)]
    fn from(system_time: SystemTime) -> Self {
        let duration = match system_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Duration::from(duration),
            Err(err) => -Duration::from(err.duration()),
        };

        Self::unix_epoch() + duration
    }
}

#[cfg(feature = "std")]
#[allow(clippy::fallible_impl_from)]
impl From<DateTime> for SystemTime {
    fn from(datetime: DateTime) -> Self {
        let duration = datetime - DateTime::unix_epoch();

        match duration.sign() {
            Sign::Positive => {
                Self::UNIX_EPOCH
                    + StdDuration::try_from(duration).unwrap_or_else(|_| unreachable!(
                        "The value is guaranteed to be positive (and is convertible to StdDuration)."
                    ))
            }
            Sign::Negative => {
                Self::UNIX_EPOCH
                    + StdDuration::try_from(-duration).unwrap_or_else(|_| unreachable!(
                        "The value is guaranteed to be positive (and is convertible to StdDuration)."
                    ))
            }
            Sign::Zero => Self::UNIX_EPOCH,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    macro_rules! ymd {
        ($year:literal, $month:literal, $day:literal) => {
            Date::from_ymd($year, $month, $day)
        };
    }

    #[test]
    fn new() {
        assert_eq!(
            DateTime::new(ymd!(2019, 1, 1), Time::midnight()),
            ymd!(2019, 1, 1).midnight(),
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn now() {
        assert!(DateTime::now().year() >= 2019);
    }

    #[test]
    fn unix_epoch() {
        assert_eq!(DateTime::unix_epoch(), ymd!(1970, 1, 1).midnight());
    }

    #[test]
    fn from_unix_timestamp() {
        assert_eq!(DateTime::from_unix_timestamp(0), DateTime::unix_epoch());
        assert_eq!(
            DateTime::from_unix_timestamp(1_546_300_800),
            ymd!(2019, 1, 1).midnight(),
        );
    }

    #[test]
    fn timestamp() {
        assert_eq!(DateTime::unix_epoch().timestamp(), 0);
        assert_eq!(ymd!(2019, 1, 1).midnight().timestamp(), 1_546_300_800);
    }

    #[test]
    fn date() {
        assert_eq!(ymd!(2019, 1, 1).midnight().date(), ymd!(2019, 1, 1));
    }

    #[test]
    fn time() {
        assert_eq!(ymd!(2019, 1, 1).midnight().time(), Time::midnight());
    }

    #[test]
    fn year() {
        assert_eq!(ymd!(2019, 1, 1).midnight().year(), 2019);
        assert_eq!(ymd!(2019, 12, 31).midnight().year(), 2019);
        assert_eq!(ymd!(2020, 1, 1).midnight().year(), 2020);
    }

    #[test]
    fn month() {
        assert_eq!(ymd!(2019, 1, 1).midnight().month(), 1);
        assert_eq!(ymd!(2019, 12, 31).midnight().month(), 12);
    }

    #[test]
    fn day() {
        assert_eq!(ymd!(2019, 1, 1).midnight().day(), 1);
        assert_eq!(ymd!(2019, 12, 31).midnight().day(), 31);
    }

    #[test]
    fn month_day() {
        assert_eq!(ymd!(2019, 1, 1).midnight().month_day(), (1, 1));
        assert_eq!(ymd!(2019, 12, 31).midnight().month_day(), (12, 31));
    }

    #[test]
    fn ordinal() {
        assert_eq!(ymd!(2019, 1, 1).midnight().ordinal(), 1);
        assert_eq!(ymd!(2019, 12, 31).midnight().ordinal(), 365);
    }

    #[test]
    fn week() {
        assert_eq!(ymd!(2019, 1, 1).midnight().week(), 1);
        assert_eq!(ymd!(2019, 10, 4).midnight().week(), 40);
        assert_eq!(ymd!(2020, 1, 1).midnight().week(), 1);
        assert_eq!(ymd!(2020, 12, 31).midnight().week(), 53);
        assert_eq!(ymd!(2021, 1, 1).midnight().week(), 53);
    }

    #[test]
    fn sunday_based_week() {
        assert_eq!(ymd!(2019, 1, 1).midnight().sunday_based_week(), 0);
        assert_eq!(ymd!(2020, 1, 1).midnight().sunday_based_week(), 0);
        assert_eq!(ymd!(2020, 12, 31).midnight().sunday_based_week(), 52);
        assert_eq!(ymd!(2021, 1, 1).midnight().sunday_based_week(), 0);
    }

    #[test]
    fn monday_based_week() {
        assert_eq!(ymd!(2019, 1, 1).midnight().monday_based_week(), 0);
        assert_eq!(ymd!(2020, 1, 1).midnight().monday_based_week(), 0);
        assert_eq!(ymd!(2020, 12, 31).midnight().monday_based_week(), 52);
        assert_eq!(ymd!(2021, 1, 1).midnight().monday_based_week(), 0);
    }

    #[test]
    fn weekday() {
        use Weekday::*;
        assert_eq!(ymd!(2019, 1, 1).midnight().weekday(), Tuesday);
        assert_eq!(ymd!(2019, 2, 1).midnight().weekday(), Friday);
        assert_eq!(ymd!(2019, 3, 1).midnight().weekday(), Friday);
        assert_eq!(ymd!(2019, 4, 1).midnight().weekday(), Monday);
        assert_eq!(ymd!(2019, 5, 1).midnight().weekday(), Wednesday);
        assert_eq!(ymd!(2019, 6, 1).midnight().weekday(), Saturday);
        assert_eq!(ymd!(2019, 7, 1).midnight().weekday(), Monday);
        assert_eq!(ymd!(2019, 8, 1).midnight().weekday(), Thursday);
        assert_eq!(ymd!(2019, 9, 1).midnight().weekday(), Sunday);
        assert_eq!(ymd!(2019, 10, 1).midnight().weekday(), Tuesday);
        assert_eq!(ymd!(2019, 11, 1).midnight().weekday(), Friday);
        assert_eq!(ymd!(2019, 12, 1).midnight().weekday(), Sunday);
    }

    #[test]
    fn hour() {
        assert_eq!(ymd!(2019, 1, 1).with_hms(0, 0, 0).hour(), 0);
        assert_eq!(ymd!(2019, 1, 1).with_hms(23, 59, 59).hour(), 23);
    }

    #[test]
    fn minute() {
        assert_eq!(ymd!(2019, 1, 1).with_hms(0, 0, 0).minute(), 0);
        assert_eq!(ymd!(2019, 1, 1).with_hms(23, 59, 59).minute(), 59);
    }

    #[test]
    fn second() {
        assert_eq!(ymd!(2019, 1, 1).with_hms(0, 0, 0).second(), 0);
        assert_eq!(ymd!(2019, 1, 1).with_hms(23, 59, 59).second(), 59);
    }

    #[test]
    fn millisecond() {
        assert_eq!(ymd!(2019, 1, 1).with_hms_milli(0, 0, 0, 0).millisecond(), 0);
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_milli(23, 59, 59, 999)
                .millisecond(),
            999
        );
    }

    #[test]
    fn microsecond() {
        assert_eq!(ymd!(2019, 1, 1).with_hms_micro(0, 0, 0, 0).microsecond(), 0);
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_micro(23, 59, 59, 999_999)
                .microsecond(),
            999_999
        );
    }

    #[test]
    fn nanosecond() {
        assert_eq!(ymd!(2019, 1, 1).with_hms_nano(0, 0, 0, 0).nanosecond(), 0);
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_nano(23, 59, 59, 999_999_999)
                .nanosecond(),
            999_999_999
        );
    }

    #[test]
    fn using_offset() {
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .using_offset(UtcOffset::UTC)
                .timestamp(),
            1_546_300_800,
        );
    }

    #[test]
    fn format() {
        assert_eq!(
            ymd!(2019, 1, 2).midnight().format("%F %r"),
            "2019-01-02 12:00:00 am"
        );
    }

    #[test]
    fn format_language() {
        assert_eq!(
            ymd!(2019, 1, 2)
                .midnight()
                .format_language("%c", Language::en),
            "Wed Jan 2 0:00:00 2019",
        );
        assert_eq!(
            ymd!(2019, 1, 2)
                .midnight()
                .format_language("%c", Language::es),
            "Mi enero 2 0:00:00 2019",
        );
    }

    #[test]
    fn parse() {
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
            Ok(Date::from_iso_ywd(2019, 1, Weekday::Wednesday).with_hms(12, 0, 0)),
        );
    }

    #[test]
    fn parse_language() {
        assert_eq!(
            DateTime::parse_language("January 02 2019 12:00:00 am", "%B %d %Y %r", Language::en),
            Ok(ymd!(2019, 1, 2).midnight()),
        );
        assert_eq!(
            DateTime::parse_language("02 enero 2019 00:00:00", "%d %B %Y %T", Language::es),
            Ok(ymd!(2019, 1, 2).midnight()),
        );
    }

    #[test]
    fn add_duration() {
        assert_eq!(
            ymd!(2019, 1, 1).midnight() + 5.days(),
            ymd!(2019, 1, 6).midnight(),
        );
        assert_eq!(
            ymd!(2019, 12, 31).midnight() + 1.days(),
            ymd!(2020, 1, 1).midnight(),
        );
        assert_eq!(
            ymd!(2019, 12, 31).with_hms(23, 59, 59) + 2.seconds(),
            ymd!(2020, 1, 1).with_hms(0, 0, 1),
        );
        assert_eq!(
            ymd!(2020, 1, 1).with_hms(0, 0, 1) + (-2).seconds(),
            ymd!(2019, 12, 31).with_hms(23, 59, 59),
        );
        assert_eq!(
            ymd!(1999, 12, 31).with_hms(23, 0, 0) + 1.hours(),
            ymd!(2000, 1, 1).midnight(),
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn std_add_duration() {
        assert_eq!(
            SystemTime::from(ymd!(2019, 1, 1).midnight()) + 5.days(),
            SystemTime::from(ymd!(2019, 1, 6).midnight()),
        );
        assert_eq!(
            SystemTime::from(ymd!(2019, 12, 31).midnight()) + 1.days(),
            SystemTime::from(ymd!(2020, 1, 1).midnight()),
        );
        assert_eq!(
            SystemTime::from(ymd!(2019, 12, 31).with_hms(23, 59, 59)) + 2.seconds(),
            SystemTime::from(ymd!(2020, 1, 1).with_hms(0, 0, 1)),
        );
        assert_eq!(
            SystemTime::from(ymd!(2020, 1, 1).with_hms(0, 0, 1)) + (-2).seconds(),
            SystemTime::from(ymd!(2019, 12, 31).with_hms(23, 59, 59)),
        );
    }

    #[test]
    fn add_std_duration() {
        assert_eq!(
            ymd!(2019, 1, 1).midnight() + 5.std_days(),
            ymd!(2019, 1, 6).midnight(),
        );
        assert_eq!(
            ymd!(2019, 12, 31).midnight() + 1.std_days(),
            ymd!(2020, 1, 1).midnight(),
        );
        assert_eq!(
            ymd!(2019, 12, 31).with_hms(23, 59, 59) + 2.std_seconds(),
            ymd!(2020, 1, 1).with_hms(0, 0, 1),
        );
    }

    #[test]
    fn add_assign_duration() {
        let mut ny19 = ymd!(2019, 1, 1).midnight();
        ny19 += 5.days();
        assert_eq!(ny19, ymd!(2019, 1, 6).midnight());

        let mut nye20 = ymd!(2019, 12, 31).midnight();
        nye20 += 1.days();
        assert_eq!(nye20, ymd!(2020, 1, 1).midnight());

        let mut nye20t = ymd!(2019, 12, 31).with_hms(23, 59, 59);
        nye20t += 2.seconds();
        assert_eq!(nye20t, ymd!(2020, 1, 1).with_hms(0, 0, 1));

        let mut ny20t = ymd!(2020, 1, 1).with_hms(0, 0, 1);
        ny20t += (-2).seconds();
        assert_eq!(ny20t, ymd!(2019, 12, 31).with_hms(23, 59, 59));
    }

    #[test]
    fn add_assign_std_duration() {
        let mut ny19 = ymd!(2019, 1, 1).midnight();
        ny19 += 5.std_days();
        assert_eq!(ny19, ymd!(2019, 1, 6).midnight());

        let mut nye20 = ymd!(2019, 12, 31).midnight();
        nye20 += 1.std_days();
        assert_eq!(nye20, ymd!(2020, 1, 1).midnight());

        let mut nye20t = ymd!(2019, 12, 31).with_hms(23, 59, 59);
        nye20t += 2.std_seconds();
        assert_eq!(nye20t, ymd!(2020, 1, 1).with_hms(0, 0, 1));
    }

    #[test]
    #[cfg(feature = "std")]
    fn std_add_assign_duration() {
        let mut ny19 = SystemTime::from(ymd!(2019, 1, 1).midnight());
        ny19 += 5.days();
        assert_eq!(ny19, ymd!(2019, 1, 6).midnight());

        let mut nye20 = SystemTime::from(ymd!(2019, 12, 31).midnight());
        nye20 += 1.days();
        assert_eq!(nye20, ymd!(2020, 1, 1).midnight());

        let mut nye20t = SystemTime::from(ymd!(2019, 12, 31).with_hms(23, 59, 59));
        nye20t += 2.seconds();
        assert_eq!(nye20t, ymd!(2020, 1, 1).with_hms(0, 0, 1));

        let mut ny20t = SystemTime::from(ymd!(2020, 1, 1).with_hms(0, 0, 1));
        ny20t += (-2).seconds();
        assert_eq!(ny20t, ymd!(2019, 12, 31).with_hms(23, 59, 59));
    }

    #[test]
    fn sub_duration() {
        assert_eq!(
            ymd!(2019, 1, 6).midnight() - 5.days(),
            ymd!(2019, 1, 1).midnight(),
        );
        assert_eq!(
            ymd!(2020, 1, 1).midnight() - 1.days(),
            ymd!(2019, 12, 31).midnight(),
        );
        assert_eq!(
            ymd!(2020, 1, 1).with_hms(0, 0, 1) - 2.seconds(),
            ymd!(2019, 12, 31).with_hms(23, 59, 59),
        );
        assert_eq!(
            ymd!(2019, 12, 31).with_hms(23, 59, 59) - (-2).seconds(),
            ymd!(2020, 1, 1).with_hms(0, 0, 1),
        );
        assert_eq!(
            ymd!(1999, 12, 31).with_hms(23, 0, 0) - (-1).hours(),
            ymd!(2000, 1, 1).midnight(),
        );
    }

    #[test]
    fn sub_std_duration() {
        assert_eq!(
            ymd!(2019, 1, 6).midnight() - 5.std_days(),
            ymd!(2019, 1, 1).midnight(),
        );
        assert_eq!(
            ymd!(2020, 1, 1).midnight() - 1.std_days(),
            ymd!(2019, 12, 31).midnight(),
        );
        assert_eq!(
            ymd!(2020, 1, 1).with_hms(0, 0, 1) - 2.std_seconds(),
            ymd!(2019, 12, 31).with_hms(23, 59, 59),
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn std_sub_duration() {
        assert_eq!(
            SystemTime::from(ymd!(2019, 1, 6).midnight()) - 5.days(),
            SystemTime::from(ymd!(2019, 1, 1).midnight()),
        );
        assert_eq!(
            SystemTime::from(ymd!(2020, 1, 1).midnight()) - 1.days(),
            SystemTime::from(ymd!(2019, 12, 31).midnight()),
        );
        assert_eq!(
            SystemTime::from(ymd!(2020, 1, 1).with_hms(0, 0, 1)) - 2.seconds(),
            SystemTime::from(ymd!(2019, 12, 31).with_hms(23, 59, 59)),
        );
        assert_eq!(
            SystemTime::from(ymd!(2019, 12, 31).with_hms(23, 59, 59)) - (-2).seconds(),
            SystemTime::from(ymd!(2020, 1, 1).with_hms(0, 0, 1)),
        );
    }

    #[test]
    fn sub_assign_duration() {
        let mut ny19 = ymd!(2019, 1, 6).midnight();
        ny19 -= 5.days();
        assert_eq!(ny19, ymd!(2019, 1, 1).midnight());

        let mut ny20 = ymd!(2020, 1, 1).midnight();
        ny20 -= 1.days();
        assert_eq!(ny20, ymd!(2019, 12, 31).midnight());

        let mut ny20t = ymd!(2020, 1, 1).with_hms(0, 0, 1);
        ny20t -= 2.seconds();
        assert_eq!(ny20t, ymd!(2019, 12, 31).with_hms(23, 59, 59));

        let mut nye20t = ymd!(2019, 12, 31).with_hms(23, 59, 59);
        nye20t -= (-2).seconds();
        assert_eq!(nye20t, ymd!(2020, 1, 1).with_hms(0, 0, 1));
    }

    #[test]
    fn sub_assign_std_duration() {
        let mut ny19 = ymd!(2019, 1, 6).midnight();
        ny19 -= 5.std_days();
        assert_eq!(ny19, ymd!(2019, 1, 1).midnight());

        let mut ny20 = ymd!(2020, 1, 1).midnight();
        ny20 -= 1.std_days();
        assert_eq!(ny20, ymd!(2019, 12, 31).midnight());

        let mut ny20t = ymd!(2020, 1, 1).with_hms(0, 0, 1);
        ny20t -= 2.std_seconds();
        assert_eq!(ny20t, ymd!(2019, 12, 31).with_hms(23, 59, 59));
    }

    #[test]
    #[cfg(feature = "std")]
    fn std_sub_assign_duration() {
        let mut ny19 = SystemTime::from(ymd!(2019, 1, 6).midnight());
        ny19 -= 5.days();
        assert_eq!(ny19, ymd!(2019, 1, 1).midnight());

        let mut ny20 = SystemTime::from(ymd!(2020, 1, 1).midnight());
        ny20 -= 1.days();
        assert_eq!(ny20, ymd!(2019, 12, 31).midnight());

        let mut ny20t = SystemTime::from(ymd!(2020, 1, 1).with_hms(0, 0, 1));
        ny20t -= 2.seconds();
        assert_eq!(ny20t, ymd!(2019, 12, 31).with_hms(23, 59, 59));

        let mut nye20t = SystemTime::from(ymd!(2019, 12, 31).with_hms(23, 59, 59));
        nye20t -= (-2).seconds();
        assert_eq!(nye20t, ymd!(2020, 1, 1).with_hms(0, 0, 1));
    }

    #[test]
    fn sub_datetime() {
        assert_eq!(
            ymd!(2019, 1, 2).midnight() - ymd!(2019, 1, 1).midnight(),
            1.days()
        );
        assert_eq!(
            ymd!(2019, 1, 1).midnight() - ymd!(2019, 1, 2).midnight(),
            (-1).days()
        );
        assert_eq!(
            ymd!(2020, 1, 1).midnight() - ymd!(2019, 12, 31).midnight(),
            1.days()
        );
        assert_eq!(
            ymd!(2019, 12, 31).midnight() - ymd!(2020, 1, 1).midnight(),
            (-1).days()
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn std_sub_datetime() {
        assert_eq!(
            SystemTime::from(ymd!(2019, 1, 2).midnight()) - ymd!(2019, 1, 1).midnight(),
            1.days()
        );
        assert_eq!(
            SystemTime::from(ymd!(2019, 1, 1).midnight()) - ymd!(2019, 1, 2).midnight(),
            (-1).days()
        );
        assert_eq!(
            SystemTime::from(ymd!(2020, 1, 1).midnight()) - ymd!(2019, 12, 31).midnight(),
            1.days()
        );
        assert_eq!(
            SystemTime::from(ymd!(2019, 12, 31).midnight()) - ymd!(2020, 1, 1).midnight(),
            (-1).days()
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn sub_std() {
        assert_eq!(
            ymd!(2019, 1, 2).midnight() - SystemTime::from(ymd!(2019, 1, 1).midnight()),
            1.days()
        );
        assert_eq!(
            ymd!(2019, 1, 1).midnight() - SystemTime::from(ymd!(2019, 1, 2).midnight()),
            (-1).days()
        );
        assert_eq!(
            ymd!(2020, 1, 1).midnight() - SystemTime::from(ymd!(2019, 12, 31).midnight()),
            1.days()
        );
        assert_eq!(
            ymd!(2019, 12, 31).midnight() - SystemTime::from(ymd!(2020, 1, 1).midnight()),
            (-1).days()
        );
    }

    #[test]
    fn ord() {
        use Ordering::*;
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .partial_cmp(&ymd!(2019, 1, 1).midnight()),
            Some(Equal)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .partial_cmp(&ymd!(2020, 1, 1).midnight()),
            Some(Less)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .partial_cmp(&ymd!(2019, 2, 1).midnight()),
            Some(Less)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .partial_cmp(&ymd!(2019, 1, 2).midnight()),
            Some(Less)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .partial_cmp(&ymd!(2019, 1, 1).with_hms(1, 0, 0)),
            Some(Less)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .partial_cmp(&ymd!(2019, 1, 1).with_hms(0, 1, 0)),
            Some(Less)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .partial_cmp(&ymd!(2019, 1, 1).with_hms(0, 0, 1)),
            Some(Less)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .midnight()
                .partial_cmp(&ymd!(2019, 1, 1).with_hms_nano(0, 0, 0, 1)),
            Some(Less)
        );
        assert_eq!(
            ymd!(2020, 1, 1)
                .midnight()
                .partial_cmp(&ymd!(2019, 1, 1).midnight()),
            Some(Greater)
        );
        assert_eq!(
            ymd!(2019, 2, 1)
                .midnight()
                .partial_cmp(&ymd!(2019, 1, 1).midnight()),
            Some(Greater)
        );
        assert_eq!(
            ymd!(2019, 1, 2)
                .midnight()
                .partial_cmp(&ymd!(2019, 1, 1).midnight()),
            Some(Greater)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(1, 0, 0)
                .partial_cmp(&ymd!(2019, 1, 1).midnight()),
            Some(Greater)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(0, 1, 0)
                .partial_cmp(&ymd!(2019, 1, 1).midnight()),
            Some(Greater)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms(0, 0, 1)
                .partial_cmp(&ymd!(2019, 1, 1).midnight()),
            Some(Greater)
        );
        assert_eq!(
            ymd!(2019, 1, 1)
                .with_hms_nano(0, 0, 0, 1)
                .partial_cmp(&ymd!(2019, 1, 1).midnight()),
            Some(Greater)
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn eq_std() {
        let now_datetime = DateTime::now();
        let now_systemtime = SystemTime::from(now_datetime);
        assert_eq!(now_datetime, now_systemtime);
    }

    #[test]
    #[cfg(feature = "std")]
    fn std_eq() {
        #[cfg(feature = "std")]
        let now_datetime = DateTime::now();
        let now_systemtime = SystemTime::from(now_datetime);
        assert_eq!(now_datetime, now_systemtime);
    }

    #[test]
    #[cfg(feature = "std")]
    fn ord_std() {
        assert_eq!(
            ymd!(2019, 1, 1).midnight(),
            SystemTime::from(ymd!(2019, 1, 1).midnight())
        );
        assert!(ymd!(2019, 1, 1).midnight() < SystemTime::from(ymd!(2020, 1, 1).midnight()));
        assert!(ymd!(2019, 1, 1).midnight() < SystemTime::from(ymd!(2019, 2, 1).midnight()));
        assert!(ymd!(2019, 1, 1).midnight() < SystemTime::from(ymd!(2019, 1, 2).midnight()));
        assert!(ymd!(2019, 1, 1).midnight() < SystemTime::from(ymd!(2019, 1, 1).with_hms(1, 0, 0)));
        assert!(ymd!(2019, 1, 1).midnight() < SystemTime::from(ymd!(2019, 1, 1).with_hms(0, 1, 0)));
        assert!(ymd!(2019, 1, 1).midnight() < SystemTime::from(ymd!(2019, 1, 1).with_hms(0, 0, 1)));
        assert!(
            ymd!(2019, 1, 1).midnight()
                < SystemTime::from(ymd!(2019, 1, 1).with_hms_milli(0, 0, 0, 1))
        );
        assert!(ymd!(2020, 1, 1).midnight() > SystemTime::from(ymd!(2019, 1, 1).midnight()));
        assert!(ymd!(2019, 2, 1).midnight() > SystemTime::from(ymd!(2019, 1, 1).midnight()));
        assert!(ymd!(2019, 1, 2).midnight() > SystemTime::from(ymd!(2019, 1, 1).midnight()));
        assert!(ymd!(2019, 1, 1).with_hms(1, 0, 0) > SystemTime::from(ymd!(2019, 1, 1).midnight()));
        assert!(ymd!(2019, 1, 1).with_hms(0, 1, 0) > SystemTime::from(ymd!(2019, 1, 1).midnight()));
        assert!(ymd!(2019, 1, 1).with_hms(0, 0, 1) > SystemTime::from(ymd!(2019, 1, 1).midnight()));
        assert!(
            ymd!(2019, 1, 1).with_hms_nano(0, 0, 0, 1)
                > SystemTime::from(ymd!(2019, 1, 1).midnight())
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn std_ord() {
        assert_eq!(
            SystemTime::from(ymd!(2019, 1, 1).midnight()),
            ymd!(2019, 1, 1).midnight()
        );
        assert!(SystemTime::from(ymd!(2019, 1, 1).midnight()) < ymd!(2020, 1, 1).midnight());
        assert!(SystemTime::from(ymd!(2019, 1, 1).midnight()) < ymd!(2019, 2, 1).midnight());
        assert!(SystemTime::from(ymd!(2019, 1, 1).midnight()) < ymd!(2019, 1, 2).midnight());
        assert!(SystemTime::from(ymd!(2019, 1, 1).midnight()) < ymd!(2019, 1, 1).with_hms(1, 0, 0));
        assert!(SystemTime::from(ymd!(2019, 1, 1).midnight()) < ymd!(2019, 1, 1).with_hms(0, 1, 0));
        assert!(SystemTime::from(ymd!(2019, 1, 1).midnight()) < ymd!(2019, 1, 1).with_hms(0, 0, 1));
        assert!(
            SystemTime::from(ymd!(2019, 1, 1).midnight())
                < ymd!(2019, 1, 1).with_hms_nano(0, 0, 0, 1)
        );
        assert!(SystemTime::from(ymd!(2020, 1, 1).midnight()) > ymd!(2019, 1, 1).midnight());
        assert!(SystemTime::from(ymd!(2019, 2, 1).midnight()) > ymd!(2019, 1, 1).midnight());
        assert!(SystemTime::from(ymd!(2019, 1, 2).midnight()) > ymd!(2019, 1, 1).midnight());
        assert!(SystemTime::from(ymd!(2019, 1, 1).with_hms(1, 0, 0)) > ymd!(2019, 1, 1).midnight());
        assert!(SystemTime::from(ymd!(2019, 1, 1).with_hms(0, 1, 0)) > ymd!(2019, 1, 1).midnight());
        assert!(SystemTime::from(ymd!(2019, 1, 1).with_hms(0, 0, 1)) > ymd!(2019, 1, 1).midnight());
        assert!(
            SystemTime::from(ymd!(2019, 1, 1).with_hms_milli(0, 0, 0, 1))
                > ymd!(2019, 1, 1).midnight()
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn from_std() {
        assert_eq!(
            DateTime::from(SystemTime::UNIX_EPOCH),
            DateTime::unix_epoch()
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn to_std() {
        assert_eq!(
            SystemTime::from(DateTime::unix_epoch()),
            SystemTime::UNIX_EPOCH
        );
    }
}
