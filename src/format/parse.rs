//! Parsing for various types.

use super::{parse_with_language, FormatItem, Padding, Specifier};
use crate::{Language, UtcOffset, Weekday};
use core::fmt::{self, Display, Formatter};
use core::num::{NonZeroU16, NonZeroU8};
use core::ops::Range;
use core::str::FromStr;
#[cfg(feature = "std")]
use std::error::Error;

/// Helper type to avoid repeating the error type.
pub(crate) type ParseResult<T> = Result<T, ParseError>;

/// An error ocurred while parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseError {
    /// The second present was not valid.
    InvalidSecond,
    /// The minute present was not valid.
    InvalidMinute,
    /// The hour present was not valid.
    InvalidHour,
    /// The AM/PM was not valid.
    InvalidAmPm,
    /// The month present was not valid.
    InvalidMonth,
    /// The year present was not valid.
    InvalidYear,
    /// The week present was not valid.
    InvalidWeek,
    /// The day of week present was not valid.
    InvalidDayOfWeek,
    /// The day of month present was not valid.
    InvalidDayOfMonth,
    /// The day of year present was not valid.
    InvalidDayOfYear,
    /// The UTC offset present was not valid.
    InvalidOffset,
    /// There was no character following a `%`.
    MissingFormatSpecifier,
    /// The character following `%` is not valid.
    InvalidFormatSpecifier(char),
    /// A character literal was expected to be present but was not.
    UnexpectedCharacter {
        /// The character that was expected to be present.
        expected: char,
        /// The character that was present in the string.
        actual: char,
    },
    /// The string ended, but there should be more content.
    UnexpectedEndOfString,
    /// There was not enough information provided to create the requested type.
    InsufficientInformation,
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    __nonexhaustive,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use ParseError::*;
        match self {
            InvalidSecond => f.write_str("invalid second"),
            InvalidMinute => f.write_str("invalid minute"),
            InvalidHour => f.write_str("invalid hour"),
            InvalidAmPm => f.write_str("invalid am/pm"),
            InvalidMonth => f.write_str("invalid month"),
            InvalidYear => f.write_str("invalid year"),
            InvalidWeek => f.write_str("invalid week"),
            InvalidDayOfWeek => f.write_str("invalid day of week"),
            InvalidDayOfMonth => f.write_str("invalid day of month"),
            InvalidDayOfYear => f.write_str("invalid day of year"),
            InvalidOffset => f.write_str("invalid offset"),
            MissingFormatSpecifier => f.write_str("missing format specifier after `%`"),
            InvalidFormatSpecifier(c) => write!(f, "invalid format specifier `{}` after `%`", c),
            UnexpectedCharacter { expected, actual } => {
                write!(f, "expected character `{}`, found `{}`", expected, actual)
            }
            UnexpectedEndOfString => f.write_str("unexpected end of string"),
            InsufficientInformation => {
                f.write_str("insufficient information provided to create the requested type")
            }
            __nonexhaustive => panic!(
                "`__nonexhaustive` is hidden in the documentation for a reason! Don't use it."
            ),
        }
    }
}

#[cfg(feature = "std")]
impl Error for ParseError {}

/// A value representing a time that is either "AM" or "PM".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AmPm {
    /// A time before noon.
    AM,
    /// A time at or after noon.
    PM,
}

/// All information gathered from parsing a provided string.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ParsedItems {
    /// Year the ISO week belongs to.
    pub(crate) week_based_year: Option<i32>,
    /// The year the month, day, and ordinal day belong to.
    pub(crate) year: Option<i32>,
    /// One-indexed month number.
    pub(crate) month: Option<NonZeroU8>,
    /// Day of the month.
    pub(crate) day: Option<NonZeroU8>,
    /// Day of the week.
    pub(crate) weekday: Option<Weekday>,
    /// Day of the year.
    pub(crate) ordinal_day: Option<NonZeroU16>,
    /// ISO week within the year. Week 1 contains the year's first Thursday.
    pub(crate) iso_week: Option<NonZeroU8>,
    /// Week number, counted from the first Sunday. May be zero.
    pub(crate) sunday_week: Option<u8>,
    /// Week number, counted from the first Monday. May be zero.
    pub(crate) monday_week: Option<u8>,
    /// Hour in the 12-hour clock.
    pub(crate) hour_12: Option<NonZeroU8>,
    /// Hour in the 24-hour clock.
    pub(crate) hour_24: Option<u8>,
    /// Minute within the hour.
    pub(crate) minute: Option<u8>,
    /// Second within the minute.
    pub(crate) second: Option<u8>,
    /// The UTC offset of the datetime.
    pub(crate) offset: Option<UtcOffset>,
    /// Whether the hour indicated is AM or PM.
    pub(crate) am_pm: Option<AmPm>,
}

impl ParsedItems {
    /// Create a new `ParsedItems` with nothing known.
    const fn new() -> Self {
        Self {
            week_based_year: None,
            year: None,
            month: None,
            day: None,
            weekday: None,
            ordinal_day: None,
            iso_week: None,
            sunday_week: None,
            monday_week: None,
            hour_12: None,
            hour_24: None,
            minute: None,
            second: None,
            offset: None,
            am_pm: None,
        }
    }
}

/// Attempt to consume the provided character.
pub(crate) fn try_consume_char(s: &mut &str, expected: char) -> ParseResult<()> {
    match s.char_indices().next() {
        Some((index, actual_char)) if actual_char == expected => {
            *s = &s[(index + actual_char.len_utf8())..];
            Ok(())
        }
        Some((_, actual)) => Err(ParseError::UnexpectedCharacter { expected, actual }),
        None => Err(ParseError::UnexpectedEndOfString),
    }
}

/// Attempt to consume the provided string.
pub(crate) fn try_consume_str(s: &mut &str, expected: &str) -> ParseResult<()> {
    if s.starts_with(expected) {
        *s = &s[expected.len()..];
        Ok(())
    } else {
        // Iterate through the characters, returning the error where differing.
        for c in expected.chars() {
            try_consume_char(s, c)?;
        }
        unreachable!("The previous loop should always cause the function to return.");
    }
}

/// Attempt to find one of the strings provided, returning the first value.
pub(crate) fn try_consume_first_match<T: Copy>(
    s: &mut &str,
    opts: impl IntoIterator<Item = (impl AsRef<str>, T)>,
) -> Option<T> {
    opts.into_iter().find_map(|(expected, value)| {
        if try_consume_str(s, expected.as_ref()).is_ok() {
            Some(value)
        } else {
            None
        }
    })
}

/// Attempt to consume a number of digits. Consumes the maximum amount possible
/// within the range provided.
pub(crate) fn try_consume_digits<T: FromStr>(s: &mut &str, num_digits: Range<usize>) -> Option<T> {
    // Determine how many digits the string starts with, up to the upper limit
    // of the range.
    let len = s
        .chars()
        .take(num_digits.end)
        .take_while(char::is_ascii_digit)
        .count();

    // We don't have enough digits.
    if len < num_digits.start {
        return None;
    }

    // Because we're only dealing with ASCII digits here, we know that the
    // length is equal to the number of bytes, as ASCII values are always one
    // byte in Unicode.
    let digits = &s[..len];
    *s = &s[len..];
    digits.parse::<T>().ok()
}

/// Attempt to consume a number of digits. Consumes the maximum amount possible
/// within the range provided. Returns `None` if the value is not within the
/// allowed range.
// TODO Is there some way to allow both `Range` and `RangeInclusive`? It would
// be better for readability in some places.
pub(crate) fn try_consume_digits_in_range<T: FromStr + PartialOrd>(
    s: &mut &str,
    num_digits: Range<usize>,
    range: Range<T>,
) -> Option<T> {
    try_consume_digits(s, num_digits).filter(|value| range.contains(value))
}

/// Attempt to consume an exact number of digits.
pub(crate) fn try_consume_exact_digits<T: FromStr>(s: &mut &str, num_digits: usize) -> Option<T> {
    // Ensure all the necessary characters are ASCII digits.
    if !s.chars().take(num_digits).all(|c| c.is_ascii_digit()) {
        return None;
    }

    // Because we're only dealing with ASCII digits here, we know that the
    // length is equal to the number of bytes, as ASCII values are always one
    // byte in Unicode.
    let digits = &s[..num_digits];
    *s = &s[num_digits..];
    digits.parse::<T>().ok()
}

/// Attempt to consume an exact number of digits. Returns `None` if the value is
/// not within the allowed range.
pub(crate) fn try_consume_exact_digits_in_range<T: FromStr + PartialOrd>(
    s: &mut &str,
    num_digits: usize,
    range: Range<T>,
) -> Option<T> {
    try_consume_exact_digits(s, num_digits).filter(|value| range.contains(value))
}

/// Consume all leading padding up to the number of characters.
///
/// Returns the number of characters trimmed.
pub(crate) fn consume_padding(s: &mut &str, padding: Padding, max_chars: usize) -> usize {
    let pad_char = match padding {
        Padding::Space => ' ',
        Padding::Zero => '0',
        Padding::None => return 0,
        Padding::Default => unreachable!(
            "Default padding depends on context. This value should replaced \
             prior to calling `consume_padding`."
        ),
    };

    let pad_width = s
        .chars()
        .take(max_chars)
        .take_while(|&c| c == pad_char)
        .count();
    *s = &s[pad_width..];
    pad_width
}

/// Attempt to parse the string with the provided format and language, returning
/// a struct containing all information found.
pub(crate) fn parse(s: &str, format: &str, language: Language) -> ParseResult<ParsedItems> {
    use super::{date, offset, time};

    // Make a copy of the provided string, letting us mutate as necessary.
    let mut s = <&str>::clone(&s);

    let mut items = ParsedItems::new();

    /// Parse the provided specifier with the given parameters.
    macro_rules! parse {
        ($module:ident, $specifier:ident $(, $params:expr)*) => {
            paste::expr! {
                $module::[<parse_ $specifier>](&mut items, &mut s $(, $params)*)?
            }
        };
    }

    macro_rules! parse_char {
        ($c:literal) => {
            try_consume_char(&mut s, $c)?
        };
    }

    for item in parse_with_language(format, language) {
        match item {
            FormatItem::Literal(expected) => try_consume_str(&mut s, expected)?,
            FormatItem::Specifier(specifier) => {
                use Specifier::*;
                match specifier {
                    a { language } => parse!(date, a, language),
                    A { language } => parse!(date, A, language),
                    b { language } => parse!(date, b, language),
                    B { language } => parse!(date, B, language),
                    c { language } => {
                        parse!(date, a, language);
                        parse_char!(' ');
                        parse!(date, b, language);
                        parse_char!(' ');
                        parse!(date, d, Padding::None);
                        parse_char!(' ');
                        parse!(time, H, Padding::None);
                        parse_char!(':');
                        parse!(time, M, Padding::Default);
                        parse_char!(':');
                        parse!(time, S, Padding::Default);
                        parse_char!(' ');
                        parse!(date, Y, Padding::None);
                    }
                    C { padding } => parse!(date, C, padding),
                    d { padding } => parse!(date, d, padding),
                    D => {
                        parse!(date, m, Padding::Default);
                        parse_char!('/');
                        parse!(date, d, Padding::Default);
                        parse_char!('/');
                        parse!(date, y, Padding::Default);
                    }
                    e { padding } => parse!(date, e, padding),
                    F => {
                        parse!(date, Y, Padding::None);
                        parse_char!('-');
                        parse!(date, m, Padding::Default);
                        parse_char!('-');
                        parse!(date, d, Padding::Default);
                    }
                    g { padding } => parse!(date, g, padding),
                    G { padding } => parse!(date, G, padding),
                    H { padding } => parse!(time, H, padding),
                    I { padding } => parse!(time, I, padding),
                    j { padding } => parse!(date, j, padding),
                    M { padding } => parse!(time, M, padding),
                    m { padding } => parse!(date, m, padding),
                    p => parse!(time, p),
                    P => parse!(time, P),
                    r => {
                        parse!(time, I, Padding::None);
                        parse_char!(':');
                        parse!(time, M, Padding::Default);
                        parse_char!(':');
                        parse!(time, S, Padding::Default);
                        parse_char!(' ');
                        parse!(time, p);
                    }
                    R => {
                        parse!(time, H, Padding::None);
                        parse_char!(':');
                        parse!(time, M, Padding::Default);
                    }
                    S { padding } => parse!(time, S, padding),
                    T => {
                        parse!(time, H, Padding::None);
                        parse_char!(':');
                        parse!(time, M, Padding::Default);
                        parse_char!(':');
                        parse!(time, S, Padding::Default);
                    }
                    u => parse!(date, u),
                    V { padding } => parse!(date, V, padding),
                    w => parse!(date, w),
                    y { padding } => parse!(date, y, padding),
                    z => parse!(offset, z),
                    Y { padding } => parse!(date, Y, padding),
                    _ => unimplemented!(),
                }
            }
        }
    }

    Ok(items)
}
