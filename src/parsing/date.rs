//! Parsing implementations for the `UtcOffset` struct.

use crate::{
    format_description::modifier,
    parsing::combinator::{exactly_n_digits_padded, first_match, first_string_of, lazy_mut},
    Weekday,
};
use core::num::{NonZeroU16, NonZeroU8};

/// Parse the "year" component of a `Date`.
pub(crate) fn parse_year(input: &mut &str, modifiers: modifier::Year) -> Option<i32> {
    match modifiers.repr {
        modifier::YearRepr::Full => lazy_mut(|input| {
            let sign = first_string_of(&["-", "+"])(input);
            let year = exactly_n_digits_padded::<u32>(
                if cfg!(feature = "large-dates") { 6 } else { 4 },
                modifiers.padding,
            )(input)?;
            match sign {
                Some("-") => Some(-(year as i32)),
                None if modifiers.sign_is_mandatory || year >= 10_000 => None,
                _ => Some(year as i32),
            }
        })(input),
        modifier::YearRepr::LastTwo => {
            Some(exactly_n_digits_padded::<u32>(2, modifiers.padding)(input)? as i32)
        }
    }
}

/// Parse the "month" component of a `Date`.
pub(crate) fn parse_month(input: &mut &str, modifiers: modifier::Month) -> Option<NonZeroU8> {
    exactly_n_digits_padded(2, modifiers.padding)(input)
}

/// Parse the "week number" component of a `Date`.
pub(crate) fn parse_week(input: &mut &str, modifiers: modifier::WeekNumber) -> Option<u8> {
    exactly_n_digits_padded(2, modifiers.padding)(input)
}

/// Parse the "weekday" component of a `Date`.
pub(crate) fn parse_weekday(input: &mut &str, modifiers: modifier::Weekday) -> Option<Weekday> {
    first_match(match (modifiers.repr, modifiers.one_indexed) {
        (modifier::WeekdayRepr::Short, _) => [
            ("Mon", Weekday::Monday),
            ("Tue", Weekday::Tuesday),
            ("Wed", Weekday::Wednesday),
            ("Thu", Weekday::Thursday),
            ("Fri", Weekday::Friday),
            ("Sat", Weekday::Saturday),
            ("Sun", Weekday::Sunday),
        ]
        .iter(),
        (modifier::WeekdayRepr::Long, _) => [
            ("Monday", Weekday::Monday),
            ("Tuesday", Weekday::Tuesday),
            ("Wednesday", Weekday::Wednesday),
            ("Thursday", Weekday::Thursday),
            ("Friday", Weekday::Friday),
            ("Saturday", Weekday::Saturday),
            ("Sunday", Weekday::Sunday),
        ]
        .iter(),
        (modifier::WeekdayRepr::Sunday, false) => [
            ("1", Weekday::Monday),
            ("2", Weekday::Tuesday),
            ("3", Weekday::Wednesday),
            ("4", Weekday::Thursday),
            ("5", Weekday::Friday),
            ("6", Weekday::Saturday),
            ("0", Weekday::Sunday),
        ]
        .iter(),
        (modifier::WeekdayRepr::Sunday, true) => [
            ("2", Weekday::Monday),
            ("3", Weekday::Tuesday),
            ("4", Weekday::Wednesday),
            ("5", Weekday::Thursday),
            ("6", Weekday::Friday),
            ("7", Weekday::Saturday),
            ("1", Weekday::Sunday),
        ]
        .iter(),
        (modifier::WeekdayRepr::Monday, false) => [
            ("0", Weekday::Monday),
            ("1", Weekday::Tuesday),
            ("2", Weekday::Wednesday),
            ("3", Weekday::Thursday),
            ("4", Weekday::Friday),
            ("5", Weekday::Saturday),
            ("6", Weekday::Sunday),
        ]
        .iter(),
        (modifier::WeekdayRepr::Monday, true) => [
            ("1", Weekday::Monday),
            ("2", Weekday::Tuesday),
            ("3", Weekday::Wednesday),
            ("4", Weekday::Thursday),
            ("5", Weekday::Friday),
            ("6", Weekday::Saturday),
            ("7", Weekday::Sunday),
        ]
        .iter(),
    })(input)
}

/// Parse the "ordinal" component of a `Date`.
pub(crate) fn parse_ordinal(input: &mut &str, modifiers: modifier::Ordinal) -> Option<NonZeroU16> {
    exactly_n_digits_padded(3, modifiers.padding)(input)
}

/// Parse the "day" component of a `Date`.
pub(crate) fn parse_day(input: &mut &str, modifiers: modifier::Day) -> Option<NonZeroU8> {
    exactly_n_digits_padded(2, modifiers.padding)(input)
}
