//! Formatting helpers for a `Time`.

#![allow(non_snake_case)]

use crate::{
    format::{
        parse::{
            try_consume_exact_digits_in_range, try_consume_first_match,
            AmPm::{AM, PM},
        },
        Padding, ParseError, ParseResult, ParsedItems,
    },
    shim::*,
    Time,
};
use core::{
    fmt::{self, Formatter},
    num::NonZeroU8,
};

/// Hour in 24h format (`00`-`23`)
#[inline(always)]
pub(crate) fn fmt_H(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, time.hour())
}

/// Hour in 24h format (`00`-`23`)
#[inline(always)]
pub(crate) fn parse_H(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.hour_24 =
        try_consume_exact_digits_in_range(s, 2, 0..24, padding.default_to(Padding::Zero))
            .ok_or(ParseError::InvalidHour)?
            .into();
    Ok(())
}

/// Hour in 12h format (`01`-`12`)
#[inline(always)]
pub(crate) fn fmt_I(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(
        f,
        padding(Zero),
        2,
        (time.hour() as i8 - 1).rem_euclid_shim(12) + 1
    )
}

/// Hour in 12h format (`01`-`12`)
#[inline(always)]
pub(crate) fn parse_I(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.hour_12 =
        try_consume_exact_digits_in_range(s, 2, 1..=12, padding.default_to(Padding::Zero))
            .map(NonZeroU8::new)
            .ok_or(ParseError::InvalidHour)?;
    Ok(())
}

/// Minutes, zero-padded (`00`-`59`)
#[inline(always)]
pub(crate) fn fmt_M(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, time.minute())
}

/// Minutes, zero-added (`00`-`59`)
#[inline(always)]
pub(crate) fn parse_M(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.minute =
        try_consume_exact_digits_in_range(s, 2, 0..60, padding.default_to(Padding::Zero))
            .ok_or(ParseError::InvalidMinute)?
            .into();
    Ok(())
}

/// am/pm
#[inline(always)]
pub(crate) fn fmt_p(f: &mut Formatter<'_>, time: Time) -> fmt::Result {
    if time.hour() < 12 {
        f.write_str("am")
    } else {
        f.write_str("pm")
    }
}

/// am/pm
#[inline(always)]
pub(crate) fn parse_p(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.am_pm = try_consume_first_match(s, [("am", AM), ("pm", PM)].iter().cloned())
        .ok_or(ParseError::InvalidAmPm)?
        .into();
    Ok(())
}

/// AM/PM
#[inline(always)]
pub(crate) fn fmt_P(f: &mut Formatter<'_>, time: Time) -> fmt::Result {
    if time.hour() < 12 {
        f.write_str("AM")
    } else {
        f.write_str("PM")
    }
}

/// AM/PM
#[inline(always)]
pub(crate) fn parse_P(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.am_pm = try_consume_first_match(s, [("AM", AM), ("PM", PM)].iter().cloned())
        .ok_or(ParseError::InvalidAmPm)?
        .into();
    Ok(())
}

/// Seconds, zero-padded (`00`-`59`)
#[inline(always)]
pub(crate) fn fmt_S(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, time.second())
}

/// Seconds, zero-added (`00`-`59`)
#[inline(always)]
pub(crate) fn parse_S(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.second =
        try_consume_exact_digits_in_range(s, 2, 0..60, padding.default_to(Padding::Zero))
            .ok_or(ParseError::InvalidMinute)?
            .into();
    Ok(())
}
