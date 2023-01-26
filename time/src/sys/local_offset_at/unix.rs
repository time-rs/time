//! Get the system's UTC offset on Unix.

use libtz::Timezone;

use crate::{OffsetDateTime, UtcOffset};

/// Obtain the system's UTC offset.
pub(super) fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    let tz = Timezone::default().ok()?;
    let tm = tz.localtime(datetime.unix_timestamp()).ok()?;
    let seconds = tm.tm_gmtoff.try_into().ok()?;
    UtcOffset::from_whole_seconds(seconds).ok()
}
