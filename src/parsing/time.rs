//! Parsing implementations for the `Time` struct.

use crate::{
    format_description::modifier,
    parsing::combinator::{any_digit, exactly_n, exactly_n_digits_padded, first_match, n_to_m},
};

/// Indicate whether the hour is "am" or "pm".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Period {
    #[allow(clippy::missing_docs_in_private_items)]
    Am,
    #[allow(clippy::missing_docs_in_private_items)]
    Pm,
}

/// Parse the "hour" component of a `Time`.
pub(crate) fn parse_hour(input: &mut &str, modifiers: modifier::Hour) -> Option<u8> {
    exactly_n_digits_padded(2, modifiers.padding)(input)
}

/// Parse the "minute" component of a `Time`.
pub(crate) fn parse_minute(input: &mut &str, modifiers: modifier::Minute) -> Option<u8> {
    exactly_n_digits_padded(2, modifiers.padding)(input)
}

/// Parse the "second" component of a `Time`.
pub(crate) fn parse_second(input: &mut &str, modifiers: modifier::Second) -> Option<u8> {
    exactly_n_digits_padded(2, modifiers.padding)(input)
}

/// Parse the "period" component of a `Time`. Required if the hour is on a 12-hour clock.
pub(crate) fn parse_period(input: &mut &str, modifiers: modifier::Period) -> Option<Period> {
    first_match(if modifiers.is_uppercase {
        [("AM", Period::Am), ("PM", Period::Pm)].iter()
    } else {
        [("am", Period::Am), ("pm", Period::Pm)].iter()
    })(input)
}

/// Parse the "subsecond" component of a `Time`.
pub(crate) fn parse_subsecond(input: &mut &str, modifiers: modifier::Subsecond) -> Option<u32> {
    let raw_digits = match modifiers.digits {
        modifier::SubsecondDigits::One => exactly_n(1, any_digit)(input),
        modifier::SubsecondDigits::Two => exactly_n(2, any_digit)(input),
        modifier::SubsecondDigits::Three => exactly_n(3, any_digit)(input),
        modifier::SubsecondDigits::Four => exactly_n(4, any_digit)(input),
        modifier::SubsecondDigits::Five => exactly_n(5, any_digit)(input),
        modifier::SubsecondDigits::Six => exactly_n(6, any_digit)(input),
        modifier::SubsecondDigits::Seven => exactly_n(7, any_digit)(input),
        modifier::SubsecondDigits::Eight => exactly_n(8, any_digit)(input),
        modifier::SubsecondDigits::Nine => exactly_n(9, any_digit)(input),
        modifier::SubsecondDigits::OneOrMore => n_to_m(1, 9, any_digit)(input),
    }?;
    let raw_num: u32 = raw_digits.parse().ok()?;
    let adjustment_factor = 10_u32.pow(9 - raw_digits.len() as u32);

    Some(raw_num * adjustment_factor)
}
