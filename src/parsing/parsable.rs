//! A trait that can be used to parse an item from an input.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::error;
use crate::format_description::{well_known, FormatItem};
use crate::parsing::{Parsed, ParsedItem};

/// Seal the trait to prevent downstream users from implementing it, while still allowing it to
/// exist in generic bounds.
pub(crate) mod sealed {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Parse the item using a format description and an input.
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
    pub trait Parsable {
        /// An error that may be returned when parsing.
        type Error: Into<error::Parse>;

        /// Parse the item into the provided [`Parsed`] struct.
        ///
        /// This method can be used to parse part of a type without parsing the full value.
        fn parse_into<'a>(
            &self,
            input: &'a str,
            parsed: &mut Parsed,
        ) -> Result<&'a str, Self::Error>;

        /// Parse the item into a new [`Parsed`] struct.
        ///
        /// This method can only be used to parse a complete value of a type. If any characters
        /// remain after parsing, an error will be returned.
        fn parse(&self, input: &str) -> Result<Parsed, error::Parse> {
            let mut parsed = Parsed::new();
            let remaining = match self.parse_into(input, &mut parsed) {
                Ok(value) => value,
                Err(err) => return Err(err.into()),
            };
            if remaining.is_empty() {
                Ok(parsed)
            } else {
                Err(error::Parse::UnexpectedTrailingCharacters)
            }
        }
    }
}

impl sealed::Parsable for FormatItem<'_> {
    type Error = error::ParseFromDescription;

    fn parse_into<'a>(
        &self,
        mut input: &'a str,
        parsed: &mut Parsed,
    ) -> Result<&'a str, Self::Error> {
        match self {
            Self::Literal(literal) => {
                input = input
                    .strip_prefix(literal)
                    .ok_or(error::ParseFromDescription::InvalidLiteral)?;
            }
            Self::Component(component) => input = parsed.parse_component(input, *component)?,
            Self::Compound(compound) => input = compound.parse_into(input, parsed)?,
        }
        Ok(input)
    }
}

impl sealed::Parsable for &[FormatItem<'_>] {
    type Error = error::ParseFromDescription;

    fn parse_into<'a>(
        &self,
        mut input: &'a str,
        parsed: &mut Parsed,
    ) -> Result<&'a str, Self::Error> {
        for item in self.iter() {
            input = item.parse_into(input, parsed)?;
        }
        Ok(input)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl sealed::Parsable for Vec<FormatItem<'_>> {
    type Error = error::ParseFromDescription;

    fn parse_into<'a>(&self, input: &'a str, parsed: &mut Parsed) -> Result<&'a str, Self::Error> {
        self.as_slice().parse_into(input, parsed)
    }
}

impl sealed::Parsable for well_known::Rfc3339 {
    type Error = error::ParseFromDescription;

    fn parse_into<'a>(&self, input: &'a str, parsed: &mut Parsed) -> Result<&'a str, Self::Error> {
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
