//! Parse a format description into a standardized representation.

use crate::format_description::{
    component::{Component, NakedComponent},
    error::InvalidFormatDescription,
    helper, modifier, FormatDescription,
};
use alloc::vec::Vec;

/// The item parsed and remaining chunk of the format description after one iteration.
#[derive(Debug)]
struct ParsedItem<'a> {
    /// The item that was parsed.
    item: FormatDescription<'a>,
    /// What is left of the input string after the item was parsed.
    remaining: &'a str,
}

/// Parse a component from the format description. Neither the leading nor trailing bracket should
/// be present in the parameter.
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
        // Trim any whitespace between the component name and the first modifier.
        s = helper::consume_whitespace(s, index);
    } else {
        component_name = s;
        // There is no whitespace remaining, so the full input is the component name.
        s = "";
    }

    Ok(NakedComponent::parse(component_name, component_index)?
        .attach_modifiers(&modifier::Modifiers::parse(component_name, s, index)?))
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

impl<'a> FormatDescription<'a> {
    /// Parse a sequence of items from the format description.
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
    pub fn parse(mut s: &'a str) -> Result<Self, InvalidFormatDescription> {
        let mut compound = Vec::new();
        let mut loc = 0;

        while !s.is_empty() {
            let ParsedItem { item, remaining } = parse_item(s, &mut loc)?;
            s = remaining;
            compound.push(item);
        }

        Ok(FormatDescription::OwnedCompound(compound))
    }
}
