//! Implementations of the low-level parser combinators.

use core::{str::FromStr, u128};

/// Marker trait for integers.
pub(crate) trait Integer: FromStr {}
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}
impl Integer for isize {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}
impl Integer for usize {}

/// Parse a string.
pub(crate) fn string<'a>(expected: &'a str) -> impl Fn(&mut &'a str) -> Option<()> {
    move |input| {
        let remaining = input.strip_prefix(expected)?;
        *input = remaining;
        Some(())
    }
}

/// Parse one of the provided strings.
pub(crate) fn first_string_of<'a>(
    expected_one_of: &'a [&str],
) -> impl Fn(&mut &'a str) -> Option<&'a str> {
    move |input| {
        for &expected in expected_one_of {
            if string(expected)(input).is_some() {
                return Some(expected);
            }
        }
        None
    }
}

/// Map the resulting value to a new value (that may or may not be the same type).
pub(crate) fn map<'a, T, U>(
    parser: impl Fn(&mut &'a str) -> Option<T>,
    map_fn: impl Fn(T) -> U,
) -> impl Fn(&mut &'a str) -> Option<U> {
    move |input| parser(input).map(|v| map_fn(v))
}

/// Map the resulting value to a new value (that may or may not be the same type).
pub(crate) fn flat_map<'a, T, U>(
    parser: impl Fn(&mut &'a str) -> Option<T>,
    map_fn: impl Fn(T) -> Option<U>,
) -> impl Fn(&mut &'a str) -> Option<U> {
    move |input| parser(input).and_then(|v| map_fn(v))
}

/// Consume between `n` and `m` instances of the provided parser.
pub(crate) fn n_to_m<'a, 'b: 'a, T>(
    n: usize,
    m: usize,
    parser: impl Fn(&mut &'a str) -> Option<T>,
) -> impl Fn(&mut &'b str) -> Option<&'a str> {
    debug_assert!(m >= n);
    move |orig_input| {
        // We don't want to mutate the input if the parser fails.
        let mut input = *orig_input;

        // Mandatory
        for _ in 0..n {
            parser(&mut input)?;
        }

        // Optional
        for _ in n..m {
            if parser(&mut input).is_none() {
                break;
            };
        }

        // Find out how much was consumed. We can finally mutate the true input, returning the chunk
        // at the front.
        let (ret_val, remaining_input) = orig_input.split_at(orig_input.len() - input.len());
        *orig_input = remaining_input;
        Some(ret_val)
    }
}

/// Consume exactly `n` instances of the provided parser.
pub(crate) fn exactly_n<'a, 'b: 'a, T>(
    n: usize,
    parser: impl Fn(&mut &'a str) -> Option<T>,
) -> impl Fn(&mut &'b str) -> Option<&'a str> {
    n_to_m(n, n, parser)
}

/// Consume between `n` and `m` digits, returning the numerical value.
pub(crate) fn n_to_m_digits<'a, T: Integer>(
    n: usize,
    m: usize,
) -> impl Fn(&mut &'a str) -> Option<T> {
    debug_assert!(m >= n);
    flat_map(
        n_to_m(n, m, pred(any_char, char::is_ascii_digit)),
        |value| value.parse().ok(),
    )
}

/// Consume exactly `n` digits, returning the numerical value.
pub(crate) fn exactly_n_digits<'a, T: Integer>(n: usize) -> impl Fn(&mut &'a str) -> Option<T> {
    n_to_m_digits(n, n)
}

/// Consume exactly one character.
pub(crate) fn any_char(input: &mut &str) -> Option<char> {
    let value = input.chars().next()?;
    *input = &input[value.len_utf8()..];
    Some(value)
}

/// Filter the output based on a predicate.
pub(crate) fn pred<'a, T>(
    parser: impl Fn(&mut &'a str) -> Option<T>,
    predicate: impl Fn(&T) -> bool,
) -> impl Fn(&mut &'a str) -> Option<T> {
    move |orig_input| {
        let mut input = *orig_input;
        let value = parser(&mut input).filter(|v| predicate(v))?;
        *orig_input = input;
        Some(value)
    }
}

/// Indicate that the parser need not succeed, as the parsed value is optional.
///
/// To remain consistent with the other combinators, this method returns `Option<_>`. However, it is
/// _guaranteed_ to return `Some(_)`. The contained value may still be `None`, indicating that the
/// attempted parse was unsuccessful.
pub(crate) fn opt<'a, T>(
    parser: impl Fn(&mut &'a str) -> Option<T>,
) -> impl Fn(&mut &'a str) -> Option<Option<T>> {
    map(parser, Some)
}
