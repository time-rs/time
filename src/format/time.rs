//! Formatting helpers for a `Time`.

#![allow(non_snake_case)]

use crate::{format::Padding, Time};
use core::fmt::{self, Formatter};

/// Hour in 24h format (`00`-`23`)
pub(crate) fn fmt_H(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, time.hour())
}

/// Hour in 12h format (`01`-`12`)
pub(crate) fn fmt_I(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, (time.hour() as i8 - 1).rem_euclid(12) + 1)
}

/// Minutes, zero-padded (`00`-`59`)
pub(crate) fn fmt_M(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, time.minute())
}

/// am/pm
pub(crate) fn fmt_p(f: &mut Formatter<'_>, time: Time) -> fmt::Result {
    if time.hour() < 12 {
        f.write_str("am")
    } else {
        f.write_str("pm")
    }
}

/// AM/PM
pub(crate) fn fmt_P(f: &mut Formatter<'_>, time: Time) -> fmt::Result {
    if time.hour() < 12 {
        f.write_str("AM")
    } else {
        f.write_str("PM")
    }
}

/// Seconds, zero-padded (`00`-`59`)
pub(crate) fn fmt_S(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, time.second())
}
