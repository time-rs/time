//! Parsing implementations for the `UtcOffset` struct.

use crate::parsing::combinator::{exactly_n_digits, first_string_of, opt};

/// Parse the "hour" component of a `UtcOffset`.
pub(crate) fn parse_offset_hour(input: &mut &str) -> Option<i8> {
    let (sign, hour) = try_parse_all!(
        input,
        opt(first_string_of(&["-", "+"])),
        exactly_n_digits::<i8>(2),
    );

    Some(match sign {
        Some("-") => -hour,
        _ => hour,
    })
}

/// Parse the "minute" component of a `UtcOffset`.
pub(crate) fn parse_offset_minute(input: &mut &str) -> Option<u8> {
    exactly_n_digits(2)(input)
}

/// Parse the "second" component of a `UtcOffset`.
pub(crate) fn parse_offset_second(input: &mut &str) -> Option<u8> {
    exactly_n_digits(2)(input)
}
