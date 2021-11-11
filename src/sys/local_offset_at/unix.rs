//! Get the system's UTC offset on Unix.

#[cfg(unsound_local_offset)]
use core::convert::TryInto;
#[cfg(unsound_local_offset)]
use core::mem::MaybeUninit;

use crate::{OffsetDateTime, UtcOffset};

/// Obtain the system's UTC offset.
// See #293 for details.
#[cfg(not(unsound_local_offset))]
#[allow(clippy::missing_const_for_fn)]
pub(super) fn local_offset_at(_datetime: OffsetDateTime) -> Option<UtcOffset> {
    None
}

/// Convert the given Unix timestamp to a `libc::tm`. Returns `None` on any error.
#[cfg(unsound_local_offset)]
fn timestamp_to_tm(timestamp: i64) -> Option<libc::tm> {
    extern "C" {
        #[cfg_attr(target_os = "netbsd", link_name = "__tzset50")]
        fn tzset();
    }

    // The exact type of `timestamp` beforehand can vary, so this conversion is necessary.
    #[allow(clippy::useless_conversion)]
    let timestamp = timestamp.try_into().ok()?;

    let mut tm = MaybeUninit::uninit();

    // Update timezone information from system. `localtime_r` does not do this for us.
    //
    // Safety: tzset is thread-safe.
    #[allow(unsafe_code)]
    unsafe {
        tzset();
    }

    // Safety: We are calling a system API, which mutates the `tm` variable. If a null
    // pointer is returned, an error occurred.
    #[allow(unsafe_code)]
    let tm_ptr = unsafe { libc::localtime_r(&timestamp, tm.as_mut_ptr()) };

    if tm_ptr.is_null() {
        None
    } else {
        // Safety: The value was initialized, as we no longer have a null pointer.
        #[allow(unsafe_code)]
        {
            Some(unsafe { tm.assume_init() })
        }
    }
}

/// Convert a `libc::tm` to a `UtcOffset`. Returns `None` on any error.
// `tm_gmtoff` extension
#[cfg(unsound_local_offset)]
#[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
fn tm_to_offset(tm: libc::tm) -> Option<UtcOffset> {
    let seconds: i32 = tm.tm_gmtoff.try_into().ok()?;
    UtcOffset::from_hms(
        (seconds / 3_600) as _,
        ((seconds / 60) % 60) as _,
        (seconds % 60) as _,
    )
    .ok()
}

/// Convert a `libc::tm` to a `UtcOffset`. Returns `None` on any error.
#[cfg(unsound_local_offset)]
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
fn tm_to_offset(tm: libc::tm) -> Option<UtcOffset> {
    use core::convert::TryFrom;

    use crate::Date;

    let mut tm = tm;
    if tm.tm_sec == 60 {
        // Leap seconds are not currently supported.
        tm.tm_sec = 59;
    }

    let local_timestamp =
        Date::from_ordinal_date(1900 + tm.tm_year, u16::try_from(tm.tm_yday).ok()? + 1)
            .ok()?
            .with_hms(
                tm.tm_hour.try_into().ok()?,
                tm.tm_min.try_into().ok()?,
                tm.tm_sec.try_into().ok()?,
            )
            .ok()?
            .assume_utc()
            .unix_timestamp();

    let diff_secs: i32 = (local_timestamp - datetime.unix_timestamp())
        .try_into()
        .ok()?;

    UtcOffset::from_hms(
        (diff_secs / 3_600) as _,
        ((diff_secs / 60) % 60) as _,
        (diff_secs % 60) as _,
    )
    .ok()
}

/// Obtain the system's UTC offset.
#[cfg(unsound_local_offset)]
pub(super) fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    tm_to_offset(timestamp_to_tm(datetime.unix_timestamp())?)
}
