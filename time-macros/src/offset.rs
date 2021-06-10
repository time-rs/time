use std::iter::Peekable;

use proc_macro::{
    token_stream, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::helpers::{self, consume_ident, consume_number, consume_punct};
use crate::{Error, ToTokens};

#[derive(Clone, Copy)]
pub(crate) struct Offset {
    pub(crate) hours: i8,
    pub(crate) minutes: i8,
    pub(crate) seconds: i8,
}

impl Offset {
    pub(crate) fn parse(chars: &mut Peekable<token_stream::IntoIter>) -> Result<Self, Error> {
        if consume_ident("utc", chars).is_ok() || consume_ident("UTC", chars).is_ok() {
            return Ok(Self {
                hours: 0,
                minutes: 0,
                seconds: 0,
            });
        }

        let sign = if consume_punct('+', chars).is_ok() {
            1
        } else if consume_punct('-', chars).is_ok() {
            -1
        } else if let Some(tree) = chars.next() {
            return Err(Error::UnexpectedToken { tree });
        } else {
            return Err(Error::MissingComponent { name: "sign" });
        };

        let hours = consume_number::<i8>("hour", chars)?;
        let mut minutes = 0;
        let mut seconds = 0;

        if consume_punct(':', chars).is_ok() {
            minutes = consume_number::<i8>("minute", chars)?;

            if consume_punct(':', chars).is_ok() {
                seconds = consume_number::<i8>("second", chars)?;
            }
        }

        if hours >= 24 {
            Err(Error::InvalidComponent {
                name: "hour",
                value: hours.to_string(),
            })
        } else if minutes >= 60 {
            Err(Error::InvalidComponent {
                name: "minute",
                value: minutes.to_string(),
            })
        } else if seconds >= 60 {
            Err(Error::InvalidComponent {
                name: "second",
                value: seconds.to_string(),
            })
        } else {
            Ok(Self {
                hours: sign * hours,
                minutes: sign * minutes,
                seconds: sign * seconds,
            })
        }
    }
}

impl ToTokens for Offset {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(helpers::const_block(
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("UtcOffset", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("__from_hms_unchecked", Span::call_site())),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    [
                        TokenTree::Literal(Literal::i8_unsuffixed(self.hours)),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Literal(Literal::i8_unsuffixed(self.minutes)),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Literal(Literal::i8_unsuffixed(self.seconds)),
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
                TokenTree::Ident(Ident::new("UtcOffset", Span::call_site())),
            ]
            .iter()
            .cloned()
            .collect(),
        ));
    }
}
