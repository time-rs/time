//! Well-known formats, typically RFCs.

#[cfg(feature = "formatting")]
use core::fmt;

use crate::error;
#[cfg(feature = "formatting")]
use crate::format_description::modifier::Padding;
use crate::format_description::FormatDescription;
#[cfg(feature = "formatting")]
use crate::formatting::format_number;
#[cfg(feature = "parsing")]
use crate::parsing::{Parsed, ParsedItem};
#[cfg(feature = "formatting")]
use crate::{Date, Time, UtcOffset};

/// The format described in [RFC 3339](https://tools.ietf.org/html/rfc3339#section-5.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rfc3339;

impl<'a> FormatDescription<'a> for Rfc3339 {
    #[cfg(feature = "formatting")]
    type FormatError = error::Format;
    #[cfg(feature = "parsing")]
    type ParseError = error::ParseFromDescription;

    #[cfg(feature = "formatting")]
    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), Self::FormatError> {
        let date = date.ok_or(error::Format::InsufficientTypeInformation)?;
        let time = time.ok_or(error::Format::InsufficientTypeInformation)?;
        let offset = offset.ok_or(error::Format::InsufficientTypeInformation)?;

        let year = date.year();

        if !(0..10_000).contains(&year) {
            return Err(error::Format::InvalidComponent("year"));
        }
        if offset.seconds != 0 {
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        format_number(output, year as u32, Padding::Zero, 4)?;
        output.write_char('-')?;
        format_number(output, date.month(), Padding::Zero, 2)?;
        output.write_char('-')?;
        format_number(output, date.day(), Padding::Zero, 2)?;
        output.write_char('T')?;
        format_number(output, time.hour, Padding::Zero, 2)?;
        output.write_char(':')?;
        format_number(output, time.minute, Padding::Zero, 2)?;
        output.write_char(':')?;
        format_number(output, time.second, Padding::Zero, 2)?;

        if time.nanosecond != 0 {
            output.write_char('.')?;

            let (value, width) = match time.nanosecond {
                nanos if nanos % 10 != 0 => (nanos, 9),
                nanos if (nanos / 10) % 10 != 0 => (nanos / 10, 8),
                nanos if (nanos / 100) % 10 != 0 => (nanos / 100, 7),
                nanos if (nanos / 1_000) % 10 != 0 => (nanos / 1_000, 6),
                nanos if (nanos / 10_000) % 10 != 0 => (nanos / 10_000, 5),
                nanos if (nanos / 100_000) % 10 != 0 => (nanos / 100_000, 4),
                nanos if (nanos / 1_000_000) % 10 != 0 => (nanos / 1_000_000, 3),
                nanos if (nanos / 10_000_000) % 10 != 0 => (nanos / 10_000_000, 2),
                nanos => (nanos / 100_000_000, 1),
            };
            format_number(output, value, Padding::Zero, width)?;
        }

        if offset == UtcOffset::UTC {
            output.write_char('Z')?;
            return Ok(());
        }

        output.write_char(if offset.hours < 0 || offset.minutes < 0 {
            '-'
        } else {
            '+'
        })?;
        format_number(output, offset.hours.abs() as u8, Padding::Zero, 2)?;
        output.write_char(':')?;
        format_number(output, offset.minutes.abs() as u8, Padding::Zero, 2)?;

        Ok(())
    }

    #[cfg(feature = "parsing")]
    fn parse_into(&self, input: &'a str, parsed: &mut Parsed) -> Result<&'a str, Self::ParseError> {
        use crate::error::ParseFromDescription::{InvalidComponent, InvalidLiteral};
        use crate::parsing::combinator::{
            any_digit, ascii_char, ascii_char_ignore_case, exactly_n_digits, n_to_m, sign,
        };

        let dash = ascii_char(b'-');
        let colon = ascii_char(b':');

        let input = exactly_n_digits(4)(input)
            .ok_or(InvalidComponent("year"))?
            .assign_value_to_with(&mut parsed.year, |year: u32| year as i32);
        let input = dash(input).ok_or(InvalidLiteral)?.unwrap();
        let input = exactly_n_digits(2)(input)
            .ok_or(InvalidComponent("month"))?
            .assign_value_to(&mut parsed.month);
        let input = dash(input).ok_or(InvalidLiteral)?.unwrap();
        let input = exactly_n_digits(2)(input)
            .ok_or(InvalidComponent("day"))?
            .assign_value_to(&mut parsed.day);
        let input = ascii_char_ignore_case(b'T')(input)
            .ok_or(InvalidLiteral)?
            .unwrap();
        let input = exactly_n_digits(2)(input)
            .ok_or(InvalidComponent("hour"))?
            .assign_value_to(&mut parsed.hour_24);
        let input = colon(input).ok_or(InvalidLiteral)?.unwrap();
        let input = exactly_n_digits(2)(input)
            .ok_or(InvalidComponent("minute"))?
            .assign_value_to(&mut parsed.minute);
        let input = colon(input).ok_or(InvalidLiteral)?.unwrap();
        let input = exactly_n_digits(2)(input)
            .ok_or(InvalidComponent("second"))?
            .assign_value_to(&mut parsed.second);
        let input = if let Some(ParsedItem(input, ())) = ascii_char(b'.')(input) {
            let ParsedItem(mut input, raw_digits) =
                n_to_m(1, 9, any_digit)(input).ok_or(InvalidComponent("subsecond"))?;

            // Consume any remaining digits as allowed by the spec. They are discarded, as we only
            // have nanosecond precision.
            while let Some(ParsedItem(new_input, _)) = any_digit(input) {
                input = new_input;
            }

            let raw_num: u32 = raw_digits
                .parse()
                .map_err(|_| InvalidComponent("subsecond"))?;
            let adjustment_factor = 10_u32.pow(9 - raw_digits.len() as u32);
            ParsedItem(input, raw_num * adjustment_factor).assign_value_to(&mut parsed.subsecond)
        } else {
            input
        };

        if let Some(ParsedItem(input, ())) = ascii_char_ignore_case(b'Z')(input) {
            parsed.offset_hour = Some(0);
            parsed.offset_minute = Some(0);
            parsed.offset_second = Some(0);
            return Ok(input);
        }

        let ParsedItem(input, offset_sign) = sign(input).ok_or(InvalidComponent("offset_hour"))?;
        let input = exactly_n_digits(2)(input)
            .ok_or(InvalidComponent("offset_hour"))?
            .assign_value_to_with(&mut parsed.offset_hour, |offset_hour: u8| {
                if offset_sign == '-' {
                    -(offset_hour as i8)
                } else {
                    offset_hour as _
                }
            });
        let input = colon(input).ok_or(InvalidLiteral)?.unwrap();
        let input = exactly_n_digits(2)(input)
            .ok_or(InvalidComponent("offset_minute"))?
            .assign_value_to(&mut parsed.offset_minute);

        Ok(input)
    }
}
