//! Implementations of the low-level parser combinators.

use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
use core::str::FromStr;

use crate::format_description::modifier::Padding;
use crate::parsing::ParsedItem;

/// Marker trait for integers.
pub(crate) trait Integer: FromStr {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}
impl Integer for usize {}
impl Integer for NonZeroU8 {}
impl Integer for NonZeroU16 {}
impl Integer for NonZeroU32 {}
impl Integer for NonZeroU64 {}
impl Integer for NonZeroU128 {}
impl Integer for NonZeroUsize {}

/// Parse a string.
pub(crate) fn string<'a, 'b: 'a>(
    expected: &'b str,
) -> impl Fn(&'a str) -> Option<ParsedItem<'a, &'a str>> {
    move |input| Some(ParsedItem(input.strip_prefix(expected)?, expected))
}

/// Parse a "+" or "-" sign. Returns the ASCII byte representing the sign, if present.
pub(crate) fn sign(input: &str) -> Option<ParsedItem<'_, char>> {
    if let Some(remaining) = input.strip_prefix('-') {
        Some(ParsedItem(remaining, '-'))
    } else {
        let remaining = input.strip_prefix('+')?;
        Some(ParsedItem(remaining, '+'))
    }
}

/// Consume the first matching string, returning its associated value.
pub(crate) fn first_match<'a, 'b: 'a, T: Copy + 'a>(
    mut options: impl Iterator<Item = &'a (&'b str, T)>,
) -> impl FnMut(&'b str) -> Option<ParsedItem<'b, T>> {
    move |input| options.find_map(|&(expected, t)| Some(ParsedItem(string(expected)(input)?.0, t)))
}

/// Consume between `n` and `m` instances of the provided parser.
pub(crate) fn n_to_m<'a, T>(
    n: u8,
    m: u8,
    parser: impl Fn(&'a str) -> Option<ParsedItem<'a, T>>,
) -> impl Fn(&'a str) -> Option<ParsedItem<'a, &'a str>> {
    debug_assert!(m >= n);
    move |mut input| {
        // We need to keep this to determine the total length eventually consumed.
        let orig_input = input;

        // Mandatory
        for _ in 0..n {
            input = parser(input)?.0;
        }

        // Optional
        for _ in n..m {
            match parser(input) {
                Some(parsed) => input = parsed.0,
                None => break,
            }
        }

        Some(ParsedItem(
            input,
            &orig_input[..(orig_input.len() - input.len())],
        ))
    }
}

/// Consume exactly `n` instances of the provided parser.
pub(crate) fn exactly_n<'a, T>(
    n: u8,
    parser: impl Fn(&'a str) -> Option<ParsedItem<'a, T>>,
) -> impl Fn(&'a str) -> Option<ParsedItem<'a, &'a str>> {
    n_to_m(n, n, parser)
}

/// Consume between `n` and `m` digits, returning the numerical value.
pub(crate) fn n_to_m_digits<'a, T: Integer>(
    n: u8,
    m: u8,
) -> impl Fn(&'a str) -> Option<ParsedItem<'a, T>> {
    debug_assert!(m >= n);
    move |input| n_to_m(n, m, any_digit)(input)?.flat_map(|value| value.parse().ok())
}

/// Consume exactly `n` digits, returning the numerical value.
pub(crate) fn exactly_n_digits_padded<'a, T: Integer>(
    n: u8,
    padding: Padding,
) -> impl Fn(&'a str) -> Option<ParsedItem<'a, T>> {
    n_to_m_digits_padded(n, n, padding)
}

/// Consume between `n` and `m` digits, returning the numerical value.
pub(crate) fn n_to_m_digits_padded<'a, T: Integer>(
    n: u8,
    m: u8,
    padding: Padding,
) -> impl Fn(&'a str) -> Option<ParsedItem<'a, T>> {
    debug_assert!(m >= n);
    move |input| match padding {
        Padding::None => return n_to_m_digits(1, m)(input),
        Padding::Space => {
            let ParsedItem(input, value) = n_to_m(0, n - 1, ascii_char(b' '))(input)?;
            let pad_width = value.len() as u8;
            n_to_m_digits(n - pad_width, m - pad_width)(input)
        }
        Padding::Zero => return n_to_m_digits(n, m)(input),
    }
}

/// Consume exactly one digit.
pub(crate) fn any_digit(input: &str) -> Option<ParsedItem<'_, u8>> {
    match input.as_bytes() {
        [c, ..] if c.is_ascii_digit() => Some(ParsedItem(&input[1..], *c)),
        _ => None,
    }
}

/// Consume exactly one of the provided ASCII characters.
pub(crate) fn ascii_char(char: u8) -> impl Fn(&str) -> Option<ParsedItem<'_, ()>> {
    move |input| match input.as_bytes() {
        [c, ..] if *c == char => Some(ParsedItem(&input[1..], ())),
        _ => None,
    }
}

/// Optionally consume an input with a given parser.
pub(crate) fn opt<'a, T>(
    parser: impl Fn(&'a str) -> Option<ParsedItem<'a, T>>,
) -> impl Fn(&'a str) -> ParsedItem<'a, Option<T>> {
    move |input| match parser(input) {
        Some(value) => value.map(Some),
        None => ParsedItem(input, None),
    }
}
