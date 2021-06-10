use std::iter::Peekable;

use proc_macro::{
    token_stream, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::helpers::{self, consume_ident, consume_number, consume_punct};
use crate::{Error, ToTokens};

enum Period {
    Am,
    Pm,
    _24,
}

#[derive(Clone, Copy)]
pub(crate) struct Time {
    pub(crate) hour: u8,
    pub(crate) minute: u8,
    pub(crate) second: u8,
    pub(crate) nanosecond: u32,
}

impl Time {
    pub(crate) fn parse(chars: &mut Peekable<token_stream::IntoIter>) -> Result<Self, Error> {
        let hour = consume_number("hour", chars)?;
        consume_punct(':', chars)?;
        let minute = consume_number::<u8>("minute", chars)?;
        let second: f64 = if consume_punct(':', chars).is_ok() {
            consume_number("second", chars)?
        } else {
            0.
        };
        let period = if consume_ident("am", chars).is_ok() || consume_ident("AM", chars).is_ok() {
            Period::Am
        } else if consume_ident("pm", chars).is_ok() || consume_ident("PM", chars).is_ok() {
            Period::Pm
        } else {
            Period::_24
        };

        let hour = match (hour, period) {
            (12, Period::Am) => 0,
            (12, Period::Pm) => 12,
            (hour, Period::Am) | (hour, Period::_24) => hour,
            (hour, Period::Pm) => hour + 12,
        };

        if hour >= 24 {
            Err(Error::InvalidComponent {
                name: "hour",
                value: hour.to_string(),
            })
        } else if minute >= 60 {
            Err(Error::InvalidComponent {
                name: "minute",
                value: minute.to_string(),
            })
        } else if second >= 60. {
            Err(Error::InvalidComponent {
                name: "second",
                value: second.to_string(),
            })
        } else {
            Ok(Self {
                hour,
                minute,
                second: second.trunc() as _,
                nanosecond: (second.fract() * 1_000_000_000.).round() as _,
            })
        }
    }
}

impl ToTokens for Time {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(helpers::const_block(
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("Time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("__from_hms_nanos_unchecked", Span::call_site())),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    [
                        TokenTree::Literal(Literal::u8_unsuffixed(self.hour)),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Literal(Literal::u8_unsuffixed(self.minute)),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Literal(Literal::u8_unsuffixed(self.second)),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Literal(Literal::u32_unsuffixed(self.nanosecond)),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                )),
            ]
            .iter()
            .cloned()
            .collect::<TokenStream>(),
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("Time", Span::call_site())),
            ]
            .iter()
            .cloned()
            .collect(),
        ));
    }
}
