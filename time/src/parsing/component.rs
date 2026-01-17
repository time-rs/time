//! Parsing implementations for all [`Component`](crate::format_description::Component)s.

use core::num::NonZero;

use num_conv::prelude::*;

use crate::convert::*;
use crate::format_description::modifier;
use crate::parsing::ParsedItem;
use crate::parsing::combinator::{
    ExactlyNDigits, Sign, any_digit, exactly_n_digits_padded, first_match, n_to_m_digits,
    n_to_m_digits_padded, opt, sign,
};
use crate::{Month, Weekday};

/// Parse the "year" component of a `Date`.
pub(crate) fn parse_year(
    input: &[u8],
    modifiers: modifier::Year,
) -> Option<ParsedItem<'_, (i32, bool)>> {
    match modifiers.repr {
        modifier::YearRepr::Full => {
            let ParsedItem(input, sign) = opt(sign)(input);

            if let Some(sign) = sign {
                let ParsedItem(input, year) = if cfg!(feature = "large-dates")
                    && modifiers.range == modifier::YearRange::Extended
                {
                    n_to_m_digits_padded::<4, 6, u32>(modifiers.padding)(input)?
                } else {
                    exactly_n_digits_padded::<4, u32>(modifiers.padding)(input)?
                };

                Some(ParsedItem(
                    input,
                    match sign {
                        Sign::Negative => (-year.cast_signed(), true),
                        Sign::Positive => (year.cast_signed(), false),
                    },
                ))
            } else if modifiers.sign_is_mandatory {
                None
            } else {
                let ParsedItem(input, year) =
                    exactly_n_digits_padded::<4, u32>(modifiers.padding)(input)?;
                Some(ParsedItem(input, (year.cast_signed(), false)))
            }
        }
        modifier::YearRepr::Century => {
            let ParsedItem(input, sign) = opt(sign)(input);

            if let Some(sign) = sign {
                let ParsedItem(input, year) = if cfg!(feature = "large-dates")
                    && modifiers.range == modifier::YearRange::Extended
                {
                    n_to_m_digits_padded::<2, 4, u32>(modifiers.padding)(input)?
                } else {
                    exactly_n_digits_padded::<2, u32>(modifiers.padding)(input)?
                };

                Some(ParsedItem(
                    input,
                    match sign {
                        Sign::Negative => (-year.cast_signed(), true),
                        Sign::Positive => (year.cast_signed(), false),
                    },
                ))
            } else if modifiers.sign_is_mandatory {
                None
            } else {
                let ParsedItem(input, year) =
                    n_to_m_digits_padded::<1, 2, u32>(modifiers.padding)(input)?;
                Some(ParsedItem(input, (year.cast_signed(), false)))
            }
        }
        modifier::YearRepr::LastTwo => Some(
            exactly_n_digits_padded::<2, u32>(modifiers.padding)(input)?
                .map(|v| (v.cast_signed(), false)),
        ),
    }
}

/// Parse the "month" component of a `Date`.
pub(crate) fn parse_month(
    input: &[u8],
    modifiers: modifier::Month,
) -> Option<ParsedItem<'_, Month>> {
    use Month::*;
    let ParsedItem(remaining, value) = first_match(
        match modifiers.repr {
            modifier::MonthRepr::Numerical => {
                return exactly_n_digits_padded::<2, _>(modifiers.padding)(input)?
                    .flat_map(|n| Month::from_number(NonZero::new(n)?).ok());
            }
            modifier::MonthRepr::Long => [
                (b"January".as_slice(), January),
                (b"February".as_slice(), February),
                (b"March".as_slice(), March),
                (b"April".as_slice(), April),
                (b"May".as_slice(), May),
                (b"June".as_slice(), June),
                (b"July".as_slice(), July),
                (b"August".as_slice(), August),
                (b"September".as_slice(), September),
                (b"October".as_slice(), October),
                (b"November".as_slice(), November),
                (b"December".as_slice(), December),
            ],
            modifier::MonthRepr::Short => [
                (b"Jan".as_slice(), January),
                (b"Feb".as_slice(), February),
                (b"Mar".as_slice(), March),
                (b"Apr".as_slice(), April),
                (b"May".as_slice(), May),
                (b"Jun".as_slice(), June),
                (b"Jul".as_slice(), July),
                (b"Aug".as_slice(), August),
                (b"Sep".as_slice(), September),
                (b"Oct".as_slice(), October),
                (b"Nov".as_slice(), November),
                (b"Dec".as_slice(), December),
            ],
        },
        modifiers.case_sensitive,
    )(input)?;
    Some(ParsedItem(remaining, value))
}

/// Parse the "week number" component of a `Date`.
pub(crate) fn parse_week_number(
    input: &[u8],
    modifiers: modifier::WeekNumber,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
}

/// Parse the "weekday" component of a `Date`.
pub(crate) fn parse_weekday(
    input: &[u8],
    modifiers: modifier::Weekday,
) -> Option<ParsedItem<'_, Weekday>> {
    match (modifiers.repr, modifiers.one_indexed) {
        (modifier::WeekdayRepr::Short, _) if modifiers.case_sensitive => match input {
            [b'M', b'o', b'n', rest @ ..] => Some(ParsedItem(rest, Weekday::Monday)),
            [b'T', b'u', b'e', rest @ ..] => Some(ParsedItem(rest, Weekday::Tuesday)),
            [b'W', b'e', b'd', rest @ ..] => Some(ParsedItem(rest, Weekday::Wednesday)),
            [b'T', b'h', b'u', rest @ ..] => Some(ParsedItem(rest, Weekday::Thursday)),
            [b'F', b'r', b'i', rest @ ..] => Some(ParsedItem(rest, Weekday::Friday)),
            [b'S', b'a', b't', rest @ ..] => Some(ParsedItem(rest, Weekday::Saturday)),
            [b'S', b'u', b'n', rest @ ..] => Some(ParsedItem(rest, Weekday::Sunday)),
            _ => None,
        },
        (modifier::WeekdayRepr::Short, _) => match input {
            [b'M' | b'm', b'O' | b'o', b'N' | b'n', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Monday))
            }
            [b'T' | b't', b'U' | b'u', b'E' | b'e', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Tuesday))
            }
            [b'W' | b'w', b'E' | b'e', b'D' | b'd', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Wednesday))
            }
            [b'T' | b't', b'H' | b'h', b'U' | b'u', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Thursday))
            }
            [b'F' | b'f', b'R' | b'r', b'I' | b'i', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Friday))
            }
            [b'S' | b's', b'A' | b'a', b'T' | b't', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Saturday))
            }
            [b'S' | b's', b'U' | b'u', b'N' | b'n', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Sunday))
            }
            _ => None,
        },
        (modifier::WeekdayRepr::Long, _) if modifiers.case_sensitive => match input {
            [b'M', b'o', b'n', b'd', b'a', b'y', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Monday))
            }
            [b'T', b'u', b'e', b's', b'd', b'a', b'y', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Tuesday))
            }
            [
                b'W',
                b'e',
                b'd',
                b'n',
                b'e',
                b's',
                b'd',
                b'a',
                b'y',
                rest @ ..,
            ] => Some(ParsedItem(rest, Weekday::Wednesday)),
            [b'T', b'h', b'u', b'r', b's', b'd', b'a', b'y', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Thursday))
            }
            [b'F', b'r', b'i', b'd', b'a', b'y', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Friday))
            }
            [b'S', b'a', b't', b'u', b'r', b'd', b'a', b'y', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Saturday))
            }
            [b'S', b'u', b'n', b'd', b'a', b'y', rest @ ..] => {
                Some(ParsedItem(rest, Weekday::Sunday))
            }
            _ => None,
        },
        (modifier::WeekdayRepr::Long, _) => match input {
            [
                b'M' | b'm',
                b'O' | b'o',
                b'N' | b'n',
                b'D' | b'd',
                b'A' | b'a',
                b'Y' | b'y',
                rest @ ..,
            ] => Some(ParsedItem(rest, Weekday::Monday)),
            [
                b'T' | b't',
                b'U' | b'u',
                b'E' | b'e',
                b'S' | b's',
                b'D' | b'd',
                b'A' | b'a',
                b'Y' | b'y',
                rest @ ..,
            ] => Some(ParsedItem(rest, Weekday::Tuesday)),
            [
                b'W' | b'w',
                b'E' | b'e',
                b'D' | b'd',
                b'N' | b'n',
                b'E' | b'e',
                b'S' | b's',
                b'D' | b'd',
                b'A' | b'a',
                b'Y' | b'y',
                rest @ ..,
            ] => Some(ParsedItem(rest, Weekday::Wednesday)),
            [
                b'T' | b't',
                b'H' | b'h',
                b'U' | b'u',
                b'R' | b'r',
                b'S' | b's',
                b'D' | b'd',
                b'A' | b'a',
                b'Y' | b'y',
                rest @ ..,
            ] => Some(ParsedItem(rest, Weekday::Thursday)),
            [
                b'F' | b'f',
                b'R' | b'r',
                b'I' | b'i',
                b'D' | b'd',
                b'A' | b'a',
                b'Y' | b'y',
                rest @ ..,
            ] => Some(ParsedItem(rest, Weekday::Friday)),
            [
                b'S' | b's',
                b'A' | b'a',
                b'T' | b't',
                b'U' | b'u',
                b'R' | b'r',
                b'D' | b'd',
                b'A' | b'a',
                b'Y' | b'y',
                rest @ ..,
            ] => Some(ParsedItem(rest, Weekday::Saturday)),
            [
                b'S' | b's',
                b'U' | b'u',
                b'N' | b'n',
                b'D' | b'd',
                b'A' | b'a',
                b'Y' | b'y',
                rest @ ..,
            ] => Some(ParsedItem(rest, Weekday::Sunday)),
            _ => None,
        },
        (modifier::WeekdayRepr::Sunday, false) => match input {
            [b'1', rest @ ..] => Some(ParsedItem(rest, Weekday::Monday)),
            [b'2', rest @ ..] => Some(ParsedItem(rest, Weekday::Tuesday)),
            [b'3', rest @ ..] => Some(ParsedItem(rest, Weekday::Wednesday)),
            [b'4', rest @ ..] => Some(ParsedItem(rest, Weekday::Thursday)),
            [b'5', rest @ ..] => Some(ParsedItem(rest, Weekday::Friday)),
            [b'6', rest @ ..] => Some(ParsedItem(rest, Weekday::Saturday)),
            [b'0', rest @ ..] => Some(ParsedItem(rest, Weekday::Sunday)),
            _ => None,
        },
        (modifier::WeekdayRepr::Sunday, true) => match input {
            [b'2', rest @ ..] => Some(ParsedItem(rest, Weekday::Monday)),
            [b'3', rest @ ..] => Some(ParsedItem(rest, Weekday::Tuesday)),
            [b'4', rest @ ..] => Some(ParsedItem(rest, Weekday::Wednesday)),
            [b'5', rest @ ..] => Some(ParsedItem(rest, Weekday::Thursday)),
            [b'6', rest @ ..] => Some(ParsedItem(rest, Weekday::Friday)),
            [b'7', rest @ ..] => Some(ParsedItem(rest, Weekday::Saturday)),
            [b'1', rest @ ..] => Some(ParsedItem(rest, Weekday::Sunday)),
            _ => None,
        },
        (modifier::WeekdayRepr::Monday, false) => match input {
            [b'0', rest @ ..] => Some(ParsedItem(rest, Weekday::Monday)),
            [b'1', rest @ ..] => Some(ParsedItem(rest, Weekday::Tuesday)),
            [b'2', rest @ ..] => Some(ParsedItem(rest, Weekday::Wednesday)),
            [b'3', rest @ ..] => Some(ParsedItem(rest, Weekday::Thursday)),
            [b'4', rest @ ..] => Some(ParsedItem(rest, Weekday::Friday)),
            [b'5', rest @ ..] => Some(ParsedItem(rest, Weekday::Saturday)),
            [b'6', rest @ ..] => Some(ParsedItem(rest, Weekday::Sunday)),
            _ => None,
        },
        (modifier::WeekdayRepr::Monday, true) => match input {
            [b'1', rest @ ..] => Some(ParsedItem(rest, Weekday::Monday)),
            [b'2', rest @ ..] => Some(ParsedItem(rest, Weekday::Tuesday)),
            [b'3', rest @ ..] => Some(ParsedItem(rest, Weekday::Wednesday)),
            [b'4', rest @ ..] => Some(ParsedItem(rest, Weekday::Thursday)),
            [b'5', rest @ ..] => Some(ParsedItem(rest, Weekday::Friday)),
            [b'6', rest @ ..] => Some(ParsedItem(rest, Weekday::Saturday)),
            [b'7', rest @ ..] => Some(ParsedItem(rest, Weekday::Sunday)),
            _ => None,
        },
    }
}

/// Parse the "ordinal" component of a `Date`.
#[inline]
pub(crate) fn parse_ordinal(
    input: &[u8],
    modifiers: modifier::Ordinal,
) -> Option<ParsedItem<'_, NonZero<u16>>> {
    exactly_n_digits_padded::<3, _>(modifiers.padding)(input)
        .and_then(|parsed| parsed.flat_map(NonZero::new))
}

/// Parse the "day" component of a `Date`.
#[inline]
pub(crate) fn parse_day(
    input: &[u8],
    modifiers: modifier::Day,
) -> Option<ParsedItem<'_, NonZero<u8>>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
        .and_then(|parsed| parsed.flat_map(NonZero::new))
}

/// Indicate whether the hour is "am" or "pm".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Period {
    #[allow(clippy::missing_docs_in_private_items)]
    Am,
    #[allow(clippy::missing_docs_in_private_items)]
    Pm,
}

/// Parse the "hour" component of a `Time`.
#[inline]
pub(crate) fn parse_hour(input: &[u8], modifiers: modifier::Hour) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
}

/// Parse the "minute" component of a `Time`.
#[inline]
pub(crate) fn parse_minute(
    input: &[u8],
    modifiers: modifier::Minute,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
}

/// Parse the "second" component of a `Time`.
#[inline]
pub(crate) fn parse_second(
    input: &[u8],
    modifiers: modifier::Second,
) -> Option<ParsedItem<'_, u8>> {
    exactly_n_digits_padded::<2, _>(modifiers.padding)(input)
}

/// Parse the "period" component of a `Time`. Required if the hour is on a 12-hour clock.
#[inline]
pub(crate) fn parse_period(
    input: &[u8],
    modifiers: modifier::Period,
) -> Option<ParsedItem<'_, Period>> {
    let (rest, period) = match (modifiers.is_uppercase, modifiers.case_sensitive, input) {
        (true, _, [b'A', b'M', rest @ ..]) => (rest, Period::Am),
        (true, _, [b'P', b'M', rest @ ..]) => (rest, Period::Pm),
        (false, _, [b'a', b'm', rest @ ..]) => (rest, Period::Am),
        (false, _, [b'p', b'm', rest @ ..]) => (rest, Period::Pm),
        (_, false, [b'A' | b'a', b'M' | b'm', rest @ ..]) => (rest, Period::Am),
        (_, false, [b'P' | b'p', b'M' | b'm', rest @ ..]) => (rest, Period::Pm),
        _ => return None,
    };
    Some(ParsedItem(rest, period))
}

/// Parse the "subsecond" component of a `Time`.
pub(crate) fn parse_subsecond(
    input: &[u8],
    modifiers: modifier::Subsecond,
) -> Option<ParsedItem<'_, u32>> {
    use modifier::SubsecondDigits::*;
    Some(match modifiers.digits {
        One => ExactlyNDigits::<1>::parse(input)?.map(|v| v.extend::<u32>() * 100_000_000),
        Two => ExactlyNDigits::<2>::parse(input)?.map(|v| v.extend::<u32>() * 10_000_000),
        Three => ExactlyNDigits::<3>::parse(input)?.map(|v| v.extend::<u32>() * 1_000_000),
        Four => ExactlyNDigits::<4>::parse(input)?.map(|v| v.extend::<u32>() * 100_000),
        Five => ExactlyNDigits::<5>::parse(input)?.map(|v| v * 10_000),
        Six => ExactlyNDigits::<6>::parse(input)?.map(|v| v * 1_000),
        Seven => ExactlyNDigits::<7>::parse(input)?.map(|v| v * 100),
        Eight => ExactlyNDigits::<8>::parse(input)?.map(|v| v * 10),
        Nine => ExactlyNDigits::<9>::parse(input)?,
        OneOrMore => {
            let ParsedItem(mut input, mut value) =
                any_digit(input)?.map(|v| (v - b'0').extend::<u32>() * 100_000_000);

            let mut multiplier = 10_000_000;
            while let Some(ParsedItem(new_input, digit)) = any_digit(input) {
                value += (digit - b'0').extend::<u32>() * multiplier;
                input = new_input;
                multiplier /= 10;
            }

            ParsedItem(input, value)
        }
    })
}

/// Parse the "hour" component of a `UtcOffset`.
///
/// Returns the value and whether the value is negative. This is used for when "-0" is parsed.
pub(crate) fn parse_offset_hour(
    input: &[u8],
    modifiers: modifier::OffsetHour,
) -> Option<ParsedItem<'_, (i8, bool)>> {
    let ParsedItem(input, sign) = opt(sign)(input);
    let ParsedItem(input, hour) = exactly_n_digits_padded::<2, u8>(modifiers.padding)(input)?;
    match sign {
        Some(Sign::Negative) => Some(ParsedItem(input, (-hour.cast_signed(), true))),
        None if modifiers.sign_is_mandatory => None,
        _ => Some(ParsedItem(input, (hour.cast_signed(), false))),
    }
}

/// Parse the "minute" component of a `UtcOffset`.
#[inline]
pub(crate) fn parse_offset_minute(
    input: &[u8],
    modifiers: modifier::OffsetMinute,
) -> Option<ParsedItem<'_, i8>> {
    Some(
        exactly_n_digits_padded::<2, u8>(modifiers.padding)(input)?
            .map(|offset_minute| offset_minute.cast_signed()),
    )
}

/// Parse the "second" component of a `UtcOffset`.
#[inline]
pub(crate) fn parse_offset_second(
    input: &[u8],
    modifiers: modifier::OffsetSecond,
) -> Option<ParsedItem<'_, i8>> {
    Some(
        exactly_n_digits_padded::<2, u8>(modifiers.padding)(input)?
            .map(|offset_second| offset_second.cast_signed()),
    )
}

/// Ignore the given number of bytes.
#[inline]
pub(crate) fn parse_ignore(
    input: &[u8],
    modifiers: modifier::Ignore,
) -> Option<ParsedItem<'_, ()>> {
    let modifier::Ignore { count } = modifiers;
    let input = input.get((count.get().extend())..)?;
    Some(ParsedItem(input, ()))
}

/// Parse the Unix timestamp component.
pub(crate) fn parse_unix_timestamp(
    input: &[u8],
    modifiers: modifier::UnixTimestamp,
) -> Option<ParsedItem<'_, i128>> {
    let ParsedItem(input, sign) = opt(sign)(input);
    let ParsedItem(input, nano_timestamp) = match modifiers.precision {
        modifier::UnixTimestampPrecision::Second => {
            n_to_m_digits::<1, 14, u128>(input)?.map(|val| val * Nanosecond::per_t::<u128>(Second))
        }
        modifier::UnixTimestampPrecision::Millisecond => n_to_m_digits::<1, 17, u128>(input)?
            .map(|val| val * Nanosecond::per_t::<u128>(Millisecond)),
        modifier::UnixTimestampPrecision::Microsecond => n_to_m_digits::<1, 20, u128>(input)?
            .map(|val| val * Nanosecond::per_t::<u128>(Microsecond)),
        modifier::UnixTimestampPrecision::Nanosecond => n_to_m_digits::<1, 23, _>(input)?,
    };

    match sign {
        Some(Sign::Negative) => Some(ParsedItem(input, -nano_timestamp.cast_signed())),
        None if modifiers.sign_is_mandatory => None,
        _ => Some(ParsedItem(input, nano_timestamp.cast_signed())),
    }
}

/// Parse the `end` component, which represents the end of input. If any input is remaining, `None`
/// is returned.
#[inline]
pub(crate) const fn parse_end(input: &[u8], end: modifier::End) -> Option<ParsedItem<'_, ()>> {
    let modifier::End {} = end;

    if input.is_empty() {
        Some(ParsedItem(input, ()))
    } else {
        None
    }
}
