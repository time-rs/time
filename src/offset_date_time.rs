use crate::{
    format::parse::{parse, ParsedItems},
    internal_prelude::*,
};
#[cfg(std)]
use core::convert::{From, TryFrom};
use core::{
    cmp::Ordering,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};
#[cfg(std)]
use std::time::SystemTime;

/// A [`PrimitiveDateTime`] with a [`UtcOffset`].
///
/// All comparisons are performed using the UTC time.
// Internally, an `OffsetDateTime` is a thin wrapper around a
// [`PrimitiveDateTime`] coupled with a [`UtcOffset`]. This offset is added to
// the date, time, or datetime as necessary for presentation or returning from a
// function.
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    serde,
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
    /// Create a new `OffsetDateTime` with the current date and time in UTC.
    ///
    /// ```rust
    /// # use time::{OffsetDateTime, offset};
    /// assert!(OffsetDateTime::now().year() >= 2019);
    /// assert_eq!(OffsetDateTime::now().offset(), offset!(UTC));
    /// ```
    #[inline(always)]
    #[cfg(std)]
    #[cfg_attr(docs, doc(cfg(feature = "std")))]
    pub fn now() -> Self {
        SystemTime::now().into()
    }

    /// Create a new `OffsetDateTime` with the current date and time in the
    /// local offset.
    ///
    /// ```rust
    /// # use time::{OffsetDateTime, offset};
    /// assert!(OffsetDateTime::now_local().year() >= 2019);
    /// ```
    #[inline(always)]
    #[cfg(std)]
    #[cfg_attr(docs, doc(cfg(feature = "std")))]
    pub fn now_local() -> Self {
        let t = Self::now();
        t.to_offset(UtcOffset::local_offset_at(t))
    }

    /// Attempt to create a new `OffsetDateTime` with the current date and time
    /// in the local offset. If the offset cannot be determined, an error is
    /// returned.
    ///
    /// ```rust,no_run
    /// # use time::{OffsetDateTime, offset};
    /// assert!(OffsetDateTime::try_now_local().is_ok());
    /// ```
    #[inline]
    #[cfg(std)]
    #[cfg_attr(docs, doc(cfg(feature = "std")))]
    pub fn try_now_local() -> Result<Self, IndeterminateOffsetError> {
        let t = Self::now();
        Ok(t.to_offset(UtcOffset::try_local_offset_at(t)?))
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
        Self {
            utc_datetime: self.utc_datetime,
            offset,
        }
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
        Date {
            year: 1970,
            ordinal: 1,
        }
        .midnight()
        .assume_utc()
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
        OffsetDateTime::unix_epoch() + Duration::seconds(timestamp)
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
        (self - Self::unix_epoch()).whole_seconds()
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
    pub fn format(self, format: impl AsRef<str>) -> String {
        DeferredFormat::new(format.as_ref())
            .with_date(self.date())
            .with_time(self.time())
            .with_offset(self.offset())
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
    pub fn parse(s: impl AsRef<str>, format: impl AsRef<str>) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s.as_ref(), format.as_ref())?)
    }

    /// Given the items already parsed, attempt to create an `OffsetDateTime`.
    #[inline(always)]
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        let offset = UtcOffset::try_from_parsed_items(items)?;
        Ok(PrimitiveDateTime::try_from_parsed_items(items)?.assume_offset(offset))
    }
}

impl Display for OffsetDateTime {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.date(), self.time(), self.offset())
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
        Self {
            utc_datetime: self.utc_datetime + duration,
            offset: self.offset,
        }
    }
}

impl Add<StdDuration> for OffsetDateTime {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: StdDuration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime + duration,
            offset: self.offset,
        }
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
        Self {
            utc_datetime: self.utc_datetime - duration,
            offset: self.offset,
        }
    }
}

impl Sub<StdDuration> for OffsetDateTime {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: StdDuration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime - duration,
            offset: self.offset,
        }
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

#[cfg(std)]
impl Add<Duration> for SystemTime {
    type Output = Self;

    #[inline(always)]
    fn add(self, duration: Duration) -> Self::Output {
        if duration.is_zero() {
            self
        } else if duration.is_positive() {
            self + duration.abs_std()
        } else {
            // duration.is_negative()
            self - duration.abs_std()
        }
    }
}

#[cfg(std)]
impl AddAssign<Duration> for SystemTime {
    #[inline(always)]
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

#[cfg(std)]
impl Sub<Duration> for SystemTime {
    type Output = Self;

    #[inline(always)]
    fn sub(self, duration: Duration) -> Self::Output {
        (OffsetDateTime::from(self) - duration).into()
    }
}

#[cfg(std)]
impl SubAssign<Duration> for SystemTime {
    #[inline(always)]
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

#[cfg(std)]
impl Sub<SystemTime> for OffsetDateTime {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: SystemTime) -> Self::Output {
        self - Self::from(rhs)
    }
}

#[cfg(std)]
impl Sub<OffsetDateTime> for SystemTime {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: OffsetDateTime) -> Self::Output {
        OffsetDateTime::from(self) - rhs
    }
}

#[cfg(std)]
impl PartialEq<SystemTime> for OffsetDateTime {
    #[inline(always)]
    fn eq(&self, rhs: &SystemTime) -> bool {
        self == &Self::from(*rhs)
    }
}

#[cfg(std)]
impl PartialEq<OffsetDateTime> for SystemTime {
    #[inline(always)]
    fn eq(&self, rhs: &OffsetDateTime) -> bool {
        &OffsetDateTime::from(*self) == rhs
    }
}

#[cfg(std)]
impl PartialOrd<SystemTime> for OffsetDateTime {
    #[inline(always)]
    fn partial_cmp(&self, other: &SystemTime) -> Option<Ordering> {
        self.partial_cmp(&Self::from(*other))
    }
}

#[cfg(std)]
impl PartialOrd<OffsetDateTime> for SystemTime {
    #[inline(always)]
    fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
        OffsetDateTime::from(*self).partial_cmp(other)
    }
}

#[cfg(std)]
impl From<SystemTime> for OffsetDateTime {
    // There is definitely some way to have this conversion be infallible, but
    // it won't be an issue for over 500 years.
    #[inline(always)]
    fn from(system_time: SystemTime) -> Self {
        let duration = match system_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Duration::try_from(duration)
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
            Err(err) => -Duration::try_from(err.duration())
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
        };

        Self::unix_epoch() + duration
    }
}

#[cfg(std)]
impl From<OffsetDateTime> for SystemTime {
    #[inline]
    fn from(datetime: OffsetDateTime) -> Self {
        let duration = datetime - OffsetDateTime::unix_epoch();

        if duration.is_zero() {
            Self::UNIX_EPOCH
        } else if duration.is_positive() {
            Self::UNIX_EPOCH + duration.abs_std()
        } else {
            // duration.is_negative()
            Self::UNIX_EPOCH - duration.abs_std()
        }
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(date)]
mod test {
    use super::*;

    #[test]
    #[cfg(std)]
    fn now() {
        assert!(OffsetDateTime::now().year() >= 2019);
        assert_eq!(OffsetDateTime::now().offset(), offset!(UTC));
    }

    #[test]
    #[cfg(std)]
    fn now_local() {
        assert!(OffsetDateTime::now().year() >= 2019);
        assert_eq!(
            OffsetDateTime::now_local().offset(),
            UtcOffset::current_local_offset()
        );
    }

    #[test]
    fn to_offset() {
        assert_eq!(
            date!(2000-01-01)
                .midnight()
                .assume_utc()
                .to_offset(offset!(-1))
                .year(),
            1999,
        );

        let sydney = date!(2000-01-01).midnight().assume_offset(offset!(+11));
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
            date!(1970-1-1).midnight().assume_utc(),
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
            date!(2019-01-01).midnight().assume_utc(),
        );
    }

    #[test]
    fn offset() {
        assert_eq!(
            date!(2019-01-01).midnight().assume_utc().offset(),
            offset!(UTC),
        );
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .assume_offset(offset!(+1))
                .offset(),
            offset!(+1),
        );
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .assume_utc()
                .to_offset(offset!(+1))
                .offset(),
            offset!(+1),
        );
    }

    #[test]
    fn timestamp() {
        assert_eq!(OffsetDateTime::unix_epoch().timestamp(), 0);
        assert_eq!(
            OffsetDateTime::unix_epoch()
                .to_offset(offset!(+1))
                .timestamp(),
            0,
        );
        assert_eq!(
            date!(1970-01-01)
                .midnight()
                .assume_offset(offset!(-1))
                .timestamp(),
            3_600,
        );
    }

    #[test]
    fn date() {
        assert_eq!(
            date!(2019-01-01).midnight().assume_utc().date(),
            date!(2019-01-01),
        );
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .assume_utc()
                .to_offset(offset!(-1))
                .date(),
            date!(2018-12-31),
        );
    }

    #[test]
    fn time() {
        assert_eq!(
            date!(2019-01-01).midnight().assume_utc().time(),
            time!(0:00),
        );
        assert_eq!(
            date!(2019-01-01)
                .midnight()
                .assume_utc()
                .to_offset(offset!(-1))
                .time(),
            time!(23:00),
        );
    }

    #[test]
    fn year() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().year(), 2019);
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .assume_utc()
                .to_offset(offset!(+1))
                .year(),
            2020,
        );
        assert_eq!(date!(2020-01-01).midnight().assume_utc().year(), 2020);
    }

    #[test]
    fn month() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().month(), 1);
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .assume_utc()
                .to_offset(offset!(+1))
                .month(),
            1,
        );
    }

    #[test]
    fn day() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().day(), 1);
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .assume_utc()
                .to_offset(offset!(+1))
                .day(),
            1,
        );
    }

    #[test]
    fn month_day() {
        assert_eq!(
            date!(2019-01-01).midnight().assume_utc().month_day(),
            (1, 1),
        );
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .assume_utc()
                .to_offset(offset!(+1))
                .month_day(),
            (1, 1),
        );
    }

    #[test]
    fn ordinal() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().ordinal(), 1);
        assert_eq!(
            date!(2019-12-31)
                .with_time(time!(23:00))
                .assume_utc()
                .to_offset(offset!(+1))
                .ordinal(),
            1,
        );
    }

    #[test]
    fn week() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().week(), 1);
        assert_eq!(date!(2020-01-01).midnight().assume_utc().week(), 1);
        assert_eq!(date!(2020-12-31).midnight().assume_utc().week(), 53);
        assert_eq!(date!(2021-01-01).midnight().assume_utc().week(), 53);
    }

    #[test]
    fn weekday() {
        use Weekday::*;
        assert_eq!(date!(2019-01-01).midnight().assume_utc().weekday(), Tuesday);
        assert_eq!(date!(2019-02-01).midnight().assume_utc().weekday(), Friday);
        assert_eq!(date!(2019-03-01).midnight().assume_utc().weekday(), Friday);
    }

    #[test]
    fn hour() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().hour(), 0);
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59))
                .assume_utc()
                .to_offset(offset!(-2))
                .hour(),
            21,
        );
    }

    #[test]
    fn minute() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().minute(), 0);
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59))
                .assume_utc()
                .to_offset(offset!(+0:30))
                .minute(),
            29,
        );
    }

    #[test]
    fn second() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().second(), 0);
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59))
                .assume_utc()
                .to_offset(offset!(+0:00:30))
                .second(),
            29,
        );
    }

    #[test]
    fn millisecond() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().millisecond(), 0);
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59.999))
                .assume_utc()
                .millisecond(),
            999,
        );
    }

    #[test]
    fn microsecond() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().microsecond(), 0);
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59.999_999))
                .assume_utc()
                .microsecond(),
            999_999,
        );
    }

    #[test]
    fn nanosecond() {
        assert_eq!(date!(2019-01-01).midnight().assume_utc().nanosecond(), 0);
        assert_eq!(
            date!(2019-01-01)
                .with_time(time!(23:59:59.999_999_999))
                .assume_utc()
                .nanosecond(),
            999_999_999,
        );
    }

    #[test]
    fn format() {
        assert_eq!(
            date!(2019-01-02).midnight().assume_utc().format("%F %r %z"),
            "2019-01-02 12:00:00 am +0000",
        );
    }

    #[test]
    fn parse() {
        assert_eq!(
            OffsetDateTime::parse("2019-01-02 00:00:00 +0000", "%F %T %z"),
            Ok(date!(2019-01-02).midnight().assume_utc()),
        );
        assert_eq!(
            OffsetDateTime::parse("2019-002 23:59:59 +0000", "%Y-%j %T %z"),
            Ok(date!(2019-002).with_time(time!(23:59:59)).assume_utc())
        );
        assert_eq!(
            OffsetDateTime::parse("2019-W01-3 12:00:00 pm +0000", "%G-W%V-%u %r %z"),
            Ok(date!(2019-W01-3).with_time(time!(12:00)).assume_utc())
        );
        assert_eq!(
            OffsetDateTime::parse("2019-01-02 03:04:05 +0600", "%F %T %z"),
            Ok(date!(2019-01-02)
                .with_time(time!(3:04:05))
                .assume_offset(offset!(+6)))
        );
    }

    #[test]
    fn partial_eq() {
        assert_eq!(
            date!(2000-01-01)
                .midnight()
                .assume_utc()
                .to_offset(offset!(-1)),
            date!(2000-01-01).midnight().assume_utc(),
        );
    }

    #[test]
    fn partial_ord() {
        let t1 = date!(2019-01-01).midnight().assume_utc();
        let t2 = date!(2019-01-01)
            .midnight()
            .assume_utc()
            .to_offset(offset!(-1));
        assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Equal));
    }

    #[test]
    fn ord() {
        let t1 = date!(2019-01-01).midnight().assume_utc();
        let t2 = date!(2019-01-01)
            .midnight()
            .assume_utc()
            .to_offset(offset!(-1));
        assert_eq!(t1, t2);

        let t1 = date!(2019-01-01).midnight().assume_utc();
        let t2 = date!(2019-01-01)
            .with_time(time!(0:00:00.000_000_001))
            .assume_utc();
        assert!(t2 > t1);
    }

    #[test]
    #[cfg(std)]
    fn hash() {
        use std::{collections::hash_map::DefaultHasher, hash::Hash};

        assert_eq!(
            {
                let mut hasher = DefaultHasher::new();
                date!(2019-01-01).midnight().assume_utc().hash(&mut hasher);
                hasher.finish()
            },
            {
                let mut hasher = DefaultHasher::new();
                date!(2019-01-01)
                    .midnight()
                    .assume_utc()
                    .to_offset(offset!(-1))
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
                date!(2019-01-01).midnight().assume_utc().hash(&mut hasher);
                hasher.finish()
            }
        );
    }

    #[test]
    fn add_duration() {
        assert_eq!(
            date!(2019-01-01).midnight().assume_utc() + 5.days(),
            date!(2019-01-06).midnight().assume_utc(),
        );
        assert_eq!(
            date!(2019-12-31).midnight().assume_utc() + 1.days(),
            date!(2020-01-01).midnight().assume_utc(),
        );
        assert_eq!(
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc() + 2.seconds(),
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc(),
        );
        assert_eq!(
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc() + (-2).seconds(),
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc(),
        );
        assert_eq!(
            date!(1999-12-31).with_time(time!(23:00)).assume_utc() + 1.hours(),
            date!(2000-01-01).midnight().assume_utc(),
        );
    }

    #[test]
    fn add_std_duration() {
        assert_eq!(
            date!(2019-01-01).midnight().assume_utc() + 5.std_days(),
            date!(2019-01-06).midnight().assume_utc(),
        );
        assert_eq!(
            date!(2019-12-31).midnight().assume_utc() + 1.std_days(),
            date!(2020-01-01).midnight().assume_utc(),
        );
        assert_eq!(
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc() + 2.std_seconds(),
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc(),
        );
    }

    #[test]
    fn add_assign_duration() {
        let mut ny19 = date!(2019-01-01).midnight().assume_utc();
        ny19 += 5.days();
        assert_eq!(ny19, date!(2019-01-06).midnight().assume_utc());

        let mut nye20 = date!(2019-12-31).midnight().assume_utc();
        nye20 += 1.days();
        assert_eq!(nye20, date!(2020-01-01).midnight().assume_utc());

        let mut nye20t = date!(2019-12-31).with_time(time!(23:59:59)).assume_utc();
        nye20t += 2.seconds();
        assert_eq!(
            nye20t,
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc()
        );

        let mut ny20t = date!(2020-01-01).with_time(time!(0:00:01)).assume_utc();
        ny20t += (-2).seconds();
        assert_eq!(
            ny20t,
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc()
        );
    }

    #[test]
    fn add_assign_std_duration() {
        let mut ny19 = date!(2019-01-01).midnight().assume_utc();
        ny19 += 5.std_days();
        assert_eq!(ny19, date!(2019-01-06).midnight().assume_utc());

        let mut nye20 = date!(2019-12-31).midnight().assume_utc();
        nye20 += 1.std_days();
        assert_eq!(nye20, date!(2020-01-01).midnight().assume_utc());

        let mut nye20t = date!(2019-12-31).with_time(time!(23:59:59)).assume_utc();
        nye20t += 2.std_seconds();
        assert_eq!(
            nye20t,
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc()
        );
    }

    #[test]
    fn sub_duration() {
        assert_eq!(
            date!(2019-01-06).midnight().assume_utc() - 5.days(),
            date!(2019-01-01).midnight().assume_utc(),
        );
        assert_eq!(
            date!(2020-01-01).midnight().assume_utc() - 1.days(),
            date!(2019-12-31).midnight().assume_utc(),
        );
        assert_eq!(
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc() - 2.seconds(),
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc(),
        );
        assert_eq!(
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc() - (-2).seconds(),
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc(),
        );
        assert_eq!(
            date!(1999-12-31).with_time(time!(23:00)).assume_utc() - (-1).hours(),
            date!(2000-01-01).midnight().assume_utc(),
        );
    }

    #[test]
    fn sub_std_duration() {
        assert_eq!(
            date!(2019-01-06).midnight().assume_utc() - 5.std_days(),
            date!(2019-01-01).midnight().assume_utc(),
        );
        assert_eq!(
            date!(2020-01-01).midnight().assume_utc() - 1.std_days(),
            date!(2019-12-31).midnight().assume_utc(),
        );
        assert_eq!(
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc() - 2.std_seconds(),
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc(),
        );
    }

    #[test]
    fn sub_assign_duration() {
        let mut ny19 = date!(2019-01-06).midnight().assume_utc();
        ny19 -= 5.days();
        assert_eq!(ny19, date!(2019-01-01).midnight().assume_utc());

        let mut ny20 = date!(2020-01-01).midnight().assume_utc();
        ny20 -= 1.days();
        assert_eq!(ny20, date!(2019-12-31).midnight().assume_utc());

        let mut ny20t = date!(2020-01-01).with_time(time!(0:00:01)).assume_utc();
        ny20t -= 2.seconds();
        assert_eq!(
            ny20t,
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc()
        );

        let mut nye20t = date!(2019-12-31).with_time(time!(23:59:59)).assume_utc();
        nye20t -= (-2).seconds();
        assert_eq!(
            nye20t,
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc()
        );
    }

    #[test]
    fn sub_assign_std_duration() {
        let mut ny19 = date!(2019-01-06).midnight().assume_utc();
        ny19 -= 5.std_days();
        assert_eq!(ny19, date!(2019-01-01).midnight().assume_utc());

        let mut ny20 = date!(2020-01-01).midnight().assume_utc();
        ny20 -= 1.std_days();
        assert_eq!(ny20, date!(2019-12-31).midnight().assume_utc());

        let mut ny20t = date!(2020-01-01).with_time(time!(0:00:01)).assume_utc();
        ny20t -= 2.std_seconds();
        assert_eq!(
            ny20t,
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc()
        );
    }

    #[test]
    #[cfg(std)]
    fn std_add_duration() {
        assert_eq!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc()) + 5.days(),
            SystemTime::from(date!(2019-01-06).midnight().assume_utc()),
        );
        assert_eq!(
            SystemTime::from(date!(2019-12-31).midnight().assume_utc()) + 1.days(),
            SystemTime::from(date!(2020-01-01).midnight().assume_utc()),
        );
        assert_eq!(
            SystemTime::from(date!(2019-12-31).with_time(time!(23:59:59)).assume_utc())
                + 2.seconds(),
            SystemTime::from(date!(2020-01-01).with_time(time!(0:00:01)).assume_utc()),
        );
        assert_eq!(
            SystemTime::from(date!(2020-01-01).with_time(time!(0:00:01)).assume_utc())
                + (-2).seconds(),
            SystemTime::from(date!(2019-12-31).with_time(time!(23:59:59)).assume_utc()),
        );
    }

    #[test]
    #[cfg(std)]
    fn std_add_assign_duration() {
        let mut ny19 = SystemTime::from(date!(2019-01-01).midnight().assume_utc());
        ny19 += 5.days();
        assert_eq!(ny19, date!(2019-01-06).midnight().assume_utc());

        let mut nye20 = SystemTime::from(date!(2019-12-31).midnight().assume_utc());
        nye20 += 1.days();
        assert_eq!(nye20, date!(2020-01-01).midnight().assume_utc());

        let mut nye20t =
            SystemTime::from(date!(2019-12-31).with_time(time!(23:59:59)).assume_utc());
        nye20t += 2.seconds();
        assert_eq!(
            nye20t,
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc()
        );

        let mut ny20t = SystemTime::from(date!(2020-01-01).with_time(time!(0:00:01)).assume_utc());
        ny20t += (-2).seconds();
        assert_eq!(
            ny20t,
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc()
        );
    }

    #[test]
    #[cfg(std)]
    fn std_sub_duration() {
        assert_eq!(
            SystemTime::from(date!(2019-01-06).midnight().assume_utc()) - 5.days(),
            SystemTime::from(date!(2019-01-01).midnight().assume_utc()),
        );
        assert_eq!(
            SystemTime::from(date!(2020-01-01).midnight().assume_utc()) - 1.days(),
            SystemTime::from(date!(2019-12-31).midnight().assume_utc()),
        );
        assert_eq!(
            SystemTime::from(date!(2020-01-01).with_time(time!(0:00:01)).assume_utc())
                - 2.seconds(),
            SystemTime::from(date!(2019-12-31).with_time(time!(23:59:59)).assume_utc()),
        );
        assert_eq!(
            SystemTime::from(date!(2019-12-31).with_time(time!(23:59:59)).assume_utc())
                - (-2).seconds(),
            SystemTime::from(date!(2020-01-01).with_time(time!(0:00:01)).assume_utc()),
        );
    }

    #[test]
    #[cfg(std)]
    fn std_sub_assign_duration() {
        let mut ny19 = SystemTime::from(date!(2019-01-06).midnight().assume_utc());
        ny19 -= 5.days();
        assert_eq!(ny19, date!(2019-01-01).midnight().assume_utc());

        let mut ny20 = SystemTime::from(date!(2020-01-01).midnight().assume_utc());
        ny20 -= 1.days();
        assert_eq!(ny20, date!(2019-12-31).midnight().assume_utc());

        let mut ny20t = SystemTime::from(date!(2020-01-01).with_time(time!(0:00:01)).assume_utc());
        ny20t -= 2.seconds();
        assert_eq!(
            ny20t,
            date!(2019-12-31).with_time(time!(23:59:59)).assume_utc()
        );

        let mut nye20t =
            SystemTime::from(date!(2019-12-31).with_time(time!(23:59:59)).assume_utc());
        nye20t -= (-2).seconds();
        assert_eq!(
            nye20t,
            date!(2020-01-01).with_time(time!(0:00:01)).assume_utc()
        );
    }

    #[test]
    fn sub_self() {
        assert_eq!(
            date!(2019-01-02).midnight().assume_utc() - date!(2019-01-01).midnight().assume_utc(),
            1.days(),
        );
        assert_eq!(
            date!(2019-01-01).midnight().assume_utc() - date!(2019-01-02).midnight().assume_utc(),
            (-1).days(),
        );
        assert_eq!(
            date!(2020-01-01).midnight().assume_utc() - date!(2019-12-31).midnight().assume_utc(),
            1.days(),
        );
        assert_eq!(
            date!(2019-12-31).midnight().assume_utc() - date!(2020-01-01).midnight().assume_utc(),
            (-1).days(),
        );
    }

    #[test]
    #[cfg(std)]
    fn std_sub() {
        assert_eq!(
            SystemTime::from(date!(2019-01-02).midnight().assume_utc())
                - date!(2019-01-01).midnight().assume_utc(),
            1.days()
        );
        assert_eq!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc())
                - date!(2019-01-02).midnight().assume_utc(),
            (-1).days()
        );
        assert_eq!(
            SystemTime::from(date!(2020-01-01).midnight().assume_utc())
                - date!(2019-12-31).midnight().assume_utc(),
            1.days()
        );
        assert_eq!(
            SystemTime::from(date!(2019-12-31).midnight().assume_utc())
                - date!(2020-01-01).midnight().assume_utc(),
            (-1).days()
        );
    }

    #[test]
    #[cfg(std)]
    fn sub_std() {
        assert_eq!(
            date!(2019-01-02).midnight().assume_utc()
                - SystemTime::from(date!(2019-01-01).midnight().assume_utc()),
            1.days()
        );
        assert_eq!(
            date!(2019-01-01).midnight().assume_utc()
                - SystemTime::from(date!(2019-01-02).midnight().assume_utc()),
            (-1).days()
        );
        assert_eq!(
            date!(2020-01-01).midnight().assume_utc()
                - SystemTime::from(date!(2019-12-31).midnight().assume_utc()),
            1.days()
        );
        assert_eq!(
            date!(2019-12-31).midnight().assume_utc()
                - SystemTime::from(date!(2020-01-01).midnight().assume_utc()),
            (-1).days()
        );
    }

    #[test]
    #[cfg(std)]
    #[allow(deprecated)]
    fn eq_std() {
        let now_datetime = OffsetDateTime::now();
        let now_systemtime = SystemTime::from(now_datetime);
        assert_eq!(now_datetime, now_systemtime);
    }

    #[test]
    #[cfg(std)]
    #[allow(deprecated)]
    fn std_eq() {
        let now_datetime = OffsetDateTime::now();
        let now_systemtime = SystemTime::from(now_datetime);
        assert_eq!(now_datetime, now_systemtime);
    }

    #[test]
    #[cfg(std)]
    fn ord_std() {
        assert_eq!(
            date!(2019-01-01).midnight().assume_utc(),
            SystemTime::from(date!(2019-01-01).midnight().assume_utc())
        );
        assert!(
            date!(2019-01-01).midnight().assume_utc()
                < SystemTime::from(date!(2020-01-01).midnight().assume_utc())
        );
        assert!(
            date!(2019-01-01).midnight().assume_utc()
                < SystemTime::from(date!(2019-02-01).midnight().assume_utc())
        );
        assert!(
            date!(2019-01-01).midnight().assume_utc()
                < SystemTime::from(date!(2019-01-02).midnight().assume_utc())
        );
        assert!(
            date!(2019-01-01).midnight().assume_utc()
                < SystemTime::from(date!(2019-01-01).with_time(time!(1:00:00)).assume_utc())
        );
        assert!(
            date!(2019-01-01).midnight().assume_utc()
                < SystemTime::from(date!(2019-01-01).with_time(time!(0:01:00)).assume_utc())
        );
        assert!(
            date!(2019-01-01).midnight().assume_utc()
                < SystemTime::from(date!(2019-01-01).with_time(time!(0:00:01)).assume_utc())
        );
        assert!(
            date!(2019-01-01).midnight().assume_utc()
                < SystemTime::from(date!(2019-01-01).with_time(time!(0:00:00.001)).assume_utc())
        );
        assert!(
            date!(2020-01-01).midnight().assume_utc()
                > SystemTime::from(date!(2019-01-01).midnight().assume_utc())
        );
        assert!(
            date!(2019-02-01).midnight().assume_utc()
                > SystemTime::from(date!(2019-01-01).midnight().assume_utc())
        );
        assert!(
            date!(2019-01-02).midnight().assume_utc()
                > SystemTime::from(date!(2019-01-01).midnight().assume_utc())
        );
        assert!(
            date!(2019-01-01).with_time(time!(1:00:00)).assume_utc()
                > SystemTime::from(date!(2019-01-01).midnight().assume_utc())
        );
        assert!(
            date!(2019-01-01).with_time(time!(0:01:00)).assume_utc()
                > SystemTime::from(date!(2019-01-01).midnight().assume_utc())
        );
        assert!(
            date!(2019-01-01).with_time(time!(0:00:01)).assume_utc()
                > SystemTime::from(date!(2019-01-01).midnight().assume_utc())
        );
        assert!(
            date!(2019-01-01)
                .with_time(time!(0:00:00.000_000_001))
                .assume_utc()
                > SystemTime::from(date!(2019-01-01).midnight().assume_utc())
        );
    }

    #[test]
    #[cfg(std)]
    fn std_ord() {
        assert_eq!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc()),
            date!(2019-01-01).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc())
                < date!(2020-01-01).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc())
                < date!(2019-02-01).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc())
                < date!(2019-01-02).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc())
                < date!(2019-01-01).with_time(time!(1:00:00)).assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc())
                < date!(2019-01-01).with_time(time!(0:01:00)).assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc())
                < date!(2019-01-01).with_time(time!(0:00:01)).assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).midnight().assume_utc())
                < date!(2019-01-01)
                    .with_time(time!(0:00:00.000_000_001))
                    .assume_utc()
        );
        assert!(
            SystemTime::from(date!(2020-01-01).midnight().assume_utc())
                > date!(2019-01-01).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-02-01).midnight().assume_utc())
                > date!(2019-01-01).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-02).midnight().assume_utc())
                > date!(2019-01-01).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).with_time(time!(1:00:00)).assume_utc())
                > date!(2019-01-01).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).with_time(time!(0:01:00)).assume_utc())
                > date!(2019-01-01).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).with_time(time!(0:00:01)).assume_utc())
                > date!(2019-01-01).midnight().assume_utc()
        );
        assert!(
            SystemTime::from(date!(2019-01-01).with_time(time!(0:00:00.001)).assume_utc())
                > date!(2019-01-01).midnight().assume_utc()
        );
    }

    #[test]
    #[cfg(std)]
    fn from_std() {
        assert_eq!(
            OffsetDateTime::from(SystemTime::UNIX_EPOCH),
            OffsetDateTime::unix_epoch()
        );
    }

    #[test]
    #[cfg(std)]
    fn to_std() {
        assert_eq!(
            SystemTime::from(OffsetDateTime::unix_epoch()),
            SystemTime::UNIX_EPOCH
        );
    }
}
