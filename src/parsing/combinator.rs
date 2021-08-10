//! Implementations of the low-level parser combinators.

use crate::format_description::modifier::Padding;
use crate::parsing::shim::{Integer, IntegerParseBytes};
use crate::parsing::ParsedItem;

/// Parse a "+" or "-" sign. Returns the ASCII byte representing the sign, if present.
pub(crate) const fn sign(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
    match input {
        [b'-', remaining @ ..] => Some(ParsedItem(remaining, b'-')),
        [b'+', remaining @ ..] => Some(ParsedItem(remaining, b'+')),
        _ => None,
    }
}

/// Consume the first matching item, returning its associated value.
// Due to compiler limitations, we must accept a `&str` instead of `&[u8]`.
pub(crate) fn first_match<'a, 'b: 'a, T: Copy + 'a>(
    mut options: impl Iterator<Item = &'a (&'b str, T)>,
    case_sensitive: bool,
) -> impl FnMut(&'b [u8]) -> Option<ParsedItem<'b, T>> {
    move |input| {
        options.find_map(|&(expected, t)| {
            if case_sensitive {
                Some(ParsedItem(input.strip_prefix(expected.as_bytes())?, t))
            } else {
                let n = expected.len();
                if n <= input.len() {
                    let (head, tail) = input.split_at(n);
                    if head.eq_ignore_ascii_case(expected.as_bytes()) {
                        return Some(ParsedItem(tail, t));
                    }
                }
                None
            }
        })
    }
}

/// Consume between `n` and `m` instances of the provided parser.
pub(crate) fn n_to_m<
    'a,
    T,
    P: Fn(&'a [u8]) -> Option<ParsedItem<'a, T>>,
    const N: u8,
    const M: u8,
>(
    parser: P,
) -> impl Fn(&'a [u8]) -> Option<ParsedItem<'a, &'a [u8]>> {
    debug_assert!(M >= N);
    move |mut input| {
        // We need to keep this to determine the total length eventually consumed.
        let orig_input = input;

        // Mandatory
        for _ in 0..N {
            input = parser(input)?.0;
        }

        // Optional
        for _ in N..M {
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

/// Consume between `n` and `m` digits, returning the numerical value.
pub(crate) fn n_to_m_digits<T: Integer, const N: u8, const M: u8>(
    input: &[u8],
) -> Option<ParsedItem<'_, T>> {
    debug_assert!(M >= N);
    n_to_m::<_, _, N, M>(any_digit)(input)?.flat_map(|value| value.parse_bytes())
}

/// Consume exactly `n` digits, returning the numerical value.
pub(crate) fn exactly_n_digits<T: Integer, const N: u8>(input: &[u8]) -> Option<ParsedItem<'_, T>> {
    n_to_m_digits::<_, N, N>(input)
}

/// Consume exactly `n` digits, returning the numerical value.
pub(crate) fn exactly_n_digits_padded<'a, T: Integer, const N: u8>(
    padding: Padding,
) -> impl Fn(&'a [u8]) -> Option<ParsedItem<'a, T>> {
    n_to_m_digits_padded::<_, N, N>(padding)
}

/// Consume between `n` and `m` digits, returning the numerical value.
pub(crate) fn n_to_m_digits_padded<'a, T: Integer, const N: u8, const M: u8>(
    padding: Padding,
) -> impl Fn(&'a [u8]) -> Option<ParsedItem<'a, T>> {
    debug_assert!(M >= N);
    move |mut input| match padding {
        Padding::None => n_to_m_digits::<_, 1, M>(input),
        Padding::Space => {
            debug_assert!(N > 0);

            let mut orig_input = input;
            for _ in 0..(N - 1) {
                match ascii_char::<b' '>(input) {
                    Some(parsed) => input = parsed.0,
                    None => break,
                }
            }
            let pad_width = (orig_input.len() - input.len()) as u8;

            orig_input = input;
            for _ in 0..(N - pad_width) {
                input = any_digit(input)?.0;
            }
            for _ in N..M {
                match any_digit(input) {
                    Some(parsed) => input = parsed.0,
                    None => break,
                }
            }

            ParsedItem(input, &orig_input[..(orig_input.len() - input.len())])
                .flat_map(|value| value.parse_bytes())
        }
        Padding::Zero => n_to_m_digits::<_, N, M>(input),
    }
}

/// Consume exactly one digit.
pub(crate) const fn any_digit(input: &[u8]) -> Option<ParsedItem<'_, u8>> {
    match input {
        [c, remaining @ ..] if c.is_ascii_digit() => Some(ParsedItem(remaining, *c)),
        _ => None,
    }
}

/// Consume exactly one of the provided ASCII characters.
pub(crate) fn ascii_char<const CHAR: u8>(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    debug_assert!(CHAR.is_ascii_graphic() || CHAR.is_ascii_whitespace());
    match input {
        [c, remaining @ ..] if *c == CHAR => Some(ParsedItem(remaining, ())),
        _ => None,
    }
}

/// Consume exactly one of the provided ASCII characters, case-insensitive.
pub(crate) fn ascii_char_ignore_case<const CHAR: u8>(input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    debug_assert!(CHAR.is_ascii_graphic() || CHAR.is_ascii_whitespace());
    match input {
        [c, remaining @ ..] if c.eq_ignore_ascii_case(&CHAR) => Some(ParsedItem(remaining, ())),
        _ => None,
    }
}

/// Optionally consume an input with a given parser.
pub(crate) fn opt<'a, T>(
    parser: impl Fn(&'a [u8]) -> Option<ParsedItem<'a, T>>,
) -> impl Fn(&'a [u8]) -> ParsedItem<'a, Option<T>> {
    move |input| match parser(input) {
        Some(value) => value.map(Some),
        None => ParsedItem(input, None),
    }
}
