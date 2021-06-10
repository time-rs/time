use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::format_description::error::InvalidFormatDescription;
use crate::format_description::modifier;
use crate::format_description::modifier::Modifiers;
use crate::ToTokens;

pub(crate) enum Component {
    Day(modifier::Day),
    Month(modifier::Month),
    Ordinal(modifier::Ordinal),
    Weekday(modifier::Weekday),
    WeekNumber(modifier::WeekNumber),
    Year(modifier::Year),
    Hour(modifier::Hour),
    Minute(modifier::Minute),
    Period(modifier::Period),
    Second(modifier::Second),
    Subsecond(modifier::Subsecond),
    OffsetHour(modifier::OffsetHour),
    OffsetMinute(modifier::OffsetMinute),
    OffsetSecond(modifier::OffsetSecond),
}

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (name, inner_tokens) = match self {
            Self::Day(modifier) => ("Day", modifier.to_token_stream()),
            Self::Month(modifier) => ("Month", modifier.to_token_stream()),
            Self::Ordinal(modifier) => ("Ordinal", modifier.to_token_stream()),
            Self::Weekday(modifier) => ("Weekday", modifier.to_token_stream()),
            Self::WeekNumber(modifier) => ("WeekNumber", modifier.to_token_stream()),
            Self::Year(modifier) => ("Year", modifier.to_token_stream()),
            Self::Hour(modifier) => ("Hour", modifier.to_token_stream()),
            Self::Minute(modifier) => ("Minute", modifier.to_token_stream()),
            Self::Period(modifier) => ("Period", modifier.to_token_stream()),
            Self::Second(modifier) => ("Second", modifier.to_token_stream()),
            Self::Subsecond(modifier) => ("Subsecond", modifier.to_token_stream()),
            Self::OffsetHour(modifier) => ("OffsetHour", modifier.to_token_stream()),
            Self::OffsetMinute(modifier) => ("OffsetMinute", modifier.to_token_stream()),
            Self::OffsetSecond(modifier) => ("OffsetSecond", modifier.to_token_stream()),
        };

        tokens.extend(
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::mixed_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("format_description", Span::mixed_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("Component", Span::mixed_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new(name, Span::mixed_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, inner_tokens)),
            ]
            .iter()
            .cloned()
            .collect::<TokenStream>(),
        );
    }
}

pub(crate) enum NakedComponent {
    Day,
    Month,
    Ordinal,
    Weekday,
    WeekNumber,
    Year,
    Hour,
    Minute,
    Period,
    Second,
    Subsecond,
    OffsetHour,
    OffsetMinute,
    OffsetSecond,
}

impl NakedComponent {
    pub(crate) fn parse(
        component_name: &str,
        component_index: usize,
    ) -> Result<Self, InvalidFormatDescription> {
        match component_name {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            "ordinal" => Ok(Self::Ordinal),
            "weekday" => Ok(Self::Weekday),
            "week_number" => Ok(Self::WeekNumber),
            "year" => Ok(Self::Year),
            "hour" => Ok(Self::Hour),
            "minute" => Ok(Self::Minute),
            "period" => Ok(Self::Period),
            "second" => Ok(Self::Second),
            "subsecond" => Ok(Self::Subsecond),
            "offset_hour" => Ok(Self::OffsetHour),
            "offset_minute" => Ok(Self::OffsetMinute),
            "offset_second" => Ok(Self::OffsetSecond),
            "" => Err(InvalidFormatDescription::MissingComponentName {
                index: component_index,
            }),
            _ => Err(InvalidFormatDescription::InvalidComponentName {
                name: component_name.to_owned(),
                index: component_index,
            }),
        }
    }

    pub(crate) fn attach_modifiers(self, modifiers: Modifiers) -> Component {
        match self {
            Self::Day => Component::Day(modifier::Day {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::Month => Component::Month(modifier::Month {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.month_repr.unwrap_or_default(),
            }),
            Self::Ordinal => Component::Ordinal(modifier::Ordinal {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::Weekday => Component::Weekday(modifier::Weekday {
                repr: modifiers.weekday_repr.unwrap_or_default(),
                one_indexed: modifiers.weekday_is_one_indexed.unwrap_or(true),
            }),
            Self::WeekNumber => Component::WeekNumber(modifier::WeekNumber {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.week_number_repr.unwrap_or_default(),
            }),
            Self::Year => Component::Year(modifier::Year {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.year_repr.unwrap_or_default(),
                iso_week_based: modifiers.year_is_iso_week_based.unwrap_or_default(),
                sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
            }),
            Self::Hour => Component::Hour(modifier::Hour {
                padding: modifiers.padding.unwrap_or_default(),
                is_12_hour_clock: modifiers.hour_is_12_hour_clock.unwrap_or_default(),
            }),
            Self::Minute => Component::Minute(modifier::Minute {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::Period => Component::Period(modifier::Period {
                is_uppercase: modifiers.period_is_uppercase.unwrap_or(true),
            }),
            Self::Second => Component::Second(modifier::Second {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::Subsecond => Component::Subsecond(modifier::Subsecond {
                digits: modifiers.subsecond_digits.unwrap_or_default(),
            }),
            Self::OffsetHour => Component::OffsetHour(modifier::OffsetHour {
                sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::OffsetMinute => Component::OffsetMinute(modifier::OffsetMinute {
                padding: modifiers.padding.unwrap_or_default(),
            }),
            Self::OffsetSecond => Component::OffsetSecond(modifier::OffsetSecond {
                padding: modifiers.padding.unwrap_or_default(),
            }),
        }
    }
}
