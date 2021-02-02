//! Parsing implementations for the `Time` struct.

use crate::parsing::combinator::{any_char, exactly_n_digits, n_to_m, pred};

/// Indicate whether the hour is "am" or "pm".
#[derive(Debug)]
pub(crate) enum Period {
    #[allow(clippy::missing_docs_in_private_items)]
    Am,
    #[allow(clippy::missing_docs_in_private_items)]
    Pm,
}

/// Parse the "hour" component of a `Time`.
pub(crate) fn parse_hour(input: &mut &str) -> Option<u8> {
    exactly_n_digits(2)(input)
}

/// Parse the "minute" component of a `Time`.
pub(crate) fn parse_minute(input: &mut &str) -> Option<u8> {
    exactly_n_digits(2)(input)
}

/// Parse the "second" component of a `Time`.
pub(crate) fn parse_second(input: &mut &str) -> Option<u8> {
    exactly_n_digits(2)(input)
}

/// Parse the "period" component of a `Time`. Required if the hour is on a 12-hour clock.
pub(crate) fn parse_period(input: &mut &str) -> Option<Period> {
    first_string_of_map!(
        "am" => Period::Am,
        "pm" => Period::Pm,
    )(input)
}

/// Parse the "subsecond" component of a `Time`.
pub(crate) fn parse_subsecond(input: &mut &str) -> Option<u32> {
    let raw_digits = n_to_m(1, 9, pred(any_char, char::is_ascii_digit))(input)?;
    let raw_num: u32 = raw_digits.parse().ok()?;
    let adjustment_factor = 10_u32.pow(9 - raw_digits.len() as u32);

    Some(raw_num * adjustment_factor)
}
