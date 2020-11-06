use crate::{error, Date, Duration, PrimitiveDateTime, Time, UtcOffset, Weekday};
#[cfg(feature = "alloc")]
use crate::{
    format::parse::{parse, ParsedItems},
    DeferredFormat, Format, ParseResult,
};
#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
use const_fn::const_fn;
#[cfg(feature = "std")]
use core::convert::{From, TryFrom};
#[cfg(feature = "alloc")]
use core::fmt::{self, Display};
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};
#[cfg(feature = "std")]
use std::time::SystemTime;

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
        into = "crate::serde::OffsetDateTime",
        try_from = "crate::serde::OffsetDateTime"
    )
)]
#[derive(Debug, Clone, Copy, Eq)]
pub struct OffsetDateTime {
    /// The [`PrimitiveDateTime`], which is _always_ UTC.
    utc_datetime: PrimitiveDateTime,
    /// The [`UtcOffset`], which will be added to the [`PrimitiveDateTime`] as
    /// necessary.
    offset: UtcOffset,
}

impl OffsetDateTime {
    /// Create a new `OffsetDateTime` from the provided [`PrimitiveDateTime`]
    /// and [`UtcOffset`]. The [`PrimitiveDateTime`] is assumed to be in UTC.
    pub(crate) const fn new(utc_datetime: PrimitiveDateTime, offset: UtcOffset) -> Self {
        Self {
            utc_datetime,
            offset,
        }
    }

    /// Create a new `OffsetDateTime` with the current date and time in UTC.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::offset;
    /// assert!(OffsetDateTime::now_utc().year() >= 2019);
    /// assert_eq!(OffsetDateTime::now_utc().offset(), offset!("UTC"));
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "std")))]
    pub fn now_utc() -> Self {
        SystemTime::now().into()
    }

    /// Attempt to create a new `OffsetDateTime` with the current date and time
    /// in the local offset. If the offset cannot be determined, an error is
    /// returned.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// assert!(OffsetDateTime::now_local().is_ok());
    /// ```
    #[cfg(feature = "local-offset")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "local-offset")))]
    pub fn now_local() -> Result<Self, error::IndeterminateOffset> {
        let t = Self::now_utc();
        Ok(t.to_offset(UtcOffset::local_offset_at(t)?))
    }

    /// Convert the `OffsetDateTime` from the current [`UtcOffset`] to the
    /// provided [`UtcOffset`].
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(
    ///     datetime!("2000-01-01 0:00 UTC")
    ///         .to_offset(offset!("-1"))
    ///         .year(),
    ///     1999,
    /// );
    ///
    /// // Let's see what time Sydney's new year's celebration is in New York
    /// // and Los Angeles.
    ///
    /// // Construct midnight on new year's in Sydney.
    /// let sydney = datetime!("2000-01-01 0:00 +11");
    /// let new_york = sydney.to_offset(offset!("-5"));
    /// let los_angeles = sydney.to_offset(offset!("-8"));
    /// assert_eq!(sydney.hour(), 0);
    /// assert_eq!(new_york.hour(), 8);
    /// assert_eq!(los_angeles.hour(), 5);
    /// ```
    pub const fn to_offset(self, offset: UtcOffset) -> Self {
        Self {
            utc_datetime: self.utc_datetime,
            offset,
        }
    }

    /// Midnight, 1 January, 1970 (UTC).
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     OffsetDateTime::unix_epoch(),
    ///     datetime!("1970-01-01 0:00 UTC"),
    /// );
    /// ```
    pub const fn unix_epoch() -> Self {
        Date::from_yo_unchecked(1970, 1).midnight().assume_utc()
    }

    /// Create an `OffsetDateTime` from the provided [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(0),
    ///     Ok(OffsetDateTime::unix_epoch()),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(1_546_300_800),
    ///     Ok(datetime!("2019-01-01 0:00 UTC")),
    /// );
    /// ```
    ///
    /// If you have a timestamp-nanosecond pair, you can use something along the
    /// lines of the following:
    ///
    /// ```rust
    /// # use time::{Duration, OffsetDateTime, ext::NumericalDuration};
    /// let (timestamp, nanos) = (1, 500_000_000);
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(timestamp)? + Duration::nanoseconds(nanos),
    ///     OffsetDateTime::unix_epoch() + 1.5.seconds()
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_unix_timestamp(timestamp: i64) -> Result<Self, error::ComponentRange> {
        let unix_epoch_julian_date = Date::from_yo_unchecked(1970, 1).julian_day();

        let whole_days = timestamp / 86_400;
        let date = const_try!(Date::from_julian_day(unix_epoch_julian_date + whole_days));

        let hour = match (timestamp % 86_400 / 3_600) % 24 {
            value if value < 0 => value + 24,
            value => value,
        };
        let minute = match (timestamp % 3_600 / 60) % 60 {
            value if value < 0 => value + 60,
            value => value,
        };
        let second = match timestamp % 60 {
            value if value < 0 => value + 60,
            value => value,
        };
        let time = Time::from_hms_nanos_unchecked(hour as u8, minute as u8, second as u8, 0);

        Ok(PrimitiveDateTime::new(date, time).assume_utc())
    }

    /// Construct an `OffsetDateTime` from the provided Unix timestamp (in
    /// nanoseconds).
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp_nanos(0),
    ///     Ok(OffsetDateTime::unix_epoch()),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp_nanos(1_546_300_800_000_000_000),
    ///     Ok(datetime!("2019-01-01 0:00 UTC")),
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn from_unix_timestamp_nanos(timestamp: i128) -> Result<Self, error::ComponentRange> {
        let unix_epoch_julian_date = Date::from_yo_unchecked(1970, 1).julian_day();

        // Performing the division early lets us use an i64 instead of an i128.
        // This leads to significant performance gains.
        let timestamp_seconds = (timestamp / 1_000_000_000) as i64;

        let whole_days = timestamp_seconds / 86_400;
        let date = const_try!(Date::from_julian_day(unix_epoch_julian_date + whole_days));

        let hour = match (timestamp_seconds % 86_400 / 3_600) % 24 {
            value if value < 0 => value + 24,
            value => value,
        };
        let minute = match (timestamp_seconds % 3_600 / 60_000) % 60 {
            value if value < 0 => value + 60,
            value => value,
        };
        let second = match timestamp_seconds % 60 {
            value if value < 0 => value + 60,
            value => value,
        };
        let nanos = match timestamp % 1_000_000_000 {
            value if value < 0 => value + 1_000_000_000,
            value => value,
        };
        let time =
            Time::from_hms_nanos_unchecked(hour as u8, minute as u8, second as u8, nanos as u32);

        Ok(PrimitiveDateTime::new(date, time).assume_utc())
    }

    /// Get the [`UtcOffset`].
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").offset(), offset!("UTC"));
    /// assert_eq!(datetime!("2019-01-01 0:00 +1").offset(), offset!("+1"));
    /// ```
    pub const fn offset(self) -> UtcOffset {
        self.offset
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("1970-01-01 0:00 UTC").unix_timestamp(), 0);
    /// assert_eq!(datetime!("1970-01-01 0:00 -1").unix_timestamp(), 3_600);
    /// ```
    pub fn unix_timestamp(self) -> i64 {
        (self - Self::unix_epoch()).whole_seconds()
    }

    /// Get the Unix timestamp in nanoseconds.
    ///
    /// ```rust
    /// use time_macros::datetime;
    /// assert_eq!(datetime!("1970-01-01 0:00 UTC").unix_timestamp_nanos(), 0);
    /// assert_eq!(
    ///     datetime!("1970-01-01 0:00 -1").unix_timestamp_nanos(),
    ///     3_600_000_000_000,
    /// );
    /// ```
    pub fn unix_timestamp_nanos(self) -> i128 {
        (self - Self::unix_epoch()).whole_nanoseconds()
    }

    /// Get the [`Date`] in the stored offset.
    ///
    /// ```rust
    /// # use time_macros::{date, datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").date(), date!("2019-01-01"));
    /// assert_eq!(
    ///     datetime!("2019-01-01 0:00 UTC")
    ///         .to_offset(offset!("-1"))
    ///         .date(),
    ///     date!("2018-12-31"),
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn date(self) -> Date {
        self.utc_datetime.utc_to_offset(self.offset()).date()
    }

    /// Get the [`Time`] in the stored offset.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset, time};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").time(), time!("0:00"));
    /// assert_eq!(
    ///     datetime!("2019-01-01 0:00 UTC")
    ///         .to_offset(offset!("-1"))
    ///         .time(),
    ///     time!("23:00")
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn time(self) -> Time {
        self.utc_datetime.utc_to_offset(self.offset()).time()
    }

    /// Get the year of the date in the stored offset.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").year(), 2019);
    /// assert_eq!(
    ///     datetime!("2019-12-31 23:00 UTC")
    ///         .to_offset(offset!("+1"))
    ///         .year(),
    ///     2020,
    /// );
    /// assert_eq!(datetime!("2020-01-01 0:00 UTC").year(), 2020);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn year(self) -> i32 {
        self.date().year()
    }

    /// Get the month of the date in the stored offset. If fetching both the
    /// month and day, it is more efficient to use
    /// [`OffsetDateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").month(), 1);
    /// assert_eq!(
    ///     datetime!("2019-12-31 23:00 UTC")
    ///         .to_offset(offset!("+1"))
    ///         .month(),
    ///     1,
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn month(self) -> u8 {
        self.date().month()
    }

    /// Get the day of the date in the stored offset. If fetching both the month
    /// and day, it is more efficient to use [`OffsetDateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").day(), 1);
    /// assert_eq!(
    ///     datetime!("2019-12-31 23:00 UTC")
    ///         .to_offset(offset!("+1"))
    ///         .day(),
    ///     1,
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn day(self) -> u8 {
        self.date().day()
    }

    /// Get the month and day of the date in the stored offset.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").month_day(), (1, 1));
    /// assert_eq!(
    ///     datetime!("2019-12-31 23:00 UTC")
    ///         .to_offset(offset!("+1"))
    ///         .month_day(),
    ///     (1, 1),
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn month_day(self) -> (u8, u8) {
        self.date().month_day()
    }

    /// Get the day of the year of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=366`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").ordinal(), 1);
    /// assert_eq!(
    ///     datetime!("2019-12-31 23:00 UTC")
    ///         .to_offset(offset!("+1"))
    ///         .ordinal(),
    ///     1,
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn ordinal(self) -> u16 {
        self.date().ordinal()
    }

    /// Get the ISO 8601 year and week number in the stored offset.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").iso_year_week(), (2019, 1));
    /// assert_eq!(datetime!("2019-10-04 0:00 UTC").iso_year_week(), (2019, 40));
    /// assert_eq!(datetime!("2020-01-01 0:00 UTC").iso_year_week(), (2020, 1));
    /// assert_eq!(datetime!("2020-12-31 0:00 UTC").iso_year_week(), (2020, 53));
    /// assert_eq!(datetime!("2021-01-01 0:00 UTC").iso_year_week(), (2020, 53));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn iso_year_week(self) -> (i32, u8) {
        self.date().iso_year_week()
    }

    /// Get the ISO week number of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").week(), 1);
    /// assert_eq!(datetime!("2020-01-01 0:00 UTC").week(), 1);
    /// assert_eq!(datetime!("2020-12-31 0:00 UTC").week(), 53);
    /// assert_eq!(datetime!("2021-01-01 0:00 UTC").week(), 53);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn week(self) -> u8 {
        self.date().week()
    }

    /// Get the weekday of the date in the stored offset.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
    ///
    /// ```rust
    /// # use time::Weekday::*;
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").weekday(), Tuesday);
    /// assert_eq!(datetime!("2019-02-01 0:00 UTC").weekday(), Friday);
    /// assert_eq!(datetime!("2019-03-01 0:00 UTC").weekday(), Friday);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn weekday(self) -> Weekday {
        self.date().weekday()
    }

    /// Get the clock hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").hour(), 0);
    /// assert_eq!(
    ///     datetime!("2019-01-01 23:59:59 UTC")
    ///         .to_offset(offset!("-2"))
    ///         .hour(),
    ///     21,
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn hour(self) -> u8 {
        self.time().hour()
    }

    /// Get the minute within the hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").minute(), 0);
    /// assert_eq!(
    ///     datetime!("2019-01-01 23:59:59 UTC")
    ///         .to_offset(offset!("+0:30"))
    ///         .minute(),
    ///     29,
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn minute(self) -> u8 {
        self.time().minute()
    }

    /// Get the second within the minute in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").second(), 0);
    /// assert_eq!(
    ///     datetime!("2019-01-01 23:59:59 UTC")
    ///         .to_offset(offset!("+0:00:30"))
    ///         .second(),
    ///     29,
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn second(self) -> u8 {
        self.time().second()
    }

    // Because a `UtcOffset` is limited in resolution to one second, any
    // subsecond value will not change when adjusting for the offset.

    /// Get the milliseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").millisecond(), 0);
    /// assert_eq!(datetime!("2019-01-01 23:59:59.999 UTC").millisecond(), 999);
    /// ```
    pub const fn millisecond(self) -> u16 {
        self.utc_datetime.time().millisecond()
    }

    /// Get the microseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").microsecond(), 0);
    /// assert_eq!(
    ///     datetime!("2019-01-01 23:59:59.999_999 UTC").microsecond(),
    ///     999_999,
    /// );
    /// ```
    pub const fn microsecond(self) -> u32 {
        self.utc_datetime.time().microsecond()
    }

    /// Get the nanoseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(datetime!("2019-01-01 0:00 UTC").nanosecond(), 0);
    /// assert_eq!(
    ///     datetime!("2019-01-01 23:59:59.999_999_999 UTC").nanosecond(),
    ///     999_999_999,
    /// );
    /// ```
    pub const fn nanosecond(self) -> u32 {
        self.utc_datetime.time().nanosecond()
    }
}

/// Methods that replace part of the `OffsetDateTime`.
impl OffsetDateTime {
    /// Replace the time, which is assumed to be in the stored offset. The date
    /// and offset components are unchanged.
    ///
    /// ```rust
    /// # use time_macros::{datetime, time};
    /// assert_eq!(
    ///     datetime!("2020-01-01 5:00 UTC").replace_time(time!("12:00")),
    ///     datetime!("2020-01-01 12:00 UTC")
    /// );
    /// assert_eq!(
    ///     datetime!("2020-01-01 12:00 -5").replace_time(time!("7:00")),
    ///     datetime!("2020-01-01 7:00 -5")
    /// );
    /// assert_eq!(
    ///     datetime!("2020-01-01 0:00 +1").replace_time(time!("12:00")),
    ///     datetime!("2020-01-01 12:00 +1")
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    #[const_fn("1.46")]
    pub const fn replace_time(self, time: Time) -> Self {
        self.utc_datetime
            .utc_to_offset(self.offset)
            .replace_time(time)
            .assume_offset(self.offset)
    }

    /// Replace the date, which is assumed to be in the stored offset. The time
    /// and offset components are unchanged.
    ///
    /// ```rust
    /// # use time_macros::{datetime, date};
    /// assert_eq!(
    ///     datetime!("2020-01-01 12:00 UTC").replace_date(date!("2020-01-30")),
    ///     datetime!("2020-01-30 12:00 UTC")
    /// );
    /// assert_eq!(
    ///     datetime!("2020-01-01 0:00 +1").replace_date(date!("2020-01-30")),
    ///     datetime!("2020-01-30 0:00 +1")
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    #[const_fn("1.46")]
    pub const fn replace_date(self, date: Date) -> Self {
        self.utc_datetime
            .utc_to_offset(self.offset)
            .replace_date(date)
            .assume_offset(self.offset)
    }

    /// Replace the date and time, which are assumed to be in the stored offset.
    /// The offset component remains unchanged.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!("2020-01-01 12:00 UTC").replace_date_time(datetime!("2020-01-30 16:00")),
    ///     datetime!("2020-01-30 16:00 UTC")
    /// );
    /// assert_eq!(
    ///     datetime!("2020-01-01 12:00 +1").replace_date_time(datetime!("2020-01-30 0:00")),
    ///     datetime!("2020-01-30 0:00 +1")
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    #[const_fn("1.46")]
    pub const fn replace_date_time(self, date_time: PrimitiveDateTime) -> Self {
        date_time.assume_offset(self.offset)
    }

    /// Replace the offset. The date and time components remain unchanged.
    ///
    /// ```rust
    /// # use time_macros::{datetime, offset};
    /// assert_eq!(
    ///     datetime!("2020-01-01 0:00 UTC").replace_offset(offset!("-5")),
    ///     datetime!("2020-01-01 0:00 -5")
    /// );
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    #[const_fn("1.46")]
    pub const fn replace_offset(self, offset: UtcOffset) -> Self {
        self.utc_datetime.assume_offset(offset)
    }
}

/// Methods that allow formatting the `OffsetDateTime`.
#[cfg(feature = "alloc")]
impl OffsetDateTime {
    /// Format the `OffsetDateTime` using the provided string.
    ///
    /// ```rust
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     datetime!("2019-01-02 0:00 UTC").format("%F %r %z"),
    ///     "2019-01-02 12:00:00 am +0000",
    /// );
    /// ```
    pub fn format<'a>(self, format: impl Into<Format<'a>>) -> String {
        DeferredFormat::new(format.into())
            .with_date(self.date())
            .with_time(self.time())
            .with_offset(self.offset())
            .to_string()
    }

    /// Attempt to parse an `OffsetDateTime` using the provided string.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use time_macros::datetime;
    /// assert_eq!(
    ///     OffsetDateTime::parse("2019-01-02 00:00:00 +0000", "%F %T %z"),
    ///     Ok(datetime!("2019-01-02 0:00 UTC")),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::parse("2019-002 23:59:59 +0000", "%Y-%j %T %z"),
    ///     Ok(datetime!("2019-002 23:59:59 UTC")),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::parse("2019-W01-3 12:00:00 pm +0000", "%G-W%V-%u %r %z"),
    ///     Ok(datetime!("2019-W01-3 12:00 UTC")),
    /// );
    /// ```
    pub fn parse<'a>(s: impl AsRef<str>, format: impl Into<Format<'a>>) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s.as_ref(), &format.into())?)
    }

    /// Given the items already parsed, attempt to create an `OffsetDateTime`.
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        let offset = UtcOffset::try_from_parsed_items(items)?;
        Ok(PrimitiveDateTime::try_from_parsed_items(items)?.assume_offset(offset))
    }
}

#[cfg(feature = "alloc")]
impl Display for OffsetDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.date(), self.time(), self.offset())
    }
}

impl PartialEq for OffsetDateTime {
    fn eq(&self, rhs: &Self) -> bool {
        self.utc_datetime.eq(&rhs.utc_datetime)
    }
}

impl PartialOrd for OffsetDateTime {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for OffsetDateTime {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.utc_datetime.cmp(&rhs.utc_datetime)
    }
}

impl Hash for OffsetDateTime {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // We need to distinguish this from a `PrimitiveDateTime`, which would
        // otherwise conflict.
        hasher.write(b"OffsetDateTime");
        self.utc_datetime.hash(hasher);
    }
}

impl Add<Duration> for OffsetDateTime {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime + duration,
            offset: self.offset,
        }
    }
}

impl Add<StdDuration> for OffsetDateTime {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime + duration,
            offset: self.offset,
        }
    }
}

impl AddAssign<Duration> for OffsetDateTime {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for OffsetDateTime {
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for OffsetDateTime {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime - duration,
            offset: self.offset,
        }
    }
}

impl Sub<StdDuration> for OffsetDateTime {
    type Output = Self;

    fn sub(self, duration: StdDuration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime - duration,
            offset: self.offset,
        }
    }
}

impl SubAssign<Duration> for OffsetDateTime {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for OffsetDateTime {
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<OffsetDateTime> for OffsetDateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.utc_datetime - rhs.utc_datetime
    }
}

#[cfg(feature = "std")]
impl Add<Duration> for SystemTime {
    type Output = Self;

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

#[cfg(feature = "std")]
impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

#[cfg(feature = "std")]
impl Sub<Duration> for SystemTime {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        (OffsetDateTime::from(self) - duration).into()
    }
}

#[cfg(feature = "std")]
impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

#[cfg(feature = "std")]
impl Sub<SystemTime> for OffsetDateTime {
    type Output = Duration;

    fn sub(self, rhs: SystemTime) -> Self::Output {
        self - Self::from(rhs)
    }
}

#[cfg(feature = "std")]
impl Sub<OffsetDateTime> for SystemTime {
    type Output = Duration;

    fn sub(self, rhs: OffsetDateTime) -> Self::Output {
        OffsetDateTime::from(self) - rhs
    }
}

#[cfg(feature = "std")]
impl PartialEq<SystemTime> for OffsetDateTime {
    fn eq(&self, rhs: &SystemTime) -> bool {
        self == &Self::from(*rhs)
    }
}

#[cfg(feature = "std")]
impl PartialEq<OffsetDateTime> for SystemTime {
    fn eq(&self, rhs: &OffsetDateTime) -> bool {
        &OffsetDateTime::from(*self) == rhs
    }
}

#[cfg(feature = "std")]
impl PartialOrd<SystemTime> for OffsetDateTime {
    fn partial_cmp(&self, other: &SystemTime) -> Option<Ordering> {
        self.partial_cmp(&Self::from(*other))
    }
}

#[cfg(feature = "std")]
impl PartialOrd<OffsetDateTime> for SystemTime {
    fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
        OffsetDateTime::from(*self).partial_cmp(other)
    }
}

#[cfg(feature = "std")]
impl From<SystemTime> for OffsetDateTime {
    // There is definitely some way to have this conversion be infallible, but
    // it won't be an issue for over 500 years.
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

#[cfg(feature = "std")]
impl From<OffsetDateTime> for SystemTime {
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
