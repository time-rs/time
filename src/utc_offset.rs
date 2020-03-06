use crate::{
    format::{parse, ParsedItems},
    internal_prelude::*,
};
use core::fmt::{self, Display};
#[cfg(cargo_web)]
use stdweb::js;

/// An offset from UTC.
///
/// Guaranteed to store values up to ±23:59:59. Any values outside this range
/// may have incidental support that can change at any time without notice. If
/// you need support outside this range, please file an issue with your use
/// case.
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    serde,
    serde(from = "crate::serde::UtcOffset", into = "crate::serde::UtcOffset")
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
    /// # use time::{UtcOffset, offset};
    /// assert_eq!(UtcOffset::UTC, offset!(UTC));
    /// ```
    pub const UTC: Self = Self::seconds(0);

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_hours(1).as_hours(), 1);
    /// assert_eq!(UtcOffset::east_hours(2).as_minutes(), 120);
    /// ```
    #[inline(always)]
    pub const fn east_hours(hours: u8) -> Self {
        Self::hours(hours as i8)
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_hours(1).as_hours(), -1);
    /// assert_eq!(UtcOffset::west_hours(2).as_minutes(), -120);
    /// ```
    #[inline(always)]
    pub const fn west_hours(hours: u8) -> Self {
        Self::hours(-(hours as i8))
    }

    /// Create a `UtcOffset` representing an offset by the number of hours
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::hours(2).as_minutes(), 120);
    /// assert_eq!(UtcOffset::hours(-2).as_minutes(), -120);
    /// ```
    #[inline(always)]
    pub const fn hours(hours: i8) -> Self {
        Self::seconds(hours as i32 * 3_600)
    }

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_minutes(60).as_hours(), 1);
    /// ```
    #[inline(always)]
    pub const fn east_minutes(minutes: u16) -> Self {
        Self::minutes(minutes as i16)
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_minutes(60).as_hours(), -1);
    /// ```
    #[inline(always)]
    pub const fn west_minutes(minutes: u16) -> Self {
        Self::minutes(-(minutes as i16))
    }

    /// Create a `UtcOffset` representing a offset by the number of minutes
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::minutes(60).as_hours(), 1);
    /// assert_eq!(UtcOffset::minutes(-60).as_hours(), -1);
    /// ```
    #[inline(always)]
    pub const fn minutes(minutes: i16) -> Self {
        Self::seconds(minutes as i32 * 60)
    }

    /// Create a `UtcOffset` representing an easterly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::east_seconds(3_600).as_hours(), 1);
    /// assert_eq!(UtcOffset::east_seconds(1_800).as_minutes(), 30);
    /// ```
    #[inline(always)]
    pub const fn east_seconds(seconds: u32) -> Self {
        Self::seconds(seconds as i32)
    }

    /// Create a `UtcOffset` representing a westerly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::west_seconds(3_600).as_hours(), -1);
    /// assert_eq!(UtcOffset::west_seconds(1_800).as_minutes(), -30);
    /// ```
    #[inline(always)]
    pub const fn west_seconds(seconds: u32) -> Self {
        Self::seconds(-(seconds as i32))
    }

    /// Create a `UtcOffset` representing an offset by the number of seconds
    /// provided.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::seconds(3_600).as_hours(), 1);
    /// assert_eq!(UtcOffset::seconds(-3_600).as_hours(), -1);
    /// ```
    #[inline(always)]
    pub const fn seconds(seconds: i32) -> Self {
        Self { seconds }
    }

    /// Get the number of seconds from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_seconds(), 0);
    /// assert_eq!(UtcOffset::hours(12).as_seconds(), 43_200);
    /// assert_eq!(UtcOffset::hours(-12).as_seconds(), -43_200);
    /// ```
    #[inline(always)]
    pub const fn as_seconds(self) -> i32 {
        self.seconds
    }

    /// Get the number of minutes from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_minutes(), 0);
    /// assert_eq!(UtcOffset::hours(12).as_minutes(), 720);
    /// assert_eq!(UtcOffset::hours(-12).as_minutes(), -720);
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_minutes(self) -> i16 {
        (self.as_seconds() / 60) as i16
    }

    /// Get the number of hours from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::UTC.as_hours(), 0);
    /// assert_eq!(UtcOffset::hours(12).as_hours(), 12);
    /// assert_eq!(UtcOffset::hours(-12).as_hours(), -12);
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_hours(self) -> i8 {
        (self.as_seconds() / 3_600) as i8
    }

    /// Convert a `UtcOffset` to ` Duration`. Useful for implementing operators.
    #[inline(always)]
    pub(crate) const fn as_duration(self) -> Duration {
        Duration::seconds(self.seconds as i64)
    }

    /// Obtain the system's UTC offset at a known moment in time. If the offset
    /// cannot be determined, UTC is returned.
    ///
    /// ```rust,no_run
    /// # use time::{UtcOffset, OffsetDateTime};
    /// let unix_epoch = OffsetDateTime::unix_epoch();
    /// let local_offset = UtcOffset::local_offset_at(unix_epoch);
    /// println!("{}", local_offset.format("%z"));
    /// ```
    #[inline(always)]
    #[cfg(std)]
    pub fn local_offset_at(datetime: OffsetDateTime) -> Self {
        try_local_offset_at(datetime).unwrap_or(Self::UTC)
    }

    /// Attempt to obtain the system's UTC offset at a known moment in time. If
    /// the offset cannot be determined, an error is returned.
    ///
    /// ```rust,no_run
    /// # use time::{UtcOffset, OffsetDateTime};
    /// let unix_epoch = OffsetDateTime::unix_epoch();
    /// let local_offset = UtcOffset::try_local_offset_at(unix_epoch);
    /// assert!(local_offset.is_ok());
    /// ```
    #[inline(always)]
    #[cfg(std)]
    pub fn try_local_offset_at(datetime: OffsetDateTime) -> Result<Self, IndeterminateOffsetError> {
        try_local_offset_at(datetime).ok_or_else(IndeterminateOffsetError::new)
    }

    /// Obtain the system's current UTC offset. If the offset cannot be
    /// determined, UTC is returned.
    ///
    /// ```rust,no_run
    /// # use time::UtcOffset;
    /// let local_offset = UtcOffset::current_local_offset();
    /// println!("{}", local_offset.format("%z"));
    /// ```
    #[inline(always)]
    #[cfg(std)]
    pub fn current_local_offset() -> Self {
        let now = OffsetDateTime::now();
        try_local_offset_at(now).unwrap_or(Self::UTC)
    }

    /// Attempt to obtain the system's current UTC offset. If the offset cannot
    /// be determined, an error is returned.
    ///
    /// ```rust,no_run
    /// # use time::UtcOffset;
    /// let local_offset = UtcOffset::try_current_local_offset();
    /// assert!(local_offset.is_ok());
    /// ```
    #[inline(always)]
    #[cfg(std)]
    pub fn try_current_local_offset() -> Result<Self, IndeterminateOffsetError> {
        let now = OffsetDateTime::now();
        try_local_offset_at(now).ok_or_else(IndeterminateOffsetError::new)
    }
}

/// Methods that allow parsing and formatting the `UtcOffset`.
impl UtcOffset {
    /// Format the `UtcOffset` using the provided string.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::hours(2).format("%z"), "+0200");
    /// assert_eq!(UtcOffset::hours(-2).format("%z"), "-0200");
    /// ```
    #[inline(always)]
    pub fn format(self, format: impl AsRef<str>) -> String {
        DeferredFormat::new(format.as_ref())
            .with_offset(self)
            .to_string()
    }

    /// Attempt to parse the `UtcOffset` using the provided string.
    ///
    /// ```rust
    /// # use time::UtcOffset;
    /// assert_eq!(UtcOffset::parse("+0200", "%z"), Ok(UtcOffset::hours(2)));
    /// assert_eq!(UtcOffset::parse("-0200", "%z"), Ok(UtcOffset::hours(-2)));
    /// ```
    #[inline(always)]
    pub fn parse(s: impl AsRef<str>, format: impl AsRef<str>) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s.as_ref(), format.as_ref())?)
    }

    /// Given the items already parsed, attempt to create a `UtcOffset`.
    #[inline(always)]
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        items.offset.ok_or(ParseError::InsufficientInformation)
    }
}

impl Display for UtcOffset {
    #[inline]
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
#[cfg(std)]
#[allow(clippy::too_many_lines)]
fn try_local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    cfg_if::cfg_if! {
        if #[cfg(target_family = "unix")] {
            use core::convert::TryInto;
            use standback::mem::MaybeUninit;

            /// Convert the given Unix timestamp to a `libc::tm`. Returns `None`
            /// on any error.
            fn timestamp_to_tm(timestamp: i64) -> Option<libc::tm> {
                extern "C" {
                    fn tzset();
                }

                let timestamp = timestamp.try_into().ok()?;

                let mut tm = MaybeUninit::uninit();

                // Update timezone information from system. `localtime_r` does
                // not do this for us.
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

            let tm = timestamp_to_tm(datetime.timestamp())?;

            // `tm_gmtoff` extension
            #[cfg(not(target_os = "solaris"))]
            {
                tm.tm_gmtoff.try_into().ok().map(UtcOffset::seconds)
            }

            // No `tm_gmtoff` extension
            #[cfg(target_os = "solaris")]
            {
                use crate::Date;
                use core::convert::TryFrom;

                let mut tm = tm;
                if tm.tm_sec == 60 {
                    // Leap seconds are not currently supported.
                    tm.tm_sec = 59;
                }

                let local_timestamp =
                    Date::try_from_yo(1900 + tm.tm_year, u16::try_from(tm.tm_yday).ok()? + 1)
                        .ok()?
                        .try_with_hms(
                            tm.tm_hour.try_into().ok()?,
                            tm.tm_min.try_into().ok()?,
                            tm.tm_sec.try_into().ok()?,
                        )
                        .ok()?
                        .assume_utc()
                        .timestamp();

                (local_timestamp - datetime.timestamp())
                    .try_into()
                    .ok()
                    .map(UtcOffset::seconds)
            }
        } else if #[cfg(target_family = "windows")] {
            use core::convert::TryInto;
            use standback::mem::MaybeUninit;
            use crate::offset;
            use winapi::{
                shared::minwindef::FILETIME,
                um::{
                    minwinbase::SYSTEMTIME,
                    timezoneapi::{SystemTimeToFileTime, SystemTimeToTzSpecificLocalTime},
                },
            };

            /// Convert a `SYSTEMTIME` to a `FILETIME`. Returns `None` if any
            /// error occurred.
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

            /// Convert a `FILETIME` to an `i64`, representing a number of
            /// seconds.
            fn filetime_to_secs(filetime: &FILETIME) -> i64 {
                /// FILETIME represents 100-nanosecond intervals
                const FT_TO_SECS: i64 = 10_000_000;
                ((filetime.dwHighDateTime as i64) << 32 | filetime.dwLowDateTime as i64) /
                    FT_TO_SECS
            }

            /// Convert an `OffsetDateTime` to a `SYSTEMTIME`.
            fn offset_to_systemtime(datetime: OffsetDateTime) -> SYSTEMTIME {
                let (month, day_of_month) = datetime.to_offset(offset!(UTC)).month_day();
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
            let systime_utc = offset_to_systemtime(datetime.to_offset(offset!(UTC)));

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

            diff_secs.try_into().ok().map(UtcOffset::seconds)
        } else if #[cfg(cargo_web)] {
            use stdweb::unstable::TryInto;

            let timestamp_utc = datetime.timestamp();
            let low_bits = (timestamp_utc & 0xFF_FF_FF_FF) as i32;
            let high_bits = (timestamp_utc >> 32) as i32;

            let timezone_offset = js! {
                return
                    new Date(((@{high_bits} << 32) + @{low_bits}) * 1000)
                        .getTimezoneOffset() * -60;
            };

            timezone_offset.try_into().ok().map(UtcOffset::seconds)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::offset;

    #[test]
    fn hours() {
        assert_eq!(UtcOffset::hours(1).as_seconds(), 3_600);
        assert_eq!(UtcOffset::hours(-1).as_seconds(), -3_600);
        assert_eq!(UtcOffset::hours(23).as_seconds(), 82_800);
        assert_eq!(UtcOffset::hours(-23).as_seconds(), -82_800);
    }

    #[test]
    fn directional_hours() {
        assert_eq!(UtcOffset::east_hours(1), offset!(+1));
        assert_eq!(UtcOffset::west_hours(1), offset!(-1));
    }

    #[test]
    fn minutes() {
        assert_eq!(UtcOffset::minutes(1).as_seconds(), 60);
        assert_eq!(UtcOffset::minutes(-1).as_seconds(), -60);
        assert_eq!(UtcOffset::minutes(1_439).as_seconds(), 86_340);
        assert_eq!(UtcOffset::minutes(-1_439).as_seconds(), -86_340);
    }

    #[test]
    fn directional_minutes() {
        assert_eq!(UtcOffset::east_minutes(1), offset!(+0:01));
        assert_eq!(UtcOffset::west_minutes(1), offset!(-0:01));
    }

    #[test]
    fn seconds() {
        assert_eq!(UtcOffset::seconds(1).as_seconds(), 1);
        assert_eq!(UtcOffset::seconds(-1).as_seconds(), -1);
        assert_eq!(UtcOffset::seconds(86_399).as_seconds(), 86_399);
        assert_eq!(UtcOffset::seconds(-86_399).as_seconds(), -86_399);
    }

    #[test]
    fn directional_seconds() {
        assert_eq!(UtcOffset::east_seconds(1), offset!(+0:00:01));
        assert_eq!(UtcOffset::west_seconds(1), offset!(-0:00:01));
    }

    #[test]
    fn as_hours() {
        assert_eq!(offset!(+1).as_hours(), 1);
        assert_eq!(offset!(+0:59).as_hours(), 0);
        assert_eq!(offset!(-1).as_hours(), -1);
        assert_eq!(offset!(-0:59).as_hours(), -0);
    }

    #[test]
    fn as_minutes() {
        assert_eq!(offset!(+1).as_minutes(), 60);
        assert_eq!(offset!(+0:01).as_minutes(), 1);
        assert_eq!(offset!(+0:00:59).as_minutes(), 0);
        assert_eq!(offset!(-1).as_minutes(), -60);
        assert_eq!(offset!(-0:01).as_minutes(), -1);
        assert_eq!(offset!(-0:00:59).as_minutes(), 0);
    }

    #[test]
    fn as_seconds() {
        assert_eq!(offset!(+1).as_seconds(), 3_600);
        assert_eq!(offset!(+0:01).as_seconds(), 60);
        assert_eq!(offset!(+0:00:01).as_seconds(), 1);
        assert_eq!(offset!(-1).as_seconds(), -3_600);
        assert_eq!(offset!(-0:01).as_seconds(), -60);
        assert_eq!(offset!(-0:00:01).as_seconds(), -1);
    }

    #[test]
    fn as_duration() {
        assert_eq!(offset!(+1).as_duration(), 1.hours());
        assert_eq!(offset!(-1).as_duration(), (-1).hours());
    }

    #[test]
    fn utc_is_zero() {
        assert_eq!(UtcOffset::UTC, offset!(+0));
    }

    #[test]
    fn format() {
        assert_eq!(offset!(+1).format("%z"), "+0100");
        assert_eq!(offset!(-1).format("%z"), "-0100");
        assert_eq!(offset!(+0).format("%z"), "+0000");
        // An offset of exactly zero should always have a positive sign.
        assert_ne!(offset!(-0).format("%z"), "-0000");

        assert_eq!(offset!(+0:01).format("%z"), "+0001");
        assert_eq!(offset!(-0:01).format("%z"), "-0001");

        // Seconds are not displayed, but the sign can still change.
        assert_eq!(offset!(+0:00:01).format("%z"), "+0000");
        assert_eq!(offset!(-0:00:01).format("%z"), "-0000");
    }

    #[test]
    fn parse() {
        assert_eq!(UtcOffset::parse("+0100", "%z"), Ok(offset!(+1)));
        assert_eq!(UtcOffset::parse("-0100", "%z"), Ok(offset!(-1)));
        assert_eq!(UtcOffset::parse("+0000", "%z"), Ok(offset!(+0)));
        assert_eq!(UtcOffset::parse("-0000", "%z"), Ok(offset!(+0)));

        assert_eq!(UtcOffset::parse("+0001", "%z"), Ok(offset!(+0:01)));
        assert_eq!(UtcOffset::parse("-0001", "%z"), Ok(offset!(-0:01)));
    }

    #[test]
    fn display() {
        assert_eq!(offset!(UTC).to_string(), "+0");
        assert_eq!(offset!(+0:00:01).to_string(), "+0:00:01");
        assert_eq!(offset!(-0:00:01).to_string(), "-0:00:01");
        assert_eq!(offset!(+1).to_string(), "+1");
        assert_eq!(offset!(-1).to_string(), "-1");
        assert_eq!(offset!(+23:59).to_string(), "+23:59");
        assert_eq!(offset!(-23:59).to_string(), "-23:59");
        assert_eq!(offset!(+23:59:59).to_string(), "+23:59:59");
        assert_eq!(offset!(-23:59:59).to_string(), "-23:59:59");
    }
}
