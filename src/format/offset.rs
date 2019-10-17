//! Formatting helpers for a `UtcOffset`.

#![allow(non_snake_case)]

use crate::UtcOffset;
use core::fmt::{self, Formatter};

/// UTC offset
pub(crate) fn fmt_z(f: &mut Formatter<'_>, offset: UtcOffset) -> fmt::Result {
    let offset = offset.as_duration();

    write!(
        f,
        "{:+03}{:02}",
        offset.whole_hours(),
        offset.whole_minutes() - 60 * offset.whole_hours()
    )
}
