use std::iter::Peekable;
use std::str::Chars;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::helpers::{self, consume_char, consume_digits, consume_str};
use crate::{Error, ToTokens};

#[derive(Clone, Copy)]
pub(crate) struct Offset {
    pub(crate) hours: i8,
    pub(crate) minutes: i8,
    pub(crate) seconds: i8,
}

impl Offset {
    pub(crate) fn parse(chars: &mut Peekable<Chars<'_>>) -> Result<Self, Error> {
        if consume_str("utc", chars).is_ok() || consume_str("UTC", chars).is_ok() {
            return Ok(Self {
                hours: 0,
                minutes: 0,
                seconds: 0,
            });
        }

        let sign = match chars.next() {
            Some('+') => 1,
            Some('-') => -1,
            Some(char) => return Err(Error::UnexpectedCharacter(char)),
            None => return Err(Error::MissingComponent { name: "sign" }),
        };

        let hours = consume_digits::<i8>("hour", chars)?;
        let mut minutes = 0;
        let mut seconds = 0;

        if consume_char(':', chars).is_ok() {
            minutes = consume_digits::<i8>("minute", chars)?;

            if consume_char(':', chars).is_ok() {
                seconds = consume_digits::<i8>("second", chars)?;
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
    fn to_internal_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("UtcOffset", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("from_hms_unchecked", Span::call_site())),
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
        );
    }

    fn to_external_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(helpers::const_block(
            self.to_internal_token_stream(),
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
