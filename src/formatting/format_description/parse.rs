//! Parse a format description into a standardized representation.

use crate::formatting::format_description::{
    error::InvalidFormatDescription, helper, modifier, Component, FormatDescription,
};
use alloc::{borrow::ToOwned, vec::Vec};

/// The item parsed and remaining chunk of the format description after one
/// iteration.
#[derive(Debug)]
struct ParsedItem<'a> {
    /// The item that was parsed.
    item: FormatDescription<'a>,
    /// What is left of the input string after the item was parsed.
    remaining: &'a str,
}

/// Parse a component from the format description. Neither the leading nor
/// trailing bracket should be present in the parameter.
fn parse_component<'a>(
    mut s: &'a str,
    index: &mut usize,
) -> Result<Component, InvalidFormatDescription> {
    // Trim any whitespace between the opening bracket and the component name.
    s = helper::consume_whitespace(s, index);

    // Everything before the first whitespace is the component name.
    let component_name;
    let component_index = *index;
    if let Some(whitespace_loc) = s.find(char::is_whitespace) {
        *index += whitespace_loc;
        component_name = &s[..whitespace_loc];
        s = &s[whitespace_loc..];
        // Trim any whitespace between the component name and the first
        // modifier.
        s = helper::consume_whitespace(s, index);
    } else {
        component_name = s;
        // There is no whitespace remaining, so the full input is the component
        // name.
        s = "";
    }

    match component_name {
        "day" | "hour" | "minute" | "month" | "offset_hour" | "offset_minute" | "offset_second"
        | "ordinal" | "period" | "second" | "subsecond" | "weekday" | "week_number" | "year" => {}
        name => {
            return Err(InvalidFormatDescription::InvalidComponentName {
                name: name.to_owned(),
                index: component_index,
            })
        }
    }

    let modifiers = modifier::ParsedModifiers::parse(component_name, s, index)?;
    Ok(match component_name {
        "day" => Component::Day {
            padding: modifiers.padding.unwrap_or_default(),
        },
        "hour" => Component::Hour {
            padding: modifiers.padding.unwrap_or_default(),
            is_12_hour_clock: modifiers.hour_is_12_hour_clock.unwrap_or_default(),
        },
        "minute" => Component::Minute {
            padding: modifiers.padding.unwrap_or_default(),
        },
        "month" => Component::Month {
            padding: modifiers.padding.unwrap_or_default(),
            repr: modifiers.month_repr.unwrap_or_default(),
        },
        "offset_hour" => Component::OffsetHour {
            sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
            padding: modifiers.padding.unwrap_or_default(),
        },
        "offset_minute" => Component::OffsetMinute {
            padding: modifiers.padding.unwrap_or_default(),
        },
        "offset_second" => Component::OffsetSecond {
            padding: modifiers.padding.unwrap_or_default(),
        },
        "ordinal" => Component::Ordinal {
            padding: modifiers.padding.unwrap_or_default(),
        },
        "period" => Component::Period {
            is_uppercase: modifiers.period_is_uppercase.unwrap_or(true),
        },
        "second" => Component::Second {
            padding: modifiers.padding.unwrap_or_default(),
        },
        "subsecond" => Component::Subsecond {
            digits: modifiers.subsecond_digits.unwrap_or_default(),
        },
        "weekday" => Component::Weekday {
            repr: modifiers.weekday_repr.unwrap_or_default(),
            one_indexed: modifiers.weekday_is_one_indexed.unwrap_or(true),
        },
        "week_number" => Component::WeekNumber {
            padding: modifiers.padding.unwrap_or_default(),
            repr: modifiers.week_number_repr.unwrap_or_default(),
        },
        "year" => Component::Year {
            padding: modifiers.padding.unwrap_or_default(),
            repr: modifiers.year_repr.unwrap_or_default(),
            iso_week_based: modifiers.year_is_iso_week_based.unwrap_or_default(),
            sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
        },
        _ => unreachable!(
            "All valid component names should be caught in the above `matches!` clause."
        ),
    })
}

/// Parse a literal string from the format description.
#[allow(clippy::option_if_let_else)] // I think this style is better here.
fn parse_literal<'a>(s: &'a str, index: &mut usize) -> ParsedItem<'a> {
    let loc = s.find('[').unwrap_or_else(|| s.len());
    *index += loc;
    ParsedItem {
        item: FormatDescription::Literal(&s[..loc]),
        remaining: &s[loc..],
    }
}

/// Parse either a literal or a component from the format description.
#[allow(clippy::manual_strip)] // lint was not designed for this case
fn parse_item<'a>(
    s: &'a str,
    index: &mut usize,
) -> Result<ParsedItem<'a>, InvalidFormatDescription> {
    if s.starts_with("[[") {
        *index += 2;
        return Ok(ParsedItem {
            item: FormatDescription::Literal(&s[..1]),
            remaining: &s[2..],
        });
    }

    if s.starts_with('[') {
        if let Some(bracket_index) = s.find(']') {
            *index += 1;
            Ok(ParsedItem {
                item: FormatDescription::Component(parse_component(&s[1..bracket_index], index)?),
                remaining: &s[bracket_index + 1..],
            })
        } else {
            Err(InvalidFormatDescription::UnclosedOpeningBracket { index: *index })
        }
    } else {
        Ok(parse_literal(s, index))
    }
}

/// Parse a sequence of items from the format description.
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
#[allow(clippy::module_name_repetitions)]
pub fn parse_format_description(
    mut s: &str,
) -> Result<Vec<FormatDescription<'_>>, InvalidFormatDescription> {
    let mut compound = Vec::new();
    let mut loc = 0;

    while !s.is_empty() {
        let ParsedItem { item, remaining } = parse_item(s, &mut loc)?;
        s = remaining;
        compound.push(item);
    }

    Ok(compound)
}
