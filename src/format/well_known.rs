//! Formatting and parsing for well-known formats (typically RFCs).

use crate::{
    format::{
        date,
        parse::{
            try_consume_char, try_consume_char_case_insensitive, try_consume_exact_digits,
            try_consume_exact_digits_in_range, try_consume_first_match,
        },
        time, Padding, ParseResult, ParsedItems,
    },
    internal_prelude::*,
};
use core::fmt::{self, Formatter};

/// The format as specified by RFC3339.
pub(crate) mod rfc3339 {
    use super::*;

    /// Format `df` according to the RFC3339 specification.
    #[inline]
    pub(crate) fn fmt(df: &DeferredFormat, f: &mut Formatter<'_>) -> fmt::Result {
        // If we're using RFC3339, all three components must be present.
        // This will be enforced with typestate when Rust gains sufficient
        // capabilities (namely proper sealed traits and/or function overloading).
        #[allow(clippy::option_unwrap_used)]
        let date = df.date().unwrap();
        #[allow(clippy::option_unwrap_used)]
        let time = df.time().unwrap();
        #[allow(clippy::option_unwrap_used)]
        let offset = df.offset().unwrap();

        date::fmt_Y(f, date, Padding::Zero)?;
        f.write_str("-")?;
        date::fmt_m(f, date, Padding::Zero)?;
        f.write_str("-")?;
        date::fmt_d(f, date, Padding::Zero)?;
        f.write_str("T")?;
        time::fmt_H(f, time, Padding::Zero)?;
        f.write_str(":")?;
        time::fmt_M(f, time, Padding::Zero)?;
        f.write_str(":")?;
        time::fmt_S(f, time, Padding::Zero)?;
        write!(
            f,
            "{:+03}:{:02}",
            offset.as_hours(),
            offset.as_minutes().rem_euclid(60)
        )?;

        Ok(())
    }

    /// Parse `s` as specified by RFC3339.
    #[inline]
    pub(crate) fn parse(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
        items.year = try_consume_exact_digits::<i32>(s, 4, Padding::None)
            .ok_or(ParseError::InvalidYear)?
            .into();
        try_consume_char(s, '-')?;
        date::parse_m(items, s, Padding::Zero)?;
        try_consume_char(s, '-')?;
        date::parse_d(items, s, Padding::Zero)?;
        try_consume_char_case_insensitive(s, 'T')?;
        time::parse_H(items, s, Padding::Zero)?;
        try_consume_char(s, ':')?;
        time::parse_M(items, s, Padding::Zero)?;
        try_consume_char(s, ':')?;
        time::parse_S(items, s, Padding::Zero)?;

        if try_consume_char(s, '.').is_ok() {
            let num_digits = s.chars().take_while(char::is_ascii_digit).count();
            if num_digits == 0 {
                return Err(ParseError::InvalidNanosecond);
            }
            let num_digits_used = core::cmp::min(num_digits, 9);

            let nanos_raw: String = s.chars().take(num_digits_used).collect();
            // At most 9 decimal digits will always fit in a u32.
            // `num_digits_used` is at most 9, which can safely be cast.
            #[allow(clippy::result_unwrap_used)]
            let nanos = nanos_raw.parse::<u32>().unwrap() * 10_u32.pow(9 - num_digits_used as u32);
            items.nanosecond = Some(nanos);
            *s = &s[num_digits..];
        }

        if try_consume_char_case_insensitive(s, 'Z').is_ok() {
            items.offset = Some(UtcOffset::UTC);
        } else {
            let offset_sign =
                match try_consume_first_match(s, [("+", 1), ("-", -1)].iter().cloned()) {
                    Some(sign) => sign,
                    None => {
                        return Err(match s.chars().next() {
                            Some(actual) => ParseError::UnexpectedCharacter {
                                actual,
                                expected: '+',
                            },
                            None => ParseError::UnexpectedEndOfString,
                        })
                    }
                };
            let offset_hour = try_consume_exact_digits_in_range(s, 2, 0..=23, Padding::Zero)
                .ok_or(ParseError::InvalidOffset)?;
            try_consume_char(s, ':')?;
            let offset_minute = try_consume_exact_digits_in_range(s, 2, 0..=59, Padding::Zero)
                .ok_or(ParseError::InvalidOffset)?;
            items.offset = Some(UtcOffset::seconds(
                offset_sign * (offset_hour * 60 + offset_minute),
            ));
        }

        Ok(())
    }
}
