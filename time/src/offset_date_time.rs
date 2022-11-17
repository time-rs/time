//! The [`OffsetDateTime`] struct and its associated `impl`s.

#[cfg(feature = "std")]
use core::cmp::Ordering;
#[cfg(feature = "std")]
use core::convert::From;
use core::fmt;
use core::hash::Hash;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration as StdDuration;
#[cfg(feature = "formatting")]
use std::io;
#[cfg(feature = "std")]
use std::time::SystemTime;

use crate::date_time::offset_kind;
#[cfg(feature = "formatting")]
use crate::formatting::Formattable;
#[cfg(feature = "parsing")]
use crate::parsing::Parsable;
use crate::{error, util, Date, DateTime, Duration, Month, PrimitiveDateTime, Time, UtcOffset, Weekday};

/// The actual type doing all the work.
type Inner = DateTime<offset_kind::Fixed>;

/// A [`PrimitiveDateTime`] with a [`UtcOffset`].
///
/// All comparisons are performed using the UTC time.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OffsetDateTime(pub(crate) Inner);

impl OffsetDateTime {
    /// Midnight, 1 January, 1970 (UTC).
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::datetime;
    /// assert_eq!(OffsetDateTime::UNIX_EPOCH, datetime!(1970-01-01 0:00 UTC),);
    /// ```
    pub const UNIX_EPOCH: Self = Self(Inner::UNIX_EPOCH);

    // region: now
    /// Create a new `OffsetDateTime` with the current date and time in UTC.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::offset;
    /// assert!(OffsetDateTime::now_utc().year() >= 2019);
    /// assert_eq!(OffsetDateTime::now_utc().offset(), offset!(UTC));
    /// ```
    #[cfg(feature = "std")]
    pub fn now_utc() -> Self {
        Self(Inner::now_utc())
    }

    /// Attempt to create a new `OffsetDateTime` with the current date and time in the local offset.
    /// If the offset cannot be determined, an error is returned.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # if false {
    /// assert!(OffsetDateTime::now_local().is_ok());
    /// # }
    /// ```
    #[cfg(feature = "local-offset")]
    pub fn now_local() -> Result<Self, error::IndeterminateOffset> {
        Inner::now_local().map(Self)
    }
    // endregion now

    /// Convert the `OffsetDateTime` from the current [`UtcOffset`] to the provided [`UtcOffset`].
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(
    ///     datetime!(2000-01-01 0:00 UTC)
    ///         .to_offset(offset!(-1))
    ///         .year(),
    ///     1999,
    /// );
    ///
    /// // Let's see what time Sydney's new year's celebration is in New York and Los Angeles.
    ///
    /// // Construct midnight on new year's in Sydney.
    /// let sydney = datetime!(2000-01-01 0:00 +11);
    /// let new_york = sydney.to_offset(offset!(-5));
    /// let los_angeles = sydney.to_offset(offset!(-8));
    /// assert_eq!(sydney.hour(), 0);
    /// assert_eq!(new_york.hour(), 8);
    /// assert_eq!(los_angeles.hour(), 5);
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if the local date-time in the new offset is outside the supported range.
    pub const fn to_offset(self, offset: UtcOffset) -> Self {
        Self(self.0.to_offset(offset))
    }

    // region: constructors
    /// Create an `OffsetDateTime` from the provided Unix timestamp. Calling `.offset()` on the
    /// resulting value is guaranteed to return UTC.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(0),
    ///     Ok(OffsetDateTime::UNIX_EPOCH),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(1_546_300_800),
    ///     Ok(datetime!(2019-01-01 0:00 UTC)),
    /// );
    /// ```
    ///
    /// If you have a timestamp-nanosecond pair, you can use something along the lines of the
    /// following:
    ///
    /// ```rust
    /// # use time::{Duration, OffsetDateTime, ext::NumericalDuration};
    /// let (timestamp, nanos) = (1, 500_000_000);
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(timestamp)? + Duration::nanoseconds(nanos),
    ///     OffsetDateTime::UNIX_EPOCH + 1.5.seconds()
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    pub const fn from_unix_timestamp(timestamp: i64) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(Inner::from_unix_timestamp(timestamp))))
    }

    /// Construct an `OffsetDateTime` from the provided Unix timestamp (in nanoseconds). Calling
    /// `.offset()` on the resulting value is guaranteed to return UTC.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp_nanos(0),
    ///     Ok(OffsetDateTime::UNIX_EPOCH),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp_nanos(1_546_300_800_000_000_000),
    ///     Ok(datetime!(2019-01-01 0:00 UTC)),
    /// );
    /// ```
    pub const fn from_unix_timestamp_nanos(timestamp: i128) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(Inner::from_unix_timestamp_nanos(
            timestamp
        ))))
    }
    // endregion constructors

    // region: getters
    /// Get the [`UtcOffset`].
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).offset(), offset!(UTC));
    /// assert_eq!(datetime!(2019-01-01 0:00 +1).offset(), offset!(+1));
    /// ```
    pub const fn offset(self) -> UtcOffset {
        self.0.offset()
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(1970-01-01 0:00 UTC).unix_timestamp(), 0);
    /// assert_eq!(datetime!(1970-01-01 0:00 -1).unix_timestamp(), 3_600);
    /// ```
    pub const fn unix_timestamp(self) -> i64 {
        self.0.unix_timestamp()
    }

    /// Get the Unix timestamp in nanoseconds.
    ///
    /// ```rust
    /// use time_macros::datetime;
    /// assert_eq!(datetime!(1970-01-01 0:00 UTC).unix_timestamp_nanos(), 0);
    /// assert_eq!(
    ///     datetime!(1970-01-01 0:00 -1).unix_timestamp_nanos(),
    ///     3_600_000_000_000,
    /// );
    /// ```
    pub const fn unix_timestamp_nanos(self) -> i128 {
        self.0.unix_timestamp_nanos()
    }

    /// Get the [`Date`] in the stored offset.
    ///
    /// ```rust
    /// # use time_macros::{date, datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).date(), date!(2019-01-01));
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC)
    ///         .to_offset(offset!(-1))
    ///         .date(),
    ///     date!(2018-12-31),
    /// );
    /// ```
    pub const fn date(self) -> Date {
        self.0.date()
    }

    /// Get the [`Time`] in the stored offset.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset, time};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).time(), time!(0:00));
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC)
    ///         .to_offset(offset!(-1))
    ///         .time(),
    ///     time!(23:00)
    /// );
    /// ```
    pub const fn time(self) -> Time {
        self.0.time()
    }

    // region: date getters
    /// Get the year of the date in the stored offset.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).year(), 2019);
    /// assert_eq!(
    ///     datetime!(2019-12-31 23:00 UTC)
    ///         .to_offset(offset!(+1))
    ///         .year(),
    ///     2020,
    /// );
    /// assert_eq!(datetime!(2020-01-01 0:00 UTC).year(), 2020);
    /// ```
    pub const fn year(self) -> i32 {
        self.0.year()
    }

    /// Get the month of the date in the stored offset.
    ///
    /// ```rust
    /// # use time::Month;
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).month(), Month::January);
    /// assert_eq!(
    ///     datetime!(2019-12-31 23:00 UTC)
    ///         .to_offset(offset!(+1))
    ///         .month(),
    ///     Month::January,
    /// );
    /// ```
    pub const fn month(self) -> Month {
        self.0.month()
    }

    /// Get the day of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).day(), 1);
    /// assert_eq!(
    ///     datetime!(2019-12-31 23:00 UTC)
    ///         .to_offset(offset!(+1))
    ///         .day(),
    ///     1,
    /// );
    /// ```
    pub const fn day(self) -> u8 {
        self.0.day()
    }

    /// Get the day of the year of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=366`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).ordinal(), 1);
    /// assert_eq!(
    ///     datetime!(2019-12-31 23:00 UTC)
    ///         .to_offset(offset!(+1))
    ///         .ordinal(),
    ///     1,
    /// );
    /// ```
    pub const fn ordinal(self) -> u16 {
        self.0.ordinal()
    }

    /// Get the ISO week number of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).iso_week(), 1);
    /// assert_eq!(datetime!(2020-01-01 0:00 UTC).iso_week(), 1);
    /// assert_eq!(datetime!(2020-12-31 0:00 UTC).iso_week(), 53);
    /// assert_eq!(datetime!(2021-01-01 0:00 UTC).iso_week(), 53);
    /// ```
    pub const fn iso_week(self) -> u8 {
        self.0.iso_week()
    }

    /// Get the week number where week 1 begins on the first Sunday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).sunday_based_week(), 0);
    /// assert_eq!(datetime!(2020-01-01 0:00 UTC).sunday_based_week(), 0);
    /// assert_eq!(datetime!(2020-12-31 0:00 UTC).sunday_based_week(), 52);
    /// assert_eq!(datetime!(2021-01-01 0:00 UTC).sunday_based_week(), 0);
    /// ```
    pub const fn sunday_based_week(self) -> u8 {
        self.0.sunday_based_week()
    }

    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).monday_based_week(), 0);
    /// assert_eq!(datetime!(2020-01-01 0:00 UTC).monday_based_week(), 0);
    /// assert_eq!(datetime!(2020-12-31 0:00 UTC).monday_based_week(), 52);
    /// assert_eq!(datetime!(2021-01-01 0:00 UTC).monday_based_week(), 0);
    /// ```
    pub const fn monday_based_week(self) -> u8 {
        self.0.monday_based_week()
    }

    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time::Month;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC).to_calendar_date(),
    ///     (2019, Month::January, 1)
    /// );
    /// ```
    pub const fn to_calendar_date(self) -> (i32, Month, u8) {
        self.0.to_calendar_date()
    }

    /// Get the year and ordinal day number.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC).to_ordinal_date(),
    ///     (2019, 1)
    /// );
    /// ```
    pub const fn to_ordinal_date(self) -> (i32, u16) {
        self.0.to_ordinal_date()
    }

    /// Get the ISO 8601 year, week number, and weekday.
    ///
    /// ```rust
    /// # use time::Weekday::*;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC).to_iso_week_date(),
    ///     (2019, 1, Tuesday)
    /// );
    /// assert_eq!(
    ///     datetime!(2019-10-04 0:00 UTC).to_iso_week_date(),
    ///     (2019, 40, Friday)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00 UTC).to_iso_week_date(),
    ///     (2020, 1, Wednesday)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-12-31 0:00 UTC).to_iso_week_date(),
    ///     (2020, 53, Thursday)
    /// );
    /// assert_eq!(
    ///     datetime!(2021-01-01 0:00 UTC).to_iso_week_date(),
    ///     (2020, 53, Friday)
    /// );
    /// ```
    pub const fn to_iso_week_date(self) -> (i32, u8, Weekday) {
        self.0.to_iso_week_date()
    }

    /// Get the weekday of the date in the stored offset.
    ///
    /// ```rust
    /// # use time::Weekday::*;
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).weekday(), Tuesday);
    /// assert_eq!(datetime!(2019-02-01 0:00 UTC).weekday(), Friday);
    /// assert_eq!(datetime!(2019-03-01 0:00 UTC).weekday(), Friday);
    /// ```
    pub const fn weekday(self) -> Weekday {
        self.0.weekday()
    }

    /// Get the Julian day for the date. The time is not taken into account for this calculation.
    ///
    /// The algorithm to perform this conversion is derived from one provided by Peter Baum; it is
    /// freely available [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(-4713-11-24 0:00 UTC).to_julian_day(), 0);
    /// assert_eq!(datetime!(2000-01-01 0:00 UTC).to_julian_day(), 2_451_545);
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).to_julian_day(), 2_458_485);
    /// assert_eq!(datetime!(2019-12-31 0:00 UTC).to_julian_day(), 2_458_849);
    /// ```
    pub const fn to_julian_day(self) -> i32 {
        self.0.to_julian_day()
    }
    // endregion date getters

    // region: time getters
    /// Get the clock hour, minute, and second.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(2020-01-01 0:00:00 UTC).to_hms(), (0, 0, 0));
    /// assert_eq!(datetime!(2020-01-01 23:59:59 UTC).to_hms(), (23, 59, 59));
    /// ```
    pub const fn to_hms(self) -> (u8, u8, u8) {
        self.0.as_hms()
    }

    /// Get the clock hour, minute, second, and millisecond.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00:00 UTC).to_hms_milli(),
    ///     (0, 0, 0, 0)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 23:59:59.999 UTC).to_hms_milli(),
    ///     (23, 59, 59, 999)
    /// );
    /// ```
    pub const fn to_hms_milli(self) -> (u8, u8, u8, u16) {
        self.0.as_hms_milli()
    }

    /// Get the clock hour, minute, second, and microsecond.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00:00 UTC).to_hms_micro(),
    ///     (0, 0, 0, 0)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 23:59:59.999_999 UTC).to_hms_micro(),
    ///     (23, 59, 59, 999_999)
    /// );
    /// ```
    pub const fn to_hms_micro(self) -> (u8, u8, u8, u32) {
        self.0.as_hms_micro()
    }

    /// Get the clock hour, minute, second, and nanosecond.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00:00 UTC).to_hms_nano(),
    ///     (0, 0, 0, 0)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 23:59:59.999_999_999 UTC).to_hms_nano(),
    ///     (23, 59, 59, 999_999_999)
    /// );
    /// ```
    pub const fn to_hms_nano(self) -> (u8, u8, u8, u32) {
        self.0.as_hms_nano()
    }

    /// Get the clock hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).hour(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59 UTC)
    ///         .to_offset(offset!(-2))
    ///         .hour(),
    ///     21,
    /// );
    /// ```
    pub const fn hour(self) -> u8 {
        self.0.hour()
    }

    /// Get the minute within the hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).minute(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59 UTC)
    ///         .to_offset(offset!(+0:30))
    ///         .minute(),
    ///     29,
    /// );
    /// ```
    pub const fn minute(self) -> u8 {
        self.0.minute()
    }

    /// Get the second within the minute in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).second(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59 UTC)
    ///         .to_offset(offset!(+0:00:30))
    ///         .second(),
    ///     29,
    /// );
    /// ```
    pub const fn second(self) -> u8 {
        self.0.second()
    }

    // Because a `UtcOffset` is limited in resolution to one second, any subsecond value will not
    // change when adjusting for the offset.

    /// Get the milliseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).millisecond(), 0);
    /// assert_eq!(datetime!(2019-01-01 23:59:59.999 UTC).millisecond(), 999);
    /// ```
    pub const fn millisecond(self) -> u16 {
        self.0.millisecond()
    }

    /// Get the microseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).microsecond(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59.999_999 UTC).microsecond(),
    ///     999_999,
    /// );
    /// ```
    pub const fn microsecond(self) -> u32 {
        self.0.microsecond()
    }

    /// Get the nanoseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).nanosecond(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59.999_999_999 UTC).nanosecond(),
    ///     999_999_999,
    /// );
    /// ```
    pub const fn nanosecond(self) -> u32 {
        self.0.nanosecond()
    }
    // endregion time getters
    // endregion getters

    // region: checked arithmetic
    /// Computes `self + duration`, returning `None` if an overflow occurred.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time_macros::{datetime, offset};
    /// let datetime = Date::MIN.midnight().assume_offset(offset!(+10));
    /// assert_eq!(datetime.checked_add((-2).days()), None);
    ///
    /// let datetime = Date::MAX.midnight().assume_offset(offset!(+10));
    /// assert_eq!(datetime.checked_add(2.days()), None);
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30 +10).checked_add(27.hours()),
    ///     Some(datetime!(2019 - 11 - 26 18:30 +10))
    /// );
    /// ```
    pub const fn checked_add(self, duration: Duration) -> Option<Self> {
        Some(Self(const_try_opt!(self.0.checked_add(duration))))
    }

    /// Computes `self - duration`, returning `None` if an overflow occurred.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time_macros::{datetime, offset};
    /// let datetime = Date::MIN.midnight().assume_offset(offset!(+10));
    /// assert_eq!(datetime.checked_sub(2.days()), None);
    ///
    /// let datetime = Date::MAX.midnight().assume_offset(offset!(+10));
    /// assert_eq!(datetime.checked_sub((-2).days()), None);
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30 +10).checked_sub(27.hours()),
    ///     Some(datetime!(2019 - 11 - 24 12:30 +10))
    /// );
    /// ```
    pub const fn checked_sub(self, duration: Duration) -> Option<Self> {
        Some(Self(const_try_opt!(self.0.checked_sub(duration))))
    }
    // endregion: checked arithmetic

    // region: saturating arithmetic
    /// Computes `self + duration`, saturating value on overflow.
    ///
    /// ```
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::datetime;
    /// assert_eq!(
    #[cfg_attr(
        feature = "large-dates",
        doc = "    datetime!(-999999-01-01 0:00 +10).saturating_add((-2).days()),"
    )]
    #[cfg_attr(feature = "large-dates", doc = "    datetime!(-999999-01-01 0:00 +10)")]
    #[cfg_attr(
        not(feature = "large-dates"),
        doc = "    datetime!(-9999-01-01 0:00 +10).saturating_add((-2).days()),"
    )]
    #[cfg_attr(
        not(feature = "large-dates"),
        doc = "    datetime!(-9999-01-01 0:00 +10)"
    )]
    /// );
    ///
    /// assert_eq!(
    #[cfg_attr(
        feature = "large-dates",
        doc = "    datetime!(+999999-12-31 23:59:59.999_999_999 +10).saturating_add(2.days()),"
    )]
    #[cfg_attr(
        feature = "large-dates",
        doc = "    datetime!(+999999-12-31 23:59:59.999_999_999 +10)"
    )]
    #[cfg_attr(
        not(feature = "large-dates"),
        doc = "    datetime!(+9999-12-31 23:59:59.999_999_999 +10).saturating_add(2.days()),"
    )]
    #[cfg_attr(
        not(feature = "large-dates"),
        doc = "    datetime!(+9999-12-31 23:59:59.999_999_999 +10)"
    )]
    /// );
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30 +10).saturating_add(27.hours()),
    ///     datetime!(2019 - 11 - 26 18:30 +10)
    /// );
    /// ```
    pub const fn saturating_add(self, duration: Duration) -> Self {
        Self(self.0.saturating_add(duration))
    }

    /// Computes `self - duration`, saturating value on overflow.
    ///
    /// ```
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::datetime;
    /// assert_eq!(
    #[cfg_attr(
        feature = "large-dates",
        doc = "    datetime!(-999999-01-01 0:00 +10).saturating_sub(2.days()),"
    )]
    #[cfg_attr(feature = "large-dates", doc = "    datetime!(-999999-01-01 0:00 +10)")]
    #[cfg_attr(
        not(feature = "large-dates"),
        doc = "    datetime!(-9999-01-01 0:00 +10).saturating_sub(2.days()),"
    )]
    #[cfg_attr(
        not(feature = "large-dates"),
        doc = "    datetime!(-9999-01-01 0:00 +10)"
    )]
    /// );
    ///
    /// assert_eq!(
    #[cfg_attr(
        feature = "large-dates",
        doc = "    datetime!(+999999-12-31 23:59:59.999_999_999 +10).saturating_sub((-2).days()),"
    )]
    #[cfg_attr(
        feature = "large-dates",
        doc = "    datetime!(+999999-12-31 23:59:59.999_999_999 +10)"
    )]
    #[cfg_attr(
        not(feature = "large-dates"),
        doc = "    datetime!(+9999-12-31 23:59:59.999_999_999 +10).saturating_sub((-2).days()),"
    )]
    #[cfg_attr(
        not(feature = "large-dates"),
        doc = "    datetime!(+9999-12-31 23:59:59.999_999_999 +10)"
    )]
    /// );
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30 +10).saturating_sub(27.hours()),
    ///     datetime!(2019 - 11 - 24 12:30 +10)
    /// );
    /// ```
    pub const fn saturating_sub(self, duration: Duration) -> Self {
        Self(self.0.saturating_sub(duration))
    }
    // endregion: saturating arithmetic
}

// region: replacement
/// Methods that replace part of the `OffsetDateTime`.
impl OffsetDateTime {
    /// Replace the time, which is assumed to be in the stored offset. The date and offset
    /// components are unchanged.
    ///
    /// ```rust
    /// # use time_macros::{datetime, time};
    /// assert_eq!(
    ///     datetime!(2020-01-01 5:00 UTC).replace_time(time!(12:00)),
    ///     datetime!(2020-01-01 12:00 UTC)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 12:00 -5).replace_time(time!(7:00)),
    ///     datetime!(2020-01-01 7:00 -5)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00 +1).replace_time(time!(12:00)),
    ///     datetime!(2020-01-01 12:00 +1)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    pub const fn replace_time(self, time: Time) -> Self {
        Self(self.0.replace_time(time))
    }

    /// Replace the date, which is assumed to be in the stored offset. The time and offset
    /// components are unchanged.
    ///
    /// ```rust
    /// # use time_macros::{datetime, date};
    /// assert_eq!(
    ///     datetime!(2020-01-01 12:00 UTC).replace_date(date!(2020-01-30)),
    ///     datetime!(2020-01-30 12:00 UTC)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00 +1).replace_date(date!(2020-01-30)),
    ///     datetime!(2020-01-30 0:00 +1)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    pub const fn replace_date(self, date: Date) -> Self {
        Self(self.0.replace_date(date))
    }

    /// Replace the date and time, which are assumed to be in the stored offset. The offset
    /// component remains unchanged.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2020-01-01 12:00 UTC).replace_date_time(datetime!(2020-01-30 16:00)),
    ///     datetime!(2020-01-30 16:00 UTC)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 12:00 +1).replace_date_time(datetime!(2020-01-30 0:00)),
    ///     datetime!(2020-01-30 0:00 +1)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    pub const fn replace_date_time(self, date_time: PrimitiveDateTime) -> Self {
        Self(self.0.replace_date_time(date_time.0))
    }

    /// Replace the offset. The date and time components remain unchanged.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00 UTC).replace_offset(offset!(-5)),
    ///     datetime!(2020-01-01 0:00 -5)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    pub const fn replace_offset(self, offset: UtcOffset) -> Self {
        Self(self.0.replace_offset(offset))
    }

    /// Replace the year. The month and day will be unchanged.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022 - 02 - 18 12:00 +01).replace_year(2019),
    ///     Ok(datetime!(2019 - 02 - 18 12:00 +01))
    /// );
    /// assert!(datetime!(2022 - 02 - 18 12:00 +01).replace_year(-1_000_000_000).is_err()); // -1_000_000_000 isn't a valid year
    /// assert!(datetime!(2022 - 02 - 18 12:00 +01).replace_year(1_000_000_000).is_err()); // 1_000_000_000 isn't a valid year
    /// ```
    pub const fn replace_year(self, year: i32) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(self.0.replace_year(year))))
    }

    /// Replace the month of the year.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// # use time::Month;
    /// assert_eq!(
    ///     datetime!(2022 - 02 - 18 12:00 +01).replace_month(Month::January),
    ///     Ok(datetime!(2022 - 01 - 18 12:00 +01))
    /// );
    /// assert!(datetime!(2022 - 01 - 30 12:00 +01).replace_month(Month::February).is_err()); // 30 isn't a valid day in February
    /// ```
    pub const fn replace_month(self, month: Month) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(self.0.replace_month(month))))
    }

    /// Replace the day of the month.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022 - 02 - 18 12:00 +01).replace_day(1),
    ///     Ok(datetime!(2022 - 02 - 01 12:00 +01))
    /// );
    /// assert!(datetime!(2022 - 02 - 18 12:00 +01).replace_day(0).is_err()); // 00 isn't a valid day
    /// assert!(datetime!(2022 - 02 - 18 12:00 +01).replace_day(30).is_err()); // 30 isn't a valid day in February
    /// ```
    pub const fn replace_day(self, day: u8) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(self.0.replace_day(day))))
    }

    /// Replace the clock hour.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_hour(7),
    ///     Ok(datetime!(2022 - 02 - 18 07:02:03.004_005_006 +01))
    /// );
    /// assert!(datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_hour(24).is_err()); // 24 isn't a valid hour
    /// ```
    pub const fn replace_hour(self, hour: u8) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(self.0.replace_hour(hour))))
    }

    /// Replace the minutes within the hour.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_minute(7),
    ///     Ok(datetime!(2022 - 02 - 18 01:07:03.004_005_006 +01))
    /// );
    /// assert!(datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_minute(60).is_err()); // 60 isn't a valid minute
    /// ```
    pub const fn replace_minute(self, minute: u8) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(self.0.replace_minute(minute))))
    }

    /// Replace the seconds within the minute.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_second(7),
    ///     Ok(datetime!(2022 - 02 - 18 01:02:07.004_005_006 +01))
    /// );
    /// assert!(datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_second(60).is_err()); // 60 isn't a valid second
    /// ```
    pub const fn replace_second(self, second: u8) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(self.0.replace_second(second))))
    }

    /// Replace the milliseconds within the second.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_millisecond(7),
    ///     Ok(datetime!(2022 - 02 - 18 01:02:03.007 +01))
    /// );
    /// assert!(datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_millisecond(1_000).is_err()); // 1_000 isn't a valid millisecond
    /// ```
    pub const fn replace_millisecond(
        self,
        millisecond: u16,
    ) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(self.0.replace_millisecond(millisecond))))
    }

    /// Replace the microseconds within the second.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_microsecond(7_008),
    ///     Ok(datetime!(2022 - 02 - 18 01:02:03.007_008 +01))
    /// );
    /// assert!(datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_microsecond(1_000_000).is_err()); // 1_000_000 isn't a valid microsecond
    /// ```
    pub const fn replace_microsecond(
        self,
        microsecond: u32,
    ) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(self.0.replace_microsecond(microsecond))))
    }

    /// Replace the nanoseconds within the second.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_nanosecond(7_008_009),
    ///     Ok(datetime!(2022 - 02 - 18 01:02:03.007_008_009 +01))
    /// );
    /// assert!(datetime!(2022 - 02 - 18 01:02:03.004_005_006 +01).replace_nanosecond(1_000_000_000).is_err()); // 1_000_000_000 isn't a valid nanosecond
    /// ```
    pub const fn replace_nanosecond(self, nanosecond: u32) -> Result<Self, error::ComponentRange> {
        Ok(Self(const_try!(self.0.replace_nanosecond(nanosecond))))
    }
}
// endregion replacement

// region: rounding
/// Methods that map an `OffsetDateTime` to another rounded `OffsetDateTime`.
impl OffsetDateTime {
    /// Round down to the closest whole second.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).floor_seconds(),
    ///     datetime!(2022-04-18 03:17:12 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12 UTC).floor_seconds(),
    ///     datetime!(2022-04-18 03:17:12 UTC)
    /// );
    /// ```
    pub fn floor_seconds(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
    }

    /// Round down to the closest whole minute.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).floor_minutes(),
    ///     datetime!(2022-04-18 03:17:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:00 UTC).floor_minutes(),
    ///     datetime!(2022-04-18 03:17:00 UTC)
    /// );
    /// ```
    pub fn floor_minutes(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
    }

    /// Round down to the closest whole hour.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).floor_hours(),
    ///     datetime!(2022-04-18 03:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:00:00 UTC).floor_hours(),
    ///     datetime!(2022-04-18 03:00:00 UTC)
    /// );
    /// ```
    pub fn floor_hours(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
    }

    /// Round down to the closest whole day.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).floor_days(),
    ///     datetime!(2022-04-18 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 00:00:00 UTC).floor_days(),
    ///     datetime!(2022-04-18 00:00:00 UTC)
    /// );
    /// ```
    pub fn floor_days(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
    }

    /// Round down to the closest whole monday-based week.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-21 03:17:12.123_456_789 UTC).floor_monday_based_weeks(),
    ///     datetime!(2022-04-18 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 00:00:00 UTC).floor_monday_based_weeks(),
    ///     datetime!(2022-04-18 00:00:00 UTC)
    /// );
    /// ```
    pub fn floor_monday_based_weeks(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
            .checked_sub(Duration::days(self.weekday().number_days_from_monday() as i64)).unwrap()
    }

    /// Round down to the closest whole sunday-based week.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-21 03:17:12.123_456_789 UTC).floor_sunday_based_weeks(),
    ///     datetime!(2022-04-17 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-17 00:00:00 UTC).floor_sunday_based_weeks(),
    ///     datetime!(2022-04-17 00:00:00 UTC)
    /// );
    /// ```
    pub fn floor_sunday_based_weeks(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
            .checked_sub(Duration::days(self.weekday().number_days_from_sunday() as i64)).unwrap()
    }

    /// Round down to the closest whole month.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).floor_months(),
    ///     datetime!(2022-04-01 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-01 00:00:00 UTC).floor_months(),
    ///     datetime!(2022-04-01 00:00:00 UTC)
    /// );
    /// ```
    pub fn floor_months(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
            .replace_day(1).unwrap()
    }

    /// Round down to the closest whole year.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).floor_years(),
    ///     datetime!(2022-01-01 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-01-01 00:00:00 UTC).floor_years(),
    ///     datetime!(2022-01-01 00:00:00 UTC)
    /// );
    /// ```
    pub fn floor_years(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
            .replace_day(1).unwrap()
            .replace_month(Month::January).unwrap()
    }

    /// Round up to the closest whole second.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).ceil_seconds(),
    ///     datetime!(2022-04-18 03:17:13 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12 UTC).ceil_seconds(),
    ///     datetime!(2022-04-18 03:17:12 UTC)
    /// );
    /// ```
    pub fn ceil_seconds(self) -> Self {
        let floored = self.floor_seconds();
        if floored == self {
            self
        } else {
            floored.checked_add(Duration::seconds(1)).unwrap()
        }
    }

    /// Round up to the closest whole minute.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).ceil_minutes(),
    ///     datetime!(2022-04-18 03:18:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:00 UTC).ceil_minutes(),
    ///     datetime!(2022-04-18 03:17:00 UTC)
    /// );
    /// ```
    pub fn ceil_minutes(self) -> Self {
        let floored = self.floor_minutes();
        if floored == self {
            self
        } else {
            floored.checked_add(Duration::minutes(1)).unwrap()
        }
    }

    /// Round up to the closest whole hour.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).ceil_hours(),
    ///     datetime!(2022-04-18 04:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:00:00 UTC).ceil_hours(),
    ///     datetime!(2022-04-18 03:00:00 UTC)
    /// );
    /// ```
    pub fn ceil_hours(self) -> Self {
        let floored = self.floor_hours();
        if floored == self {
            self
        } else {
            floored.checked_add(Duration::hours(1)).unwrap()
        }
    }

    /// Round up to the closest whole day.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).ceil_days(),
    ///     datetime!(2022-04-19 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 00:00:00 UTC).ceil_days(),
    ///     datetime!(2022-04-18 00:00:00 UTC)
    /// );
    /// ```
    pub fn ceil_days(self) -> Self {
        let floored = self.floor_days();
        if floored == self {
            self
        } else {
            floored.checked_add(Duration::days(1)).unwrap()
        }
    }

    /// Round up to the closest whole monday-based week.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-20 03:17:12.123_456_789 UTC).ceil_monday_based_weeks(),
    ///     datetime!(2022-04-25 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-25 00:00:00 UTC).ceil_monday_based_weeks(),
    ///     datetime!(2022-04-25 00:00:00 UTC)
    /// );
    /// ```
    pub fn ceil_monday_based_weeks(self) -> Self {
        let floored = self.floor_monday_based_weeks();
        if floored == self {
            self
        } else {
            floored.checked_add(Duration::weeks(1)).unwrap()
        }
    }

    /// Round up to the closest whole sunday-based week.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-20 03:17:12.123_456_789 UTC).ceil_sunday_based_weeks(),
    ///     datetime!(2022-04-24 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-24 00:00:00 UTC).ceil_sunday_based_weeks(),
    ///     datetime!(2022-04-24 00:00:00 UTC)
    /// );
    /// ```
    pub fn ceil_sunday_based_weeks(self) -> Self {
        let floored = self.floor_sunday_based_weeks();
        if floored == self {
            self
        } else {
            floored.checked_add(Duration::weeks(1)).unwrap()
        }
    }

    /// Round up to the closest whole month.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).ceil_months(),
    ///     datetime!(2022-05-01 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-01 00:00:00 UTC).ceil_months(),
    ///     datetime!(2022-04-01 00:00:00 UTC)
    /// );
    /// ```
    pub fn ceil_months(self) -> Self {
        let floored = self.floor_months();
        if floored == self {
            self
        } else {
            floored.checked_add(Duration::days(util::days_in_year_month(floored.year(), floored.month()) as i64)).unwrap()
        }
    }

    /// Round up to the closest whole year.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).ceil_years(),
    ///     datetime!(2023-01-01 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-01-01 00:00:00 UTC).ceil_years(),
    ///     datetime!(2022-01-01 00:00:00 UTC)
    /// );
    /// ```
    pub fn ceil_years(self) -> Self {
        let floored = self.floor_years();
        if floored == self {
            self
        } else {
            floored.checked_add(Duration::days(util::days_in_year(floored.year()) as i64)).unwrap()
        }
    }

    /// Get the next closest whole second.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).next_second(),
    ///     datetime!(2022-04-18 03:17:13 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:14 UTC).next_second(),
    ///     datetime!(2022-04-18 03:17:15 UTC)
    /// );
    /// ```
    pub fn next_second(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .checked_add(Duration::SECOND).unwrap()
    }

    /// Get the next closest whole minute.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).next_minute(),
    ///     datetime!(2022-04-18 03:18:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:00 UTC).next_minute(),
    ///     datetime!(2022-04-18 03:18:00 UTC)
    /// );
    /// ```
    pub fn next_minute(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .checked_add(Duration::MINUTE).unwrap()
    }

    /// Get the next closest whole hour.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).next_hour(),
    ///     datetime!(2022-04-18 04:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:00:00 UTC).next_hour(),
    ///     datetime!(2022-04-18 04:00:00 UTC)
    /// );
    /// ```
    pub fn next_hour(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .checked_add(Duration::HOUR).unwrap()
    }

    /// Get the next closest whole day.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).next_day(),
    ///     datetime!(2022-04-19 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 00:00:00 UTC).next_day(),
    ///     datetime!(2022-04-19 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-12-31 00:00:00 UTC).next_day(),
    ///     datetime!(2023-01-01 00:00:00 UTC)
    /// );
    /// ```
    pub fn next_day(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
            .checked_add(Duration::DAY).unwrap()
    }

    /// Get the next closest whole monday-based week.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).next_monday_based_week(),
    ///     datetime!(2022-04-25 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-25 00:00:00 UTC).next_monday_based_week(),
    ///     datetime!(2022-05-02 00:00:00 UTC)
    /// );
    pub fn next_monday_based_week(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
            .checked_sub(Duration::days(self.weekday().number_days_from_monday() as i64)).unwrap()
            .checked_add(Duration::WEEK).unwrap()
    }

    /// Get the next closest whole sunday-based week.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).next_sunday_based_week(),
    ///     datetime!(2022-04-24 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-24 00:00:00 UTC).next_sunday_based_week(),
    ///     datetime!(2022-05-01 00:00:00 UTC)
    /// );
    /// ```
    pub fn next_sunday_based_week(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
            .checked_sub(Duration::days(self.weekday().number_days_from_sunday() as i64)).unwrap()
            .checked_add(Duration::WEEK).unwrap()
    }

    /// Get the next closest whole month.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).next_month(),
    ///     datetime!(2022-05-01 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-01 00:00:00 UTC).next_month(),
    ///     datetime!(2022-05-01 00:00:00 UTC)
    /// );
    /// ```
    pub fn next_month(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
            .replace_day(1).unwrap()
            .checked_add(Duration::days(util::days_in_year_month(self.year(), self.month()) as i64)).unwrap()
    }

    /// Get the next closest whole year.
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-04-18 03:17:12.123_456_789 UTC).next_year(),
    ///     datetime!(2023-01-01 00:00:00 UTC)
    /// );
    /// ```
    /// 
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!(2022-01-01 00:00:00 UTC).next_year(),
    ///     datetime!(2023-01-01 00:00:00 UTC)
    /// );
    /// ```
    pub fn next_year(self) -> Self {
        self
            .replace_nanosecond(0).unwrap()
            .replace_second(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_hour(0).unwrap()
            .replace_day(1).unwrap()
            .replace_month(Month::January).unwrap()
            .checked_add(Duration::days(util::days_in_year(self.year()) as i64)).unwrap()
    }
}
// endregion rounding

// region: formatting & parsing
#[cfg(feature = "formatting")]
impl OffsetDateTime {
    /// Format the `OffsetDateTime` using the provided [format
    /// description](crate::format_description).
    pub fn format_into(
        self,
        output: &mut impl io::Write,
        format: &(impl Formattable + ?Sized),
    ) -> Result<usize, error::Format> {
        self.0.format_into(output, format)
    }

    /// Format the `OffsetDateTime` using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::format_description;
    /// # use time_macros::datetime;
    /// let format = format_description::parse(
    ///     "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
    ///          sign:mandatory]:[offset_minute]:[offset_second]",
    /// )?;
    /// assert_eq!(
    ///     datetime!(2020-01-02 03:04:05 +06:07:08).format(&format)?,
    ///     "2020-01-02 03:04:05 +06:07:08"
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn format(self, format: &(impl Formattable + ?Sized)) -> Result<String, error::Format> {
        self.0.format(format)
    }
}

#[cfg(feature = "parsing")]
impl OffsetDateTime {
    /// Parse an `OffsetDateTime` from the input using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::{datetime, format_description};
    /// let format = format_description!(
    ///     "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
    ///          sign:mandatory]:[offset_minute]:[offset_second]"
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::parse("2020-01-02 03:04:05 +06:07:08", &format)?,
    ///     datetime!(2020-01-02 03:04:05 +06:07:08)
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn parse(
        input: &str,
        description: &(impl Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        Inner::parse(input, description).map(Self)
    }
}

impl fmt::Display for OffsetDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for OffsetDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
// endregion formatting & parsing

// region: trait impls
impl Add<Duration> for OffsetDateTime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0.add(rhs))
    }
}

impl Add<StdDuration> for OffsetDateTime {
    type Output = Self;

    fn add(self, rhs: StdDuration) -> Self::Output {
        Self(self.0.add(rhs))
    }
}

impl AddAssign<Duration> for OffsetDateTime {
    fn add_assign(&mut self, rhs: Duration) {
        self.0.add_assign(rhs);
    }
}

impl AddAssign<StdDuration> for OffsetDateTime {
    fn add_assign(&mut self, rhs: StdDuration) {
        self.0.add_assign(rhs);
    }
}

impl Sub<Duration> for OffsetDateTime {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0.sub(rhs))
    }
}

impl Sub<StdDuration> for OffsetDateTime {
    type Output = Self;

    fn sub(self, rhs: StdDuration) -> Self::Output {
        Self(self.0.sub(rhs))
    }
}

impl SubAssign<Duration> for OffsetDateTime {
    fn sub_assign(&mut self, rhs: Duration) {
        self.0.sub_assign(rhs);
    }
}

impl SubAssign<StdDuration> for OffsetDateTime {
    fn sub_assign(&mut self, rhs: StdDuration) {
        self.0.sub_assign(rhs);
    }
}

impl Sub for OffsetDateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(rhs.0)
    }
}

#[cfg(feature = "std")]
impl Sub<SystemTime> for OffsetDateTime {
    type Output = Duration;

    fn sub(self, rhs: SystemTime) -> Self::Output {
        self.0.sub(rhs)
    }
}

#[cfg(feature = "std")]
impl Sub<OffsetDateTime> for SystemTime {
    type Output = Duration;

    fn sub(self, rhs: OffsetDateTime) -> Self::Output {
        self.sub(rhs.0)
    }
}

#[cfg(feature = "std")]
impl PartialEq<SystemTime> for OffsetDateTime {
    fn eq(&self, rhs: &SystemTime) -> bool {
        self.0.eq(rhs)
    }
}

#[cfg(feature = "std")]
impl PartialEq<OffsetDateTime> for SystemTime {
    fn eq(&self, rhs: &OffsetDateTime) -> bool {
        self.eq(&rhs.0)
    }
}

#[cfg(feature = "std")]
impl PartialOrd<SystemTime> for OffsetDateTime {
    fn partial_cmp(&self, other: &SystemTime) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

#[cfg(feature = "std")]
impl PartialOrd<OffsetDateTime> for SystemTime {
    fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[cfg(feature = "std")]
impl From<SystemTime> for OffsetDateTime {
    fn from(system_time: SystemTime) -> Self {
        Self(Inner::from(system_time))
    }
}

#[cfg(feature = "std")]
impl From<OffsetDateTime> for SystemTime {
    fn from(datetime: OffsetDateTime) -> Self {
        datetime.0.into()
    }
}

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_os = "emscripten", target_os = "wasi")),
    feature = "wasm-bindgen"
))]
impl From<js_sys::Date> for OffsetDateTime {
    fn from(js_date: js_sys::Date) -> Self {
        Self(Inner::from(js_date))
    }
}

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_os = "emscripten", target_os = "wasi")),
    feature = "wasm-bindgen"
))]
impl From<OffsetDateTime> for js_sys::Date {
    fn from(datetime: OffsetDateTime) -> Self {
        datetime.0.into()
    }
}

// endregion trait impls
