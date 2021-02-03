//! Parsing implementations for the `UtcOffset` struct.

use crate::{
    format_description::modifier,
    parsing::combinator::{exactly_n_digits_padded, first_string_of, lazy_mut},
};

/// Parse the "hour" component of a `UtcOffset`.
pub(crate) fn parse_offset_hour(input: &mut &str, modifiers: modifier::OffsetHour) -> Option<i8> {
    lazy_mut(|input| {
        let sign = first_string_of(&["-", "+"])(input);
        let hour = exactly_n_digits_padded::<u8>(2, modifiers.padding)(input)?;
        match sign {
            Some("-") => Some(-(hour as i8)),
            None if modifiers.sign_is_mandatory => None,
            _ => Some(hour as i8),
        }
    })(input)
}

/// Parse the "minute" component of a `UtcOffset`.
pub(crate) fn parse_offset_minute(
    input: &mut &str,
    modifiers: modifier::OffsetMinute,
) -> Option<u8> {
    exactly_n_digits_padded(2, modifiers.padding)(input)
}

/// Parse the "second" component of a `UtcOffset`.
pub(crate) fn parse_offset_second(
    input: &mut &str,
    modifiers: modifier::OffsetSecond,
) -> Option<u8> {
    exactly_n_digits_padded(2, modifiers.padding)(input)
}
