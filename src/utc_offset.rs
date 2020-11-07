use crate::error;
#[cfg(feature = "local-offset")]
use crate::OffsetDateTime;
#[cfg(feature = "alloc")]
use crate::{
    format::{parse, ParsedItems},
    DeferredFormat, Duration, Format, ParseResult,
};
#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
use const_fn::const_fn;
use core::fmt::{self, Display};

/// An offset from UTC.
///
/// Guaranteed to store values up to ±23:59:59. Any values outside this range
/// may have incidental support that can change at any time without notice. If
/// you need support outside this range, please file an issue with your use
/// case.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(into = "crate::serde::UtcOffset", try_from = "crate::serde::UtcOffset")
)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UtcOffset {
    /// The number of seconds offset from UTC. Positive is east, negative is
    /// west.
    pub(crate) seconds: i32,
}

impl UtcOffset {
    /// A `UtcOffset` that is UTC.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// # use time_macros::offset;
    /// assert_eq!(UtcOffset::UTC, offset!("UTC"));
    /// ```
    pub const UTC: Self = Self::seconds_unchecked(0);

    /// Create a `UtcOffset` representing an offset by the number of seconds
    /// provided, the validity of which must be guaranteed by the caller.
    #[doc(hidden)]
    pub const fn seconds_unchecked(seconds: i32) -> Self {
        Self { seconds }
    }

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_hours(1)?.as_hours(), 1);
    /// assert_eq!(UtcOffset::east_hours(2)?.as_minutes(), 120);
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn east_hours(hours: u8) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hours in 0 => 23);
        Ok(Self::seconds_unchecked(hours as i32 * 3_600))
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_hours(1)?.as_hours(), -1);
    /// assert_eq!(UtcOffset::west_hours(2)?.as_minutes(), -120);
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn west_hours(hours: u8) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hours in 0 => 23);
        Ok(Self::seconds_unchecked(hours as i32 * -3600))
    }

    /// Create a `UtcOffset` representing an offset by the number of hours
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::hours(2)?.as_minutes(), 120);
    /// assert_eq!(UtcOffset::hours(-2)?.as_minutes(), -120);
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn hours(hours: i8) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hours in -23 => 23);
        Ok(Self::seconds_unchecked(hours as i32 * 3_600))
    }

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_minutes(60)?.as_hours(), 1);
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn east_minutes(minutes: u16) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(minutes in 0 => 1_439);
        Ok(Self::seconds_unchecked(minutes as i32 * 60))
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_minutes(60)?.as_hours(), -1);
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn west_minutes(minutes: u16) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(minutes in 0 => 1_439);
        Ok(Self::seconds_unchecked(minutes as i32 * -60))
    }

    /// Create a `UtcOffset` representing a offset by the number of minutes
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::minutes(60)?.as_hours(), 1);
    /// assert_eq!(UtcOffset::minutes(-60)?.as_hours(), -1);
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn minutes(minutes: i16) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(minutes in -1_439 => 1_439);
        Ok(Self::seconds_unchecked(minutes as i32 * 60))
    }

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_seconds(3_600)?.as_hours(), 1);
    /// assert_eq!(UtcOffset::east_seconds(1_800)?.as_minutes(), 30);
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn east_seconds(seconds: u32) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(seconds in 0 => 86_399);
        Ok(Self::seconds_unchecked(seconds as i32))
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_seconds(3_600)?.as_hours(), -1);
    /// assert_eq!(UtcOffset::west_seconds(1_800)?.as_minutes(), -30);
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn west_seconds(seconds: u32) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(seconds in 0 => 86_399);
        Ok(Self::seconds_unchecked(-(seconds as i32)))
    }

    /// Create a `UtcOffset` representing an offset by the number of seconds
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::seconds(3_600)?.as_hours(), 1);
    /// assert_eq!(UtcOffset::seconds(-3_600)?.as_hours(), -1);
    /// # Ok::<_, time::Error>(())
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn seconds(seconds: i32) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(seconds in -86_399 => 86_399);
        Ok(Self::seconds_unchecked(seconds))
    }

    /// Get the number of seconds from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_seconds(), 0);
    /// assert_eq!(UtcOffset::hours(12)?.as_seconds(), 43_200);
    /// assert_eq!(UtcOffset::hours(-12)?.as_seconds(), -43_200);
    /// # Ok::<_, time::Error>(())
    /// ```
    pub const fn as_seconds(self) -> i32 {
        self.seconds
    }

    /// Get the number of minutes from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_minutes(), 0);
    /// assert_eq!(UtcOffset::hours(12)?.as_minutes(), 720);
    /// assert_eq!(UtcOffset::hours(-12)?.as_minutes(), -720);
    /// # Ok::<_, time::Error>(())
    /// ```
    pub const fn as_minutes(self) -> i16 {
        (self.as_seconds() / 60) as i16
    }

    /// Get the number of hours from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_hours(), 0);
    /// assert_eq!(UtcOffset::hours(12)?.as_hours(), 12);
    /// assert_eq!(UtcOffset::hours(-12)?.as_hours(), -12);
    /// # Ok::<_, time::Error>(())
    /// ```
    pub const fn as_hours(self) -> i8 {
        (self.as_seconds() / 3_600) as i8
    }

    /// Convert a `UtcOffset` to ` Duration`. Useful for implementing operators.
    #[cfg(feature = "alloc")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "alloc")))]
    pub(crate) const fn as_duration(self) -> Duration {
        Duration::seconds(self.seconds as i64)
    }

    /// Attempt to obtain the system's UTC offset at a known moment in time. If
    /// the offset cannot be determined, an error is returned.
    ///
    /// ```rust
    /// # use time::{UtcOffset, OffsetDateTime};
    /// let unix_epoch = OffsetDateTime::unix_epoch();
    /// let local_offset = UtcOffset::local_offset_at(unix_epoch);
    /// assert!(local_offset.is_ok());
    /// ```
    #[cfg(feature = "local-offset")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "local-offset")))]
    pub fn local_offset_at(datetime: OffsetDateTime) -> Result<Self, error::IndeterminateOffset> {
        local_offset_at(datetime).ok_or(error::IndeterminateOffset)
    }

    /// Attempt to obtain the system's current UTC offset. If the offset cannot
    /// be determined, an error is returned.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// let local_offset = UtcOffset::current_local_offset();
    /// assert!(local_offset.is_ok());
    /// ```
    #[cfg(feature = "local-offset")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "local-offset")))]
    pub fn current_local_offset() -> Result<Self, error::IndeterminateOffset> {
        let now = OffsetDateTime::now_utc();
        local_offset_at(now).ok_or(error::IndeterminateOffset)
    }
}

/// Methods that allow parsing and formatting the `UtcOffset`.
#[cfg(feature = "alloc")]
#[cfg_attr(__time_02_docs, doc(cfg(feature = "alloc")))]
impl UtcOffset {
    /// Format the `UtcOffset` using the provided string.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::hours(2)?.format("%z"), "+0200");
    /// assert_eq!(UtcOffset::hours(-2)?.format("%z"), "-0200");
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn format<'a>(self, format: impl Into<Format<'a>>) -> String {
        DeferredFormat::new(format.into())
            .with_offset(self)
            .to_string()
    }

    /// Attempt to parse the `UtcOffset` using the provided string.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::parse("+0200", "%z"), Ok(UtcOffset::hours(2)?));
    /// assert_eq!(UtcOffset::parse("-0200", "%z"), Ok(UtcOffset::hours(-2)?));
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn parse<'a>(s: impl AsRef<str>, format: impl Into<Format<'a>>) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s.as_ref(), &format.into())?)
    }

    /// Given the items already parsed, attempt to create a `UtcOffset`.
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        items.offset.ok_or(error::Parse::InsufficientInformation)
    }
}

impl Display for UtcOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.seconds < 0 { '-' } else { '+' };
        let hours = self.as_hours().abs();
        let minutes = self.as_minutes().abs() - hours as i16 * 60;
        let seconds = self.as_seconds().abs() - hours as i32 * 3_600 - minutes as i32 * 60;

        write!(f, "{}{}", sign, hours)?;

        if minutes != 0 || seconds != 0 {
            write!(f, ":{:02}", minutes)?;
        }

        if seconds != 0 {
            write!(f, ":{:02}", seconds)?;
        }

        Ok(())
    }
}

/// Attempt to obtain the system's UTC offset. If the offset cannot be
/// determined, `None` is returned.
#[cfg(feature = "local-offset")]
#[allow(clippy::too_many_lines)]
fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    #[cfg(target_family = "unix")]
    {
        use core::{convert::TryInto, mem::MaybeUninit};

        /// Convert the given Unix timestamp to a `libc::tm`. Returns `None` on
        /// any error.
        fn timestamp_to_tm(timestamp: i64) -> Option<libc::tm> {
            extern "C" {
                #[cfg_attr(target_os = "netbsd", link_name = "__tzset50")]
                fn tzset();
            }

            // The exact type of `timestamp` beforehand can vary, so this
            // conversion is necessary.
            #[allow(clippy::useless_conversion)]
            let timestamp = timestamp.try_into().ok()?;

            let mut tm = MaybeUninit::uninit();

            // Update timezone information from system. `localtime_r` does not
            // do this for us.
            //
            // Safety: tzset is thread-safe.
            #[allow(unsafe_code)]
            unsafe {
                tzset();
            }

            // Safety: We are calling a system API, which mutates the `tm`
            // variable. If a null pointer is returned, an error occurred.
            #[allow(unsafe_code)]
            let tm_ptr = unsafe { libc::localtime_r(&timestamp, tm.as_mut_ptr()) };

            if tm_ptr.is_null() {
                None
            } else {
                // Safety: The value was initialized, as we no longer have a
                // null pointer.
                #[allow(unsafe_code)]
                {
                    Some(unsafe { tm.assume_init() })
                }
            }
        }

        let tm = timestamp_to_tm(datetime.unix_timestamp())?;

        // `tm_gmtoff` extension
        #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
        {
            UtcOffset::seconds(tm.tm_gmtoff.try_into().ok()?).ok()
        }

        // No `tm_gmtoff` extension
        #[cfg(any(target_os = "solaris", target_os = "illumos"))]
        {
            use crate::Date;
            use core::convert::TryFrom;

            let mut tm = tm;
            if tm.tm_sec == 60 {
                // Leap seconds are not currently supported.
                tm.tm_sec = 59;
            }

            let local_timestamp =
                Date::from_yo(1900 + tm.tm_year, u16::try_from(tm.tm_yday).ok()? + 1)
                    .ok()?
                    .with_hms(
                        tm.tm_hour.try_into().ok()?,
                        tm.tm_min.try_into().ok()?,
                        tm.tm_sec.try_into().ok()?,
                    )
                    .ok()?
                    .assume_utc()
                    .unix_timestamp();

            UtcOffset::seconds(
                (local_timestamp - datetime.unix_timestamp())
                    .try_into()
                    .ok()?,
            )
            .ok()
        }
    }
    #[cfg(target_family = "windows")]
    {
        use core::{convert::TryInto, mem::MaybeUninit};
        use winapi::{
            shared::minwindef::FILETIME,
            um::{
                minwinbase::SYSTEMTIME,
                timezoneapi::{SystemTimeToFileTime, SystemTimeToTzSpecificLocalTime},
            },
        };

        /// Convert a `SYSTEMTIME` to a `FILETIME`. Returns `None` if any error
        /// occurred.
        fn systemtime_to_filetime(systime: &SYSTEMTIME) -> Option<FILETIME> {
            let mut ft = MaybeUninit::uninit();

            // Safety: `SystemTimeToFileTime` is thread-safe. We are only
            // assuming initialization if the call succeeded.
            #[allow(unsafe_code)]
            {
                if 0 == unsafe { SystemTimeToFileTime(systime, ft.as_mut_ptr()) } {
                    // failed
                    None
                } else {
                    Some(unsafe { ft.assume_init() })
                }
            }
        }

        /// Convert a `FILETIME` to an `i64`, representing a number of seconds.
        fn filetime_to_secs(filetime: &FILETIME) -> i64 {
            /// FILETIME represents 100-nanosecond intervals
            const FT_TO_SECS: i64 = 10_000_000;
            ((filetime.dwHighDateTime as i64) << 32 | filetime.dwLowDateTime as i64) / FT_TO_SECS
        }

        /// Convert an [`OffsetDateTime`] to a `SYSTEMTIME`.
        fn offset_to_systemtime(datetime: OffsetDateTime) -> SYSTEMTIME {
            let (month, day_of_month) = datetime.to_offset(UtcOffset::UTC).month_day();
            SYSTEMTIME {
                wYear: datetime.year() as u16,
                wMonth: month as u16,
                wDay: day_of_month as u16,
                wDayOfWeek: 0, // ignored
                wHour: datetime.hour() as u16,
                wMinute: datetime.minute() as u16,
                wSecond: datetime.second() as u16,
                wMilliseconds: datetime.millisecond(),
            }
        }

        // This function falls back to UTC if any system call fails.
        let systime_utc = offset_to_systemtime(datetime.to_offset(UtcOffset::UTC));

        // Safety: `local_time` is only read if it is properly initialized, and
        // `SystemTimeToTzSpecificLocalTime` is thread-safe.
        #[allow(unsafe_code)]
        let systime_local = unsafe {
            let mut local_time = MaybeUninit::uninit();

            if 0 == SystemTimeToTzSpecificLocalTime(
                core::ptr::null(), // use system's current timezone
                &systime_utc,
                local_time.as_mut_ptr(),
            ) {
                // call failed
                return None;
            } else {
                local_time.assume_init()
            }
        };

        // Convert SYSTEMTIMEs to FILETIMEs so we can perform arithmetic on
        // them.
        let ft_system = systemtime_to_filetime(&systime_utc)?;
        let ft_local = systemtime_to_filetime(&systime_local)?;

        let diff_secs = filetime_to_secs(&ft_local) - filetime_to_secs(&ft_system);

        UtcOffset::seconds(diff_secs.try_into().ok()?).ok()
    }
    #[cfg(not(any(target_family = "unix", target_family = "windows")))]
    {
        // Silence the unused variable warning when appropriate.
        let _ = datetime;
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ext::NumericalDuration;

    #[test]
    fn as_duration() -> crate::Result<()> {
        assert_eq!(UtcOffset::hours(1)?.as_duration(), 1.hours());
        assert_eq!(UtcOffset::hours(-1)?.as_duration(), (-1).hours());
        Ok(())
    }
}
