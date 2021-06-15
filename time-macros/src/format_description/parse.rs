use core::mem;

use proc_macro::Span;

use crate::format_description::component::{Component, NakedComponent};
use crate::format_description::error::InvalidFormatDescription;
use crate::format_description::{helper, modifier, FormatItem};
use crate::Error;

struct ParsedItem<'a> {
    item: FormatItem<'a>,
    remaining: &'a str,
}

fn parse_component(mut s: &str, index: &mut usize) -> Result<Component, InvalidFormatDescription> {
    s = helper::consume_whitespace(s, index);

    let component_name;
    let component_index = *index;
    if let Some(whitespace_loc) = s.find(char::is_whitespace) {
        *index += whitespace_loc;
        component_name = &s[..whitespace_loc];
        s = &s[whitespace_loc..];
        s = helper::consume_whitespace(s, index);
    } else {
        *index += s.len();
        component_name = mem::take(&mut s);
    }

    Ok(NakedComponent::parse(component_name, component_index)?
        .attach_modifiers(modifier::Modifiers::parse(component_name, s, index)?))
}

fn parse_literal<'a>(s: &'a str, index: &mut usize) -> ParsedItem<'a> {
    let loc = s.find('[').unwrap_or_else(|| s.len());
    *index += loc;
    ParsedItem {
        item: FormatItem::Literal(&s[..loc]),
        remaining: &s[loc..],
    }
}

#[allow(clippy::manual_strip)]
fn parse_item<'a>(
    s: &'a str,
    index: &mut usize,
) -> Result<ParsedItem<'a>, InvalidFormatDescription> {
    if let Some(remaining) = s.strip_prefix("[[") {
        *index += 2;
        return Ok(ParsedItem {
            item: FormatItem::Literal("["),
            remaining,
        });
    }

    if s.starts_with('[') {
        if let Some(bracket_index) = s.find(']') {
            *index += 1;
            let ret_val = ParsedItem {
                item: FormatItem::Component(parse_component(&s[1..bracket_index], index)?),
                remaining: &s[bracket_index + 1..],
            };
            *index += 1;
            Ok(ret_val)
        } else {
            Err(InvalidFormatDescription::UnclosedOpeningBracket { index: *index })
        }
    } else {
        Ok(parse_literal(s, index))
    }
}

pub(crate) fn parse(mut s: &str, span: Span) -> Result<Vec<FormatItem<'_>>, Error> {
    let mut compound = Vec::new();
    let mut loc = 0;

    while !s.is_empty() {
        let ParsedItem { item, remaining } =
            parse_item(s, &mut loc).map_err(|error| Error::InvalidFormatDescription {
                error,
                span_start: Some(span),
                span_end: Some(span),
            })?;
        s = remaining;
        compound.push(item);
    }

    Ok(compound)
}
