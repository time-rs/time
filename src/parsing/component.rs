//! Parsing implementations for all [`Component`](crate::format_description::Component)s.

use crate::{
    format_description::modifier,
    parsing::combinator::{
        any_digit, exactly_n, exactly_n_digits_padded, first_match, lazy_mut, n_to_m, sign,
    },
    Weekday,
};
use core::num::{NonZeroU16, NonZeroU8};

/// Parse the "year" component of a `Date`.
pub(crate) fn parse_year(input: &mut &str, modifiers: modifier::Year) -> Option<i32> {
    match modifiers.repr {
        modifier::YearRepr::Full => lazy_mut(|input| {
            let sign = sign(input);
            let year = exactly_n_digits_padded::<u32>(
                if cfg!(feature = "large-dates") { 6 } else { 4 },
                modifiers.padding,
            )(input)?;
            match sign {
                Some('-') => Some(-(year as i32)),
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
    first_match(match modifiers.repr {
        modifier::MonthRepr::Numerical => {
            return exactly_n_digits_padded(2, modifiers.padding)(input);
        }
        modifier::MonthRepr::Long => [
            ("January", 1),
            ("February", 2),
            ("March", 3),
            ("April", 4),
            ("May", 5),
            ("June", 6),
            ("July", 7),
            ("August", 8),
            ("September", 9),
            ("October", 10),
            ("November", 11),
            ("December", 12),
        ]
        .iter(),
        modifier::MonthRepr::Short => [
            ("Jan", 1),
            ("Feb", 2),
            ("Mar", 3),
            ("Apr", 4),
            ("May", 5),
            ("Jun", 6),
            ("Jul", 7),
            ("Aug", 8),
            ("Sep", 9),
            ("Oct", 10),
            ("Nov", 11),
            ("Dec", 12),
        ]
        .iter(),
    })(input)
    .and_then(NonZeroU8::new)
}

/// Parse the "week number" component of a `Date`.
pub(crate) fn parse_week_number(input: &mut &str, modifiers: modifier::WeekNumber) -> Option<u8> {
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

/// Parse the "hour" component of a `UtcOffset`.
pub(crate) fn parse_offset_hour(input: &mut &str, modifiers: modifier::OffsetHour) -> Option<i8> {
    lazy_mut(|input| {
        let sign = sign(input);
        let hour = exactly_n_digits_padded::<u8>(2, modifiers.padding)(input)?;
        match sign {
            Some('-') => Some(-(hour as i8)),
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
