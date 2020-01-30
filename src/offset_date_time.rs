#[cfg(not(feature = "std"))]
use crate::alloc_prelude::*;
use crate::{
    format::parse::{parse, ParseResult, ParsedItems},
    offset, Date, DeferredFormat, Duration, PrimitiveDateTime, Time, UtcOffset, Weekday,
};
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};

/// A [`PrimitiveDateTime`] with a [`UtcOffset`].
///
/// All comparisons are performed using the UTC time.
// Internally, an `OffsetDateTime` is a thin wrapper around a
// [`PrimitiveDateTime`] coupled with a [`UtcOffset`]. This offset is added to
// the date, time, or datetime as necessary for presentation or returning from a
// function.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        try_from = "crate::serde::PrimitiveDateTime",
        into = "crate::serde::PrimitiveDateTime"
    )
)]
#[derive(Debug, Clone, Copy, Eq)]
pub struct OffsetDateTime {
    /// The `PrimitiveDateTime`, which is _always_ UTC.
    pub(crate) utc_datetime: PrimitiveDateTime,
    /// The `UtcOffset`, which will be added to the `PrimitiveDateTime` as necessary.
    pub(crate) offset: UtcOffset,
}

impl OffsetDateTime {
    /// Create a new `OffsetDateTime` with the current date and time (UTC).
    ///
    /// ```rust
    /// # use time::{OffsetDateTime, offset};
    /// assert!(OffsetDateTime::now().year() >= 2019);
    /// assert_eq!(OffsetDateTime::now().offset(), offset!(UTC));
    /// ```
    #[inline(always)]
    #[cfg(feature = "std")]
    #[cfg_attr(doc, doc(cfg(feature = "std")))]
    pub fn now() -> Self {
        PrimitiveDateTime::now().using_offset(offset!(UTC))
    }

    /// Convert the `OffsetDateTime` from the current `UtcOffset` to the
    /// provided `UtcOffset`.
    ///
    /// ```rust
    /// # use time::{date, OffsetDateTime, UtcOffset, offset, time};
    /// assert_eq!(
    ///     date!(2000-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .to_offset(offset!(-1))
    ///         .year(),
    ///     1999,
    /// );
    ///
    /// // Let's see what time Sydney's new year's celebration is in New York
    /// // and Los Angeles.
    ///
    /// // Construct midnight on new year's in Sydney. This is equivalent to
    /// // 13:00 UTC.
    /// let sydney = date!(1999-12-31).with_time(time!(13:00)).using_offset(offset!(+11));
    /// let new_york = sydney.to_offset(offset!(-5));
    /// let los_angeles = sydney.to_offset(offset!(-8));
    /// assert_eq!(sydney.hour(), 0);
    /// assert_eq!(new_york.hour(), 8);
    /// assert_eq!(los_angeles.hour(), 5);
    /// ```
    #[inline(always)]
    pub const fn to_offset(self, offset: UtcOffset) -> Self {
        self.utc_datetime.using_offset(offset)
    }

    /// Midnight, 1 January, 1970 (UTC).
    ///
    /// ```rust
    /// # use time::{date, OffsetDateTime, offset};
    /// assert_eq!(
    ///     OffsetDateTime::unix_epoch(),
    ///     date!(1970-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC)),
    /// );
    /// ```
    #[inline(always)]
    pub const fn unix_epoch() -> Self {
        PrimitiveDateTime::unix_epoch().using_offset(offset!(UTC))
    }

    /// Create an `OffsetDateTime` from the provided [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::{date, OffsetDateTime, offset};
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(0),
    ///     OffsetDateTime::unix_epoch(),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(1_546_300_800),
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC)),
    /// );
    /// ```
    #[inline(always)]
    pub fn from_unix_timestamp(timestamp: i64) -> Self {
        PrimitiveDateTime::from_unix_timestamp(timestamp).using_offset(offset!(UTC))
    }

    /// Get the `UtcOffset`.
    ///
    /// ```rust
    /// # use time::{date, offset};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .offset(),
    ///     offset!(UTC),
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(+1))
    ///         .offset(),
    ///     offset!(+1),
    /// );
    /// ```
    #[inline(always)]
    pub const fn offset(self) -> UtcOffset {
        self.offset
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::{PrimitiveDateTime, offset};
    /// assert_eq!(
    ///     PrimitiveDateTime::unix_epoch()
    ///         .using_offset(offset!(UTC))
    ///         .timestamp(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     PrimitiveDateTime::unix_epoch()
    ///         .using_offset(offset!(-1))
    ///         .timestamp(),
    ///     0,
    /// );
    /// ```
    #[inline(always)]
    pub fn timestamp(self) -> i64 {
        self.utc_datetime.timestamp()
    }

    /// Get the `Date` in the stored offset.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .date(),
    ///     date!(2019-01-01),
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(-1))
    ///         .date(),
    ///     date!(2018-12-31),
    /// );
    /// ```
    #[inline(always)]
    pub fn date(self) -> Date {
        (self.utc_datetime + self.offset.as_duration()).date()
    }

    /// Get the `Time` in the stored offset.
    ///
    /// ```rust
    /// # use time::{date, Time, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .time(),
    ///     time!(0:00)
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(-1))
    ///         .time(),
    ///     time!(23:00)
    /// );
    /// ```
    #[inline(always)]
    pub fn time(self) -> Time {
        (self.utc_datetime + self.offset.as_duration()).time()
    }

    /// Get the year of the date in the stored offset.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .year(),
    ///     2019,
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .using_offset(offset!(UTC))
    ///         .to_offset(offset!(+1))
    ///         .year(),
    ///     2020,
    /// );
    /// assert_eq!(
    ///     date!(2020-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
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
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .month(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .using_offset(offset!(+1))
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
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .day(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .using_offset(offset!(+1))
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
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .month_day(),
    ///     (1, 1),
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .using_offset(offset!(+1))
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
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .ordinal(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .using_offset(offset!(+1))
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
    /// # use time::{date, offset};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .iso_year_week(),
    ///     (2019, 1),
    /// );
    /// assert_eq!(
    ///     date!(2019-10-04)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .iso_year_week(),
    ///     (2019, 40),
    /// );
    /// assert_eq!(
    ///     date!(2020-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .iso_year_week(),
    ///     (2020, 1),
    /// );
    /// assert_eq!(
    ///     date!(2020-12-31)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .iso_year_week(),
    ///     (2020, 53),
    /// );
    /// assert_eq!(
    ///     date!(2021-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .iso_year_week(),
    ///     (2020, 53),
    /// );
    /// ```
    #[inline(always)]
    pub fn iso_year_week(self) -> (i32, u8) {
        self.date().iso_year_week()
    }

    /// Get the ISO week number of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::{date, offset};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .week(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2020-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .week(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2020-12-31)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .week(),
    ///     53,
    /// );
    /// assert_eq!(
    ///     date!(2021-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
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
    /// # use time::{date, offset, Weekday::*};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .weekday(),
    ///     Tuesday,
    /// );
    /// assert_eq!(
    ///     date!(2019-02-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .weekday(),
    ///     Friday,
    /// );
    /// assert_eq!(
    ///     date!(2019-03-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
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
    /// # use time::{date, time, offset};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .hour(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59))
    ///         .using_offset(offset!(-2))
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
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .minute(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59))
    ///         .using_offset(offset!(+0:30))
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
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .second(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59))
    ///         .using_offset(offset!(+0:00:30))
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
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .millisecond(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59.999))
    ///         .using_offset(offset!(UTC))
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
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .microsecond(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59.999_999))
    ///         .using_offset(offset!(UTC))
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
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
    ///         .nanosecond(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59.999_999_999))
    ///         .using_offset(offset!(UTC))
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
    /// Format the `OffsetDateTime` using the provided string.
    ///
    /// ```rust
    /// # use time::{date, offset};
    /// assert_eq!(
    ///     date!(2019-01-02)
    ///         .midnight()
    ///         .using_offset(offset!(UTC))
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
            format: crate::format::parse_fmt_string(format),
        }
        .to_string()
    }

    /// Attempt to parse an `OffsetDateTime` using the provided string.
    ///
    /// ```rust
    /// # use time::{date, OffsetDateTime, Weekday::Wednesday, time, offset};
    /// assert_eq!(
    ///     OffsetDateTime::parse("2019-01-02 00:00:00 +0000", "%F %T %z"),
    ///     Ok(date!(2019-01-02).midnight().using_offset(offset!(UTC))),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::parse("2019-002 23:59:59 +0000", "%Y-%j %T %z"),
    ///     Ok(date!(2019-002).with_time(time!(23:59:59)).using_offset(offset!(UTC))),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::parse("2019-W01-3 12:00:00 pm +0000", "%G-W%V-%u %r %z"),
    ///     Ok(date!(2019-W01-3).with_time(time!(12:00)).using_offset(offset!(UTC))),
    /// );
    /// ```
    #[inline(always)]
    pub fn parse(s: &str, format: &str) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s, format)?)
    }

    /// Given the items already parsed, attempt to create an `OffsetDateTime`.
    #[inline(always)]
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        let offset = UtcOffset::try_from_parsed_items(items)?;

        Ok(
            (PrimitiveDateTime::try_from_parsed_items(items)? - offset.as_duration())
                .using_offset(offset),
        )
    }
}

impl PartialEq for OffsetDateTime {
    #[inline(always)]
    fn eq(&self, rhs: &Self) -> bool {
        self.utc_datetime.eq(&rhs.utc_datetime)
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
        self.utc_datetime.cmp(&rhs.utc_datetime)
    }
}

impl Hash for OffsetDateTime {
    #[inline(always)]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // We need to distinguish this from a `PrimitiveDateTime`, which would
        // otherwise conflict.
        hasher.write(b"OffsetDateTime");
        self.utc_datetime.hash(hasher);
    }
}

impl Add<Duration> for OffsetDateTime {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: Duration) -> Self::Output {
        (self.utc_datetime + duration).using_offset(self.offset)
    }
}

impl Add<StdDuration> for OffsetDateTime {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: StdDuration) -> Self::Output {
        (self.utc_datetime + duration).using_offset(self.offset)
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
        (self.utc_datetime - duration).using_offset(self.offset)
    }
}

impl Sub<StdDuration> for OffsetDateTime {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: StdDuration) -> Self::Output {
        (self.utc_datetime - duration).using_offset(self.offset)
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
        self.utc_datetime - rhs.utc_datetime
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(date)]
mod test {
    use super::*;
    use crate::{date, prelude::*, time};

    #[test]
    #[cfg(feature = "std")]
    fn now() {
        assert!(OffsetDateTime::now().year() >= 2019);
        assert_eq!(OffsetDateTime::now().offset(), offset!(UTC));
    }

    #[test]
    fn to_offset() {
        assert_eq!(
            date!(2000-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .to_offset(offset!(-1))
                .year(),
            1999,
        );

        let sydney = date!(1999-12-31)
            .with_time(time!(13:00))
            .using_offset(offset!(+11));
        let new_york = sydney.to_offset(offset!(-5));
        let los_angeles = sydney.to_offset(offset!(-8));
        assert_eq!(sydney.hour(), 0);
        assert_eq!(sydney.day(), 1);
        assert_eq!(new_york.hour(), 8);
        assert_eq!(new_york.day(), 31);
        assert_eq!(los_angeles.hour(), 5);
        assert_eq!(los_angeles.day(), 31);
    }

    #[test]
    fn unix_epoch() {
        assert_eq!(
            OffsetDateTime::unix_epoch(),
            date!(1970-1-1).midnight().using_offset(offset!(UTC)),
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
            date!(2019-01-01).midnight().using_offset(offset!(UTC)),
        );
    }

    #[test]
    fn offset() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .offset(),
            offset!(UTC),
        );
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(+1))
                .offset(),
            offset!(+1),
        );
    }

    #[test]
    fn timestamp() {
        assert_eq!(OffsetDateTime::unix_epoch().timestamp(), 0);
        assert_eq!(
            PrimitiveDateTime::unix_epoch()
                .using_offset(offset!(-1))
                .timestamp(),
            0,
        );
    }

    #[test]
    fn date() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .date(),
            date!(2019-01-01),
        );
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(-1))
                .date(),
            date!(2018-12-31),
        );
    }

    #[test]
    fn time() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .time(),
            time!(0:00),
        );
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(-1))
                .time(),
            time!(23:00),
        );
    }

    #[test]
    fn year() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .year(),
            2019,
        );
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .using_offset(offset!(UTC))
                .to_offset(offset!(+1))
                .year(),
            2020,
        );
        assert_eq!(
            date!(2020-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .year(),
            2020,
        );
    }

    #[test]
    fn month() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .month(),
            1,
        );
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .using_offset(offset!(+1))
                .month(),
            1,
        );
    }

    #[test]
    fn day() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .day(),
            1,
        );
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .using_offset(offset!(+1))
                .day(),
            1,
        );
    }

    #[test]
    fn month_day() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .month_day(),
            (1, 1),
        );
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .using_offset(offset!(+1))
                .month_day(),
            (1, 1),
        );
    }

    #[test]
    fn ordinal() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .ordinal(),
            1,
        );
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .using_offset(offset!(+1))
                .ordinal(),
            1,
        );
    }

    #[test]
    fn week() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .week(),
            1,
        );
        assert_eq!(
            date!(2020-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .week(),
            1,
        );
        assert_eq!(
            date!(2020-12-31)
                .midnight()
                .using_offset(offset!(UTC))
                .week(),
            53,
        );
        assert_eq!(
            date!(2021-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .week(),
            53,
        );
    }

    #[test]
    fn weekday() {
        use Weekday::*;
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .weekday(),
            Tuesday,
        );
        assert_eq!(
            date!(2019-02-01)
                .midnight()
                .using_offset(offset!(UTC))
                .weekday(),
            Friday,
        );
        assert_eq!(
            date!(2019-03-01)
                .midnight()
                .using_offset(offset!(UTC))
                .weekday(),
            Friday,
        );
    }

    #[test]
    fn hour() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .hour(),
            0,
        );
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59))
                .using_offset(UtcOffset::hours(-2))
                .hour(),
            21,
        );
    }

    #[test]
    fn minute() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .minute(),
            0,
        );
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59))
                .using_offset(offset!(+0:30))
                .minute(),
            29,
        );
    }

    #[test]
    fn second() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .second(),
            0,
        );
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59))
                .using_offset(offset!(+0:00:30))
                .second(),
            29,
        );
    }

    #[test]
    fn millisecond() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .millisecond(),
            0,
        );
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59.999))
                .using_offset(offset!(UTC))
                .millisecond(),
            999,
        );
    }

    #[test]
    fn microsecond() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .microsecond(),
            0,
        );
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59.999_999))
                .using_offset(offset!(UTC))
                .microsecond(),
            999_999,
        );
    }

    #[test]
    fn nanosecond() {
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .using_offset(offset!(UTC))
                .nanosecond(),
            0,
        );
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59.999_999_999))
                .using_offset(offset!(UTC))
                .nanosecond(),
            999_999_999,
        );
    }

    #[test]
    fn format() {
        assert_eq!(
            date!(2019-01-02)
                .midnight()
                .using_offset(offset!(UTC))
                .format("%F %r %z"),
            "2019-01-02 12:00:00 am +0000",
        );
    }

    #[test]
    fn parse() {
        assert_eq!(
            OffsetDateTime::parse("2019-01-02 00:00:00 +0000", "%F %T %z"),
            Ok(date!(2019-01-02).midnight().using_offset(offset!(UTC))),
        );
        assert_eq!(
            OffsetDateTime::parse("2019-002 23:59:59 +0000", "%Y-%j %T %z"),
            Ok(date!(2019-002)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC)))
        );
        assert_eq!(
            OffsetDateTime::parse("2019-W01-3 12:00:00 pm +0000", "%G-W%V-%u %r %z"),
            Ok(date!(2019-W01-3)
                .with_time(time!(12:00))
                .using_offset(offset!(UTC))),
        );
    }

    #[test]
    fn partial_eq() {
        assert_eq!(
            date!(2000-01-01).midnight().using_offset(offset!(-1)),
            date!(2000-01-01).midnight().using_offset(offset!(UTC)),
        );
    }

    #[test]
    fn partial_ord() {
        let t1 = date!(2019-01-01).midnight().using_offset(offset!(UTC));
        let t2 = date!(2019-01-01).midnight().using_offset(offset!(-1));
        assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Equal));
    }

    #[test]
    fn ord() {
        let t1 = date!(2019-01-01).midnight().using_offset(offset!(UTC));
        let t2 = date!(2019-01-01).midnight().using_offset(offset!(-1));
        assert_eq!(t1, t2);

        let t1 = date!(2019-01-01).midnight().using_offset(offset!(UTC));
        let t2 = date!(2019-01-01)
            .with_time(time!(0:00:00.000_000_001))
            .using_offset(offset!(UTC));
        assert!(t2 > t1);
    }

    #[test]
    #[cfg(feature = "std")]
    fn hash() {
        use std::{collections::hash_map::DefaultHasher, hash::Hash};

        assert_eq!(
            {
                let mut hasher = DefaultHasher::new();
                date!(2019-01-01)
                    .midnight()
                    .using_offset(offset!(UTC))
                    .hash(&mut hasher);
                hasher.finish()
            },
            {
                let mut hasher = DefaultHasher::new();
                date!(2019-01-01)
                    .midnight()
                    .using_offset(offset!(-1))
                    .hash(&mut hasher);
                hasher.finish()
            }
        );

        // Ensure that a `PrimitiveDateTime` and `OffsetDateTime` don't collide,
        // even if the UTC time is the same.
        assert_ne!(
            {
                let mut hasher = DefaultHasher::new();
                date!(2019-01-01).midnight().hash(&mut hasher);
                hasher.finish()
            },
            {
                let mut hasher = DefaultHasher::new();
                date!(2019-01-01)
                    .midnight()
                    .using_offset(offset!(UTC))
                    .hash(&mut hasher);
                hasher.finish()
            }
        );
    }

    #[test]
    fn add_duration() {
        assert_eq!(
            date!(2019-01-01).midnight().using_offset(offset!(UTC)) + 5.days(),
            date!(2019-01-06).midnight().using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2019-12-31).midnight().using_offset(offset!(UTC)) + 1.days(),
            date!(2020-01-01).midnight().using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC))
                + 2.seconds(),
            date!(2020-01-01)
                .with_time(time!(0:00:01))
                .using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2020-01-01)
                .with_time(time!(0:00:01))
                .using_offset(offset!(UTC))
                + (-2).seconds(),
            date!(2019-12-31)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(1999-12-31)
                .with_time(time!(23:00))
                .using_offset(offset!(UTC))
                + 1.hours(),
            date!(2000-01-01).midnight().using_offset(offset!(UTC)),
        );
    }

    #[test]
    fn add_std_duration() {
        assert_eq!(
            date!(2019-01-01).midnight().using_offset(offset!(UTC)) + 5.std_days(),
            date!(2019-01-06).midnight().using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2019-12-31).midnight().using_offset(offset!(UTC)) + 1.std_days(),
            date!(2020-01-01).midnight().using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC))
                + 2.std_seconds(),
            date!(2020-01-01)
                .with_time(time!(0:00:01))
                .using_offset(offset!(UTC)),
        );
    }

    #[test]
    fn add_assign_duration() {
        let mut ny19 = date!(2019-01-01).midnight().using_offset(offset!(UTC));
        ny19 += 5.days();
        assert_eq!(
            ny19,
            date!(2019-01-06).midnight().using_offset(offset!(UTC))
        );

        let mut nye20 = date!(2019-12-31).midnight().using_offset(offset!(UTC));
        nye20 += 1.days();
        assert_eq!(
            nye20,
            date!(2020-01-01).midnight().using_offset(offset!(UTC))
        );

        let mut nye20t = date!(2019-12-31)
            .with_time(time!(23:59:59))
            .using_offset(offset!(UTC));
        nye20t += 2.seconds();
        assert_eq!(
            nye20t,
            date!(2020-01-01)
                .with_time(time!(0:00:01))
                .using_offset(offset!(UTC))
        );

        let mut ny20t = date!(2020-01-01)
            .with_time(time!(0:00:01))
            .using_offset(offset!(UTC));
        ny20t += (-2).seconds();
        assert_eq!(
            ny20t,
            date!(2019-12-31)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC))
        );
    }

    #[test]
    fn add_assign_std_duration() {
        let mut ny19 = date!(2019-01-01).midnight().using_offset(offset!(UTC));
        ny19 += 5.std_days();
        assert_eq!(
            ny19,
            date!(2019-01-06).midnight().using_offset(offset!(UTC))
        );

        let mut nye20 = date!(2019-12-31).midnight().using_offset(offset!(UTC));
        nye20 += 1.std_days();
        assert_eq!(
            nye20,
            date!(2020-01-01).midnight().using_offset(offset!(UTC))
        );

        let mut nye20t = date!(2019-12-31)
            .with_time(time!(23:59:59))
            .using_offset(offset!(UTC));
        nye20t += 2.std_seconds();
        assert_eq!(
            nye20t,
            date!(2020-01-01)
                .with_time(time!(0:00:01))
                .using_offset(offset!(UTC))
        );
    }

    #[test]
    fn sub_duration() {
        assert_eq!(
            date!(2019-01-06).midnight().using_offset(offset!(UTC)) - 5.days(),
            date!(2019-01-01).midnight().using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2020-01-01).midnight().using_offset(offset!(UTC)) - 1.days(),
            date!(2019-12-31).midnight().using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2020-01-01)
                .with_time(time!(0:00:01))
                .using_offset(offset!(UTC))
                - 2.seconds(),
            date!(2019-12-31)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC))
                - (-2).seconds(),
            date!(2020-01-01)
                .with_time(time!(0:00:01))
                .using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(1999-12-31)
                .with_time(time!(23:00))
                .using_offset(offset!(UTC))
                - (-1).hours(),
            date!(2000-01-01).midnight().using_offset(offset!(UTC)),
        );
    }

    #[test]
    fn sub_std_duration() {
        assert_eq!(
            date!(2019-01-06).midnight().using_offset(offset!(UTC)) - 5.std_days(),
            date!(2019-01-01).midnight().using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2020-01-01).midnight().using_offset(offset!(UTC)) - 1.std_days(),
            date!(2019-12-31).midnight().using_offset(offset!(UTC)),
        );
        assert_eq!(
            date!(2020-01-01)
                .with_time(time!(0:00:01))
                .using_offset(offset!(UTC))
                - 2.std_seconds(),
            date!(2019-12-31)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC)),
        );
    }

    #[test]
    fn sub_assign_duration() {
        let mut ny19 = date!(2019-01-06).midnight().using_offset(offset!(UTC));
        ny19 -= 5.days();
        assert_eq!(
            ny19,
            date!(2019-01-01).midnight().using_offset(offset!(UTC))
        );

        let mut ny20 = date!(2020-01-01).midnight().using_offset(offset!(UTC));
        ny20 -= 1.days();
        assert_eq!(
            ny20,
            date!(2019-12-31).midnight().using_offset(offset!(UTC))
        );

        let mut ny20t = date!(2020-01-01)
            .with_time(time!(0:00:01))
            .using_offset(offset!(UTC));
        ny20t -= 2.seconds();
        assert_eq!(
            ny20t,
            date!(2019-12-31)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC))
        );

        let mut nye20t = date!(2019-12-31)
            .with_time(time!(23:59:59))
            .using_offset(offset!(UTC));
        nye20t -= (-2).seconds();
        assert_eq!(
            nye20t,
            date!(2020-01-01)
                .with_time(time!(0:00:01))
                .using_offset(offset!(UTC))
        );
    }

    #[test]
    fn sub_assign_std_duration() {
        let mut ny19 = date!(2019-01-06).midnight().using_offset(offset!(UTC));
        ny19 -= 5.std_days();
        assert_eq!(
            ny19,
            date!(2019-01-01).midnight().using_offset(offset!(UTC))
        );

        let mut ny20 = date!(2020-01-01).midnight().using_offset(offset!(UTC));
        ny20 -= 1.std_days();
        assert_eq!(
            ny20,
            date!(2019-12-31).midnight().using_offset(offset!(UTC))
        );

        let mut ny20t = date!(2020-01-01)
            .with_time(time!(0:00:01))
            .using_offset(offset!(UTC));
        ny20t -= 2.std_seconds();
        assert_eq!(
            ny20t,
            date!(2019-12-31)
                .with_time(time!(23:59:59))
                .using_offset(offset!(UTC))
        );
    }

    #[test]
    fn sub_self() {
        assert_eq!(
            date!(2019-01-02).midnight().using_offset(offset!(UTC))
                - date!(2019-01-01).midnight().using_offset(offset!(UTC)),
            1.days(),
        );
        assert_eq!(
            date!(2019-01-01).midnight().using_offset(offset!(UTC))
                - date!(2019-01-02).midnight().using_offset(offset!(UTC)),
            (-1).days(),
        );
        assert_eq!(
            date!(2020-01-01).midnight().using_offset(offset!(UTC))
                - date!(2019-12-31).midnight().using_offset(offset!(UTC)),
            1.days(),
        );
        assert_eq!(
            date!(2019-12-31).midnight().using_offset(offset!(UTC))
                - date!(2020-01-01).midnight().using_offset(offset!(UTC)),
            (-1).days(),
        );
    }
}
