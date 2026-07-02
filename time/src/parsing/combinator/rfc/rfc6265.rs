//! RFC 6265 `cookie-date` parsing.
//!
//! This implements the user-agent date parser from RFC 6265 section 5.1.1:
//! <https://datatracker.ietf.org/doc/html/rfc6265#section-5.1.1>.
//! That algorithm tokenizes by delimiter octets, scans tokens in order, records
//! the first matching time, day, month, and year productions, then constructs a
//! UTC date-time. This does not parse a complete `Set-Cookie` header; section
//! 5.2.1 passes only the `Expires` attribute value to this algorithm.
//! See <https://datatracker.ietf.org/doc/html/rfc6265#section-5.2.1>.
//! This follows verified RFC Errata 4148 for the optional trailing data in the
//! day, year, and time productions: <https://www.rfc-editor.org/errata/eid4148>.

use num_conv::prelude::*;

use crate::error::ParseFromDescription::InvalidComponent;
use crate::error::TryFromParsed;
use crate::{Date, Month, OffsetDateTime, Time, UtcOffset, error};

pub(crate) fn invalid_component(name: &'static str) -> error::Parse {
    error::Parse::ParseFromDescription(InvalidComponent(name))
}

fn component_range(err: error::ComponentRange) -> error::Parse {
    error::Parse::TryFromParsed(TryFromParsed::ComponentRange(err))
}

#[derive(Default)]
struct Parsed {
    time: Option<(u8, u8, u8)>,
    day: Option<u8>,
    month: Option<Month>,
    year: Option<i32>,
}

fn is_delimiter(byte: u8) -> bool {
    // RFC 6265 section 5.1.1 defines delimiter as HTAB, space through slash,
    // semicolon through at-sign, left bracket through grave accent, and left
    // brace through tilde. Bytes outside those ranges are part of date-token.
    byte == b'\t'
        || (0x20..=0x2F).contains(&byte)
        || (0x3B..=0x40).contains(&byte)
        || (0x5B..=0x60).contains(&byte)
        || (0x7B..=0x7E).contains(&byte)
}

fn ends_at_non_digit(input: &[u8], index: usize) -> bool {
    // Verified RFC Errata 4148 corrects the day, year, and time productions
    // from requiring trailing non-digit data to allowing either end-of-token
    // or `non-digit *OCTET`.
    match input.get(index) {
        Some(byte) => !byte.is_ascii_digit(),
        None => true,
    }
}

fn parse_one_or_two_digits_at(input: &[u8], index: usize) -> Option<(u8, usize)> {
    let first = input.get(index).copied()?;
    if !first.is_ascii_digit() {
        return None;
    }

    let mut value = first - b'0';
    let mut index = index + 1;

    if let Some(second) = input.get(index).copied()
        && second.is_ascii_digit()
    {
        value = value * 10 + second - b'0';
        index += 1;
    }

    Some((value, index))
}

fn parse_time(input: &[u8]) -> Option<(u8, u8, u8)> {
    // time = hms-time [ non-digit *OCTET ], after RFC Errata 4148. Each hms
    // component is 1*2 DIGIT.
    // Range validation is deliberately later, matching the RFC's sequence of
    // first finding components and then rejecting invalid parsed values.
    let (hour, input_index) = parse_one_or_two_digits_at(input, 0)?;
    if input.get(input_index) != Some(&b':') {
        return None;
    }

    let (minute, input_index) = parse_one_or_two_digits_at(input, input_index + 1)?;
    if input.get(input_index) != Some(&b':') {
        return None;
    }

    let (second, input_index) = parse_one_or_two_digits_at(input, input_index + 1)?;
    ends_at_non_digit(input, input_index).then_some((hour, minute, second))
}

fn parse_day(input: &[u8]) -> Option<u8> {
    // day-of-month = 1*2 DIGIT [ non-digit *OCTET ], after RFC Errata 4148.
    let (day, input_index) = parse_one_or_two_digits_at(input, 0)?;
    ends_at_non_digit(input, input_index).then_some(day)
}

fn parse_month(input: &[u8]) -> Option<Month> {
    // month = month-name *OCTET. RFC ABNF string literals are
    // case-insensitive, so the three-letter prefix determines the month after
    // ASCII case folding. Additional trailing octets stay inside the same token
    // and are ignored by the production.
    let [a, b, c, ..] = input else {
        return None;
    };

    match (
        a.to_ascii_lowercase(),
        b.to_ascii_lowercase(),
        c.to_ascii_lowercase(),
    ) {
        (b'j', b'a', b'n') => Some(Month::January),
        (b'f', b'e', b'b') => Some(Month::February),
        (b'm', b'a', b'r') => Some(Month::March),
        (b'a', b'p', b'r') => Some(Month::April),
        (b'm', b'a', b'y') => Some(Month::May),
        (b'j', b'u', b'n') => Some(Month::June),
        (b'j', b'u', b'l') => Some(Month::July),
        (b'a', b'u', b'g') => Some(Month::August),
        (b's', b'e', b'p') => Some(Month::September),
        (b'o', b'c', b't') => Some(Month::October),
        (b'n', b'o', b'v') => Some(Month::November),
        (b'd', b'e', b'c') => Some(Month::December),
        _ => None,
    }
}

fn parse_year(input: &[u8]) -> Option<i32> {
    // year = 2*4 DIGIT [ non-digit *OCTET ], after RFC Errata 4148. The
    // adjustment below follows the RFC's numeric year-value rules: 70..=99 map
    // to 1970..=1999, and 0..=69 map to 2000..=2069.
    let mut value = 0_u16;
    let mut index = 0;

    while index < 4 {
        let Some(byte) = input.get(index).copied() else {
            break;
        };
        if !byte.is_ascii_digit() {
            break;
        }

        value = value * 10 + (byte - b'0').widen::<u16>();
        index += 1;
    }

    if !(2..=4).contains(&index) || !ends_at_non_digit(input, index) {
        return None;
    }

    // The RFC normalizes the numeric year-value, not the original token width.
    // Therefore "0069" parses as year-value 69 and maps to 2069.
    Some(match value {
        0..=69 => value.cast_signed().widen::<i32>() + 2000,
        70..=99 => value.cast_signed().widen::<i32>() + 1900,
        _ => value.cast_signed().widen::<i32>(),
    })
}

pub(crate) fn parse(input: &[u8]) -> Result<OffsetDateTime, error::Parse> {
    let mut parsed = Parsed::default();

    for token in input
        .split(|byte| is_delimiter(*byte))
        .filter(|token| !token.is_empty())
    {
        // Section 5.1.1 checks productions in this order: time, day, month,
        // year. Once a token matches a production, the RFC says to skip the
        // remaining sub-steps and continue to the next token. This is why
        // unrelated tokens such as weekday names and time zone labels are
        // ignored rather than rejected.
        if parsed.time.is_none()
            && let Some(time) = parse_time(token)
        {
            parsed.time = Some(time);
            continue;
        }

        if parsed.day.is_none()
            && let Some(day) = parse_day(token)
        {
            parsed.day = Some(day);
            continue;
        }

        if parsed.month.is_none()
            && let Some(month) = parse_month(token)
        {
            parsed.month = Some(month);
            continue;
        }

        if parsed.year.is_none()
            && let Some(year) = parse_year(token)
        {
            parsed.year = Some(year);
        }
    }

    let year = parsed.year.ok_or_else(|| invalid_component("year"))?;
    let month = parsed.month.ok_or_else(|| invalid_component("month"))?;
    let day = parsed.day.ok_or_else(|| invalid_component("day"))?;
    let (hour, minute, second) = parsed.time.ok_or_else(|| invalid_component("hour"))?;

    // Section 5.1.1 rejects missing fields, year < 1601, out-of-range
    // day/hour/minute/second values, leap seconds, and nonexistent calendar
    // dates. Calendar validity is delegated to Date::from_calendar_date below.
    if year < 1601 {
        return Err(invalid_component("year"));
    }
    if !(1..=31).contains(&day) {
        return Err(invalid_component("day"));
    }
    if hour > 23 {
        return Err(invalid_component("hour"));
    }
    if minute > 59 {
        return Err(invalid_component("minute"));
    }
    if second > 59 {
        return Err(invalid_component("second"));
    }

    let date = Date::from_calendar_date(year, month, day).map_err(component_range)?;
    let time = Time::from_hms(hour, minute, second).map_err(component_range)?;

    // The final step says to let parsed-cookie-date be the resulting date in UTC.
    Ok(OffsetDateTime::new_in_offset(date, time, UtcOffset::UTC))
}
