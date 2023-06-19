//! Get the system's UTC offset on Unix.

use std::io;
use std::path::Path;

use once_cell::sync::OnceCell;
use tz::{TimeZone, TzError};

use crate::{OffsetDateTime, UtcOffset};

static TZ: OnceCell<TimeZone> = OnceCell::new();

/// Try to obtain a [`tz::TimeZone`] instance based on the current environment or system
/// configuration.
fn try_get_timezone() -> Result<TimeZone, TzError> {
    if let Ok(var) = std::env::var("TZ") {
        return TimeZone::from_posix_tz(&var);
    }

    // The GNU libc by default uses either `/etc/localtime` or `/usr/local/etc/localtime` depending
    // on how it was configured, though there doesn't seem to be any way to detect which one it
    // would use.
    //
    // https://www.gnu.org/software/libc/manual/html_node/TZ-Variable.html#index-localtime-1
    for path in ["/etc/localtime", "/usr/local/etc/localtime"] {
        if Path::new(path).exists() {
            return TimeZone::from_posix_tz(&format!(":{path}"));
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound).into())
}

/// Obtain the system's UTC offset.
pub(super) fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    let tz = TZ.get_or_try_init(try_get_timezone).ok()?;

    match tz.find_local_time_type(datetime.unix_timestamp()) {
        Ok(ltt) => UtcOffset::from_whole_seconds(ltt.ut_offset()).ok(),
        Err(_) => None,
    }
}
