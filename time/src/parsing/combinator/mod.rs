//! Implementations of the low-level parser combinators.

pub(crate) mod rfc;

use crate::format_description::modifier::Padding;
use crate::parsing::ParsedItem;
use crate::parsing::shim::Integer;

/// The sign of a number.
#[allow(
    clippy::missing_docs_in_private_items,
    reason = "self-explanatory variants"
)]
#[derive(Debug)]
pub(crate) enum Sign {
    Negative,
    Positive,
}

/// Parse a "+" or "-" sign.
#[inline]
pub(crate) const fn sign(input: &[u8]) -> Option<ParsedItem<'_, Sign>> {
    match input {
        [b'-', remaining @ ..] => Some(ParsedItem(remaining, Sign::Negative)),
        [b'+', remaining @ ..] => Some(ParsedItem(remaining, Sign::Positive)),
        _ => None,
    }
}

/// Consume the first matching item, returning its associated value.
#[inline]
pub(crate) fn first_match<'a, T, I>(
    options: I,
    case_sensitive: bool,
) -> impl for<'b> FnMut(&'b [u8]) -> Option<ParsedItem<'b, T>>
where
    I: IntoIterator<Item = (&'a [u8], T)>,
{
    let mut options = options.into_iter();
    move |input| {
        if case_sensitive {
            options.find_map(|(expected, t)| Some(ParsedItem(input.strip_prefix(expected)?, t)))
        } else {
            options.find_map(|(expected, t)| {
                let n = expected.len();
                if n <= input.len() {
                    let (head, tail) = input.split_at(n);
                    if head.eq_ignore_ascii_case(expected) {
                        return Some(ParsedItem(tail, t));
                    }
                }
                None
            })
        }
    }
}

/// Consume zero or more instances of the provided parser. The parser must return the unit value.
#[inline]
pub(crate) fn zero_or_more<P>(parser: P) -> impl for<'a> FnMut(&'a [u8]) -> ParsedItem<'a, ()>
where
    P: for<'a> Fn(&'a [u8]) -> Option<ParsedItem<'a, ()>>,
{
    move |mut input| {
        while let Some(remaining) = parser(input) {
            input = remaining.into_inner();
        }
        ParsedItem(input, ())
    }
}

/// Consume one of or more instances of the provided parser. The parser must produce the unit value.
#[inline]
pub(crate) fn one_or_more<P>(parser: P) -> impl for<'a> Fn(&'a [u8]) -> Option<ParsedItem<'a, ()>>
where
    P: for<'a> Fn(&'a [u8]) -> Option<ParsedItem<'a, ()>>,
{
    move |mut input| {
        input = parser(input)?.into_inner();
        while let Some(remaining) = parser(input) {
            input = remaining.into_inner();
        }
        Some(ParsedItem(input, ()))
    }
}

/// Consume between `n` and `m` digits, returning the numerical value.
#[inline]
pub(crate) fn n_to_m_digits<const N: u8, const M: u8, T>(
    mut input: &[u8],
) -> Option<ParsedItem<'_, T>>
where
    T: Integer,
{
    const {
        assert!(N > 0);
        assert!(M >= N);
    }

    let mut value = T::ZERO;

    // Mandatory
    for i in 0..N {
        let digit;
        ParsedItem(input, digit) = any_digit(input)?;

        if i != T::MAX_NUM_DIGITS - 1 {
            value = value.push_digit(digit - b'0');
        } else {
            value = value.checked_push_digit(digit - b'0')?;
        }
    }

    // Optional
    for i in N..M {
        let Some(ParsedItem(new_input, digit)) = any_digit(input) else {
            break;
        };
        input = new_input;

        if i != T::MAX_NUM_DIGITS - 1 {
            value = value.push_digit(digit - b'0');
        } else {
            value = value.checked_push_digit(digit - b'0')?;
        }
    }

    Some(ParsedItem(input, value))
}

/// Consume one or two digits, returning the numerical value.
#[inline]
pub(crate) fn one_or_two_digits(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
    match input {
        [a @ b'0'..=b'9', b @ b'0'..=b'9', remaining @ ..] => {
            let a = *a - b'0';
            let b = *b - b'0';
            Some(ParsedItem(remaining, a * 10 + b))
        }
        [a @ b'0'..=b'9', remaining @ ..] => {
            let a = *a - b'0';
            Some(ParsedItem(remaining, a))
        }
        _ => None,
    }
}

/// Parse an exact number of digits without padding.
#[derive(Debug)]
pub(crate) struct ExactlyNDigits<const N: u8>;

impl ExactlyNDigits<1> {
    /// Consume exactly one digit.
    #[inline]
    pub(crate) const fn parse(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
        match input {
            [a @ b'0'..=b'9', remaining @ ..] => Some(ParsedItem(remaining, *a - b'0')),
            _ => None,
        }
    }
}

impl ExactlyNDigits<2> {
    /// Consume exactly two digits.
    #[inline]
    pub(crate) const fn parse(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
        match input {
            [a @ b'0'..=b'9', b @ b'0'..=b'9', remaining @ ..] => {
                let a = *a - b'0';
                let b = *b - b'0';
                Some(ParsedItem(remaining, a * 10 + b))
            }
            _ => None,
        }
    }
}

impl ExactlyNDigits<3> {
    /// Consume exactly three digits.
    #[inline]
    pub(crate) const fn parse(input: &[u8]) -> Option<ParsedItem<'_, u16>> {
        match input {
            [
                a @ b'0'..=b'9',
                b @ b'0'..=b'9',
                c @ b'0'..=b'9',
                remaining @ ..,
            ] => {
                let a = (*a - b'0') as u16;
                let b = (*b - b'0') as u16;
                let c = (*c - b'0') as u16;
                Some(ParsedItem(remaining, a * 100 + b * 10 + c))
            }
            _ => None,
        }
    }
}

impl ExactlyNDigits<4> {
    /// Consume exactly four digits.
    #[inline]
    pub(crate) const fn parse(input: &[u8]) -> Option<ParsedItem<'_, u16>> {
        match input {
            [
                a @ b'0'..=b'9',
                b @ b'0'..=b'9',
                c @ b'0'..=b'9',
                d @ b'0'..=b'9',
                remaining @ ..,
            ] => {
                let a = (*a - b'0') as u16;
                let b = (*b - b'0') as u16;
                let c = (*c - b'0') as u16;
                let d = (*d - b'0') as u16;
                Some(ParsedItem(remaining, a * 1000 + b * 100 + c * 10 + d))
            }
            _ => None,
        }
    }
}

impl ExactlyNDigits<5> {
    /// Consume exactly five digits.
    #[inline]
    pub(crate) const fn parse(input: &[u8]) -> Option<ParsedItem<'_, u32>> {
        match input {
            [
                a @ b'0'..=b'9',
                b @ b'0'..=b'9',
                c @ b'0'..=b'9',
                d @ b'0'..=b'9',
                e @ b'0'..=b'9',
                remaining @ ..,
            ] => {
                let a = (*a - b'0') as u32;
                let b = (*b - b'0') as u32;
                let c = (*c - b'0') as u32;
                let d = (*d - b'0') as u32;
                let e = (*e - b'0') as u32;
                Some(ParsedItem(
                    remaining,
                    a * 10000 + b * 1000 + c * 100 + d * 10 + e,
                ))
            }
            _ => None,
        }
    }
}

impl ExactlyNDigits<6> {
    /// Consume exactly six digits.
    #[inline]
    pub(crate) const fn parse(input: &[u8]) -> Option<ParsedItem<'_, u32>> {
        match input {
            [
                a @ b'0'..=b'9',
                b @ b'0'..=b'9',
                c @ b'0'..=b'9',
                d @ b'0'..=b'9',
                e @ b'0'..=b'9',
                f @ b'0'..=b'9',
                remaining @ ..,
            ] => {
                let a = (*a - b'0') as u32;
                let b = (*b - b'0') as u32;
                let c = (*c - b'0') as u32;
                let d = (*d - b'0') as u32;
                let e = (*e - b'0') as u32;
                let f = (*f - b'0') as u32;
                Some(ParsedItem(
                    remaining,
                    a * 100000 + b * 10000 + c * 1000 + d * 100 + e * 10 + f,
                ))
            }
            _ => None,
        }
    }
}

impl ExactlyNDigits<7> {
    /// Consume exactly seven digits.
    #[inline]
    pub(crate) const fn parse(input: &[u8]) -> Option<ParsedItem<'_, u32>> {
        match input {
            [
                a @ b'0'..=b'9',
                b @ b'0'..=b'9',
                c @ b'0'..=b'9',
                d @ b'0'..=b'9',
                e @ b'0'..=b'9',
                f @ b'0'..=b'9',
                g @ b'0'..=b'9',
                remaining @ ..,
            ] => {
                let a = (*a - b'0') as u32;
                let b = (*b - b'0') as u32;
                let c = (*c - b'0') as u32;
                let d = (*d - b'0') as u32;
                let e = (*e - b'0') as u32;
                let f = (*f - b'0') as u32;
                let g = (*g - b'0') as u32;
                Some(ParsedItem(
                    remaining,
                    a * 1_000_000 + b * 100_000 + c * 10_000 + d * 1_000 + e * 100 + f * 10 + g,
                ))
            }
            _ => None,
        }
    }
}

impl ExactlyNDigits<8> {
    /// Consume exactly eight digits.
    #[inline]
    pub(crate) const fn parse(input: &[u8]) -> Option<ParsedItem<'_, u32>> {
        match input {
            [
                a @ b'0'..=b'9',
                b @ b'0'..=b'9',
                c @ b'0'..=b'9',
                d @ b'0'..=b'9',
                e @ b'0'..=b'9',
                f @ b'0'..=b'9',
                g @ b'0'..=b'9',
                h @ b'0'..=b'9',
                remaining @ ..,
            ] => {
                let a = (*a - b'0') as u32;
                let b = (*b - b'0') as u32;
                let c = (*c - b'0') as u32;
                let d = (*d - b'0') as u32;
                let e = (*e - b'0') as u32;
                let f = (*f - b'0') as u32;
                let g = (*g - b'0') as u32;
                let h = (*h - b'0') as u32;
                Some(ParsedItem(
                    remaining,
                    a * 10_000_000
                        + b * 1_000_000
                        + c * 100_000
                        + d * 10_000
                        + e * 1_000
                        + f * 100
                        + g * 10
                        + h,
                ))
            }
            _ => None,
        }
    }
}

impl ExactlyNDigits<9> {
    /// Consume exactly nine digits.
    #[inline]
    pub(crate) const fn parse(input: &[u8]) -> Option<ParsedItem<'_, u32>> {
        match input {
            [
                a @ b'0'..=b'9',
                b @ b'0'..=b'9',
                c @ b'0'..=b'9',
                d @ b'0'..=b'9',
                e @ b'0'..=b'9',
                f @ b'0'..=b'9',
                g @ b'0'..=b'9',
                h @ b'0'..=b'9',
                i @ b'0'..=b'9',
                remaining @ ..,
            ] => {
                let a = (*a - b'0') as u32;
                let b = (*b - b'0') as u32;
                let c = (*c - b'0') as u32;
                let d = (*d - b'0') as u32;
                let e = (*e - b'0') as u32;
                let f = (*f - b'0') as u32;
                let g = (*g - b'0') as u32;
                let h = (*h - b'0') as u32;
                let i = (*i - b'0') as u32;
                Some(ParsedItem(
                    remaining,
                    a * 100_000_000
                        + b * 10_000_000
                        + c * 1_000_000
                        + d * 100_000
                        + e * 10_000
                        + f * 1_000
                        + g * 100
                        + h * 10
                        + i,
                ))
            }
            _ => None,
        }
    }
}

/// Consume exactly `n` digits, returning the numerical value.
pub(crate) fn exactly_n_digits_padded<const N: u8, T>(
    padding: Padding,
) -> impl for<'a> Fn(&'a [u8]) -> Option<ParsedItem<'a, T>>
where
    T: Integer,
{
    n_to_m_digits_padded::<N, N, _>(padding)
}

/// Consume between `n` and `m` digits, returning the numerical value.
pub(crate) fn n_to_m_digits_padded<const N: u8, const M: u8, T>(
    padding: Padding,
) -> impl for<'a> Fn(&'a [u8]) -> Option<ParsedItem<'a, T>>
where
    T: Integer,
{
    const {
        assert!(N > 0);
        assert!(M >= N);
    }

    move |mut input| match padding {
        Padding::None => n_to_m_digits::<1, M, _>(input),
        Padding::Space => {
            let mut value = T::ZERO;

            // Consume the padding.
            let mut pad_width = 0;
            for _ in 0..(N - 1) {
                match ascii_char::<b' '>(input) {
                    Some(parsed) => {
                        pad_width += 1;
                        input = parsed.0;
                    }
                    None => break,
                }
            }

            // Mandatory
            for i in 0..(N - pad_width) {
                let digit;
                ParsedItem(input, digit) = any_digit(input)?;

                value = if i != T::MAX_NUM_DIGITS - 1 {
                    value.push_digit(digit - b'0')
                } else {
                    value.checked_push_digit(digit - b'0')?
                };
            }

            // Optional
            for i in N..M {
                let Some(ParsedItem(new_input, digit)) = any_digit(input) else {
                    break;
                };
                input = new_input;

                value = if i - pad_width != T::MAX_NUM_DIGITS - 1 {
                    value.push_digit(digit - b'0')
                } else {
                    value.checked_push_digit(digit - b'0')?
                };
            }

            Some(ParsedItem(input, value))
        }
        Padding::Zero => n_to_m_digits::<N, M, _>(input),
    }
}

/// Consume exactly one digit.
#[inline]
pub(crate) const fn any_digit(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
    match input {
        [c @ b'0'..=b'9', remaining @ ..] => Some(ParsedItem(remaining, *c)),
        _ => None,
    }
}

/// Consume exactly one of the provided ASCII characters.
#[inline]
pub(crate) fn ascii_char<const CHAR: u8>(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    const {
        assert!(CHAR.is_ascii_graphic() || CHAR.is_ascii_whitespace());
    }
    match input {
        [c, remaining @ ..] if *c == CHAR => Some(ParsedItem(remaining, ())),
        _ => None,
    }
}

/// Consume exactly one of the provided ASCII characters, case-insensitive.
#[inline]
pub(crate) fn ascii_char_ignore_case<const CHAR: u8>(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    const {
        assert!(CHAR.is_ascii_graphic() || CHAR.is_ascii_whitespace());
    }
    match input {
        [c, remaining @ ..] if c.eq_ignore_ascii_case(&CHAR) => Some(ParsedItem(remaining, ())),
        _ => None,
    }
}

/// Optionally consume an input with a given parser.
#[inline]
pub(crate) fn opt<T>(
    parser: impl for<'a> Fn(&'a [u8]) -> Option<ParsedItem<'a, T>>,
) -> impl for<'a> Fn(&'a [u8]) -> ParsedItem<'a, Option<T>> {
    move |input| match parser(input) {
        Some(value) => value.map(Some),
        None => ParsedItem(input, None),
    }
}
